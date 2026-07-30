// build.rs — кладёт наш memory.x туда, где линкер его найдёт.
// riscv-rt's link.x делает `INCLUDE memory.x`, значит memory.x должен быть
// на пути поиска линкера. Копируем его в OUT_DIR и добавляем этот путь.
use std::{env, fs, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
