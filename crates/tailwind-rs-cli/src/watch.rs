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
        if self.verbose {
            println!("Watching for changes in: {:?}", self.source);
            println!("Output: {:?}", self.output);
        }

        // Create a channel to receive file system events
        let (tx, rx) = mpsc::channel();

        // Create a watcher
        let mut watcher = notify::recommended_watcher(tx)?;

        // Watch the source directory
        watcher.watch(&self.source, RecursiveMode::Recursive)?;

        if self.verbose {
            println!("Watcher started. Press Ctrl+C to stop.");
        }

        // Initial build
        self.build().await?;

        // Watch for changes
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(event) = event {
                        if self.should_rebuild(&event) {
                            if self.verbose {
                                println!("File changed: {:?}", event.paths);
                            }

                            // Debounce the rebuild
                            tokio::time::sleep(Duration::from_millis(self.debounce)).await;

                            // Rebuild
                            if let Err(e) = self.build().await {
                                eprintln!("Build error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Watch error: {}", e);
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
    async fn build(&self) -> Result<()> {
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
