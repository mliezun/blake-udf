extern crate bindgen;
extern crate cbindgen;

use cbindgen::{Builder, Config};
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn execute_cbindgen() {
    let crate_env = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_path = Path::new(&crate_env);
    let config = Config::from_root_or_default(crate_path);
    Builder::new()
        .with_crate(crate_path.to_str().unwrap())
        .with_config(config)
        .generate()
        .expect("Cannot generate header file!")
        .write_to_file("outputs/blake_udf.h");
}

fn execute_bindgen() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=include/udf_registration_types.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/udf_registration_types.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    execute_bindgen();
    execute_cbindgen();
}
