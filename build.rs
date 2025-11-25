use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/c/quadrature.c")
        .compile("tokamak");

    let binding = bindgen::Builder::default()
        .header("src/c/quadrature.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Can not generate binding");
    let path_out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("binding.rs");
    binding
        .write_to_file(path_out)
        .expect("Can not write binding");
}
