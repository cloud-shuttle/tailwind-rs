//! Modern CSS features utilities for tailwind-rs
//!
//! This module provides utilities for modern CSS features.
//! It includes cascade layers, custom properties, and other modern CSS capabilities.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Cascade layer values
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CascadeLayer {
    /// Base layer
    Base,
    /// Components layer
    Components,
    /// Utilities layer
    Utilities,
    /// Custom layer
    Custom(String),
}

impl fmt::Display for CascadeLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CascadeLayer::Base => write!(f, "base"),
            CascadeLayer::Components => write!(f, "components"),
            CascadeLayer::Utilities => write!(f, "utilities"),
            CascadeLayer::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl CascadeLayer {
    /// Get the CSS class name for this cascade layer
    pub fn to_class_name(&self) -> String {
        match self {
            CascadeLayer::Base => "layer-base".to_string(),
            CascadeLayer::Components => "layer-components".to_string(),
            CascadeLayer::Utilities => "layer-utilities".to_string(),
            CascadeLayer::Custom(name) => format!("layer-{}", name),
        }
    }

    /// Get the CSS value for this cascade layer
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Custom property values
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomProperty {
    /// Color custom property
    Color(String),
    /// Spacing custom property
    Spacing(String),
    /// Font size custom property
    FontSize(String),
    /// Font weight custom property
    FontWeight(String),
    /// Line height custom property
    LineHeight(String),
    /// Border radius custom property
    BorderRadius(String),
    /// Box shadow custom property
    BoxShadow(String),
    /// Z-index custom property
    ZIndex(String),
    /// Opacity custom property
    Opacity(String),
    /// Transform custom property
    Transform(String),
    /// Animation custom property
    Animation(String),
    /// Transition custom property
    Transition(String),
    /// Generic custom property
    Generic(String, String),
}

impl fmt::Display for CustomProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomProperty::Color(value) => write!(f, "--color: {}", value),
            CustomProperty::Spacing(value) => write!(f, "--spacing: {}", value),
            CustomProperty::FontSize(value) => write!(f, "--font-size: {}", value),
            CustomProperty::FontWeight(value) => write!(f, "--font-weight: {}", value),
            CustomProperty::LineHeight(value) => write!(f, "--line-height: {}", value),
            CustomProperty::BorderRadius(value) => write!(f, "--border-radius: {}", value),
            CustomProperty::BoxShadow(value) => write!(f, "--box-shadow: {}", value),
            CustomProperty::ZIndex(value) => write!(f, "--z-index: {}", value),
            CustomProperty::Opacity(value) => write!(f, "--opacity: {}", value),
            CustomProperty::Transform(value) => write!(f, "--transform: {}", value),
            CustomProperty::Animation(value) => write!(f, "--animation: {}", value),
            CustomProperty::Transition(value) => write!(f, "--transition: {}", value),
            CustomProperty::Generic(name, value) => write!(f, "--{}: {}", name, value),
        }
    }
}

impl CustomProperty {
    /// Get the CSS class name for this custom property
    pub fn to_class_name(&self) -> String {
        match self {
            CustomProperty::Color(_) => "custom-color".to_string(),
            CustomProperty::Spacing(_) => "custom-spacing".to_string(),
            CustomProperty::FontSize(_) => "custom-font-size".to_string(),
            CustomProperty::FontWeight(_) => "custom-font-weight".to_string(),
            CustomProperty::LineHeight(_) => "custom-line-height".to_string(),
            CustomProperty::BorderRadius(_) => "custom-border-radius".to_string(),
            CustomProperty::BoxShadow(_) => "custom-box-shadow".to_string(),
            CustomProperty::ZIndex(_) => "custom-z-index".to_string(),
            CustomProperty::Opacity(_) => "custom-opacity".to_string(),
            CustomProperty::Transform(_) => "custom-transform".to_string(),
            CustomProperty::Animation(_) => "custom-animation".to_string(),
            CustomProperty::Transition(_) => "custom-transition".to_string(),
            CustomProperty::Generic(name, _) => format!("custom-{}", name),
        }
    }

    /// Get the CSS value for this custom property
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Modern container query values
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModernContainerQuery {
    /// Small container
    Small,
    /// Medium container
    Medium,
    /// Large container
    Large,
    /// Extra large container
    ExtraLarge,
    /// Custom container size
    Custom(String),
}

impl fmt::Display for ModernContainerQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModernContainerQuery::Small => write!(f, "small"),
            ModernContainerQuery::Medium => write!(f, "medium"),
            ModernContainerQuery::Large => write!(f, "large"),
            ModernContainerQuery::ExtraLarge => write!(f, "extra-large"),
            ModernContainerQuery::Custom(size) => write!(f, "{}", size),
        }
    }
}

