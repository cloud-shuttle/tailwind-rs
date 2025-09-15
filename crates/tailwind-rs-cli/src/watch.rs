//! # Watch Command
//!
//! This module handles the watch command for automatically rebuilding CSS when files change.

use anyhow::Result;
use clap::Parser;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tailwind_rs_core::TailwindBuilder;
use crate::utils::{FileUtils, LogUtils, PathUtils};

/// Watch for changes and rebuild automatically
#[derive(Parser)]
pub struct WatchCommand {
    /// Source directory to watch
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

    /// Debounce delay in milliseconds
    #[arg(long, default_value = "300")]
    pub debounce: u64,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl WatchCommand {
    /// Execute the watch command
    pub async fn execute(&self) -> Result<()> {
        LogUtils::info("Starting Tailwind CSS watch mode...");
        
        if self.verbose {
            LogUtils::info(&format!("Watching directory: {:?}", self.source));
            LogUtils::info(&format!("Output file: {:?}", self.output));
            LogUtils::info(&format!("Debounce delay: {}ms", self.debounce));
        }

        // Validate source directory exists
        if !FileUtils::file_exists(&self.source) {
            LogUtils::error(&format!("Source directory does not exist: {:?}", self.source));
            return Err(anyhow::anyhow!("Source directory not found"));
        }

        // Ensure output directory exists
        if let Some(output_dir) = self.output.parent() {
            FileUtils::ensure_dir(output_dir)?;
        }

        // Create a channel to receive file system events
        let (tx, rx) = mpsc::channel();

        // Create a watcher
        let mut watcher = notify::recommended_watcher(tx)?;

        // Watch the source directory
        watcher.watch(&self.source, RecursiveMode::Recursive)?;

        LogUtils::success("Watcher started successfully");
        if self.verbose {
            LogUtils::info("Press Ctrl+C to stop watching");
        }

        // Initial build
        LogUtils::info("Performing initial build...");
        self.build()?;

        // Watch for changes
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(event) = event {
                        if self.should_rebuild(&event) {
                            if self.verbose {
                                LogUtils::info(&format!("File changed: {:?}", event.paths));
                            }

                            // Debounce the rebuild
                            tokio::time::sleep(Duration::from_millis(self.debounce)).await;

                            // Rebuild
                            if let Err(e) = self.build() {
                                LogUtils::error(&format!("Build error: {}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    LogUtils::error(&format!("Watch error: {}", e));
                    break;
                }
            }
        }

        Ok(())
    }

    /// Check if we should rebuild based on the event
    fn should_rebuild(&self, event: &Event) -> bool {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                // Only rebuild for Rust files
                event
                    .paths
                    .iter()
                    .any(|path| path.extension().map(|ext| ext == "rs").unwrap_or(false))
            }
            _ => false,
        }
    }

    /// Perform the actual build
    fn build(&self) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        let mut builder = TailwindBuilder::new()
            .scan_source(&self.source)
            .output_css(&self.output);

        if let Some(config_path) = &self.config {
            if FileUtils::file_exists(config_path) {
                builder = builder.config_file(config_path);
            }
        }

        if self.tree_shake {
            builder = builder.enable_tree_shaking();
        }

        if self.minify {
            builder = builder.enable_minification();
        }

        builder.build()?;
        
        let duration = start_time.elapsed();
        
        // Get output file size
        let output_size = if FileUtils::file_exists(&self.output) {
            std::fs::metadata(&self.output)?.len()
        } else {
            0
        };

        if self.verbose {
            LogUtils::success(&format!(
                "Rebuild completed in {:.2}s ({} bytes)",
                duration.as_secs_f64(),
                output_size
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_command_parsing() {
        use crate::Cli;
        
        let args = vec![
            "tailwind-rs",
            "watch",
            "--source",
            "custom-src",
            "--output",
            "custom-output.css",
            "--tree-shake",
            "--minify",
            "--debounce",
            "500",
            "--verbose",
        ];

        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            crate::Commands::Watch(cmd) => {
                assert_eq!(cmd.source, PathBuf::from("custom-src"));
                assert_eq!(cmd.output, PathBuf::from("custom-output.css"));
                assert!(cmd.tree_shake);
                assert!(cmd.minify);
                assert_eq!(cmd.debounce, 500);
                assert!(cmd.verbose);
            }
            _ => panic!("Expected Watch command"),
        }
    }

    #[test]
    fn test_should_rebuild() {
        let cmd = WatchCommand {
            source: PathBuf::from("src"),
            output: PathBuf::from("dist/styles.css"),
            config: None,
            tree_shake: false,
            minify: false,
            debounce: 300,
            verbose: false,
        };

        // Test with a Rust file
        let rust_event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )),
            paths: vec![PathBuf::from("src/main.rs")],
            attrs: Default::default(),
        };

        assert!(cmd.should_rebuild(&rust_event));

        // Test with a non-Rust file
        let non_rust_event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )),
            paths: vec![PathBuf::from("src/main.txt")],
            attrs: Default::default(),
        };

        assert!(!cmd.should_rebuild(&non_rust_event));
    }
}
