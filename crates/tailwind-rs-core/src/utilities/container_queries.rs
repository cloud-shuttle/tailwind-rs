//! Container query utilities for tailwind-rs
//!
//! This module provides support for CSS container queries including
//! @container queries and container-based responsive utilities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Container query types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContainerQuery {
    /// @container (inline-size > 768px)
    InlineSize(ContainerSize),
    /// @container (block-size > 768px)
    BlockSize(ContainerSize),
    /// @container (width > 768px)
    Width(ContainerSize),
    /// @container (height > 768px)
    Height(ContainerSize),
    /// @container (aspect-ratio > 16/9)
    AspectRatio(ContainerAspectRatio),
    /// @container (orientation: landscape)
    Orientation(ContainerOrientation),
}

/// Container size values
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContainerSize {
    /// 320px
    Xs,
    /// 640px
    Sm,
    /// 768px
    Md,
    /// 1024px
    Lg,
    /// 1280px
    Xl,
    /// 1536px
    Xl2,
    /// Custom size
    Custom(String),
}

/// Container aspect ratio values
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContainerAspectRatio {
    /// 1/1 (square)
    Square,
    /// 4/3
    Video,
    /// 16/9
    Widescreen,
    /// 21/9
    Ultrawide,
    /// Custom aspect ratio
    Custom(String),
}

/// Container orientation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContainerOrientation {
    /// landscape
    Landscape,
    /// portrait
    Portrait,
}

impl ContainerQuery {
    /// Create a new inline-size container query
    pub fn inline_size(size: ContainerSize) -> Self {
        Self::InlineSize(size)
    }

    /// Create a new block-size container query
    pub fn block_size(size: ContainerSize) -> Self {
        Self::BlockSize(size)
    }

    /// Create a new width container query
    pub fn width(size: ContainerSize) -> Self {
        Self::Width(size)
    }

    /// Create a new height container query
    pub fn height(size: ContainerSize) -> Self {
        Self::Height(size)
    }

    /// Create a new aspect-ratio container query
    pub fn aspect_ratio(ratio: ContainerAspectRatio) -> Self {
        Self::AspectRatio(ratio)
    }

    /// Create a new orientation container query
    pub fn orientation(orientation: ContainerOrientation) -> Self {
        Self::Orientation(orientation)
    }

    /// Convert to CSS @container query
    pub fn to_css_query(&self) -> String {
        match self {
            ContainerQuery::InlineSize(size) => {
                format!("@container (inline-size > {})", size.to_css_value())
            }
            ContainerQuery::BlockSize(size) => {
                format!("@container (block-size > {})", size.to_css_value())
            }
            ContainerQuery::Width(size) => {
                format!("@container (width > {})", size.to_css_value())
            }
            ContainerQuery::Height(size) => {
                format!("@container (height > {})", size.to_css_value())
            }
            ContainerQuery::AspectRatio(ratio) => {
                format!("@container (aspect-ratio > {})", ratio.to_css_value())
            }
            ContainerQuery::Orientation(orientation) => {
                format!("@container (orientation: {})", orientation.to_css_value())
            }
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            ContainerQuery::InlineSize(size) => {
                format!("@container/inline-size:{}", size.to_class_name())
            }
            ContainerQuery::BlockSize(size) => {
                format!("@container/block-size:{}", size.to_class_name())
            }
            ContainerQuery::Width(size) => {
                format!("@container/width:{}", size.to_class_name())
            }
            ContainerQuery::Height(size) => {
                format!("@container/height:{}", size.to_class_name())
            }
            ContainerQuery::AspectRatio(ratio) => {
                format!("@container/aspect-ratio:{}", ratio.to_class_name())
            }
            ContainerQuery::Orientation(orientation) => {
                format!("@container/orientation:{}", orientation.to_class_name())
            }
        }
    }
}

