//! Theme manager
//!
//! This module provides theme management functionality.

use super::ThemeConfig;
use std::collections::HashMap;

/// Theme manager
pub struct ThemeManager {
    themes: HashMap<String, ThemeConfig>,
    current_theme: String,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: "default".to_string(),
        };

        // Add default theme
        manager.add_theme(ThemeConfig::default());
        manager
    }

    /// Add a theme
    pub fn add_theme(&mut self, theme: ThemeConfig) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Get a theme
    pub fn get_theme(&self, name: &str) -> Option<&ThemeConfig> {
        self.themes.get(name)
    }

    /// Set current theme
    pub fn set_current_theme(&mut self, name: impl Into<String>) -> Result<(), String> {
        let name = name.into();
        if self.themes.contains_key(&name) {
            self.current_theme = name;
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    /// Get current theme
    pub fn current_theme(&self) -> &ThemeConfig {
        self.themes.get(&self.current_theme).unwrap()
    }

    /// List all theme names
    pub fn list_themes(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    /// Check if theme exists
    pub fn has_theme(&self, name: &str) -> bool {
        self.themes.contains_key(name)
    }
}
