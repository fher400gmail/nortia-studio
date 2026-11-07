//! La miniatura de las fichas (primer fotograma con ffmpeg) solo puede usar componentes de la
//! lista blanca del sidecar LGPL. Complementa `masterout-core/tests/sidecar_allowlist.rs`,
//! que cubre el resto de constructores de argumentos.

#[path = "../../masterout-core/tests/allowlist/mod.rs"]
mod allowlist;

use mastercheck_bridge::thumbnail::build_thumbnail_args;
use std::path::Path;

#[test]
fn thumbnail_args_only_use_components_in_the_sidecar_allowlist() {
    let a = allowlist::load();
    for src in ["/t/a.mp4", "/t/a.mov", "/t/a.webp", "/t/a.mkv", "/t/a.webm"] {
        let args = build_thumbnail_args(Path::new(src), Path::new("/t/out.png"));
        let bad = allowlist::violations("miniatura", &args, &a, &a.encoders_macos);
        assert!(bad.is_empty(), "{}", bad.join("\n"));
        let bad = allowlist::violations("miniatura", &args, &a, &a.encoders_windows);
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }
}
