//! Theme Fonts Module
//!
//! Handles font-related theme utilities:
//! - FontFamily enum for different font families
//! - FontSizeScale struct for font sizes
//! - FontWeightScale struct for font weights

/// Font family enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontFamily {
    Sans,
    Serif,
    Mono,
    Custom(String),
}

impl FontFamily {
    /// Returns the CSS class for the font family
    pub fn class(&self) -> &str {
        match self {
            FontFamily::Sans => "font-sans",
            FontFamily::Serif => "font-serif",
            FontFamily::Mono => "font-mono",
            FontFamily::Custom(name) => name,
        }
    }

    /// Returns the CSS font-family value
    pub fn css_value(&self) -> String {
        match self {
            FontFamily::Sans => "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, \"Noto Sans\", sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"".to_string(),
            FontFamily::Serif => "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif".to_string(),
            FontFamily::Mono => "ui-monospace, SFMono-Regular, \"SF Mono\", Monaco, Inconsolata, \"Fira Code\", \"Droid Sans Mono\", \"Source Code Pro\", Menlo, Consolas, \"Liberation Mono\", monospace".to_string(),
            FontFamily::Custom(name) => name.clone(),
        }
    }

    /// Check if this is a system font family
    pub fn is_system(&self) -> bool {
        matches!(self, FontFamily::Sans | FontFamily::Serif | FontFamily::Mono)
    }

    /// Check if this is a custom font family
    pub fn is_custom(&self) -> bool {
        matches!(self, FontFamily::Custom(_))
    }

    /// Get the font family name
    pub fn name(&self) -> &str {
        match self {
            FontFamily::Sans => "sans",
            FontFamily::Serif => "serif",
            FontFamily::Mono => "mono",
            FontFamily::Custom(name) => name,
        }
    }
}

impl std::fmt::Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Font size scale
#[derive(Debug, Clone)]
pub struct FontSizeScale {
    pub xs: String,    // 0.75rem
    pub sm: String,    // 0.875rem
    pub base: String,  // 1rem
    pub lg: String,    // 1.125rem
    pub xl: String,    // 1.25rem
    pub xxl: String,   // 1.5rem
    pub xxxl: String,  // 1.875rem
    pub xxxxl: String, // 2.25rem
}

impl Default for FontSizeScale {
    fn default() -> Self {
        Self::new()
    }
}

impl FontSizeScale {
    /// Creates a new font size scale with default values
    pub fn new() -> Self {
        Self {
            xs: "0.75rem".to_string(),
            sm: "0.875rem".to_string(),
            base: "1rem".to_string(),
            lg: "1.125rem".to_string(),
            xl: "1.25rem".to_string(),
            xxl: "1.5rem".to_string(),
            xxxl: "1.875rem".to_string(),
            xxxxl: "2.25rem".to_string(),
        }
    }

    /// Creates a custom font size scale
    pub fn custom(xs: &str, sm: &str, base: &str, lg: &str, xl: &str, xxl: &str, xxxl: &str, xxxxl: &str) -> Self {
        Self {
            xs: xs.to_string(),
            sm: sm.to_string(),
            base: base.to_string(),
            lg: lg.to_string(),
            xl: xl.to_string(),
            xxl: xxl.to_string(),
            xxxl: xxxl.to_string(),
            xxxxl: xxxxl.to_string(),
        }
    }

    /// Gets font size by key
    pub fn get(&self, key: &str) -> Option<&str> {
        match key {
            "xs" => Some(&self.xs),
            "sm" => Some(&self.sm),
            "base" => Some(&self.base),
            "lg" => Some(&self.lg),
            "xl" => Some(&self.xl),
            "2xl" => Some(&self.xxl),
            "3xl" => Some(&self.xxxl),
            "4xl" => Some(&self.xxxxl),
            _ => None,
        }
    }

    /// Returns all font sizes as key-value pairs
    pub fn all_sizes(&self) -> Vec<(&str, &str)> {
        vec![
            ("xs", self.xs.as_str()),
            ("sm", self.sm.as_str()),
            ("base", self.base.as_str()),
            ("lg", self.lg.as_str()),
            ("xl", self.xl.as_str()),
            ("2xl", self.xxl.as_str()),
            ("3xl", self.xxxl.as_str()),
            ("4xl", self.xxxxl.as_str()),
        ]
    }
}

/// Font weight scale
#[derive(Debug, Clone)]
pub struct FontWeightScale {
    pub thin: String,       // 100
    pub extralight: String, // 200
    pub light: String,      // 300
    pub normal: String,     // 400
    pub medium: String,     // 500
    pub semibold: String,   // 600
    pub bold: String,       // 700
    pub extrabold: String,  // 800
    pub black: String,      // 900
}

impl Default for FontWeightScale {
    fn default() -> Self {
        Self::new()
    }
}

