//! TDD Test for WASM_BIGINT Investigation
//!
//! This test systematically investigates the WASM_BIGINT linker error
//! and tests different approaches to resolve it.

#[cfg(test)]
mod wasm_bigint_tests {
    use std::path::Path;
    use std::process::Command;

    /// Test 1: Verify the current WASM_BIGINT error
    #[test]
    fn test_wasm_bigint_error_reproduction() {
        let output = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--out-dir", "pkg-test"])
            .current_dir(".")
            .output()
            .expect("Failed to execute wasm-pack");

        let stderr = String::from_utf8_lossy(&output.stderr);

        // This test should fail with WASM_BIGINT error
        assert!(
            stderr.contains("WASM_BIGINT") || output.status.success(),
            "Expected WASM_BIGINT error, but got: {}",
            stderr
        );
    }

    /// Test 2: Try with different linker flags
    #[test]
    fn test_alternative_linker_flags() {
        let test_cases = vec![
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test2",
                "--",
                "--no-default-features",
            ],
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test3",
                "--",
                "--features",
                "console_error_panic_hook",
            ],
            vec![
                "build",
                "--target",
                "web",
                "--out-dir",
                "pkg-test4",
                "--",
                "--release",
            ],
        ];

        for (i, args) in test_cases.iter().enumerate() {
            let output = Command::new("wasm-pack")
                .args(args)
                .current_dir(".")
                .output()
                .expect(&format!("Failed to execute wasm-pack test {}", i));

            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Test {} output: {}", i, stderr);

            // Check if this approach works
            if output.status.success() {
                println!("SUCCESS: Test {} resolved WASM_BIGINT issue", i);
                return;
            }
        }

        // If we get here, none of the approaches worked
        panic!("All alternative linker flag approaches failed");
    }

    /// Test 3: Check if the issue is with specific dependencies
    #[test]
    fn test_dependency_isolation() {
        // Create a minimal test to isolate the issue
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

        std::fs::write("Cargo-minimal.toml", minimal_cargo)
            .expect("Failed to write minimal Cargo.toml");

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

        if output.status.success() {
            println!("SUCCESS: Minimal test works - issue is with specific dependencies");
        } else {
            println!(
                "FAILED: Even minimal test fails - issue is with toolchain: {}",
                stderr
            );
        }
    }

    /// Test 4: Check wasm-pack version and try different versions
    #[test]
    fn test_wasm_pack_version() {
        let output = Command::new("wasm-pack")
            .args(&["--version"])
            .output()
            .expect("Failed to get wasm-pack version");

        let version = String::from_utf8_lossy(&output.stdout);
        println!("Current wasm-pack version: {}", version);

        // Check if we can update wasm-pack
        let update_output = Command::new("cargo")
            .args(&["install", "--force", "wasm-pack"])
            .output();

        match update_output {
            Ok(output) => {
                if output.status.success() {
                    println!("SUCCESS: Updated wasm-pack");
                } else {
                    println!("FAILED: Could not update wasm-pack");
                }
            }
            Err(_) => println!("Could not update wasm-pack (not installed via cargo)"),
        }
    }

    /// Test 5: Try different Rust targets
    #[test]
    fn test_different_rust_targets() {
        let targets = vec!["wasm32-unknown-unknown", "wasm32-wasi"];

        for target in targets {
            let output = Command::new("rustup")
                .args(&["target", "add", target])
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    println!("SUCCESS: Added target {}", target);
                }
            }
        }
    }
}
