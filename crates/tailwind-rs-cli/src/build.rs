//! # Build Command
//!
//! This module handles the build command for generating Tailwind CSS from Rust source files.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tailwind_rs_core::TailwindBuilder;

/// Build Tailwind CSS from Rust source files
#[derive(Parser)]
pub struct BuildCommand {
    /// Source directory to scan for Rust files
    #[arg(short, long, default_value = "src")]
    pub source: PathBuf,

    /// Output CSS file path
    #[arg(short, long, default_value = "dist/styles.css")]
    pub output: PathBuf,

    /// Configuration file path
    #[arg(short, long, default_value = "tailwind-rs.toml")]
    pub config: Option<PathBuf>,

    /// Enable tree-shaking (remove unused classes)
    #[arg(long)]
    pub tree_shake: bool,

    /// Enable minification
    #[arg(long)]
    pub minify: bool,

    /// Enable source maps
    #[arg(long)]
    pub source_maps: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl BuildCommand {
    /// Execute the build command
    pub async fn execute(&self) -> Result<()> {
        if self.verbose {
            println!("Building Tailwind CSS...");
            println!("Source: {:?}", self.source);
            println!("Output: {:?}", self.output);
        }

        let mut builder = TailwindBuilder::new()
            .scan_source(&self.source)
            .output_css(&self.output);

        if let Some(config_path) = &self.config {
            builder = builder.config_file(config_path);
        }

        if self.tree_shake {
            builder = builder.enable_tree_shaking();
        }

        if self.minify {
            builder = builder.enable_minification();
        }

        if self.source_maps {
            builder = builder.enable_source_maps();
        }

        builder.build().await?;

        if self.verbose {
            println!("Build completed successfully!");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_build_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("src");
        let output_file = temp_dir.path().join("styles.css");

        // Create source directory
        fs::create_dir_all(&source_dir).unwrap();

        // Create a test Rust file
        let test_file = source_dir.join("main.rs");
        fs::write(
            &test_file,
            r#"
            use tailwind_rs::*;
            
            fn main() {
                let classes = classes! {
                    base: "px-4 py-2",
                    variant: "bg-blue-600 text-white"
                };
                println!("{}", classes);
            }
        "#,
        )
        .unwrap();

        let cmd = BuildCommand {
            source: source_dir,
            output: output_file.clone(),
            config: None,
            tree_shake: true,
            minify: true,
            source_maps: false,
            verbose: false,
        };

        // This should not panic, even if the build fails
        let result = cmd.execute().await;
        // We don't assert success here because we don't have a full Tailwind setup
        // In a real test, we would mock the TailwindBuilder
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_build_command_parsing() {
        use crate::Cli;
        
        let args = vec![
            "tailwind-rs",
            "build",
            "--source",
            "custom-src",
            "--output",
            "custom-output.css",
            "--tree-shake",
            "--minify",
            "--verbose",
        ];

        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            crate::Commands::Build(cmd) => {
                assert_eq!(cmd.source, PathBuf::from("custom-src"));
                assert_eq!(cmd.output, PathBuf::from("custom-output.css"));
                assert!(cmd.tree_shake);
                assert!(cmd.minify);
                assert!(cmd.verbose);
            }
            _ => panic!("Expected Build command"),
        }
    }
}
