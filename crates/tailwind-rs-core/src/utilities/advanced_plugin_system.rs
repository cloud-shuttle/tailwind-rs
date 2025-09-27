//! Advanced Plugin System utilities for tailwind-rs
//!
//! This module provides utilities for advanced plugin system features.
//! It includes support for custom plugins, plugin composition, and plugin management.

use crate::classes::ClassBuilder;
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
}

/// Plugin composition strategies
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginComposition {
    /// Replace existing plugin
    Replace,
    /// Merge with existing plugin
    Merge,
    /// Extend existing plugin
    Extend,
    /// Prepend to existing plugin
    Prepend,
    /// Append to existing plugin
    Append,
    /// Custom composition
    Custom(String),
}

impl fmt::Display for PluginComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginComposition::Replace => write!(f, "replace"),
            PluginComposition::Merge => write!(f, "merge"),
            PluginComposition::Extend => write!(f, "extend"),
            PluginComposition::Prepend => write!(f, "prepend"),
            PluginComposition::Append => write!(f, "append"),
            PluginComposition::Custom(composition) => write!(f, "{}", composition),
        }
    }
}

impl PluginComposition {
    /// Get the CSS class name for this plugin composition
    pub fn to_class_name(&self) -> String {
        match self {
            PluginComposition::Replace => "plugin-composition-replace".to_string(),
            PluginComposition::Merge => "plugin-composition-merge".to_string(),
            PluginComposition::Extend => "plugin-composition-extend".to_string(),
            PluginComposition::Prepend => "plugin-composition-prepend".to_string(),
            PluginComposition::Append => "plugin-composition-append".to_string(),
            PluginComposition::Custom(composition) => format!("plugin-composition-{}", composition),
        }
    }

    /// Get the CSS value for this plugin composition
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Plugin lifecycle stages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginLifecycle {
    /// Plugin initialization
    Initialize,
    /// Plugin configuration
    Configure,
    /// Plugin execution
    Execute,
    /// Plugin cleanup
    Cleanup,
    /// Custom lifecycle stage
    Custom(String),
}

impl fmt::Display for PluginLifecycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginLifecycle::Initialize => write!(f, "initialize"),
            PluginLifecycle::Configure => write!(f, "configure"),
            PluginLifecycle::Execute => write!(f, "execute"),
            PluginLifecycle::Cleanup => write!(f, "cleanup"),
            PluginLifecycle::Custom(stage) => write!(f, "{}", stage),
        }
    }
}

impl PluginLifecycle {
    /// Get the CSS class name for this plugin lifecycle
    pub fn to_class_name(&self) -> String {
        match self {
            PluginLifecycle::Initialize => "plugin-lifecycle-initialize".to_string(),
            PluginLifecycle::Configure => "plugin-lifecycle-configure".to_string(),
            PluginLifecycle::Execute => "plugin-lifecycle-execute".to_string(),
            PluginLifecycle::Cleanup => "plugin-lifecycle-cleanup".to_string(),
            PluginLifecycle::Custom(stage) => format!("plugin-lifecycle-{}", stage),
        }
    }

    /// Get the CSS value for this plugin lifecycle
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Trait for adding advanced plugin system to ClassBuilder
pub trait AdvancedPluginSystemUtilities {
    /// Set plugin type
    fn plugin_type(self, plugin_type: PluginType) -> Self;
    /// Set plugin priority
    fn plugin_priority(self, priority: PluginPriority) -> Self;
    /// Set plugin configuration
    fn plugin_config(self, config: PluginConfig) -> Self;
    /// Set plugin composition
    fn plugin_composition(self, composition: PluginComposition) -> Self;
    /// Set plugin lifecycle
    fn plugin_lifecycle(self, lifecycle: PluginLifecycle) -> Self;
    /// Set plugin with custom options
    fn plugin_custom(self, name: &str, options: HashMap<String, String>) -> Self;
}

impl AdvancedPluginSystemUtilities for ClassBuilder {
    fn plugin_type(self, plugin_type: PluginType) -> Self {
        self.class(plugin_type.to_class_name())
    }

