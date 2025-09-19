//! # Stats Command
//!
//! This module handles the stats command for showing build statistics and project information.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use crate::utils::{FileUtils, LogUtils, StringUtils};

/// Show build statistics and project information
#[derive(Parser)]
pub struct StatsCommand {
    /// Source directory to analyze
    #[arg(short, long, default_value = "src")]
    pub source: PathBuf,

    /// Output CSS file to analyze
    #[arg(short, long, default_value = "dist/styles.css")]
    pub output: PathBuf,

    /// Show detailed statistics
    #[arg(long)]
    pub detailed: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl StatsCommand {
    /// Execute the stats command
    pub async fn execute(&self) -> Result<()> {
        LogUtils::info("Analyzing project statistics...");

        if self.verbose {
            LogUtils::info(&format!("Source directory: {:?}", self.source));
            LogUtils::info(&format!("Output file: {:?}", self.output));
        }

        // Analyze source files
        let source_stats = self.analyze_source()?;
        
        // Analyze output file
        let output_stats = self.analyze_output()?;

        // Display statistics
        self.display_stats(&source_stats, &output_stats)?;

        Ok(())
    }

    /// Analyze source directory
    fn analyze_source(&self) -> Result<SourceStats> {
        if !FileUtils::file_exists(&self.source) {
            return Ok(SourceStats::default());
        }

        let rust_files = FileUtils::find_rust_files(&self.source)?;
        let mut total_lines = 0;
        let mut total_classes = 0;
        let mut class_usage = std::collections::HashMap::new();

        for file in &rust_files {
            if let Ok(content) = FileUtils::read_file(file) {
                let lines = content.lines().count();
                total_lines += lines;

                // Count Tailwind classes (simple heuristic)
                let classes = self.extract_classes(&content);
                total_classes += classes.len();

                for class in classes {
                    *class_usage.entry(class).or_insert(0) += 1;
                }
            }
        }

        Ok(SourceStats {
            rust_files: rust_files.len(),
            total_lines,
            total_classes,
            class_usage,
        })
    }

    /// Analyze output CSS file
    fn analyze_output(&self) -> Result<OutputStats> {
        if !FileUtils::file_exists(&self.output) {
            return Ok(OutputStats::default());
        }

        let content = FileUtils::read_file(&self.output)?;
        let size = content.len();
        let lines = content.lines().count();
        let classes = self.count_css_classes(&content);

        Ok(OutputStats {
            size,
            lines,
            classes,
        })
    }

    /// Extract Tailwind classes from Rust code
    fn extract_classes(&self, content: &str) -> Vec<String> {
        let mut classes = Vec::new();
        
        // Simple regex-like extraction for common patterns
        for line in content.lines() {
            // Look for class strings in common patterns
            if line.contains("class") || line.contains("classes") {
                // Extract quoted strings that look like Tailwind classes
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    if word.contains('-') && StringUtils::is_valid_css_class(word.trim_matches('"')) {
                        classes.push(word.trim_matches('"').to_string());
                    }
                }
            }
        }
        
        classes
    }

    /// Count CSS classes in output
    fn count_css_classes(&self, content: &str) -> usize {
        content.matches('.').count()
    }

    /// Display statistics
    fn display_stats(&self, source: &SourceStats, output: &OutputStats) -> Result<()> {
        LogUtils::success("Project Statistics");
        println!();

        // Source statistics
        println!("📁 Source Analysis:");
        println!("  Rust files: {}", source.rust_files);
        println!("  Total lines: {}", source.total_lines);
        println!("  Tailwind classes: {}", source.total_classes);

        if self.detailed && !source.class_usage.is_empty() {
            println!("  Most used classes:");
            let mut sorted_classes: Vec<_> = source.class_usage.iter().collect();
            sorted_classes.sort_by(|a, b| b.1.cmp(a.1));
            
            for (class, count) in sorted_classes.iter().take(10) {
                println!("    {}: {} times", class, count);
            }
        }

        println!();

        // Output statistics
        println!("📄 Output Analysis:");
        println!("  File size: {} bytes", output.size);
        println!("  CSS lines: {}", output.lines);
        println!("  CSS classes: {}", output.classes);

        if output.size > 0 {
            let size_kb = output.size as f64 / 1024.0;
            println!("  Size: {:.2} KB", size_kb);
        }

        println!();

        // Summary
        if source.rust_files > 0 {
            let avg_lines = source.total_lines / source.rust_files;
            println!("📊 Summary:");
            println!("  Average lines per file: {}", avg_lines);
            println!("  Classes per file: {:.1}", source.total_classes as f64 / source.rust_files as f64);
        }

        Ok(())
    }
}

/// Source file statistics
#[derive(Debug, Default)]
struct SourceStats {
    rust_files: usize,
    total_lines: usize,
    total_classes: usize,
    class_usage: std::collections::HashMap<String, usize>,
}

/// Output file statistics
#[derive(Debug, Default)]
struct OutputStats {
    size: usize,
    lines: usize,
    classes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_stats_command_parsing() {
        use crate::Cli;
        
        let args = vec![
            "tailwind-rs",
            "stats",
            "--source",
            "custom-src",
            "--output",
            "custom-output.css",
            "--detailed",
            "--verbose",
        ];

        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            crate::Commands::Stats(cmd) => {
                assert_eq!(cmd.source, PathBuf::from("custom-src"));
                assert_eq!(cmd.output, PathBuf::from("custom-output.css"));
                assert!(cmd.detailed);
                assert!(cmd.verbose);
            }
            _ => panic!("Expected Stats command"),
        }
    }

    #[test]
    fn test_extract_classes() {
        let cmd = StatsCommand {
            source: PathBuf::from("src"),
            output: PathBuf::from("dist/styles.css"),
            detailed: false,
            verbose: false,
        };

        let content = r#"
            let classes = "px-4 py-2 bg-blue-600 text-white";
            let other = "hover:bg-blue-700 focus:ring-2";
        "#;

        let classes = cmd.extract_classes(content);
        assert!(classes.contains(&"px-4".to_string()));
        assert!(classes.contains(&"py-2".to_string()));
        assert!(classes.contains(&"bg-blue-600".to_string()));
    }

    #[test]
    fn test_count_css_classes() {
        let cmd = StatsCommand {
            source: PathBuf::from("src"),
            output: PathBuf::from("dist/styles.css"),
            detailed: false,
            verbose: false,
        };

        let css = r#"
            .px-4 { padding: 1rem; }
            .py-2 { padding: 0.5rem; }
            .bg-blue-600 { background-color: #2563eb; }
        "#;

        let count = cmd.count_css_classes(css);
        // The CSS has 3 class selectors, but there might be additional dots in the content
        assert!(count >= 3);
    }

    #[tokio::test]
    async fn test_stats_command_execution() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("src");
        let output_file = temp_dir.path().join("styles.css");

        // Create source directory with test files
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(
            source_dir.join("main.rs"),
            r#"
            use tailwind_rs::*;
            
            fn main() {
                let classes = "px-4 py-2 bg-blue-600 text-white";
                println!("{}", classes);
            }
        "#,
        ).unwrap();

        // Create output CSS file
        fs::write(
            &output_file,
            r#"
            .px-4 { padding-left: 1rem; padding-right: 1rem; }
            .py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }
            .bg-blue-600 { background-color: #2563eb; }
            .text-white { color: #ffffff; }
        "#,
        ).unwrap();

        let cmd = StatsCommand {
            source: source_dir,
            output: output_file,
            detailed: false,
            verbose: false,
        };

        // Should not panic
        let result = cmd.execute().await;
        assert!(result.is_ok());
    }
}
