//! New theme system implementation according to API documentation

use crate::color::Color;
use std::collections::HashMap;

/// Theme variant for different component styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeVariant {
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    Light,
    Dark,
}

impl ThemeVariant {
    /// Returns the associated color for the variant
    pub fn color(&self) -> Color {
        match self {
            ThemeVariant::Primary => Color::Blue,
            ThemeVariant::Secondary => Color::Gray,
            ThemeVariant::Danger => Color::Red,
            ThemeVariant::Success => Color::Green,
            ThemeVariant::Warning => Color::Yellow,
            ThemeVariant::Info => Color::Blue,
            ThemeVariant::Light => Color::Gray,
            ThemeVariant::Dark => Color::Gray,
        }
    }
}

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

/// Spacing scale for consistent spacing values
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
}

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
}

/// Font size scale
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
}

/// Font weight scale
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
}

/// Line height scale
pub struct LineHeightScale {
    pub none: String,    // 1
    pub tight: String,   // 1.25
    pub snug: String,    // 1.375
    pub normal: String,  // 1.5
    pub relaxed: String, // 1.625
    pub loose: String,   // 2
}

impl Default for LineHeightScale {
    fn default() -> Self {
        Self::new()
    }
}

impl LineHeightScale {
    /// Creates a new line height scale with default values
    pub fn new() -> Self {
        Self {
            none: "1".to_string(),
            tight: "1.25".to_string(),
            snug: "1.375".to_string(),
            normal: "1.5".to_string(),
            relaxed: "1.625".to_string(),
            loose: "2".to_string(),
        }
    }
}

/// Letter spacing scale
pub struct LetterSpacingScale {
    pub tighter: String, // -0.05em
    pub tight: String,   // -0.025em
    pub normal: String,  // 0em
    pub wide: String,    // 0.025em
    pub wider: String,   // 0.05em
    pub widest: String,  // 0.1em
}

impl Default for LetterSpacingScale {
    fn default() -> Self {
        Self::new()
    }
}

impl LetterSpacingScale {
    /// Creates a new letter spacing scale with default values
    pub fn new() -> Self {
        Self {
            tighter: "-0.05em".to_string(),
            tight: "-0.025em".to_string(),
            normal: "0em".to_string(),
            wide: "0.025em".to_string(),
            wider: "0.05em".to_string(),
            widest: "0.1em".to_string(),
        }
    }
}

/// Typography scale for the theme
pub struct TypographyScale {
    pub font_family: FontFamily,
    pub font_sizes: FontSizeScale,
    pub font_weights: FontWeightScale,
    pub line_heights: LineHeightScale,
    pub letter_spacing: LetterSpacingScale,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self::new()
    }
}

impl TypographyScale {
    /// Creates a new typography scale with default values
    pub fn new() -> Self {
        Self {
            font_family: FontFamily::Sans,
            font_sizes: FontSizeScale::new(),
            font_weights: FontWeightScale::new(),
            line_heights: LineHeightScale::new(),
            letter_spacing: LetterSpacingScale::new(),
        }
    }

    /// Sets the font family for the typography scale
    pub fn font_family(self, family: FontFamily) -> Self {
        Self {
            font_family: family,
            ..self
        }
    }
}

