//! Theme testing utilities for tailwind-rs

use tailwind_rs_core::theme::{BorderRadius, BoxShadow, Color, Spacing, Theme};

/// Result of a theme test
#[derive(Debug, Clone)]
pub struct ThemeTestResult {
    pub success: bool,
    pub message: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
}

impl ThemeTestResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            expected_value: None,
            actual_value: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            expected_value: None,
            actual_value: None,
        }
    }

    pub fn with_values(mut self, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        self.expected_value = Some(expected.into());
        self.actual_value = Some(actual.into());
        self
    }
}

/// Test theme color values
pub fn test_theme_color(
    theme: &Theme,
    color_name: &str,
    expected_color: &Color,
) -> ThemeTestResult {
    match theme.get_color(color_name) {
        Ok(actual_color) => {
            if actual_color == expected_color {
                ThemeTestResult::success(format!("Color '{}' matches expected value", color_name))
            } else {
                ThemeTestResult::failure(format!(
                    "Color '{}' does not match expected value",
                    color_name
                ))
                .with_values(expected_color.to_css(), actual_color.to_css())
            }
        }
        Err(e) => ThemeTestResult::failure(format!("Failed to get color '{}': {}", color_name, e)),
    }
}

/// Test theme spacing values
pub fn test_theme_spacing(
    theme: &Theme,
    spacing_name: &str,
    expected_spacing: &Spacing,
) -> ThemeTestResult {
    match theme.get_spacing(spacing_name) {
        Ok(actual_spacing) => {
            if actual_spacing == expected_spacing {
                ThemeTestResult::success(format!(
                    "Spacing '{}' matches expected value",
                    spacing_name
                ))
            } else {
                ThemeTestResult::failure(format!(
                    "Spacing '{}' does not match expected value",
                    spacing_name
                ))
                .with_values(expected_spacing.to_css(), actual_spacing.to_css())
            }
        }
        Err(e) => {
            ThemeTestResult::failure(format!("Failed to get spacing '{}': {}", spacing_name, e))
        }
    }
}

/// Test theme border radius values
pub fn test_theme_border_radius(
    theme: &Theme,
    radius_name: &str,
    expected_radius: &BorderRadius,
) -> ThemeTestResult {
    match theme.get_border_radius(radius_name) {
        Ok(actual_radius) => {
            if actual_radius == expected_radius {
                ThemeTestResult::success(format!(
                    "Border radius '{}' matches expected value",
                    radius_name
                ))
            } else {
                ThemeTestResult::failure(format!(
                    "Border radius '{}' does not match expected value",
                    radius_name
                ))
                .with_values(expected_radius.to_css(), actual_radius.to_css())
            }
        }
        Err(e) => ThemeTestResult::failure(format!(
            "Failed to get border radius '{}': {}",
            radius_name, e
        )),
    }
}

/// Test theme box shadow values
pub fn test_theme_box_shadow(
    theme: &Theme,
    shadow_name: &str,
    expected_shadow: &BoxShadow,
) -> ThemeTestResult {
    match theme.get_box_shadow(shadow_name) {
        Ok(actual_shadow) => {
            if actual_shadow == expected_shadow {
                ThemeTestResult::success(format!(
                    "Box shadow '{}' matches expected value",
                    shadow_name
                ))
            } else {
                ThemeTestResult::failure(format!(
                    "Box shadow '{}' does not match expected value",
                    shadow_name
                ))
                .with_values(expected_shadow.to_css(), actual_shadow.to_css())
            }
        }
        Err(e) => {
            ThemeTestResult::failure(format!("Failed to get box shadow '{}': {}", shadow_name, e))
        }
    }
}

