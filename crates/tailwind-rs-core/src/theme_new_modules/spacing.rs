//! Theme Spacing Module
//!
//! Handles spacing-related theme utilities:
//! - SpacingSize enum for different size levels
//! - SpacingScale struct for consistent spacing values

use std::collections::HashMap;

/// Spacing size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpacingSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}

impl SpacingSize {
    /// Returns all available spacing sizes
    pub fn sizes() -> &'static [SpacingSize] {
        &[
            SpacingSize::Xs,
            SpacingSize::Sm,
            SpacingSize::Md,
            SpacingSize::Lg,
            SpacingSize::Xl,
            SpacingSize::Xxl,
            SpacingSize::Xxxl,
        ]
    }

    /// Returns the CSS class suffix for the spacing size
    pub fn class_suffix(&self) -> &'static str {
        match self {
            SpacingSize::Xs => "xs",
            SpacingSize::Sm => "sm",
            SpacingSize::Md => "md",
            SpacingSize::Lg => "lg",
            SpacingSize::Xl => "xl",
            SpacingSize::Xxl => "2xl",
            SpacingSize::Xxxl => "3xl",
        }
    }

    /// Check if this is a small spacing size
    pub fn is_small(&self) -> bool {
        matches!(self, SpacingSize::Xs | SpacingSize::Sm)
    }

    /// Check if this is a large spacing size
    pub fn is_large(&self) -> bool {
        matches!(self, SpacingSize::Xxl | SpacingSize::Xxxl)
    }
}

impl std::fmt::Display for SpacingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_suffix())
    }
}

/// Spacing scale for consistent spacing values
#[derive(Debug, Clone)]
pub struct SpacingScale {
    values: HashMap<SpacingSize, String>,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self::new()
    }
}

impl SpacingScale {
    /// Creates a new spacing scale with default values
    pub fn new() -> Self {
        let mut values = HashMap::new();
        values.insert(SpacingSize::Xs, "0.125rem".to_string());
        values.insert(SpacingSize::Sm, "0.25rem".to_string());
        values.insert(SpacingSize::Md, "1rem".to_string());
        values.insert(SpacingSize::Lg, "1.5rem".to_string());
        values.insert(SpacingSize::Xl, "2rem".to_string());
        values.insert(SpacingSize::Xxl, "4rem".to_string());
        values.insert(SpacingSize::Xxxl, "8rem".to_string());

        Self { values }
    }

    /// Creates a custom spacing scale
    pub fn custom(xs: &str, sm: &str, md: &str, lg: &str, xl: &str, xl2: &str, xl3: &str) -> Self {
        let mut values = HashMap::new();
        values.insert(SpacingSize::Xs, xs.to_string());
        values.insert(SpacingSize::Sm, sm.to_string());
        values.insert(SpacingSize::Md, md.to_string());
        values.insert(SpacingSize::Lg, lg.to_string());
        values.insert(SpacingSize::Xl, xl.to_string());
        values.insert(SpacingSize::Xxl, xl2.to_string());
        values.insert(SpacingSize::Xxxl, xl3.to_string());

        Self { values }
    }

    /// Gets spacing value for a specific size
    pub fn get(&self, size: SpacingSize) -> &str {
        self.values.get(&size).map(|s| s.as_str()).unwrap_or("0rem")
    }

    /// Sets a custom value for a spacing size
    pub fn set(&mut self, size: SpacingSize, value: String) {
        self.values.insert(size, value);
    }

    /// Gets all spacing values as a vector of (size, value) tuples
    pub fn all_values(&self) -> Vec<(SpacingSize, &str)> {
        SpacingSize::sizes()
            .iter()
            .map(|&size| (size, self.get(size)))
            .collect()
    }

    /// Creates a Tailwind-compatible spacing scale
    pub fn to_tailwind_scale(&self) -> HashMap<String, String> {
        let mut scale = HashMap::new();
        for &size in SpacingSize::sizes() {
            scale.insert(size.class_suffix().to_string(), self.get(size).to_string());
        }
        scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_size_properties() {
        assert_eq!(SpacingSize::Xs.class_suffix(), "xs");
        assert_eq!(SpacingSize::Xl.class_suffix(), "xl");
        assert_eq!(SpacingSize::Xxl.class_suffix(), "2xl");

        assert!(SpacingSize::Xs.is_small());
        assert!(SpacingSize::Sm.is_small());
        assert!(!SpacingSize::Md.is_small());

        assert!(SpacingSize::Xxl.is_large());
        assert!(SpacingSize::Xxxl.is_large());
        assert!(!SpacingSize::Lg.is_large());
    }

    #[test]
    fn spacing_size_display() {
        assert_eq!(SpacingSize::Xs.to_string(), "xs");
        assert_eq!(SpacingSize::Md.to_string(), "md");
        assert_eq!(SpacingSize::Xxxl.to_string(), "3xl");
    }

    #[test]
    fn spacing_scale_default() {
        let scale = SpacingScale::new();

        assert_eq!(scale.get(SpacingSize::Xs), "0.125rem");
        assert_eq!(scale.get(SpacingSize::Md), "1rem");
        assert_eq!(scale.get(SpacingSize::Xxxl), "8rem");
    }

    #[test]
    fn spacing_scale_custom() {
        let scale = SpacingScale::custom("2px", "4px", "8px", "12px", "16px", "24px", "32px");

        assert_eq!(scale.get(SpacingSize::Xs), "2px");
        assert_eq!(scale.get(SpacingSize::Md), "8px");
        assert_eq!(scale.get(SpacingSize::Xxxl), "32px");
    }

    #[test]
    fn spacing_scale_set_and_get() {
        let mut scale = SpacingScale::new();
        scale.set(SpacingSize::Xs, "0.5rem".to_string());

        assert_eq!(scale.get(SpacingSize::Xs), "0.5rem");
        // Other values should remain unchanged
        assert_eq!(scale.get(SpacingSize::Md), "1rem");
    }

    #[test]
    fn spacing_scale_all_values() {
        let scale = SpacingScale::new();
        let values = scale.all_values();

        assert_eq!(values.len(), 7);
        assert!(values.iter().any(|(size, value)| *size == SpacingSize::Xs && *value == "0.125rem"));
        assert!(values.iter().any(|(size, value)| *size == SpacingSize::Md && *value == "1rem"));
    }

    #[test]
    fn spacing_scale_to_tailwind() {
        let scale = SpacingScale::new();
        let tailwind = scale.to_tailwind_scale();

        assert_eq!(tailwind.get("xs"), Some(&"0.125rem".to_string()));
        assert_eq!(tailwind.get("md"), Some(&"1rem".to_string()));
        assert_eq!(tailwind.get("3xl"), Some(&"8rem".to_string()));
    }
}
