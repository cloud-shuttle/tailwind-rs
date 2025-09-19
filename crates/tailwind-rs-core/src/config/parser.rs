//! Configuration parser utilities

use super::TailwindConfig;
use crate::error::Result;
use std::path::Path;

/// Configuration parser with validation
pub struct ConfigParser;

impl ConfigParser {
    /// Create a new configuration parser
    pub fn new() -> Self {
        Self
    }

    /// Parse configuration from TOML string
    pub fn parse_toml(&self, content: &str) -> Result<TailwindConfig> {
        TailwindConfig::from_str(content)
    }

    /// Parse configuration from file
    pub fn parse_file(&self, path: &Path) -> Result<TailwindConfig> {
        TailwindConfig::from_file(path)
    }

    /// Validate configuration
    pub fn validate(&self, config: &TailwindConfig) -> Result<()> {
        config.validate()
    }
}

impl Default for ConfigParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = ConfigParser::new();
        assert!(parser.validate(&TailwindConfig::new()).is_ok());
    }

    #[test]
    fn test_toml_parsing() {
        let parser = ConfigParser::new();
        let toml_content = r#"
[build]
output = "dist/styles.css"
minify = true

[theme]
name = "default"

[responsive]
breakpoints = { sm = 640, md = 768 }
container_centering = true
container_padding = 16
"#;

        let config = parser.parse_toml(toml_content).unwrap();
        assert_eq!(config.build.output, "dist/styles.css");
        assert!(config.build.minify);
    }
}
