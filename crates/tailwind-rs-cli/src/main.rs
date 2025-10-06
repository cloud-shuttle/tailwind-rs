//! Tailwind-RS CLI Tool
//!
//! A command-line interface for building and managing Tailwind CSS with Rust.
//! Provides build, watch, and development utilities for the Tailwind-RS framework.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod css_processor;
mod file_watcher;

use commands::{build, init, watch};

/// Tailwind-RS CLI - A fast, Rust-powered Tailwind CSS build tool
#[derive(Parser)]
#[command(name = "tailwind-rs")]
#[command(version, about = "A fast, Rust-powered Tailwind CSS build tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Build CSS from source files
    Build {
        /// Input CSS file (default: stdin or tailwind.css)
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Output CSS file (default: stdout or dist/output.css)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Content file patterns to scan for classes
        #[arg(short, long)]
        content: Vec<String>,

        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Minify the output CSS
        #[arg(long)]
        minify: bool,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Initialize a new Tailwind-RS project
    Init {
        /// Configuration file to create (default: tailwind.config.js)
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Create TypeScript config instead of JavaScript
        #[arg(long)]
        typescript: bool,

        /// Force overwrite existing files
        #[arg(short, long)]
        force: bool,
    },

    /// Watch source files and rebuild on changes
    Watch {
        /// Input CSS file
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Output CSS file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Content file patterns to scan for classes
        #[arg(short, long)]
        content: Vec<String>,

        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Minify the output CSS
        #[arg(long)]
        minify: bool,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Polling interval in milliseconds (default: 100)
        #[arg(long, default_value = "100")]
        poll: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging based on verbose flag
    let verbose = matches!(cli.command, Commands::Build { verbose: true, .. } | Commands::Watch { verbose: true, .. });

    if verbose {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Warn)
            .init();
    }

    match cli.command {
        Commands::Build { input, output, content, config, minify, .. } => {
            build::execute(build::BuildArgs {
                input,
                output,
                content,
                config,
                minify,
            }).await?;
        }

        Commands::Init { config, typescript, force } => {
            init::execute(init::InitArgs {
                config,
                typescript,
                force,
            }).await?;
        }

        Commands::Watch { input, output, content, config, minify, poll, .. } => {
            watch::execute(watch::WatchArgs {
                input,
                output,
                content,
                config,
                minify,
                poll_interval: poll,
            }).await?;
        }
    }

    Ok(())
}

/// CLI-specific error types
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Build error: {0}")]
    Build(#[from] build::BuildError),

    #[error("Init error: {0}")]
    Init(#[from] init::InitError),

    #[error("Watch error: {0}")]
    Watch(#[from] watch::WatchError),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("CSS processing error: {0}")]
    CssProcessor(#[from] css_processor::CssProcessorError),

    #[error("File watcher error: {0}")]
    FileWatcher(#[from] file_watcher::FileWatcherError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, CliError>;