#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    for &(src, dst) in &[
        ("src/lib.rs", "lib.rs"),
    ] {
        let src = Path::new(src);
        let dst = Path::new(&out_dir).join(dst);

        serde_codegen::expand(&src, &dst).unwrap();
    }
}
#[cfg(not(feature = "serde_codegen"))]
fn main() { }
