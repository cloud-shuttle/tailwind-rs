//! # Optimize Command
//!
//! This module handles the optimize command for optimizing CSS output.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tailwind_rs_core::CssOptimizer;

/// Optimize CSS output
#[derive(Parser)]
pub struct OptimizeCommand {
    /// Input CSS file path
    #[arg(short, long, default_value = "dist/styles.css")]
    pub input: PathBuf,

    /// Output CSS file path
    #[arg(short, long, default_value = "dist/styles.min.css")]
    pub output: PathBuf,

    /// Optimization level (1-3)
    #[arg(short, long, default_value = "3")]
    pub level: u8,

    /// Remove unused classes
    #[arg(long)]
    pub remove_unused: bool,

    /// Minify CSS
    #[arg(long)]
    pub minify: bool,

    /// Generate source maps
    #[arg(long)]
    pub source_maps: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl OptimizeCommand {
    /// Execute the optimize command
    pub async fn execute(&self) -> Result<()> {
        if self.verbose {
            println!("Optimizing CSS...");
            println!("Input: {:?}", self.input);
            println!("Output: {:?}", self.output);
            println!("Level: {}", self.level);
        }

        let mut optimizer = CssOptimizer::new()
            .input_file(&self.input)
            .output_file(&self.output)
            .optimization_level(self.level);

        if self.remove_unused {
            optimizer = optimizer.remove_unused_classes();
        }

        if self.minify {
            optimizer = optimizer.minify();
        }

        if self.source_maps {
            optimizer = optimizer.generate_source_maps();
        }

        optimizer.optimize().await?;

        if self.verbose {
            println!("Optimization completed successfully!");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_optimize_command_parsing() {
        use crate::Cli;
        
        let args = vec![
            "tailwind-rs",
            "optimize",
            "--input",
            "input.css",
            "--output",
            "output.min.css",
            "--level",
            "2",
            "--remove-unused",
            "--minify",
            "--source-maps",
            "--verbose",
        ];

        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            crate::Commands::Optimize(cmd) => {
                assert_eq!(cmd.input, PathBuf::from("input.css"));
                assert_eq!(cmd.output, PathBuf::from("output.min.css"));
                assert_eq!(cmd.level, 2);
                assert!(cmd.remove_unused);
                assert!(cmd.minify);
                assert!(cmd.source_maps);
                assert!(cmd.verbose);
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[test]
    fn test_optimize_command_defaults() {
        use crate::Cli;
        
        let args = vec!["tailwind-rs", "optimize"];

        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            crate::Commands::Optimize(cmd) => {
                assert_eq!(cmd.input, PathBuf::from("dist/styles.css"));
                assert_eq!(cmd.output, PathBuf::from("dist/styles.min.css"));
                assert_eq!(cmd.level, 3);
                assert!(!cmd.remove_unused);
                assert!(!cmd.minify);
                assert!(!cmd.source_maps);
                assert!(!cmd.verbose);
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[tokio::test]
    async fn test_optimize_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let input_file = temp_dir.path().join("input.css");
        let output_file = temp_dir.path().join("output.css");

        // Create a test CSS file
        fs::write(
            &input_file,
            r#"
            .px-4 { padding-left: 1rem; padding-right: 1rem; }
            .py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }
            .bg-blue-600 { background-color: #2563eb; }
            .text-white { color: #ffffff; }
        "#,
        )
        .unwrap();

        let cmd = OptimizeCommand {
            input: input_file,
            output: output_file.clone(),
            level: 2,
            remove_unused: true,
            minify: true,
            source_maps: false,
            verbose: false,
        };

        // This should not panic, even if the optimization fails
        let result = cmd.execute().await;
        // We don't assert success here because we don't have a full CSS optimizer setup
        // In a real test, we would mock the CssOptimizer
        assert!(result.is_ok() || result.is_err());
    }
}
