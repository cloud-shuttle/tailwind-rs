//! Build Command Implementation
//!
//! The build command scans source files for Tailwind classes and generates
//! optimized CSS output using the Tailwind-RS CSS generator.

use crate::{config::Config, css_processor::CssProcessor, CliError};
use std::path::PathBuf;
use tokio::fs;
use anyhow::Result;

/// Arguments for the build command
#[derive(Debug)]
pub struct BuildArgs {
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
}

/// Execute the build command
pub async fn execute(args: BuildArgs) -> Result<(), CliError> {
    println!("🔨 Building CSS with Tailwind-RS...");

    // Load configuration
    let config = Config::load(args.config.as_deref())?;
    let config = if config.is_none() {
        // Try to find default config files
        Config::load_from_defaults()?
            .ok_or_else(|| CliError::Config(crate::config::ConfigError::NotFound))?
    } else {
        config.unwrap()
    };

    // Determine input file
    let input_path = args.input
        .or_else(|| config.input.clone())
        .unwrap_or_else(|| PathBuf::from("tailwind.css"));

    // Read input CSS
    let input_css = if input_path.exists() {
        fs::read_to_string(&input_path).await
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

    let content_patterns = if content_patterns.is_empty() {
        eprintln!("⚠️  Warning: No content patterns specified. Using defaults.");
        vec![
            "src/**/*.{html,js,ts,jsx,tsx}".to_string(),
            "public/index.html".to_string(),
        ]
    } else {
        content_patterns
    };

    // Scan content files for classes
    println!("📂 Scanning content files...");
    let mut css_processor = CssProcessor::new()?;
    let classes = css_processor.scan_content_files(&content_patterns).await?;

    println!("🎨 Found {} unique Tailwind classes", classes.len());

    // Generate CSS
    println!("🎨 Generating CSS...");
    let output_css = css_processor.generate_css(&input_css, &classes, args.minify).await?;

    // Determine output path
    let output_path = args.output
        .or_else(|| config.output.clone())
        .unwrap_or_else(|| PathBuf::from("dist/output.css"));

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).await
            .map_err(|e| CliError::Io(e))?;
    }

    // Write output CSS
    fs::write(&output_path, &output_css).await
        .map_err(|e| CliError::Io(e))?;

    let size_kb = output_css.len() / 1024;
    println!("✅ CSS built successfully!");
    println!("📄 Output: {} ({} KB)", output_path.display(), size_kb);

    if args.minify {
        println!("🗜️  Output is minified");
    }

    Ok(())
}

/// Build-specific error types
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Failed to scan content files: {0}")]
    ContentScan(String),

    #[error("Failed to generate CSS: {0}")]
    CssGeneration(String),

    #[error("Invalid configuration: {0}")]
    Config(String),
}