impl ModernContainerQuery {
    /// Get the CSS class name for this container query
    pub fn to_class_name(&self) -> String {
        match self {
            ModernContainerQuery::Small => "container-small".to_string(),
            ModernContainerQuery::Medium => "container-medium".to_string(),
            ModernContainerQuery::Large => "container-large".to_string(),
            ModernContainerQuery::ExtraLarge => "container-extra-large".to_string(),
            ModernContainerQuery::Custom(size) => format!("container-{}", size),
        }
    }

    /// Get the CSS value for this container query
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Trait for adding modern CSS features to ClassBuilder
pub trait ModernCssFeaturesUtilities {
    /// Set cascade layer to base
    fn layer_base(self) -> Self;
    /// Set cascade layer to components
    fn layer_components(self) -> Self;
    /// Set cascade layer to utilities
    fn layer_utilities(self) -> Self;
    /// Set cascade layer with custom name
    fn layer_custom(self, name: &str) -> Self;
    /// Set cascade layer with custom value
    fn layer_custom_value(self, layer: CascadeLayer) -> Self;
    /// Set custom property with name and value
    fn custom_property(self, name: &str, _value: &str) -> Self;
    /// Set custom property with custom value
    fn custom_property_value(self, property: CustomProperty) -> Self;
    /// Set container query to small
    fn container_small(self) -> Self;
    /// Set container query to medium
    fn container_medium(self) -> Self;
    /// Set container query to large
    fn container_large(self) -> Self;
    /// Set container query to extra large
    fn container_extra_large(self) -> Self;
    /// Set container query with custom size
    fn container_custom(self, size: &str) -> Self;
    /// Set container query with custom value
    fn container_custom_value(self, query: ModernContainerQuery) -> Self;
}

impl ModernCssFeaturesUtilities for ClassBuilder {
    fn layer_base(self) -> Self {
        self.class("layer-base")
    }

    fn layer_components(self) -> Self {
        self.class("layer-components")
    }

    fn layer_utilities(self) -> Self {
        self.class("layer-utilities")
    }

    fn layer_custom(self, name: &str) -> Self {
        let class_name = format!("layer-{}", name);
        self.class(class_name)
    }

    fn layer_custom_value(self, layer: CascadeLayer) -> Self {
        self.class(&layer.to_class_name())
    }

    fn custom_property(self, name: &str, value: &str) -> Self {
        self.custom(name, value)
    }

    fn custom_property_value(self, property: CustomProperty) -> Self {
        self.class(&property.to_class_name())
    }

    fn container_small(self) -> Self {
        self.class("container-small")
    }

    fn container_medium(self) -> Self {
        self.class("container-medium")
    }

    fn container_large(self) -> Self {
        self.class("container-large")
    }

    fn container_extra_large(self) -> Self {
        self.class("container-extra-large")
    }

    fn container_custom(self, size: &str) -> Self {
        let class_name = format!("container-{}", size);
        self.class(class_name)
    }

