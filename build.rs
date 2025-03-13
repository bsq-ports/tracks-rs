#[cfg(feature = "ffi")]
extern crate cbindgen;

fn main() {
    #[cfg(feature = "ffi")]
    {
        use std::env;
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        println!("crate_dir: {}", crate_dir);

        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .with_language(cbindgen::Language::C)
            .with_namespaces(&["Tracks", "ffi"])
            .with_cpp_compat(true)
            .with_pragma_once(true)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("./shared/bindings.h");
    }
}