impl ContainerSize {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            ContainerSize::Xs => "320px".to_string(),
            ContainerSize::Sm => "640px".to_string(),
            ContainerSize::Md => "768px".to_string(),
            ContainerSize::Lg => "1024px".to_string(),
            ContainerSize::Xl => "1280px".to_string(),
            ContainerSize::Xl2 => "1536px".to_string(),
            ContainerSize::Custom(size) => size.clone(),
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            ContainerSize::Xs => "xs".to_string(),
            ContainerSize::Sm => "sm".to_string(),
            ContainerSize::Md => "md".to_string(),
            ContainerSize::Lg => "lg".to_string(),
            ContainerSize::Xl => "xl".to_string(),
            ContainerSize::Xl2 => "2xl".to_string(),
            ContainerSize::Custom(size) => size.clone(),
        }
    }
}

impl ContainerAspectRatio {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            ContainerAspectRatio::Square => "1/1".to_string(),
            ContainerAspectRatio::Video => "4/3".to_string(),
            ContainerAspectRatio::Widescreen => "16/9".to_string(),
            ContainerAspectRatio::Ultrawide => "21/9".to_string(),
            ContainerAspectRatio::Custom(ratio) => ratio.clone(),
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            ContainerAspectRatio::Square => "square".to_string(),
            ContainerAspectRatio::Video => "video".to_string(),
            ContainerAspectRatio::Widescreen => "widescreen".to_string(),
            ContainerAspectRatio::Ultrawide => "ultrawide".to_string(),
            ContainerAspectRatio::Custom(ratio) => ratio.clone(),
        }
    }
}

impl ContainerOrientation {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            ContainerOrientation::Landscape => "landscape".to_string(),
            ContainerOrientation::Portrait => "portrait".to_string(),
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            ContainerOrientation::Landscape => "landscape".to_string(),
            ContainerOrientation::Portrait => "portrait".to_string(),
        }
    }
}

impl fmt::Display for ContainerQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_query())
    }
}

impl fmt::Display for ContainerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_value())
    }
}

impl fmt::Display for ContainerAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_value())
    }
}

