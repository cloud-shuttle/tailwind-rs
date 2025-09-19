//! # Config Command
//!
//! This module handles the config command for managing Tailwind-rs configuration.

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::utils::{FileUtils, LogUtils};

/// Manage configuration
#[derive(Parser)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(clap::Subcommand)]
pub enum ConfigAction {
    /// Initialize a new configuration file
    Init {
        /// Configuration file path
        #[arg(short, long, default_value = "tailwind-rs.toml")]
        file: PathBuf,

        /// Use default configuration
        #[arg(long)]
        default: bool,
    },
    /// Validate configuration file
    Validate {
        /// Configuration file path
        #[arg(short, long, default_value = "tailwind-rs.toml")]
        file: PathBuf,
    },
    /// Show current configuration
    Show {
        /// Configuration file path
        #[arg(short, long, default_value = "tailwind-rs.toml")]
        file: PathBuf,
    },
}

/// Tailwind-rs configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TailwindRsConfig {
    /// Build configuration
    pub build: BuildConfig,
    /// Optimization configuration
    pub optimize: OptimizeConfig,
    /// Watch configuration
    pub watch: WatchConfig,
}

/// Build configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Source directory
    pub source: String,
    /// Output CSS file
    pub output: String,
    /// Enable tree-shaking
    pub tree_shake: bool,
    /// Enable minification
    pub minify: bool,
    /// Enable source maps
    pub source_maps: bool,
}

/// Optimization configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeConfig {
    /// Optimization level (1-3)
    pub level: u8,
    /// Remove unused classes
    pub remove_unused: bool,
    /// Minify CSS
    pub minify: bool,
    /// Generate source maps
    pub source_maps: bool,
}

/// Watch configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchConfig {
    /// Debounce delay in milliseconds
    pub debounce: u64,
    /// Watch patterns
    pub patterns: Vec<String>,
}

impl ConfigCommand {
    /// Execute the config command
    pub async fn execute(&self) -> Result<()> {
        match &self.action {
            ConfigAction::Init { file, default } => self.init_config(file, *default).await,
            ConfigAction::Validate { file } => self.validate_config(file).await,
            ConfigAction::Show { file } => self.show_config(file).await,
        }
    }

    /// Initialize a new configuration file
    async fn init_config(&self, file: &PathBuf, default: bool) -> Result<()> {
        // Check if file already exists
        if FileUtils::file_exists(file) {
            LogUtils::warning(&format!("Configuration file already exists: {:?}", file));
            return Ok(());
        }

        // Ensure directory exists
        if let Some(parent) = file.parent() {
            FileUtils::ensure_dir(parent)?;
        }

        let config = if default {
            LogUtils::info("Creating default configuration...");
            Self::default_config()
        } else {
            LogUtils::info("Creating interactive configuration...");
            Self::interactive_config()?
        };

        let toml_content = toml::to_string_pretty(&config)?;
        FileUtils::write_file(file, &toml_content)?;

        LogUtils::success(&format!("Configuration file created: {:?}", file));
        Ok(())
    }

    /// Validate configuration file
    async fn validate_config(&self, file: &PathBuf) -> Result<()> {
        if !FileUtils::file_exists(file) {
            LogUtils::error(&format!("Configuration file does not exist: {:?}", file));
            return Err(anyhow::anyhow!("Configuration file not found"));
        }

        let content = FileUtils::read_file(file)?;
        let _config: TailwindRsConfig = toml::from_str(&content)?;

        LogUtils::success(&format!("Configuration file is valid: {:?}", file));
        Ok(())
    }

    /// Show current configuration
    async fn show_config(&self, file: &PathBuf) -> Result<()> {
        if !FileUtils::file_exists(file) {
            LogUtils::error(&format!("Configuration file does not exist: {:?}", file));
            return Err(anyhow::anyhow!("Configuration file not found"));
        }

        let content = FileUtils::read_file(file)?;
        let config: TailwindRsConfig = toml::from_str(&content)?;

        LogUtils::info("Current configuration:");
        println!("{:#?}", config);
        Ok(())
    }

    /// Create default configuration
    fn default_config() -> TailwindRsConfig {
        TailwindRsConfig {
            build: BuildConfig {
                source: "src".to_string(),
                output: "dist/styles.css".to_string(),
                tree_shake: true,
                minify: true,
                source_maps: false,
            },
            optimize: OptimizeConfig {
                level: 3,
                remove_unused: true,
                minify: true,
                source_maps: false,
            },
            watch: WatchConfig {
                debounce: 300,
                patterns: vec!["**/*.rs".to_string()],
            },
        }
    }

    /// Create interactive configuration
    fn interactive_config() -> Result<TailwindRsConfig> {
        // For now, just return the default config
        // In a real implementation, this would prompt the user for input
        Ok(Self::default_config())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_command_parsing() {
        use crate::Cli;
        
        let args = vec!["tailwind-rs", "config", "init", "--file", "custom.toml"];

        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            crate::Commands::Config(cmd) => {
                match cmd.action {
                    ConfigAction::Init { file, default } => {
                        assert_eq!(file, PathBuf::from("custom.toml"));
                        assert!(!default);
                    }
                    _ => panic!("Expected Init action"),
                }
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_default_config() {
        let config = ConfigCommand::default_config();

        assert_eq!(config.build.source, "src");
        assert_eq!(config.build.output, "dist/styles.css");
        assert!(config.build.tree_shake);
        assert!(config.build.minify);
        assert!(!config.build.source_maps);

        assert_eq!(config.optimize.level, 3);
        assert!(config.optimize.remove_unused);
        assert!(config.optimize.minify);
        assert!(!config.optimize.source_maps);

        assert_eq!(config.watch.debounce, 300);
        assert_eq!(config.watch.patterns, vec!["**/*.rs"]);
    }

    #[test]
    fn test_config_serialization() {
        let config = ConfigCommand::default_config();
        let toml_content = toml::to_string_pretty(&config).unwrap();

        // Should be able to deserialize it back
        let deserialized: TailwindRsConfig = toml::from_str(&toml_content).unwrap();

        assert_eq!(config.build.source, deserialized.build.source);
        assert_eq!(config.build.output, deserialized.build.output);
        assert_eq!(config.build.tree_shake, deserialized.build.tree_shake);
        assert_eq!(config.build.minify, deserialized.build.minify);
        assert_eq!(config.build.source_maps, deserialized.build.source_maps);
    }

    #[tokio::test]
    async fn test_config_init() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("tailwind-rs.toml");

        let cmd = ConfigCommand {
            action: ConfigAction::Init {
                file: config_file.clone(),
                default: true,
            },
        };

        cmd.execute().await.unwrap();

        // Check that the file was created
        assert!(config_file.exists());

        // Check that it contains valid TOML
        let content = fs::read_to_string(&config_file).unwrap();
        let _config: TailwindRsConfig = toml::from_str(&content).unwrap();
    }

    #[tokio::test]
    async fn test_config_validate() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("tailwind-rs.toml");

        // Create a valid config file
        let config = ConfigCommand::default_config();
        let toml_content = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_file, toml_content).unwrap();

        let cmd = ConfigCommand {
            action: ConfigAction::Validate { file: config_file },
        };

        // Should not panic
        cmd.execute().await.unwrap();
    }
}
