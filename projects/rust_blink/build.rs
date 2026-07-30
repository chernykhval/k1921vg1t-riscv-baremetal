// build.rs — скрипт, который cargo запускает ПЕРЕД сборкой (на хосте).
// Его задача — добавить папку проекта в пути поиска линкера, чтобы
// нашёлся наш link.x, указанный в .cargo/config.toml.
use std::env;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={dir}");
    // пересобрать, если link.x изменился
    println!("cargo:rerun-if-changed=link.x");
}