/// Shadow scale
pub struct ShadowScale {
    pub sm: String,
    pub base: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
    pub inner: String,
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadowScale {
    /// Creates a new shadow scale with default values
    pub fn new() -> Self {
        Self {
            sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            base: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
            md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
            xxl: "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
            inner: "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
        }
    }
}

/// Border scale
pub struct BorderScale {
    pub none: String,
    pub sm: String,
    pub base: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

impl Default for BorderScale {
    fn default() -> Self {
        Self::new()
    }
}

impl BorderScale {
    /// Creates a new border scale with default values
    pub fn new() -> Self {
        Self {
            none: "0px".to_string(),
            sm: "1px".to_string(),
            base: "2px".to_string(),
            md: "4px".to_string(),
            lg: "8px".to_string(),
            xl: "16px".to_string(),
        }
    }
}

/// Animation scale
pub struct AnimationScale {
    pub none: String,
    pub spin: String,
    pub ping: String,
    pub pulse: String,
    pub bounce: String,
}

impl Default for AnimationScale {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationScale {
    /// Creates a new animation scale with default values
    pub fn new() -> Self {
        Self {
            none: "none".to_string(),
            spin: "spin 1s linear infinite".to_string(),
            ping: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            pulse: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            bounce: "bounce 1s infinite".to_string(),
        }
    }
}

/// Main theme structure according to API documentation
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub background_color: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub success_color: Color,
    pub warning_color: Color,
    pub error_color: Color,
    pub info_color: Color,
    pub spacing: SpacingScale,
    pub typography: TypographyScale,
    pub shadows: ShadowScale,
    pub borders: BorderScale,
    pub animations: AnimationScale,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl Theme {
    /// Creates a new theme with default values
    pub fn new() -> Self {
        Self {
            primary_color: Color::Blue,
            secondary_color: Color::Gray,
            accent_color: Color::Blue,
            background_color: Color::Gray, // Using Gray as placeholder for White
            text_color: Color::Gray,
            border_color: Color::Gray,
            success_color: Color::Green,
            warning_color: Color::Yellow,
            error_color: Color::Red,
            info_color: Color::Blue,
            spacing: SpacingScale::new(),
            typography: TypographyScale::new(),
            shadows: ShadowScale::new(),
            borders: BorderScale::new(),
            animations: AnimationScale::new(),
        }
    }

    /// Sets the primary color for the theme
    pub fn primary_color(self, color: Color) -> Self {
        Self {
            primary_color: color,
            ..self
        }
    }

    /// Sets the secondary color for the theme
    pub fn secondary_color(self, color: Color) -> Self {
        Self {
            secondary_color: color,
            ..self
        }
    }

    /// Sets the accent color for the theme
    pub fn accent_color(self, color: Color) -> Self {
        Self {
            accent_color: color,
            ..self
        }
    }

    /// Sets the background color for the theme
    pub fn background_color(self, color: Color) -> Self {
        Self {
            background_color: color,
            ..self
        }
    }

    /// Sets the text color for the theme
    pub fn text_color(self, color: Color) -> Self {
        Self {
            text_color: color,
            ..self
        }
    }

    /// Applies theme to a component
    pub fn apply_to_component(&self, component: &dyn ThemedComponent) -> String {
        component.apply_theme(self)
    }
}

/// Trait for components that support theming
pub trait ThemedComponent {
    /// Returns the base classes for the component
    fn base_classes(&self) -> &str;

    /// Applies the theme to the component
    fn apply_theme(&self, theme: &Theme) -> String;

    /// Returns available theme variants for the component
    fn theme_variants(&self) -> Vec<ThemeVariant> {
        vec![
            ThemeVariant::Primary,
            ThemeVariant::Secondary,
            ThemeVariant::Danger,
            ThemeVariant::Success,
        ]
    }
}

/// Theme preset enum for predefined themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemePreset {
    Light,
    Dark,
    Professional,
    Minimal,
    Vibrant,
}

impl ThemePreset {
    /// Creates a theme from the preset
    pub fn create(&self) -> Theme {
        match self {
            ThemePreset::Light => Theme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .background_color(Color::Gray) // Using Gray as placeholder for White
                .text_color(Color::Gray),
            ThemePreset::Dark => Theme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .background_color(Color::Gray) // Using Gray as placeholder for Black
                .text_color(Color::Gray), // Using Gray as placeholder for White
            ThemePreset::Professional => Theme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .accent_color(Color::Blue),
            ThemePreset::Minimal => Theme::new()
                .primary_color(Color::Gray)
                .secondary_color(Color::Gray)
                .accent_color(Color::Gray),
            ThemePreset::Vibrant => Theme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Green)
                .accent_color(Color::Yellow),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new();
        assert_eq!(theme.primary_color, Color::Blue);
        assert_eq!(theme.secondary_color, Color::Gray);
        assert_eq!(theme.accent_color, Color::Blue);
    }

    #[test]
    fn test_theme_primary_color() {
        let theme = Theme::new().primary_color(Color::Green);
        assert_eq!(theme.primary_color, Color::Green);
    }

    #[test]
    fn test_theme_secondary_color() {
        let theme = Theme::new().secondary_color(Color::Purple);
        assert_eq!(theme.secondary_color, Color::Purple);
    }

    #[test]
    fn test_theme_accent_color() {
        let theme = Theme::new().accent_color(Color::Orange);
        assert_eq!(theme.accent_color, Color::Orange);
    }

    #[test]
    fn test_theme_variant_color() {
        assert_eq!(ThemeVariant::Primary.color(), Color::Blue);
        assert_eq!(ThemeVariant::Secondary.color(), Color::Gray);
        assert_eq!(ThemeVariant::Danger.color(), Color::Red);
        assert_eq!(ThemeVariant::Success.color(), Color::Green);
    }

    #[test]
    fn test_spacing_scale_new() {
        let spacing = SpacingScale::new();
        assert_eq!(spacing.get(SpacingSize::Xs), "0.125rem");
        assert_eq!(spacing.get(SpacingSize::Sm), "0.25rem");
        assert_eq!(spacing.get(SpacingSize::Md), "1rem");
        assert_eq!(spacing.get(SpacingSize::Lg), "1.5rem");
    }