    fn container_custom_value(self, query: ModernContainerQuery) -> Self {
        self.class(&query.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_cascade_layer_enum_values() {
        assert_eq!(CascadeLayer::Base.to_string(), "base");
        assert_eq!(CascadeLayer::Components.to_string(), "components");
        assert_eq!(CascadeLayer::Utilities.to_string(), "utilities");
        assert_eq!(
            CascadeLayer::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_cascade_layer_class_names() {
        assert_eq!(CascadeLayer::Base.to_class_name(), "layer-base");
        assert_eq!(CascadeLayer::Components.to_class_name(), "layer-components");
        assert_eq!(CascadeLayer::Utilities.to_class_name(), "layer-utilities");
        assert_eq!(
            CascadeLayer::Custom("custom".to_string()).to_class_name(),
            "layer-custom"
        );
    }

    #[test]
    fn test_custom_property_enum_values() {
        assert_eq!(
            CustomProperty::Color("red".to_string()).to_string(),
            "--color: red"
        );
        assert_eq!(
            CustomProperty::Spacing("1rem".to_string()).to_string(),
            "--spacing: 1rem"
        );
        assert_eq!(
            CustomProperty::FontSize("16px".to_string()).to_string(),
            "--font-size: 16px"
        );
        assert_eq!(
            CustomProperty::Generic("custom".to_string(), "value".to_string()).to_string(),
            "--custom: value"
        );
    }

    #[test]
    fn test_custom_property_class_names() {
        assert_eq!(
            CustomProperty::Color("red".to_string()).to_class_name(),
            "custom-color"
        );
        assert_eq!(
            CustomProperty::Spacing("1rem".to_string()).to_class_name(),
            "custom-spacing"
        );
        assert_eq!(
            CustomProperty::FontSize("16px".to_string()).to_class_name(),
            "custom-font-size"
        );
        assert_eq!(
            CustomProperty::Generic("custom".to_string(), "value".to_string()).to_class_name(),
            "custom-custom"
        );
    }

    #[test]
    fn test_container_query_enum_values() {
        assert_eq!(ModernContainerQuery::Small.to_string(), "small");
        assert_eq!(ModernContainerQuery::Medium.to_string(), "medium");
        assert_eq!(ModernContainerQuery::Large.to_string(), "large");
        assert_eq!(ModernContainerQuery::ExtraLarge.to_string(), "extra-large");
        assert_eq!(
            ModernContainerQuery::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_container_query_class_names() {
        assert_eq!(
            ModernContainerQuery::Small.to_class_name(),
            "container-small"
        );
        assert_eq!(
            ModernContainerQuery::Medium.to_class_name(),
            "container-medium"
        );
        assert_eq!(
            ModernContainerQuery::Large.to_class_name(),
            "container-large"
        );
        assert_eq!(
            ModernContainerQuery::ExtraLarge.to_class_name(),
            "container-extra-large"
        );
        assert_eq!(
            ModernContainerQuery::Custom("custom".to_string()).to_class_name(),
            "container-custom"
        );
    }

    #[test]
    fn test_modern_css_features_utilities() {
        let classes = ClassBuilder::new()
            .layer_base()
            .layer_components()
            .layer_utilities()
            .custom_property("color", "red")
            .custom_property("spacing", "1rem")
            .container_small()
            .container_medium()
            .container_large();

        let result = classes.build();
        assert!(result.classes.contains("layer-base"));
        assert!(result.classes.contains("layer-components"));
        assert!(result.classes.contains("layer-utilities"));
        assert!(result.custom.contains_key("color"));
        assert_eq!(result.custom.get("color"), Some(&"red".to_string()));
        assert!(result.custom.contains_key("spacing"));
        assert_eq!(result.custom.get("spacing"), Some(&"1rem".to_string()));
        assert!(result.classes.contains("container-small"));
        assert!(result.classes.contains("container-medium"));
        assert!(result.classes.contains("container-large"));
    }

    #[test]
    fn test_modern_css_features_css_values() {
        assert_eq!(CascadeLayer::Base.to_css_value(), "base");
        assert_eq!(CascadeLayer::Components.to_css_value(), "components");
        assert_eq!(CascadeLayer::Utilities.to_css_value(), "utilities");
        assert_eq!(
            CustomProperty::Color("red".to_string()).to_css_value(),
            "--color: red"
        );
        assert_eq!(
            CustomProperty::Spacing("1rem".to_string()).to_css_value(),
            "--spacing: 1rem"
        );
        assert_eq!(ModernContainerQuery::Small.to_css_value(), "small");
        assert_eq!(ModernContainerQuery::Medium.to_css_value(), "medium");
        assert_eq!(ModernContainerQuery::Large.to_css_value(), "large");
    }

    #[test]
    fn test_modern_css_features_serialization() {
        let layer = CascadeLayer::Base;
        let serialized = serde_json::to_string(&layer).unwrap();
        let deserialized: CascadeLayer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(layer, deserialized);

        let property = CustomProperty::Color("red".to_string());
        let serialized = serde_json::to_string(&property).unwrap();
        let deserialized: CustomProperty = serde_json::from_str(&serialized).unwrap();
        assert_eq!(property, deserialized);

        let query = ModernContainerQuery::Small;
        let serialized = serde_json::to_string(&query).unwrap();
        let deserialized: ModernContainerQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(query, deserialized);
    }

    #[test]
    fn test_modern_css_features_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .layer_custom("theme")
            .custom_property("primary-color", "#3b82f6")
            .custom_property("secondary-color", "#64748b")
            .container_custom("sidebar")
            .container_custom("main");

        let result = classes.build();
        assert!(result.classes.contains("layer-theme"));
        assert!(result.custom.contains_key("primary-color"));
        assert_eq!(
            result.custom.get("primary-color"),
            Some(&"#3b82f6".to_string())
        );
        assert!(result.custom.contains_key("secondary-color"));
        assert_eq!(
            result.custom.get("secondary-color"),
            Some(&"#64748b".to_string())
        );
        assert!(result.classes.contains("container-sidebar"));
        assert!(result.classes.contains("container-main"));
    }

    #[test]
    fn test_modern_css_features_custom_values() {
        let classes = ClassBuilder::new()
            .layer_custom_value(CascadeLayer::Custom("theme".to_string()))
            .custom_property_value(CustomProperty::Color("blue".to_string()))
            .custom_property_value(CustomProperty::Spacing("2rem".to_string()))
            .container_custom_value(ModernContainerQuery::Custom("sidebar".to_string()));

        let result = classes.build();
        assert!(result.classes.contains("layer-theme"));
        assert!(result.classes.contains("custom-color"));
        assert!(result.classes.contains("custom-spacing"));
        assert!(result.classes.contains("container-sidebar"));
    }

    #[test]
    fn test_modern_css_features_all_variants() {
        let classes = ClassBuilder::new()
            .layer_base()
            .layer_components()
            .layer_utilities()
            .layer_custom("theme")
            .custom_property("color", "red")
            .custom_property("spacing", "1rem")
            .custom_property("font-size", "16px")
            .custom_property("font-weight", "bold")
            .custom_property("line-height", "1.5")
            .custom_property("border-radius", "8px")
            .custom_property("box-shadow", "0 4px 6px -1px rgb(0 0 0 / 0.1)")
            .custom_property("z-index", "10")
            .custom_property("opacity", "0.8")
            .custom_property("transform", "rotate(45deg)")
            .custom_property("animation", "fadeIn 0.5s ease-in-out")
            .custom_property("transition", "all 0.3s ease")
            .container_small()
            .container_medium()
            .container_large()
            .container_extra_large()
            .container_custom("sidebar");

        let result = classes.build();
        assert!(result.classes.contains("layer-base"));
        assert!(result.classes.contains("layer-components"));
        assert!(result.classes.contains("layer-utilities"));
        assert!(result.classes.contains("layer-theme"));
        // Check custom properties
        assert!(result.custom.contains_key("color"));
        assert_eq!(result.custom.get("color"), Some(&"red".to_string()));
        assert!(result.custom.contains_key("spacing"));
        assert_eq!(result.custom.get("spacing"), Some(&"1rem".to_string()));
        assert!(result.custom.contains_key("font-size"));
        assert_eq!(result.custom.get("font-size"), Some(&"16px".to_string()));
        assert!(result.custom.contains_key("font-weight"));
        assert_eq!(result.custom.get("font-weight"), Some(&"bold".to_string()));
        assert!(result.custom.contains_key("line-height"));
        assert_eq!(result.custom.get("line-height"), Some(&"1.5".to_string()));
        assert!(result.custom.contains_key("border-radius"));
        assert_eq!(result.custom.get("border-radius"), Some(&"8px".to_string()));
        assert!(result.custom.contains_key("box-shadow"));
        assert_eq!(
            result.custom.get("box-shadow"),
            Some(&"0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string())
        );
        assert!(result.custom.contains_key("z-index"));
        assert_eq!(result.custom.get("z-index"), Some(&"10".to_string()));
        assert!(result.custom.contains_key("opacity"));
        assert_eq!(result.custom.get("opacity"), Some(&"0.8".to_string()));
        assert!(result.custom.contains_key("transform"));
        assert_eq!(
            result.custom.get("transform"),
            Some(&"rotate(45deg)".to_string())
        );
        assert!(result.custom.contains_key("animation"));
        assert_eq!(
            result.custom.get("animation"),
            Some(&"fadeIn 0.5s ease-in-out".to_string())
        );
        assert!(result.custom.contains_key("transition"));
        assert_eq!(
            result.custom.get("transition"),
            Some(&"all 0.3s ease".to_string())
        );
        assert!(result.classes.contains("container-small"));
        assert!(result.classes.contains("container-medium"));
        assert!(result.classes.contains("container-large"));
        assert!(result.classes.contains("container-extra-large"));
        assert!(result.classes.contains("container-sidebar"));
    }
}
