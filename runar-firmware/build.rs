use std::path::PathBuf;

fn main() {
    let linker_script = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    )
    .join("link.ld");
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
}