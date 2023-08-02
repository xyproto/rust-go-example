use std::process::Command;
use std::path::PathBuf;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir.clone());

    let go_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src").join("add.go");

    // Build the Go library
    let status = Command::new("go")
        .args(&["build", "-o", out_path.join("libadd.so").to_str().unwrap(), "-buildmode=c-shared", go_file.to_str().unwrap()])
        .status()
        .expect("Failed to build the Go library");

    if !status.success() {
        panic!("Could not build the Go library");
    }

    // Link the library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=add");
}
