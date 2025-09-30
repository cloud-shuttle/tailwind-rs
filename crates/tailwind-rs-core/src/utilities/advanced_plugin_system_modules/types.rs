//! Advanced Plugin System Types Module
//!
//! Core types and enums for the advanced plugin system:
//! - PluginType: Different types of plugins (Utility, Component, Base, Variant, Custom)
//! - PluginPriority: Priority levels for plugin execution
//! - PluginConfig: Configuration options for plugins
//! - PluginComposition: Strategies for composing plugins

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Plugin type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginType {
    /// Utility plugin
    Utility,
    /// Component plugin
    Component,
    /// Base plugin
    Base,
    /// Variant plugin
    Variant,
    /// Custom plugin
    Custom(String),
}

impl fmt::Display for PluginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginType::Utility => write!(f, "utility"),
            PluginType::Component => write!(f, "component"),
            PluginType::Base => write!(f, "base"),
            PluginType::Variant => write!(f, "variant"),
            PluginType::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl PluginType {
    /// Get the CSS class name for this plugin type
    pub fn to_class_name(&self) -> String {
        match self {
            PluginType::Utility => "plugin-utility".to_string(),
            PluginType::Component => "plugin-component".to_string(),
            PluginType::Base => "plugin-base".to_string(),
            PluginType::Variant => "plugin-variant".to_string(),
            PluginType::Custom(name) => format!("plugin-{}", name),
        }
    }

    /// Get the CSS value for this plugin type
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }

    /// Check if this is a standard plugin type
    pub fn is_standard(&self) -> bool {
        !matches!(self, PluginType::Custom(_))
    }

    /// Check if this plugin type requires composition
    pub fn requires_composition(&self) -> bool {
        matches!(self, PluginType::Component | PluginType::Variant)
    }
}

/// Plugin priority levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
    /// Custom priority value
    Custom(u32),
}

impl fmt::Display for PluginPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginPriority::Low => write!(f, "low"),
            PluginPriority::Normal => write!(f, "normal"),
            PluginPriority::High => write!(f, "high"),
            PluginPriority::Critical => write!(f, "critical"),
            PluginPriority::Custom(value) => write!(f, "{}", value),
        }
    }
}

impl PluginPriority {
    /// Get the CSS class name for this plugin priority
    pub fn to_class_name(&self) -> String {
        match self {
            PluginPriority::Low => "plugin-priority-low".to_string(),
            PluginPriority::Normal => "plugin-priority-normal".to_string(),
            PluginPriority::High => "plugin-priority-high".to_string(),
            PluginPriority::Critical => "plugin-priority-critical".to_string(),
            PluginPriority::Custom(value) => format!("plugin-priority-{}", value),
        }
    }

    /// Get the CSS value for this plugin priority
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }

    /// Get the numeric value for priority comparison
    pub fn numeric_value(&self) -> u32 {
        match self {
            PluginPriority::Low => 1,
            PluginPriority::Normal => 2,
            PluginPriority::High => 3,
            PluginPriority::Critical => 4,
            PluginPriority::Custom(value) => *value,
        }
    }

    /// Check if this priority is higher than another
    pub fn is_higher_than(&self, other: &PluginPriority) -> bool {
        self.numeric_value() > other.numeric_value()
    }

    /// Check if this priority is lower than another
    pub fn is_lower_than(&self, other: &PluginPriority) -> bool {
        self.numeric_value() < other.numeric_value()
    }
}

impl Default for PluginPriority {
    fn default() -> Self {
        PluginPriority::Normal
    }
}

/// Plugin configuration options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginConfig {
    /// Enable plugin
    Enable,
    /// Disable plugin
    Disable,
    /// Configure plugin with options
    Configure(HashMap<String, String>),
    /// Custom configuration
    Custom(String),
}

impl fmt::Display for PluginConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginConfig::Enable => write!(f, "enable"),
            PluginConfig::Disable => write!(f, "disable"),
            PluginConfig::Configure(options) => {
                let options_str = options
                    .iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "configure:{}", options_str)
            }
            PluginConfig::Custom(config) => write!(f, "{}", config),
        }
    }
}

impl PluginConfig {
    /// Get the CSS class name for this plugin config
    pub fn to_class_name(&self) -> String {
        match self {
            PluginConfig::Enable => "plugin-config-enable".to_string(),
            PluginConfig::Disable => "plugin-config-disable".to_string(),
            PluginConfig::Configure(_) => "plugin-config-configure".to_string(),
            PluginConfig::Custom(config) => format!("plugin-config-{}", config),
        }
    }

