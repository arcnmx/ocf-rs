#[cfg(feature = "syntex")]
fn main() {
    extern crate syntex;
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    for &(src, dst) in &[
        ("src/lib.rs", "lib.rs"),
    ] {
        let src = Path::new(src);
        let dst = Path::new(&out_dir).join(dst);

        let mut registry = syntex::Registry::new();

        serde_codegen::register(&mut registry);
        registry.expand("ocf", &src, &dst).unwrap();
    }
}
#[cfg(not(feature = "syntex"))]
fn main() { }
