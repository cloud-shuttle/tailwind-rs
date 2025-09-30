//! Filter Utilities Types Module
//!
//! Core types and enums for filter utilities:
//! - FilterType: Different types of CSS filters
//! - BlurSize: Predefined blur sizes
//! - FilterValue: Represents filter values and configurations

/// Types of CSS filters supported by Tailwind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterType {
    /// Blur filter
    Blur,
    /// Brightness filter
    Brightness,
    /// Contrast filter
    Contrast,
    /// Drop-shadow filter
    DropShadow,
    /// Grayscale filter
    Grayscale,
    /// Hue-rotate filter
    HueRotate,
    /// Invert filter
    Invert,
    /// Opacity filter
    Opacity,
    /// Saturate filter
    Saturate,
    /// Sepia filter
    Sepia,
    /// Backdrop filter (for backdrop-filter)
    Backdrop,
}

impl FilterType {
    /// Get the CSS function name for this filter type
    pub fn css_function(&self) -> &'static str {
        match self {
            FilterType::Blur => "blur",
            FilterType::Brightness => "brightness",
            FilterType::Contrast => "contrast",
            FilterType::DropShadow => "drop-shadow",
            FilterType::Grayscale => "grayscale",
            FilterType::HueRotate => "hue-rotate",
            FilterType::Invert => "invert",
            FilterType::Opacity => "opacity",
            FilterType::Saturate => "saturate",
            FilterType::Sepia => "sepia",
            FilterType::Backdrop => "backdrop",
        }
    }

    /// Check if this filter type is a percentage-based filter
    pub fn is_percentage_based(&self) -> bool {
        matches!(self, FilterType::Brightness | FilterType::Contrast | FilterType::Grayscale | FilterType::Invert | FilterType::Opacity | FilterType::Saturate | FilterType::Sepia)
    }

    /// Check if this filter type is an angle-based filter
    pub fn is_angle_based(&self) -> bool {
        matches!(self, FilterType::HueRotate)
    }

    /// Check if this filter type is a length-based filter
    pub fn is_length_based(&self) -> bool {
        matches!(self, FilterType::Blur)
    }

    /// Check if this filter type is a color-based filter
    pub fn is_color_based(&self) -> bool {
        matches!(self, FilterType::DropShadow)
    }
}

/// Predefined blur sizes for Tailwind CSS
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlurSize {
    /// No blur
    None,
    /// Extra small blur
    Xs,
    /// Small blur
    Sm,
    /// Medium blur
    Md,
    /// Large blur
    Lg,
    /// Extra large blur
    Xl,
    /// 2x extra large blur
    Xxl,
    /// 3x extra large blur
    Xxxl,
}

impl BlurSize {
    /// Get the CSS variable name for this blur size
    pub fn css_variable(&self) -> &'static str {
        match self {
            BlurSize::None => "0",
            BlurSize::Xs => "var(--blur-xs)",
            BlurSize::Sm => "var(--blur-sm)",
            BlurSize::Md => "var(--blur-md)",
            BlurSize::Lg => "var(--blur-lg)",
            BlurSize::Xl => "var(--blur-xl)",
            BlurSize::Xxl => "var(--blur-2xl)",
            BlurSize::Xxxl => "var(--blur-3xl)",
        }
    }

    /// Get the class suffix for this blur size
    pub fn class_suffix(&self) -> &'static str {
        match self {
            BlurSize::None => "none",
            BlurSize::Xs => "xs",
            BlurSize::Sm => "sm",
            BlurSize::Md => "md",
            BlurSize::Lg => "lg",
            BlurSize::Xl => "xl",
            BlurSize::Xxl => "2xl",
            BlurSize::Xxxl => "3xl",
        }
    }
}

/// Represents a filter value configuration
#[derive(Debug, Clone, PartialEq)]
pub enum FilterValue {
    /// No filter applied
    None,
    /// Predefined value (like blur sizes)
    Predefined(String),
    /// Custom CSS variable reference
    Variable(String),
    /// Arbitrary CSS value
    Arbitrary(String),
    /// Percentage value (0-100%)
    Percentage(f32),
    /// Angle value in degrees
    Angle(f32),
    /// Length value (for blur, etc.)
    Length(String),
    /// Color value (for drop-shadow)
    Color(String),
}

impl FilterValue {
    /// Convert to CSS value string
    pub fn to_css_value(&self) -> String {
        match self {
            FilterValue::None => "none".to_string(),
            FilterValue::Predefined(val) => val.clone(),
            FilterValue::Variable(var) => format!("var({})", var),
            FilterValue::Arbitrary(val) => val.clone(),
            FilterValue::Percentage(pct) => format!("{}%", pct),
            FilterValue::Angle(deg) => format!("{}deg", deg),
            FilterValue::Length(len) => len.clone(),
            FilterValue::Color(col) => col.clone(),
        }
    }

    /// Check if this value represents no filtering
    pub fn is_none(&self) -> bool {
        matches!(self, FilterValue::None)
    }

