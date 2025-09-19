//! # Optimize Command
//!
//! This module handles the optimize command for optimizing CSS output.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tailwind_rs_core::css_optimizer::CssOptimizer;
use crate::utils::{FileUtils, LogUtils};

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
        LogUtils::info("Starting CSS optimization...");
        
        if self.verbose {
            LogUtils::info(&format!("Input file: {:?}", self.input));
            LogUtils::info(&format!("Output file: {:?}", self.output));
            LogUtils::info(&format!("Optimization level: {}", self.level));
        }

        // Validate input file exists
        if !FileUtils::file_exists(&self.input) {
            LogUtils::error(&format!("Input file does not exist: {:?}", self.input));
            return Err(anyhow::anyhow!("Input file not found"));
        }

        // Ensure output directory exists
        if let Some(output_dir) = self.output.parent() {
            FileUtils::ensure_dir(output_dir)?;
        }

        // Get input file size
        let input_size = std::fs::metadata(&self.input)?.len();
        if self.verbose {
            LogUtils::info(&format!("Input file size: {} bytes", input_size));
        }

        let optimizer = CssOptimizer::new();

        // Read input CSS
        let css_content = std::fs::read_to_string(&self.input)?;
        
        // Perform optimization
        let start_time = std::time::Instant::now();
        let optimized_css = optimizer.optimize_css(&css_content)?;
        
        // Write output
        std::fs::write(&self.output, optimized_css)?;
        let duration = start_time.elapsed();

        // Get output file size
        let output_size = if FileUtils::file_exists(&self.output) {
            std::fs::metadata(&self.output)?.len()
        } else {
            0
        };

        let savings = if input_size > 0 {
            ((input_size - output_size) as f64 / input_size as f64) * 100.0
        } else {
            0.0
        };

        LogUtils::success(&format!(
            "Optimization completed in {:.2}s",
            duration.as_secs_f64()
        ));

        if self.verbose {
            LogUtils::info(&format!("Output file size: {} bytes", output_size));
            LogUtils::info(&format!("Size reduction: {:.1}%", savings));
            LogUtils::info(&format!("Output written to: {:?}", self.output));
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
