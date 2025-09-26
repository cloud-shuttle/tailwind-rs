use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to rerun this build script if any of these files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=../../crates/");

    // Set up the output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dist_dir = out_dir.join("../../dist");

    // Create dist directory if it doesn't exist
    std::fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");

    // Copy the HTML file to the dist directory
    let html_src = "index.html";
    let html_dst = dist_dir.join("index.html");
    std::fs::copy(html_src, html_dst).expect("Failed to copy index.html");

    // Note: WASM_BIGINT is a wasm-opt flag, not a linker flag
    // It should be handled by wasm-pack or wasm-opt during the build process
    // Removing these incorrect linker arguments
}