impl FontWeightScale {
    /// Creates a new font weight scale with default values
    pub fn new() -> Self {
        Self {
            thin: "100".to_string(),
            extralight: "200".to_string(),
            light: "300".to_string(),
            normal: "400".to_string(),
            medium: "500".to_string(),
            semibold: "600".to_string(),
            bold: "700".to_string(),
            extrabold: "800".to_string(),
            black: "900".to_string(),
        }
    }

    /// Creates a custom font weight scale
    pub fn custom(thin: &str, extralight: &str, light: &str, normal: &str, medium: &str, semibold: &str, bold: &str, extrabold: &str, black: &str) -> Self {
        Self {
            thin: thin.to_string(),
            extralight: extralight.to_string(),
            light: light.to_string(),
            normal: normal.to_string(),
            medium: medium.to_string(),
            semibold: semibold.to_string(),
            bold: bold.to_string(),
            extrabold: extrabold.to_string(),
            black: black.to_string(),
        }
    }

    /// Gets font weight by key
    pub fn get(&self, key: &str) -> Option<&str> {
        match key {
            "thin" => Some(&self.thin),
            "extralight" => Some(&self.extralight),
            "light" => Some(&self.light),
            "normal" => Some(&self.normal),
            "medium" => Some(&self.medium),
            "semibold" => Some(&self.semibold),
            "bold" => Some(&self.bold),
            "extrabold" => Some(&self.extrabold),
            "black" => Some(&self.black),
            _ => None,
        }
    }

    /// Returns all font weights as key-value pairs
    pub fn all_weights(&self) -> Vec<(&str, &str)> {
        vec![
            ("thin", self.thin.as_str()),
            ("extralight", self.extralight.as_str()),
            ("light", self.light.as_str()),
            ("normal", self.normal.as_str()),
            ("medium", self.medium.as_str()),
            ("semibold", self.semibold.as_str()),
            ("bold", self.bold.as_str()),
            ("extrabold", self.extrabold.as_str()),
            ("black", self.black.as_str()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_family_classes() {
        assert_eq!(FontFamily::Sans.class(), "font-sans");
        assert_eq!(FontFamily::Serif.class(), "font-serif");
        assert_eq!(FontFamily::Mono.class(), "font-mono");
        assert_eq!(FontFamily::Custom("custom-font".to_string()).class(), "custom-font");
    }

    #[test]
    fn font_family_css_values() {
        assert!(FontFamily::Sans.css_value().contains("system-ui"));
        assert!(FontFamily::Serif.css_value().contains("Georgia"));
        assert!(FontFamily::Mono.css_value().contains("monospace"));
        assert_eq!(FontFamily::Custom("Custom Font".to_string()).css_value(), "Custom Font");
    }

    #[test]
    fn font_family_properties() {
        assert!(FontFamily::Sans.is_system());
        assert!(FontFamily::Custom("test".to_string()).is_custom());
        assert!(!FontFamily::Sans.is_custom());

        assert_eq!(FontFamily::Sans.name(), "sans");
        assert_eq!(FontFamily::Custom("custom".to_string()).name(), "custom");
    }

    #[test]
    fn font_size_scale_default() {
        let scale = FontSizeScale::new();

        assert_eq!(scale.xs, "0.75rem");
        assert_eq!(scale.base, "1rem");
        assert_eq!(scale.xxxxl, "2.25rem");
    }

    #[test]
    fn font_size_scale_get() {
        let scale = FontSizeScale::new();

        assert_eq!(scale.get("xs"), Some("0.75rem"));
        assert_eq!(scale.get("base"), Some("1rem"));
        assert_eq!(scale.get("4xl"), Some("2.25rem"));
        assert_eq!(scale.get("invalid"), None);
    }

    #[test]
    fn font_size_scale_all_sizes() {
        let scale = FontSizeScale::new();
        let sizes = scale.all_sizes();

        assert_eq!(sizes.len(), 8);
        assert!(sizes.contains(&("base", "1rem")));
        assert!(sizes.contains(&("4xl", "2.25rem")));
    }

    #[test]
    fn font_weight_scale_default() {
        let scale = FontWeightScale::new();

        assert_eq!(scale.thin, "100");
        assert_eq!(scale.normal, "400");
        assert_eq!(scale.black, "900");
    }

    #[test]
    fn font_weight_scale_get() {
        let scale = FontWeightScale::new();

        assert_eq!(scale.get("thin"), Some("100"));
        assert_eq!(scale.get("normal"), Some("400"));
        assert_eq!(scale.get("bold"), Some("700"));
        assert_eq!(scale.get("invalid"), None);
    }

    #[test]
    fn font_weight_scale_all_weights() {
        let scale = FontWeightScale::new();
        let weights = scale.all_weights();

        assert_eq!(weights.len(), 9);
        assert!(weights.contains(&("normal", "400")));
        assert!(weights.contains(&("bold", "700")));
        assert!(weights.contains(&("black", "900")));
    }
}
