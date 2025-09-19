//! TOML configuration structures for tailwind-rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TOML representation of the main configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TailwindConfigToml {
    pub build: BuildConfigToml,
    pub theme: crate::theme::ThemeToml,
    pub responsive: ResponsiveConfigToml,
    pub plugins: Option<Vec<String>>,
    pub custom: Option<HashMap<String, toml::Value>>,
}

/// TOML representation of build configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildConfigToml {
    pub input: Option<Vec<String>>,
    pub output: Option<String>,
    pub watch: Option<bool>,
    pub minify: Option<bool>,
    pub source_maps: Option<bool>,
    pub purge: Option<bool>,
    pub additional_css: Option<Vec<String>>,
    pub postcss_plugins: Option<Vec<String>>,
}

/// TOML representation of responsive configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfigToml {
    pub breakpoints: HashMap<String, u32>,
    pub container_centering: bool,
    pub container_padding: u32,
}

impl From<super::TailwindConfig> for TailwindConfigToml {
    fn from(config: super::TailwindConfig) -> Self {
        Self {
            build: BuildConfigToml {
                input: Some(config.build.input),
                output: Some(config.build.output),
                watch: Some(config.build.watch),
                minify: Some(config.build.minify),
                source_maps: Some(config.build.source_maps),
                purge: Some(config.build.purge),
                additional_css: Some(config.build.additional_css),
                postcss_plugins: Some(config.build.postcss_plugins),
            },
            theme: config.theme.into(),
            responsive: ResponsiveConfigToml {
                breakpoints: super::TailwindConfig::convert_breakpoints_to_toml(&config.responsive.breakpoints),
                container_centering: false, // Default value since this field doesn't exist in ResponsiveConfig
                container_padding: 0, // Default value since this field doesn't exist in ResponsiveConfig
            },
            plugins: Some(config.plugins),
            custom: Some(super::TailwindConfig::convert_json_to_toml_values(&config.custom)),
        }
    }
}

impl From<ResponsiveConfigToml> for crate::responsive::ResponsiveConfig {
    fn from(toml_config: ResponsiveConfigToml) -> Self {
        let mut breakpoints = HashMap::new();
        for (key, width) in toml_config.breakpoints {
            if let Ok(breakpoint) = key.parse::<crate::responsive::Breakpoint>() {
                breakpoints.insert(breakpoint, crate::responsive::responsive_config::BreakpointConfig { 
                    min_width: width,
                    max_width: None,
                    enabled: true,
                    media_query: None,
                });
            }
        }
        
        Self {
            breakpoints,
            defaults: crate::responsive::responsive_config::ResponsiveDefaults {
                default_breakpoint: crate::responsive::Breakpoint::Sm,
                include_base: true,
                mobile_first: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_config_conversion() {
        let config = crate::config::TailwindConfig::new();
        let toml_config: TailwindConfigToml = config.clone().into();
        
        assert_eq!(toml_config.build.output, Some(config.build.output));
        assert_eq!(toml_config.build.minify, Some(config.build.minify));
    }

    #[test]
    fn test_responsive_config_conversion() {
        let mut breakpoints = HashMap::new();
        breakpoints.insert("sm".to_string(), 640);
        breakpoints.insert("md".to_string(), 768);
        
        let toml_config = ResponsiveConfigToml {
            breakpoints,
            container_centering: true,
            container_padding: 16,
        };
        
        let responsive_config: crate::responsive::ResponsiveConfig = toml_config.into();
        assert!(!responsive_config.breakpoints.is_empty());
    }
}
