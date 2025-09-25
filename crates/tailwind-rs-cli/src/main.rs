//! # Tailwind-rs CLI
//!
//! This is the main CLI tool for the Tailwind-rs build system.
//! It follows our TDD-first approach (ADR-001) and comprehensive testing pyramid strategy (ADR-002).

use anyhow::Result;
use clap::{Parser, Subcommand};

mod build;
mod config;
mod optimize;
mod stats;
mod utils;
mod watch;

use build::BuildCommand;
use config::ConfigCommand;
use optimize::OptimizeCommand;
use stats::StatsCommand;
use watch::WatchCommand;

/// Tailwind-rs CLI - The first-class Tailwind CSS integration for Rust web frameworks
#[derive(Parser)]
#[command(name = "tailwind-rs")]
#[command(about = "Tailwind-rs CLI - Build and optimize Tailwind CSS for Rust web frameworks")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build Tailwind CSS from Rust source files
    Build(BuildCommand),
    /// Watch for changes and rebuild automatically
    Watch(WatchCommand),
    /// Optimize CSS output
    Optimize(OptimizeCommand),
    /// Manage configuration
    Config(ConfigCommand),
    /// Show build statistics and project information
    Stats(StatsCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Build(cmd) => cmd.execute().await,
        Commands::Watch(cmd) => cmd.execute().await,
        Commands::Optimize(cmd) => cmd.execute().await,
        Commands::Config(cmd) => cmd.execute().await,
        Commands::Stats(cmd) => cmd.execute().await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_cli_help() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "--help"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Tailwind-rs CLI"));
    }

    #[test]
    fn test_cli_version() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "--version"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("tailwind-rs"));
    }

    #[test]
    fn test_build_command_help() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "build", "--help"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Build Tailwind CSS"));
    }

    #[test]
    fn test_watch_command_help() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "watch", "--help"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Watch for changes"));
    }

    #[test]
    fn test_optimize_command_help() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "optimize", "--help"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Optimize CSS output"));
    }

    #[test]
    fn test_config_command_help() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "config", "--help"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Manage configuration"));
    }

    #[test]
    fn test_stats_command_help() {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "tailwind-rs", "--", "stats", "--help"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Show build statistics"));
    }
}