impl fmt::Display for ContainerOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_size_container_query() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        assert_eq!(query.to_css_query(), "@container (inline-size > 768px)");
        assert_eq!(query.to_class_name(), "@container/inline-size:md");
    }

    #[test]
    fn test_block_size_container_query() {
        let query = ContainerQuery::block_size(ContainerSize::Lg);
        assert_eq!(query.to_css_query(), "@container (block-size > 1024px)");
        assert_eq!(query.to_class_name(), "@container/block-size:lg");
    }

    #[test]
    fn test_width_container_query() {
        let query = ContainerQuery::width(ContainerSize::Sm);
        assert_eq!(query.to_css_query(), "@container (width > 640px)");
        assert_eq!(query.to_class_name(), "@container/width:sm");
    }

    #[test]
    fn test_height_container_query() {
        let query = ContainerQuery::height(ContainerSize::Xl);
        assert_eq!(query.to_css_query(), "@container (height > 1280px)");
        assert_eq!(query.to_class_name(), "@container/height:xl");
    }

    #[test]
    fn test_aspect_ratio_container_query() {
        let query = ContainerQuery::aspect_ratio(ContainerAspectRatio::Widescreen);
        assert_eq!(query.to_css_query(), "@container (aspect-ratio > 16/9)");
        assert_eq!(query.to_class_name(), "@container/aspect-ratio:widescreen");
    }

    #[test]
    fn test_orientation_container_query() {
        let query = ContainerQuery::orientation(ContainerOrientation::Landscape);
        assert_eq!(query.to_css_query(), "@container (orientation: landscape)");
        assert_eq!(query.to_class_name(), "@container/orientation:landscape");
    }

    #[test]
    fn test_custom_container_size() {
        let size = ContainerSize::Custom("500px".to_string());
        assert_eq!(size.to_css_value(), "500px");
        assert_eq!(size.to_class_name(), "500px");
    }

    #[test]
    fn test_custom_aspect_ratio() {
        let ratio = ContainerAspectRatio::Custom("3/2".to_string());
        assert_eq!(ratio.to_css_value(), "3/2");
        assert_eq!(ratio.to_class_name(), "3/2");
    }

    #[test]
    fn test_container_size_values() {
        assert_eq!(ContainerSize::Xs.to_css_value(), "320px");
        assert_eq!(ContainerSize::Sm.to_css_value(), "640px");
        assert_eq!(ContainerSize::Md.to_css_value(), "768px");
        assert_eq!(ContainerSize::Lg.to_css_value(), "1024px");
        assert_eq!(ContainerSize::Xl.to_css_value(), "1280px");
        assert_eq!(ContainerSize::Xl2.to_css_value(), "1536px");
    }

    #[test]
    fn test_container_size_class_names() {
        assert_eq!(ContainerSize::Xs.to_class_name(), "xs");
        assert_eq!(ContainerSize::Sm.to_class_name(), "sm");
        assert_eq!(ContainerSize::Md.to_class_name(), "md");
        assert_eq!(ContainerSize::Lg.to_class_name(), "lg");
        assert_eq!(ContainerSize::Xl.to_class_name(), "xl");
        assert_eq!(ContainerSize::Xl2.to_class_name(), "2xl");
    }

    #[test]
    fn test_aspect_ratio_values() {
        assert_eq!(ContainerAspectRatio::Square.to_css_value(), "1/1");
        assert_eq!(ContainerAspectRatio::Video.to_css_value(), "4/3");
        assert_eq!(ContainerAspectRatio::Widescreen.to_css_value(), "16/9");
        assert_eq!(ContainerAspectRatio::Ultrawide.to_css_value(), "21/9");
    }

    #[test]
    fn test_aspect_ratio_class_names() {
        assert_eq!(ContainerAspectRatio::Square.to_class_name(), "square");
        assert_eq!(ContainerAspectRatio::Video.to_class_name(), "video");
        assert_eq!(
            ContainerAspectRatio::Widescreen.to_class_name(),
            "widescreen"
        );
        assert_eq!(ContainerAspectRatio::Ultrawide.to_class_name(), "ultrawide");
    }

    #[test]
    fn test_orientation_values() {
        assert_eq!(ContainerOrientation::Landscape.to_css_value(), "landscape");
        assert_eq!(ContainerOrientation::Portrait.to_css_value(), "portrait");
    }

    #[test]
    fn test_orientation_class_names() {
        assert_eq!(ContainerOrientation::Landscape.to_class_name(), "landscape");
        assert_eq!(ContainerOrientation::Portrait.to_class_name(), "portrait");
    }

    #[test]
    fn test_container_query_display() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        assert_eq!(format!("{}", query), "@container (inline-size > 768px)");
    }

    #[test]
    fn test_container_size_display() {
        let size = ContainerSize::Lg;
        assert_eq!(format!("{}", size), "1024px");
    }

    #[test]
    fn test_aspect_ratio_display() {
        let ratio = ContainerAspectRatio::Widescreen;
        assert_eq!(format!("{}", ratio), "16/9");
    }

    #[test]
    fn test_orientation_display() {
        let orientation = ContainerOrientation::Landscape;
        assert_eq!(format!("{}", orientation), "landscape");
    }

    #[test]
    fn test_container_query_serialization() {
        let query = ContainerQuery::inline_size(ContainerSize::Md);
        let serialized = serde_json::to_string(&query).unwrap();
        let deserialized: ContainerQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(query, deserialized);
    }

    #[test]
    fn test_container_size_serialization() {
        let size = ContainerSize::Lg;
        let serialized = serde_json::to_string(&size).unwrap();
        let deserialized: ContainerSize = serde_json::from_str(&serialized).unwrap();
        assert_eq!(size, deserialized);
    }

    #[test]
    fn test_aspect_ratio_serialization() {
        let ratio = ContainerAspectRatio::Widescreen;
        let serialized = serde_json::to_string(&ratio).unwrap();
        let deserialized: ContainerAspectRatio = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ratio, deserialized);
    }

    #[test]
    fn test_orientation_serialization() {
        let orientation = ContainerOrientation::Landscape;
        let serialized = serde_json::to_string(&orientation).unwrap();
        let deserialized: ContainerOrientation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(orientation, deserialized);
    }
}
