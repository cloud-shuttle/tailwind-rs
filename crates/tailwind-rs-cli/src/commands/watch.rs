//! Watch Command Implementation
//!
//! The watch command monitors source files for changes and automatically
//! rebuilds CSS when modifications are detected.

use crate::{config::Config, css_processor::CssProcessor, file_watcher::FileWatcher, CliError};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time;
use anyhow::Result;

/// Arguments for the watch command
#[derive(Debug)]
pub struct WatchArgs {
    /// Input CSS file path
    pub input: Option<PathBuf>,
    /// Output CSS file path
    pub output: Option<PathBuf>,
    /// Content file patterns to scan
    pub content: Vec<String>,
    /// Configuration file path
    pub config: Option<PathBuf>,
    /// Whether to minify output
    pub minify: bool,
    /// Polling interval in milliseconds
    pub poll_interval: u64,
}

/// Execute the watch command
pub async fn execute(args: WatchArgs) -> Result<(), CliError> {
    println!("👀 Watching for file changes...");

    // Load configuration
    let config = Config::load(args.config.as_deref())?;
    let config = if config.is_none() {
        Config::load_from_defaults()?
            .ok_or_else(|| CliError::Config(crate::config::ConfigError::NotFound))?
    } else {
        config.unwrap()
    };

    // Determine content patterns
    let content_patterns = if args.content.is_empty() {
        config.content.clone()
    } else {
        args.content.clone()
    };

    if content_patterns.is_empty() {
        eprintln!("⚠️  Warning: No content patterns specified. Using defaults.");
    }

    // Create file watcher
    let mut watcher = FileWatcher::new(content_patterns.clone(), Duration::from_millis(args.poll_interval))?;

    // Perform initial build
    if let Err(e) = perform_build(&args, &config).await {
        eprintln!("❌ Initial build failed: {}", e);
        return Err(e);
    }

    println!("✅ Initial build complete. Watching for changes...");
    println!("📂 Watching patterns: {:?}", content_patterns);

    // Watch loop
    loop {
        // Wait for file changes
        match watcher.wait_for_changes().await {
            Ok(changed_files) => {
                println!("📝 Files changed: {:?}", changed_files.len());

                // Perform rebuild
                match perform_build(&args, &config).await {
                    Ok(_) => println!("✅ Rebuild complete"),
                    Err(e) => eprintln!("❌ Rebuild failed: {}", e),
                }
            }
            Err(e) => {
                eprintln!("❌ File watching error: {}", e);
                return Err(CliError::FileWatcher(e));
            }
        }

        // Small delay to prevent rapid rebuilds
        time::sleep(Duration::from_millis(100)).await;
    }
}

/// Perform a build operation
async fn perform_build(args: &WatchArgs, config: &Config) -> Result<(), CliError> {
    // Create CSS processor
    let mut css_processor = CssProcessor::new()?;

    // Determine input file
    let input_path = args.input
        .clone()
        .or_else(|| config.input.clone())
        .unwrap_or_else(|| PathBuf::from("tailwind.css"));

    // Read input CSS
    let input_css = if input_path.exists() {
        tokio::fs::read_to_string(&input_path).await
            .map_err(|e| CliError::Io(e))?
    } else {
        // Create basic input CSS with Tailwind directives
        r#"@tailwind base;
@tailwind components;
@tailwind utilities;"#.to_string()
    };

    // Determine content patterns
    let content_patterns = if args.content.is_empty() {
        config.content.clone()
    } else {
        args.content.clone()
    };

    // Scan content files for classes
    let classes = css_processor.scan_content_files(&content_patterns).await?;

    // Generate CSS
    let output_css = css_processor.generate_css(&input_css, &classes, args.minify).await?;

    // Determine output path
    let output_path = args.output
        .clone()
        .or_else(|| config.output.clone())
        .unwrap_or_else(|| PathBuf::from("dist/output.css"));

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await
            .map_err(|e| CliError::Io(e))?;
    }

    // Write output CSS
    tokio::fs::write(&output_path, &output_css).await
        .map_err(|e| CliError::Io(e))?;

    Ok(())
}

/// Watch-specific error types
#[derive(Debug, thiserror::Error)]
pub enum WatchError {
    #[error("File watcher initialization failed: {0}")]
    WatcherInit(String),

    #[error("Build failed during watch: {0}")]
    BuildFailed(String),

    #[error("Configuration error: {0}")]
    Config(String),
}
