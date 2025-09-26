//! TDD Investigation for WASM_BIGINT Issue
//!
//! This test systematically investigates the WASM_BIGINT linker error
//! using cargo nextest for better error reporting and faster builds.

use std::fs;
use std::process::Command;

/// Test 1: Reproduce the WASM_BIGINT error with detailed output
#[test]
fn reproduce_wasm_bigint_error() {
    println!("🔍 Investigating WASM_BIGINT linker error...");

    let output = Command::new("wasm-pack")
        .args(&["build", "--target", "web", "--out-dir", "pkg-test"])
        .current_dir(".")
        .output()
        .expect("Failed to execute wasm-pack");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("📊 wasm-pack exit code: {}", output.status);
    println!("📝 STDOUT:\n{}", stdout);
    println!("❌ STDERR:\n{}", stderr);

    // This test documents the current error state
    if stderr.contains("WASM_BIGINT") {
        println!("✅ Successfully reproduced WASM_BIGINT error");
    } else if output.status.success() {
        println!("🎉 WASM_BIGINT error resolved!");
    } else {
        println!("⚠️ Different error encountered: {}", stderr);
    }
}

/// Test 2: Check wasm-pack version and configuration
#[test]
fn check_wasm_pack_environment() {
    println!("🔧 Checking wasm-pack environment...");

    // Check wasm-pack version
    let version_output = Command::new("wasm-pack")
        .args(&["--version"])
        .output()
        .expect("Failed to get wasm-pack version");

    let version = String::from_utf8_lossy(&version_output.stdout);
    println!("📦 wasm-pack version: {}", version);

    // Check Rust toolchain
    let rustc_output = Command::new("rustc")
        .args(&["--version"])
        .output()
        .expect("Failed to get rustc version");

    let rustc_version = String::from_utf8_lossy(&rustc_output.stdout);
    println!("🦀 rustc version: {}", rustc_version);

    // Check if wasm32 target is installed
    let target_output = Command::new("rustup")
        .args(&["target", "list", "--installed"])
        .output()
        .expect("Failed to list installed targets");

    let targets = String::from_utf8_lossy(&target_output.stdout);
    println!("🎯 Installed targets:\n{}", targets);

    if targets.contains("wasm32-unknown-unknown") {
        println!("✅ wasm32-unknown-unknown target is installed");
    } else {
        println!("❌ wasm32-unknown-unknown target is NOT installed");
    }
}

/// Test 3: Try minimal WASM build to isolate the issue
#[test]
fn test_minimal_wasm_build() {
    println!("🧪 Testing minimal WASM build...");

    // Create a minimal Cargo.toml for testing
    let minimal_cargo = r#"
[package]
name = "minimal-wasm-test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
"#;

    let minimal_lib = r#"
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "Hello from WASM!".to_string()
}
"#;

    // Write minimal files
    fs::write("Cargo-minimal.toml", minimal_cargo).expect("Failed to write minimal Cargo.toml");
    fs::create_dir_all("src-minimal").expect("Failed to create src-minimal directory");
    fs::write("src-minimal/lib.rs", minimal_lib).expect("Failed to write minimal lib.rs");

    // Try to build minimal WASM
    let output = Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--out-dir",
            "pkg-minimal",
            "--manifest-path",
            "Cargo-minimal.toml",
        ])
        .current_dir(".")
        .output()
        .expect("Failed to execute minimal wasm-pack test");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("📊 Minimal build exit code: {}", output.status);
    println!("📝 Minimal build STDOUT:\n{}", stdout);
    println!("❌ Minimal build STDERR:\n{}", stderr);

    if output.status.success() {
        println!("✅ Minimal WASM build succeeded - issue is with specific dependencies");
    } else if stderr.contains("WASM_BIGINT") {
        println!("❌ Even minimal build fails with WASM_BIGINT - issue is with toolchain");
    } else {
        println!("⚠️ Minimal build failed with different error");
    }

    // Cleanup
    let _ = fs::remove_file("Cargo-minimal.toml");
    let _ = fs::remove_dir_all("src-minimal");
    let _ = fs::remove_dir_all("pkg-minimal");
}

/// Test 4: Try different wasm-pack flags and configurations
#[test]
fn test_alternative_build_configurations() {
    println!("🔧 Testing alternative build configurations...");

    let test_configs = vec![
        (
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test1",
                "--",
                "--no-default-features",
            ],
            "No default features",
        ),
        (
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test2",
                "--",
                "--features",
                "console_error_panic_hook",
            ],
            "Console error panic hook",
        ),
        (
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test3",
                "--",
                "--release",
            ],
            "Release build",
        ),
        (
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test4",
                "--",
                "--",
                "-C",
                "target-feature=+bulk-memory",
            ],
            "Bulk memory feature",
        ),
    ];

    for (i, (args, description)) in test_configs.iter().enumerate() {
        println!("🧪 Test {}: {}", i + 1, description);

        let output = Command::new("wasm-pack")
            .args(args)
            .current_dir(".")
            .output()
            .expect(&format!("Failed to execute wasm-pack test {}", i + 1));

        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("📊 Exit code: {}", output.status);
        if !stdout.is_empty() {
            println!("📝 STDOUT:\n{}", stdout);
        }
        if !stderr.is_empty() {
            println!("❌ STDERR:\n{}", stderr);
        }

        if output.status.success() {
            println!("🎉 SUCCESS: {} resolved the issue!", description);
            return;
        } else if stderr.contains("WASM_BIGINT") {
            println!("❌ Still getting WASM_BIGINT error with {}", description);
        } else {
            println!("⚠️ Different error with {}: {}", description, stderr);
        }

        println!("---");
    }

    println!("❌ All alternative configurations failed");
}

/// Test 5: Check for known workarounds and solutions
#[test]
fn test_known_workarounds() {
    println!("🔍 Testing known workarounds...");

    // Check if we can use a different linker
    let linker_test = Command::new("rustc").args(&["--print", "cfg"]).output();

    if let Ok(output) = linker_test {
        let cfg = String::from_utf8_lossy(&output.stdout);
        println!("🔧 Rust configuration:\n{}", cfg);
    }

    // Try with explicit linker flags
    let output = Command::new("wasm-pack")
        .args(&["build", "--target", "web", "--out-dir", "pkg-workaround"])
        .env("RUSTFLAGS", "-C target-feature=+bulk-memory")
        .current_dir(".")
        .output()
        .expect("Failed to execute workaround test");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("📊 Workaround test exit code: {}", output.status);
    println!("📝 Workaround STDOUT:\n{}", stdout);
    println!("❌ Workaround STDERR:\n{}", stderr);

    if output.status.success() {
        println!("🎉 SUCCESS: RUSTFLAGS workaround resolved the issue!");
    } else {
        println!("❌ RUSTFLAGS workaround did not resolve the issue");
    }
}