    fn plugin_priority(self, priority: PluginPriority) -> Self {
        self.class(priority.to_class_name())
    }

    fn plugin_config(self, config: PluginConfig) -> Self {
        self.class(config.to_class_name())
    }

    fn plugin_composition(self, composition: PluginComposition) -> Self {
        self.class(composition.to_class_name())
    }

    fn plugin_lifecycle(self, lifecycle: PluginLifecycle) -> Self {
        self.class(lifecycle.to_class_name())
    }

    fn plugin_custom(self, name: &str, _options: HashMap<String, String>) -> Self {
        let plugin_class = format!("plugin-{}", name);
        self.class(&plugin_class)
    }
}

/// Convenience methods for common plugin patterns
pub trait AdvancedPluginSystemConvenience {
    /// Set utility plugin
    fn plugin_utility(self) -> Self;
    /// Set component plugin
    fn plugin_component(self) -> Self;
    /// Set base plugin
    fn plugin_base(self) -> Self;
    /// Set variant plugin
    fn plugin_variant(self) -> Self;
    /// Set high priority plugin
    fn plugin_high_priority(self) -> Self;
    /// Set critical priority plugin
    fn plugin_critical_priority(self) -> Self;
    /// Set enabled plugin
    fn plugin_enabled(self) -> Self;
    /// Set disabled plugin
    fn plugin_disabled(self) -> Self;
    /// Set merge composition
    fn plugin_merge(self) -> Self;
    /// Set extend composition
    fn plugin_extend(self) -> Self;
    /// Set initialize lifecycle
    fn plugin_initialize(self) -> Self;
    /// Set execute lifecycle
    fn plugin_execute(self) -> Self;
}

impl AdvancedPluginSystemConvenience for ClassBuilder {
    fn plugin_utility(self) -> Self {
        self.plugin_type(PluginType::Utility)
    }

    fn plugin_component(self) -> Self {
        self.plugin_type(PluginType::Component)
    }

    fn plugin_base(self) -> Self {
        self.plugin_type(PluginType::Base)
    }

    fn plugin_variant(self) -> Self {
        self.plugin_type(PluginType::Variant)
    }

    fn plugin_high_priority(self) -> Self {
        self.plugin_priority(PluginPriority::High)
    }

    fn plugin_critical_priority(self) -> Self {
        self.plugin_priority(PluginPriority::Critical)
    }

    fn plugin_enabled(self) -> Self {
        self.plugin_config(PluginConfig::Enable)
    }

    fn plugin_disabled(self) -> Self {
        self.plugin_config(PluginConfig::Disable)
    }

    fn plugin_merge(self) -> Self {
        self.plugin_composition(PluginComposition::Merge)
    }

    fn plugin_extend(self) -> Self {
        self.plugin_composition(PluginComposition::Extend)
    }

    fn plugin_initialize(self) -> Self {
        self.plugin_lifecycle(PluginLifecycle::Initialize)
    }