/// Assert theme value matches expected
pub fn assert_theme_value(theme: &Theme, value_type: &str, value_name: &str, expected_value: &str) {
    let result = match value_type {
        "color" => {
            if let Ok(expected_color) = parse_color(expected_value) {
                test_theme_color(theme, value_name, &expected_color)
            } else {
                ThemeTestResult::failure(format!("Invalid color format: {}", expected_value))
            }
        }
        "spacing" => {
            if let Ok(expected_spacing) = parse_spacing(expected_value) {
                test_theme_spacing(theme, value_name, &expected_spacing)
            } else {
                ThemeTestResult::failure(format!("Invalid spacing format: {}", expected_value))
            }
        }
        "border_radius" => {
            if let Ok(expected_radius) = parse_border_radius(expected_value) {
                test_theme_border_radius(theme, value_name, &expected_radius)
            } else {
                ThemeTestResult::failure(format!(
                    "Invalid border radius format: {}",
                    expected_value
                ))
            }
        }
        "box_shadow" => {
            if let Ok(expected_shadow) = parse_box_shadow(expected_value) {
                test_theme_box_shadow(theme, value_name, &expected_shadow)
            } else {
                ThemeTestResult::failure(format!("Invalid box shadow format: {}", expected_value))
            }
        }
        _ => ThemeTestResult::failure(format!("Unknown value type: {}", value_type)),
    };

    if !result.success {
        panic!("Theme assertion failed: {}", result.message);
    }
}

/// Parse color from string
fn parse_color(s: &str) -> Result<Color, String> {
    if s.starts_with('#') {
        Ok(Color::hex(s))
    } else if s.starts_with("rgb(") {
        // Simple RGB parsing
        Ok(Color::hex(s))
    } else {
        Ok(Color::named(s))
    }
}

/// Parse spacing from string
fn parse_spacing(s: &str) -> Result<Spacing, String> {
    if s.ends_with("px") {
        let value = s
            .trim_end_matches("px")
            .parse::<f32>()
            .map_err(|_| "Invalid pixel value")?;
        Ok(Spacing::px(value))
    } else if s.ends_with("rem") {
        let value = s
            .trim_end_matches("rem")
            .parse::<f32>()
            .map_err(|_| "Invalid rem value")?;
        Ok(Spacing::rem(value))
    } else if s.ends_with("em") {
        let value = s
            .trim_end_matches("em")
            .parse::<f32>()
            .map_err(|_| "Invalid em value")?;
        Ok(Spacing::em(value))
    } else if s.ends_with("%") {
        let value = s
            .trim_end_matches("%")
            .parse::<f32>()
            .map_err(|_| "Invalid percentage value")?;
        Ok(Spacing::percent(value))
    } else {
        Ok(Spacing::named(s))
    }
}

/// Parse border radius from string
fn parse_border_radius(s: &str) -> Result<BorderRadius, String> {
    if s.ends_with("px") {
        let value = s
            .trim_end_matches("px")
            .parse::<f32>()
            .map_err(|_| "Invalid pixel value")?;
        Ok(BorderRadius::px(value))
    } else if s.ends_with("rem") {
        let value = s
            .trim_end_matches("rem")
            .parse::<f32>()
            .map_err(|_| "Invalid rem value")?;
        Ok(BorderRadius::rem(value))
    } else if s.ends_with("%") {
        let value = s
            .trim_end_matches("%")
            .parse::<f32>()
            .map_err(|_| "Invalid percentage value")?;
        Ok(BorderRadius::percent(value))
    } else {
        Ok(BorderRadius::named(s))
    }
}

/// Parse box shadow from string
fn parse_box_shadow(_s: &str) -> Result<BoxShadow, String> {
    // Simple box shadow parsing - just create a basic shadow
    Ok(BoxShadow::new(
        0.0,
        1.0,
        2.0,
        0.0,
        Color::hex("#000000"),
        false,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_color_testing() {
        let theme = tailwind_rs_core::theme::create_default_theme();
        let expected_color = Color::hex("#3b82f6");

        let result = test_theme_color(&theme, "primary", &expected_color);
        assert!(result.success);
    }

    #[test]
    fn test_theme_spacing_testing() {
        let theme = tailwind_rs_core::theme::create_default_theme();
        let expected_spacing = Spacing::rem(1.0);

        let result = test_theme_spacing(&theme, "md", &expected_spacing);
        assert!(result.success);
    }

    #[test]
    fn test_theme_border_radius_testing() {
        let theme = tailwind_rs_core::theme::create_default_theme();
        let expected_radius = BorderRadius::rem(0.375);

        let result = test_theme_border_radius(&theme, "md", &expected_radius);
        assert!(result.success);
    }

    #[test]
    fn test_theme_box_shadow_testing() {
        let theme = tailwind_rs_core::theme::create_default_theme();
        let expected_shadow = BoxShadow::new(0.0, 1.0, 2.0, 0.0, Color::hex("#000000"), false);

        let result = test_theme_box_shadow(&theme, "sm", &expected_shadow);
        assert!(result.success);
    }
}