    /// Check if this value is a variable reference
    pub fn is_variable(&self) -> bool {
        matches!(self, FilterValue::Variable(_))
    }

    /// Check if this value is arbitrary
    pub fn is_arbitrary(&self) -> bool {
        matches!(self, FilterValue::Arbitrary(_))
    }
}

/// Filter configuration combining type and value
#[derive(Debug, Clone, PartialEq)]
pub struct FilterConfig {
    pub filter_type: FilterType,
    pub value: FilterValue,
}

impl FilterConfig {
    /// Create a new filter configuration
    pub fn new(filter_type: FilterType, value: FilterValue) -> Self {
        Self { filter_type, value }
    }

    /// Create a none filter configuration
    pub fn none() -> Self {
        Self {
            filter_type: FilterType::Blur, // Doesn't matter for none
            value: FilterValue::None,
        }
    }

    /// Convert to CSS filter function call
    pub fn to_css_filter(&self) -> Option<String> {
        if self.value.is_none() {
            return None;
        }

        let function = self.filter_type.css_function();
        let value = self.value.to_css_value();
        Some(format!("{}({})", function, value))
    }
}

/// Backdrop filter types (subset of regular filters)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackdropFilterType {
    /// Blur backdrop filter
    Blur,
    /// Brightness backdrop filter
    Brightness,
    /// Contrast backdrop filter
    Contrast,
    /// Grayscale backdrop filter
    Grayscale,
    /// Hue-rotate backdrop filter
    HueRotate,
    /// Invert backdrop filter
    Invert,
    /// Opacity backdrop filter
    Opacity,
    /// Saturate backdrop filter
    Saturate,
    /// Sepia backdrop filter
    Sepia,
}

impl BackdropFilterType {
    /// Get the CSS function name for this backdrop filter type
    pub fn css_function(&self) -> &'static str {
        match self {
            BackdropFilterType::Blur => "blur",
            BackdropFilterType::Brightness => "brightness",
            BackdropFilterType::Contrast => "contrast",
            BackdropFilterType::Grayscale => "grayscale",
            BackdropFilterType::HueRotate => "hue-rotate",
            BackdropFilterType::Invert => "invert",
            BackdropFilterType::Opacity => "opacity",
            BackdropFilterType::Saturate => "saturate",
            BackdropFilterType::Sepia => "sepia",
        }
    }

    /// Convert to regular filter type
    pub fn to_filter_type(&self) -> FilterType {
        match self {
            BackdropFilterType::Blur => FilterType::Blur,
            BackdropFilterType::Brightness => FilterType::Brightness,
            BackdropFilterType::Contrast => FilterType::Contrast,
            BackdropFilterType::Grayscale => FilterType::Grayscale,
            BackdropFilterType::HueRotate => FilterType::HueRotate,
            BackdropFilterType::Invert => FilterType::Invert,
            BackdropFilterType::Opacity => FilterType::Opacity,
            BackdropFilterType::Saturate => FilterType::Saturate,
            BackdropFilterType::Sepia => FilterType::Sepia,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_type_css_functions() {
        assert_eq!(FilterType::Blur.css_function(), "blur");
        assert_eq!(FilterType::Brightness.css_function(), "brightness");
        assert_eq!(FilterType::DropShadow.css_function(), "drop-shadow");
    }

    #[test]
    fn filter_type_properties() {
        assert!(FilterType::Brightness.is_percentage_based());
        assert!(FilterType::HueRotate.is_angle_based());
        assert!(FilterType::Blur.is_length_based());
        assert!(FilterType::DropShadow.is_color_based());
    }

    #[test]
    fn blur_size_properties() {
        assert_eq!(BlurSize::None.css_variable(), "0");
        assert_eq!(BlurSize::Md.css_variable(), "var(--blur-md)");
        assert_eq!(BlurSize::Xxl.class_suffix(), "2xl");
    }

    #[test]
    fn filter_value_css_conversion() {
        assert_eq!(FilterValue::None.to_css_value(), "none");
        assert_eq!(FilterValue::Percentage(50.0).to_css_value(), "50%");
        assert_eq!(FilterValue::Angle(45.0).to_css_value(), "45deg");
        assert_eq!(FilterValue::Variable("--my-var".to_string()).to_css_value(), "var(--my-var)");
    }

    #[test]
    fn filter_config_css_generation() {
        let config = FilterConfig::new(FilterType::Blur, FilterValue::Length("5px".to_string()));
        assert_eq!(config.to_css_filter(), Some("blur(5px)".to_string()));

        let none_config = FilterConfig::none();
        assert_eq!(none_config.to_css_filter(), None);
    }

    #[test]
    fn backdrop_filter_type_conversion() {
        assert_eq!(BackdropFilterType::Blur.to_filter_type(), FilterType::Blur);
        assert_eq!(BackdropFilterType::Brightness.to_filter_type(), FilterType::Brightness);
    }
}