    fn plugin_execute(self) -> Self {
        self.plugin_lifecycle(PluginLifecycle::Execute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_plugin_type_enum_values() {
        assert_eq!(PluginType::Utility.to_string(), "utility");
        assert_eq!(PluginType::Component.to_string(), "component");
        assert_eq!(PluginType::Base.to_string(), "base");
        assert_eq!(PluginType::Variant.to_string(), "variant");
        assert_eq!(
            PluginType::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_plugin_type_class_names() {
        assert_eq!(PluginType::Utility.to_class_name(), "plugin-utility");
        assert_eq!(PluginType::Component.to_class_name(), "plugin-component");
        assert_eq!(PluginType::Base.to_class_name(), "plugin-base");
        assert_eq!(PluginType::Variant.to_class_name(), "plugin-variant");
        assert_eq!(
            PluginType::Custom("custom".to_string()).to_class_name(),
            "plugin-custom"
        );
    }

    #[test]
    fn test_plugin_priority_enum_values() {
        assert_eq!(PluginPriority::Low.to_string(), "low");
        assert_eq!(PluginPriority::Normal.to_string(), "normal");
        assert_eq!(PluginPriority::High.to_string(), "high");
        assert_eq!(PluginPriority::Critical.to_string(), "critical");
        assert_eq!(PluginPriority::Custom(42).to_string(), "42");
    }

    #[test]
    fn test_plugin_priority_class_names() {
        assert_eq!(PluginPriority::Low.to_class_name(), "plugin-priority-low");
        assert_eq!(
            PluginPriority::Normal.to_class_name(),
            "plugin-priority-normal"
        );
        assert_eq!(PluginPriority::High.to_class_name(), "plugin-priority-high");
        assert_eq!(
            PluginPriority::Critical.to_class_name(),
            "plugin-priority-critical"
        );
        assert_eq!(
            PluginPriority::Custom(42).to_class_name(),
            "plugin-priority-42"
        );
    }

    #[test]
    fn test_plugin_config_enum_values() {
        assert_eq!(PluginConfig::Enable.to_string(), "enable");
        assert_eq!(PluginConfig::Disable.to_string(), "disable");
        assert_eq!(
            PluginConfig::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_plugin_config_class_names() {
        assert_eq!(PluginConfig::Enable.to_class_name(), "plugin-config-enable");
        assert_eq!(
            PluginConfig::Disable.to_class_name(),
            "plugin-config-disable"
        );
        assert_eq!(
            PluginConfig::Custom("custom".to_string()).to_class_name(),
            "plugin-config-custom"
        );
    }

    #[test]
    fn test_plugin_composition_enum_values() {
        assert_eq!(PluginComposition::Replace.to_string(), "replace");
        assert_eq!(PluginComposition::Merge.to_string(), "merge");
        assert_eq!(PluginComposition::Extend.to_string(), "extend");
        assert_eq!(PluginComposition::Prepend.to_string(), "prepend");
        assert_eq!(PluginComposition::Append.to_string(), "append");
        assert_eq!(
            PluginComposition::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_plugin_composition_class_names() {
        assert_eq!(
            PluginComposition::Replace.to_class_name(),
            "plugin-composition-replace"
        );
        assert_eq!(
            PluginComposition::Merge.to_class_name(),
            "plugin-composition-merge"
        );
        assert_eq!(
            PluginComposition::Extend.to_class_name(),
            "plugin-composition-extend"
        );
        assert_eq!(
            PluginComposition::Prepend.to_class_name(),
            "plugin-composition-prepend"
        );
        assert_eq!(
            PluginComposition::Append.to_class_name(),
            "plugin-composition-append"
        );
        assert_eq!(
            PluginComposition::Custom("custom".to_string()).to_class_name(),
            "plugin-composition-custom"
        );
    }

    #[test]
    fn test_plugin_lifecycle_enum_values() {
        assert_eq!(PluginLifecycle::Initialize.to_string(), "initialize");
        assert_eq!(PluginLifecycle::Configure.to_string(), "configure");
        assert_eq!(PluginLifecycle::Execute.to_string(), "execute");
        assert_eq!(PluginLifecycle::Cleanup.to_string(), "cleanup");
        assert_eq!(
            PluginLifecycle::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_plugin_lifecycle_class_names() {
        assert_eq!(
            PluginLifecycle::Initialize.to_class_name(),
            "plugin-lifecycle-initialize"
        );
        assert_eq!(
            PluginLifecycle::Configure.to_class_name(),
            "plugin-lifecycle-configure"
        );
        assert_eq!(
            PluginLifecycle::Execute.to_class_name(),
            "plugin-lifecycle-execute"
        );
        assert_eq!(
            PluginLifecycle::Cleanup.to_class_name(),
            "plugin-lifecycle-cleanup"
        );
        assert_eq!(
            PluginLifecycle::Custom("custom".to_string()).to_class_name(),
            "plugin-lifecycle-custom"
        );
    }

    #[test]
    fn test_advanced_plugin_system_utilities() {
        let classes = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .plugin_priority(PluginPriority::High)
            .plugin_config(PluginConfig::Enable)
            .plugin_composition(PluginComposition::Merge)
            .plugin_lifecycle(PluginLifecycle::Execute);

        let result = classes.build();
        assert!(result.classes.contains("plugin-utility"));
        assert!(result.classes.contains("plugin-priority-high"));
        assert!(result.classes.contains("plugin-config-enable"));
        assert!(result.classes.contains("plugin-composition-merge"));
        assert!(result.classes.contains("plugin-lifecycle-execute"));
    }

    #[test]
    fn test_advanced_plugin_system_convenience() {
        let classes = ClassBuilder::new()
            .plugin_utility()
            .plugin_component()
            .plugin_base()
            .plugin_variant()
            .plugin_high_priority()
            .plugin_critical_priority()
            .plugin_enabled()
            .plugin_disabled()
            .plugin_merge()
            .plugin_extend()
            .plugin_initialize()
            .plugin_execute();

        let result = classes.build();
        assert!(result.classes.contains("plugin-utility"));
        assert!(result.classes.contains("plugin-component"));
        assert!(result.classes.contains("plugin-base"));
        assert!(result.classes.contains("plugin-variant"));
        assert!(result.classes.contains("plugin-priority-high"));
        assert!(result.classes.contains("plugin-priority-critical"));
        assert!(result.classes.contains("plugin-config-enable"));
        assert!(result.classes.contains("plugin-config-disable"));
        assert!(result.classes.contains("plugin-composition-merge"));
        assert!(result.classes.contains("plugin-composition-extend"));
        assert!(result.classes.contains("plugin-lifecycle-initialize"));
        assert!(result.classes.contains("plugin-lifecycle-execute"));
    }

    #[test]
    fn test_advanced_plugin_system_serialization() {
        let plugin_type = PluginType::Utility;
        let serialized = serde_json::to_string(&plugin_type).unwrap();
        let deserialized: PluginType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(plugin_type, deserialized);

        let plugin_priority = PluginPriority::High;
        let serialized = serde_json::to_string(&plugin_priority).unwrap();
        let deserialized: PluginPriority = serde_json::from_str(&serialized).unwrap();
        assert_eq!(plugin_priority, deserialized);

        let plugin_config = PluginConfig::Enable;
        let serialized = serde_json::to_string(&plugin_config).unwrap();
        let deserialized: PluginConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(plugin_config, deserialized);

        let plugin_composition = PluginComposition::Merge;
        let serialized = serde_json::to_string(&plugin_composition).unwrap();
        let deserialized: PluginComposition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(plugin_composition, deserialized);

        let plugin_lifecycle = PluginLifecycle::Execute;
        let serialized = serde_json::to_string(&plugin_lifecycle).unwrap();
        let deserialized: PluginLifecycle = serde_json::from_str(&serialized).unwrap();
        assert_eq!(plugin_lifecycle, deserialized);
    }

    #[test]
    fn test_advanced_plugin_system_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .plugin_type(PluginType::Utility)
            .plugin_priority(PluginPriority::High)
            .plugin_config(PluginConfig::Enable)
            .plugin_composition(PluginComposition::Merge)
            .plugin_lifecycle(PluginLifecycle::Execute)
            .plugin_utility()
            .plugin_component()
            .plugin_base()
            .plugin_variant()
            .plugin_high_priority()
            .plugin_critical_priority()
            .plugin_enabled()
            .plugin_disabled()
            .plugin_merge()
            .plugin_extend()
            .plugin_initialize()
            .plugin_execute();

        let result = classes.build();
        assert!(result.classes.contains("plugin-utility"));
        assert!(result.classes.contains("plugin-priority-high"));
        assert!(result.classes.contains("plugin-config-enable"));
        assert!(result.classes.contains("plugin-composition-merge"));
        assert!(result.classes.contains("plugin-lifecycle-execute"));
        assert!(result.classes.contains("plugin-component"));
        assert!(result.classes.contains("plugin-base"));
        assert!(result.classes.contains("plugin-variant"));
        assert!(result.classes.contains("plugin-priority-critical"));
        assert!(result.classes.contains("plugin-config-disable"));
        assert!(result.classes.contains("plugin-composition-extend"));
        assert!(result.classes.contains("plugin-lifecycle-initialize"));
    }
}