    #[test]
    fn test_spacing_scale_custom() {
        let spacing =
            SpacingScale::custom("0.1rem", "0.2rem", "0.5rem", "1rem", "2rem", "4rem", "8rem");
        assert_eq!(spacing.get(SpacingSize::Xs), "0.1rem");
        assert_eq!(spacing.get(SpacingSize::Sm), "0.2rem");
        assert_eq!(spacing.get(SpacingSize::Md), "0.5rem");
    }

    #[test]
    fn test_font_family_class() {
        assert_eq!(FontFamily::Sans.class(), "font-sans");
        assert_eq!(FontFamily::Serif.class(), "font-serif");
        assert_eq!(FontFamily::Mono.class(), "font-mono");
        assert_eq!(
            FontFamily::Custom("custom-font".to_string()).class(),
            "custom-font"
        );
    }

    #[test]
    fn test_typography_scale_new() {
        let typography = TypographyScale::new();
        assert_eq!(typography.font_family, FontFamily::Sans);
        assert_eq!(typography.font_sizes.xs, "0.75rem");
        assert_eq!(typography.font_sizes.base, "1rem");
    }

    #[test]
    fn test_typography_scale_font_family() {
        let typography = TypographyScale::new().font_family(FontFamily::Serif);
        assert_eq!(typography.font_family, FontFamily::Serif);
    }

    #[test]
    fn test_theme_preset_light() {
        let theme = ThemePreset::Light.create();
        assert_eq!(theme.primary_color, Color::Blue);
        assert_eq!(theme.background_color, Color::Gray); // Using Gray as placeholder for White
        assert_eq!(theme.text_color, Color::Gray);
    }

    #[test]
    fn test_theme_preset_dark() {
        let theme = ThemePreset::Dark.create();
        assert_eq!(theme.primary_color, Color::Blue);
        assert_eq!(theme.background_color, Color::Gray); // Using Gray as placeholder for Black
        assert_eq!(theme.text_color, Color::Gray); // Using Gray as placeholder for White
    }

    #[test]
    fn test_theme_preset_professional() {
        let theme = ThemePreset::Professional.create();
        assert_eq!(theme.primary_color, Color::Blue);
        assert_eq!(theme.secondary_color, Color::Gray);
        assert_eq!(theme.accent_color, Color::Blue);
    }

    #[test]
    fn test_theme_preset_minimal() {
        let theme = ThemePreset::Minimal.create();
        assert_eq!(theme.primary_color, Color::Gray);
        assert_eq!(theme.secondary_color, Color::Gray);
        assert_eq!(theme.accent_color, Color::Gray);
    }

    #[test]
    fn test_theme_preset_vibrant() {
        let theme = ThemePreset::Vibrant.create();
        assert_eq!(theme.primary_color, Color::Blue);
        assert_eq!(theme.secondary_color, Color::Green);
        assert_eq!(theme.accent_color, Color::Yellow);
    }

    // Mock component for testing ThemedComponent trait
    struct MockButton {
        variant: ThemeVariant,
    }

    impl MockButton {
        fn new(variant: ThemeVariant) -> Self {
            Self { variant }
        }
    }

    impl ThemedComponent for MockButton {
        fn base_classes(&self) -> &str {
            "px-4 py-2 rounded"
        }

        fn apply_theme(&self, theme: &Theme) -> String {
            match self.variant {
                ThemeVariant::Primary => {
                    format!(
                        "{} bg-{} text-white",
                        self.base_classes(),
                        theme.primary_color.name().to_lowercase()
                    )
                }
                ThemeVariant::Secondary => {
                    format!(
                        "{} bg-{} text-{}",
                        self.base_classes(),
                        theme.secondary_color.name().to_lowercase(),
                        theme.secondary_color.name().to_lowercase()
                    )
                }
                _ => self.base_classes().to_string(),
            }
        }
    }

    #[test]
    fn test_themed_component_primary() {
        let theme = Theme::new().primary_color(Color::Blue);
        let button = MockButton::new(ThemeVariant::Primary);
        let classes = theme.apply_to_component(&button);
        assert!(classes.contains("px-4 py-2 rounded"));
        assert!(classes.contains("bg-blue"));
        assert!(classes.contains("text-white"));
    }

    #[test]
    fn test_themed_component_secondary() {
        let theme = Theme::new().secondary_color(Color::Gray);
        let button = MockButton::new(ThemeVariant::Secondary);
        let classes = theme.apply_to_component(&button);
        assert!(classes.contains("px-4 py-2 rounded"));
        assert!(classes.contains("bg-gray"));
        assert!(classes.contains("text-gray"));
    }

    #[test]
    fn test_themed_component_variants() {
        let button = MockButton::new(ThemeVariant::Primary);
        let variants = button.theme_variants();
        assert!(variants.contains(&ThemeVariant::Primary));
        assert!(variants.contains(&ThemeVariant::Secondary));
        assert!(variants.contains(&ThemeVariant::Danger));
        assert!(variants.contains(&ThemeVariant::Success));
    }
}