    /// Get the CSS value for this plugin config
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }

    /// Check if this configuration enables the plugin
    pub fn is_enabled(&self) -> bool {
        matches!(self, PluginConfig::Enable)
    }

    /// Check if this configuration disables the plugin
    pub fn is_disabled(&self) -> bool {
        matches!(self, PluginConfig::Disable)
    }

    /// Check if this configuration has custom options
    pub fn has_options(&self) -> bool {
        matches!(self, PluginConfig::Configure(_))
    }

    /// Get configuration options if available
    pub fn get_options(&self) -> Option<&HashMap<String, String>> {
        match self {
            PluginConfig::Configure(options) => Some(options),
            _ => None,
        }
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig::Enable
    }
}

/// Plugin composition strategies
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginComposition {
    /// Merge plugins
    Merge,
    /// Override plugins
    Override,
    /// Extend plugins
    Extend,
    /// Replace plugins
    Replace,
    /// Custom composition strategy
    Custom(String),
}

impl fmt::Display for PluginComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginComposition::Merge => write!(f, "merge"),
            PluginComposition::Override => write!(f, "override"),
            PluginComposition::Extend => write!(f, "extend"),
            PluginComposition::Replace => write!(f, "replace"),
            PluginComposition::Custom(strategy) => write!(f, "{}", strategy),
        }
    }
}

impl PluginComposition {
    /// Get the CSS class name for this composition strategy
    pub fn to_class_name(&self) -> String {
        match self {
            PluginComposition::Merge => "plugin-composition-merge".to_string(),
            PluginComposition::Override => "plugin-composition-override".to_string(),
            PluginComposition::Extend => "plugin-composition-extend".to_string(),
            PluginComposition::Replace => "plugin-composition-replace".to_string(),
            PluginComposition::Custom(strategy) => format!("plugin-composition-{}", strategy),
        }
    }

    /// Get the CSS value for this composition strategy
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }

    /// Check if this composition allows merging
    pub fn allows_merging(&self) -> bool {
        matches!(self, PluginComposition::Merge | PluginComposition::Extend)
    }

    /// Check if this composition allows overriding
    pub fn allows_overriding(&self) -> bool {
        matches!(self, PluginComposition::Override | PluginComposition::Replace)
    }
}

impl Default for PluginComposition {
    fn default() -> Self {
        PluginComposition::Merge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_type_display() {
        assert_eq!(PluginType::Utility.to_string(), "utility");
        assert_eq!(PluginType::Component.to_string(), "component");
        assert_eq!(PluginType::Custom("test".to_string()).to_string(), "test");
    }

    #[test]
    fn plugin_type_class_names() {
        assert_eq!(PluginType::Utility.to_class_name(), "plugin-utility");
        assert_eq!(PluginType::Custom("test".to_string()).to_class_name(), "plugin-test");
    }

    #[test]
    fn plugin_type_properties() {
        assert!(PluginType::Utility.is_standard());
        assert!(!PluginType::Custom("test".to_string()).is_standard());

        assert!(PluginType::Component.requires_composition());
        assert!(!PluginType::Utility.requires_composition());
    }

    #[test]
    fn plugin_priority_numeric_values() {
        assert_eq!(PluginPriority::Low.numeric_value(), 1);
        assert_eq!(PluginPriority::Normal.numeric_value(), 2);
        assert_eq!(PluginPriority::High.numeric_value(), 3);
        assert_eq!(PluginPriority::Critical.numeric_value(), 4);
        assert_eq!(PluginPriority::Custom(5).numeric_value(), 5);
    }

    #[test]
    fn plugin_priority_comparison() {
        assert!(PluginPriority::High.is_higher_than(&PluginPriority::Low));
        assert!(PluginPriority::Low.is_lower_than(&PluginPriority::Normal));
        assert!(!PluginPriority::High.is_lower_than(&PluginPriority::Low));
    }

    #[test]
    fn plugin_config_properties() {
        assert!(PluginConfig::Enable.is_enabled());
        assert!(PluginConfig::Disable.is_disabled());
        assert!(!PluginConfig::Enable.is_disabled());

        let mut options = HashMap::new();
        options.insert("key".to_string(), "value".to_string());
        let config = PluginConfig::Configure(options.clone());

        assert!(config.has_options());
        assert_eq!(config.get_options().unwrap(), &options);
    }

    #[test]
    fn plugin_composition_properties() {
        assert!(PluginComposition::Merge.allows_merging());
        assert!(PluginComposition::Extend.allows_merging());
        assert!(!PluginComposition::Merge.allows_overriding());

        assert!(PluginComposition::Override.allows_overriding());
        assert!(PluginComposition::Replace.allows_overriding());
        assert!(!PluginComposition::Merge.allows_overriding());
    }
}
