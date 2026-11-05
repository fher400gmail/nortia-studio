//! Borrado de temporales que no ignora los errores: en Windows un archivo puede seguir abierto
//! un instante tras `wait()` del proceso hijo (antivirus, indexador) y `remove_file` falla con
//! «acceso denegado»; se reintenta unas pocas veces antes de rendirse.

use std::io;
use std::path::Path;
use std::time::Duration;

/// Intentos totales y espera entre ellos (≈ 0,5 s en el peor caso).
pub const REMOVE_ATTEMPTS: u32 = 6;
pub const REMOVE_WAIT: Duration = Duration::from_millis(100);

/// Borra `path` con `remove`, reintentando mientras falle. Que el archivo ya no exista cuenta
/// como éxito. Devuelve el último error si se agotan los intentos.
pub fn remove_with_retry(
    path: &Path,
    attempts: u32,
    wait: Duration,
    mut remove: impl FnMut(&Path) -> io::Result<()>,
) -> io::Result<()> {
    let mut last = None;
    for n in 0..attempts.max(1) {
        match remove(path) {
            Ok(()) => return Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => last = Some(e),
        }
        if n + 1 < attempts {
            std::thread::sleep(wait);
        }
    }
    Err(last.unwrap_or_else(|| io::Error::other("remove_with_retry")))
}

type FailureHook = Box<dyn Fn(&Path, &io::Error) + Send + Sync>;
static FAILURE_HOOK: std::sync::OnceLock<FailureHook> = std::sync::OnceLock::new();

/// Registra quién anota los borrados que fallan del todo (la app lo apunta en su registro de
/// usuario). Solo se acepta la primera vez.
pub fn set_failure_hook(hook: impl Fn(&Path, &io::Error) + Send + Sync + 'static) {
    let _ = FAILURE_HOOK.set(Box::new(hook));
}

/// Igual que `remove_temp` pero con intentos, espera, borrado y aviso inyectables.
pub fn remove_temp_with(
    path: &Path,
    attempts: u32,
    wait: Duration,
    remove: impl FnMut(&Path) -> io::Result<()>,
    report: impl FnOnce(&Path, &io::Error),
) -> io::Result<()> {
    let r = remove_with_retry(path, attempts, wait, remove);
    if let Err(e) = &r {
        report(path, e);
    }
    r
}

/// Borra un temporal con reintentos. Un fallo definitivo se anota con el gancho de registro y se
/// devuelve; quien limpia dentro de un `Drop` o de un camino de error no puede propagarlo, pero ya
/// no se pierde en silencio.
pub fn remove_temp(path: &Path) -> io::Result<()> {
    remove_temp_with(
        path,
        REMOVE_ATTEMPTS,
        REMOVE_WAIT,
        |p| std::fs::remove_file(p),
        |p, e| {
            if let Some(h) = FAILURE_HOOK.get() {
                h(p, e);
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn retries_until_the_file_can_be_removed() {
        let calls = Cell::new(0);
        let r = remove_with_retry(Path::new("x.part"), 5, Duration::ZERO, |_| {
            calls.set(calls.get() + 1);
            if calls.get() < 3 {
                Err(io::Error::from(io::ErrorKind::PermissionDenied))
            } else {
                Ok(())
            }
        });
        assert!(r.is_ok());
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn gives_up_with_the_last_error_and_missing_counts_as_done() {
        let calls = Cell::new(0);
        let r = remove_with_retry(Path::new("x.part"), 4, Duration::ZERO, |_| {
            calls.set(calls.get() + 1);
            Err(io::Error::from(io::ErrorKind::PermissionDenied))
        });
        assert_eq!(r.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(calls.get(), 4);
        let r = remove_with_retry(Path::new("x.part"), 4, Duration::ZERO, |_| {
            Err(io::Error::from(io::ErrorKind::NotFound))
        });
        assert!(r.is_ok());
    }

    #[test]
    fn removes_a_real_file_and_tolerates_a_missing_one() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("a.part");
        std::fs::write(&f, b"x").unwrap();
        assert!(remove_temp(&f).is_ok());
        assert!(!f.exists());
        assert!(remove_temp(&f).is_ok()); // ya no existe: sin ruido ni pánico
    }

    #[test]
    fn a_definitive_failure_is_reported_once_and_returned() {
        let reported = Cell::new(0);
        let r = remove_temp_with(
            Path::new("x.part"),
            3,
            Duration::ZERO,
            |_| Err(io::Error::from(io::ErrorKind::PermissionDenied)),
            |p, e| {
                reported.set(reported.get() + 1);
                assert_eq!(p, Path::new("x.part"));
                assert_eq!(e.kind(), io::ErrorKind::PermissionDenied);
            },
        );
        assert_eq!(r.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(reported.get(), 1);
        let r = remove_temp_with(
            Path::new("x.part"),
            3,
            Duration::ZERO,
            |_| Ok(()),
            |_, _| reported.set(99),
        );
        assert!(r.is_ok());
        assert_eq!(reported.get(), 1, "un éxito no se anota");
    }
}
