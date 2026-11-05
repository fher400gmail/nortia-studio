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

/// Borra un temporal con reintentos. Un fallo definitivo queda en el registro (stderr solo en
/// depuración): no puede romper la exportación, pero ya no desaparece en silencio.
pub fn remove_temp(path: &Path) {
    if let Err(e) = remove_with_retry(path, REMOVE_ATTEMPTS, REMOVE_WAIT, |p| {
        std::fs::remove_file(p)
    }) {
        #[cfg(debug_assertions)]
        eprintln!("no se pudo borrar el temporal {}: {e}", path.display());
        #[cfg(not(debug_assertions))]
        let _ = e;
    }
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
        remove_temp(&f);
        assert!(!f.exists());
        remove_temp(&f); // ya no existe: sin ruido ni pánico
    }
}
