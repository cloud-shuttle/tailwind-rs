//! # Responsive Configuration Management
//!
//! This module provides configuration management for responsive design.

use super::breakpoints::Breakpoint;
use crate::error::{Result, TailwindError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for responsive design
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    /// Breakpoint configurations
    pub breakpoints: HashMap<Breakpoint, BreakpointConfig>,
    /// Default settings
    pub defaults: ResponsiveDefaults,
}

/// Configuration for a specific breakpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakpointConfig {
    /// Minimum width for this breakpoint
    pub min_width: u32,
    /// Maximum width for this breakpoint (optional)
    pub max_width: Option<u32>,
    /// Whether this breakpoint is enabled
    pub enabled: bool,
    /// Custom media query (optional)
    pub media_query: Option<String>,
}

/// Default settings for responsive design
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveDefaults {
    /// Default breakpoint to use when none is specified
    pub default_breakpoint: Breakpoint,
    /// Whether to include base breakpoint in generated classes
    pub include_base: bool,
    /// Whether to use mobile-first approach
    pub mobile_first: bool,
}

impl ResponsiveConfig {
    /// Create a new responsive configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a responsive configuration with custom breakpoints
    pub fn with_breakpoints(breakpoints: HashMap<Breakpoint, BreakpointConfig>) -> Self {
        Self {
            breakpoints,
            defaults: ResponsiveDefaults::default(),
        }
    }
    
    /// Get configuration for a specific breakpoint
    pub fn get_breakpoint_config(&self, breakpoint: Breakpoint) -> Option<&BreakpointConfig> {
        self.breakpoints.get(&breakpoint)
    }
    
    /// Set configuration for a specific breakpoint
    pub fn set_breakpoint_config(&mut self, breakpoint: Breakpoint, config: BreakpointConfig) {
        self.breakpoints.insert(breakpoint, config);
    }
    
    /// Check if a breakpoint is enabled
    pub fn is_breakpoint_enabled(&self, breakpoint: Breakpoint) -> bool {
        self.breakpoints
            .get(&breakpoint)
            .map(|config| config.enabled)
            .unwrap_or(true)
    }
    
    /// Get the minimum width for a breakpoint
    pub fn get_breakpoint_min_width(&self, breakpoint: Breakpoint) -> u32 {
        self.breakpoints
            .get(&breakpoint)
            .map(|config| config.min_width)
            .unwrap_or_else(|| breakpoint.min_width())
    }
    
    /// Get the maximum width for a breakpoint
    pub fn get_breakpoint_max_width(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.breakpoints
            .get(&breakpoint)
            .and_then(|config| config.max_width)
    }
    
    /// Get the media query for a breakpoint
    pub fn get_breakpoint_media_query(&self, breakpoint: Breakpoint) -> String {
        if let Some(config) = self.breakpoints.get(&breakpoint) {
            if let Some(ref media_query) = config.media_query {
                return media_query.clone();
            }
        }
        
        // Generate default media query
        let min_width = self.get_breakpoint_min_width(breakpoint);
        if min_width == 0 {
            String::new()
        } else {
            format!("(min-width: {}px)", min_width)
        }
    }
    
    /// Get all enabled breakpoints
    pub fn get_enabled_breakpoints(&self) -> Vec<Breakpoint> {
        self.breakpoints
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(breakpoint, _)| *breakpoint)
            .collect()
    }
    
    /// Get the appropriate breakpoint for a given screen width
    pub fn get_breakpoint_for_width(&self, screen_width: u32) -> Breakpoint {
        if screen_width >= Breakpoint::Xl2.min_width() {
            Breakpoint::Xl2
        } else if screen_width >= Breakpoint::Xl.min_width() {
            Breakpoint::Xl
        } else if screen_width >= Breakpoint::Lg.min_width() {
            Breakpoint::Lg
        } else if screen_width >= Breakpoint::Md.min_width() {
            Breakpoint::Md
        } else if screen_width >= Breakpoint::Sm.min_width() {
            Breakpoint::Sm
        } else {
            Breakpoint::Base
        }
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Check that base breakpoint exists
        if !self.breakpoints.contains_key(&Breakpoint::Base) {
            return Err(TailwindError::config(
                "Base breakpoint is required".to_string(),
            ));
        }
        
        // Check that breakpoints are in order
        let mut breakpoints: Vec<Breakpoint> = self.breakpoints.keys().cloned().collect();
        breakpoints.sort_by_key(|bp| bp.min_width());
        
        for i in 1..breakpoints.len() {
            let prev_min = self.get_breakpoint_min_width(breakpoints[i - 1]);
            let curr_min = self.get_breakpoint_min_width(breakpoints[i]);
            
            if prev_min >= curr_min {
                return Err(TailwindError::config(format!(
                    "Breakpoint {} ({}px) must be greater than {} ({}px)",
                    breakpoints[i],
                    curr_min,
                    breakpoints[i - 1],
                    prev_min
                )));
            }
        }
        
        Ok(())
    }
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        let mut breakpoints = HashMap::new();
        
        // Add default breakpoint configurations
        breakpoints.insert(
            Breakpoint::Base,
            BreakpointConfig {
                min_width: 0,
                max_width: None,
                enabled: true,
                media_query: None,
            },
        );
        
        breakpoints.insert(
            Breakpoint::Sm,
            BreakpointConfig {
                min_width: 640,
                max_width: None,
                enabled: true,
                media_query: None,
            },
        );
        
        breakpoints.insert(
            Breakpoint::Md,
            BreakpointConfig {
                min_width: 768,
                max_width: None,
                enabled: true,
                media_query: None,
            },
        );
        
        breakpoints.insert(
            Breakpoint::Lg,
            BreakpointConfig {
                min_width: 1024,
                max_width: None,
                enabled: true,
                media_query: None,
            },
        );
        
        breakpoints.insert(
            Breakpoint::Xl,
            BreakpointConfig {
                min_width: 1280,
                max_width: None,
                enabled: true,
                media_query: None,
            },
        );
        
        breakpoints.insert(
            Breakpoint::Xl2,
            BreakpointConfig {
                min_width: 1536,
                max_width: None,
                enabled: true,
                media_query: None,
            },
        );
        
        Self {
            breakpoints,
            defaults: ResponsiveDefaults::default(),
        }
    }
}

impl Default for ResponsiveDefaults {
    fn default() -> Self {
        Self {
            default_breakpoint: Breakpoint::Base,
            include_base: true,
            mobile_first: true,
        }
    }
}

/// Main responsive struct that uses the configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Responsive {
    /// The configuration
    pub config: ResponsiveConfig,
    /// Current breakpoint
    pub current_breakpoint: Breakpoint,
}

impl Responsive {
    /// Create a new responsive instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a responsive instance with custom configuration
    pub fn with_config(config: ResponsiveConfig) -> Self {
        Self {
            current_breakpoint: config.defaults.default_breakpoint,
            config,
        }
    }
    
    /// Get the current breakpoint
    pub fn get_current_breakpoint(&self) -> Breakpoint {
        self.current_breakpoint
    }
    
    /// Set the current breakpoint
    pub fn set_current_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.current_breakpoint = breakpoint;
    }
    
    /// Get the configuration
    pub fn get_config(&self) -> &ResponsiveConfig {
        &self.config
    }
    
    /// Update the configuration
    pub fn update_config(&mut self, config: ResponsiveConfig) {
        self.config = config;
    }
    
    /// Check if a breakpoint is active for the current screen width
    pub fn is_breakpoint_active(&self, breakpoint: Breakpoint, screen_width: u32) -> bool {
        if !self.config.is_breakpoint_enabled(breakpoint) {
            return false;
        }
        
        let min_width = self.config.get_breakpoint_min_width(breakpoint);
        let max_width = self.config.get_breakpoint_max_width(breakpoint);
        
        let min_active = screen_width >= min_width;
        let max_active = max_width.map_or(true, |max| screen_width < max);
        
        min_active && max_active
    }
    
    /// Get the appropriate breakpoint for a given screen width
    pub fn get_breakpoint_for_width(&self, screen_width: u32) -> Breakpoint {
        let enabled_breakpoints = self.config.get_enabled_breakpoints();
        
        // Find the highest breakpoint that is active for this screen width
        let active_breakpoints: Vec<Breakpoint> = enabled_breakpoints
            .into_iter()
            .filter(|&bp| self.is_breakpoint_active(bp, screen_width))
            .collect();
        
        if active_breakpoints.is_empty() {
            return self.config.defaults.default_breakpoint;
        }
        
        // Find the breakpoint with the highest min_width among active ones
        active_breakpoints
            .into_iter()
            .max_by_key(|bp| self.config.get_breakpoint_min_width(*bp))
            .unwrap_or(self.config.defaults.default_breakpoint)
    }
}

impl Default for Responsive {
    fn default() -> Self {
        Self {
            config: ResponsiveConfig::default(),
            current_breakpoint: Breakpoint::Base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_config_new() {
        let config = ResponsiveConfig::new();
        assert_eq!(config.breakpoints.len(), 6);
        assert!(config.breakpoints.contains_key(&Breakpoint::Base));
        assert!(config.breakpoints.contains_key(&Breakpoint::Sm));
        assert!(config.breakpoints.contains_key(&Breakpoint::Md));
        assert!(config.breakpoints.contains_key(&Breakpoint::Lg));
        assert!(config.breakpoints.contains_key(&Breakpoint::Xl));
        assert!(config.breakpoints.contains_key(&Breakpoint::Xl2));
    }

    #[test]
    fn test_responsive_config_get_breakpoint_config() {
        let config = ResponsiveConfig::new();
        let base_config = config.get_breakpoint_config(Breakpoint::Base);
        assert!(base_config.is_some());
        assert_eq!(base_config.unwrap().min_width, 0);
        
        let sm_config = config.get_breakpoint_config(Breakpoint::Sm);
        assert!(sm_config.is_some());
        assert_eq!(sm_config.unwrap().min_width, 640);
    }

    #[test]
    fn test_responsive_config_set_breakpoint_config() {
        let mut config = ResponsiveConfig::new();
        let custom_config = BreakpointConfig {
            min_width: 800,
            max_width: Some(1200),
            enabled: true,
            media_query: Some("(min-width: 800px) and (max-width: 1200px)".to_string()),
        };
        
        config.set_breakpoint_config(Breakpoint::Md, custom_config.clone());
        let retrieved_config = config.get_breakpoint_config(Breakpoint::Md);
        assert_eq!(retrieved_config, Some(&custom_config));
    }

    #[test]
    fn test_responsive_config_is_breakpoint_enabled() {
        let config = ResponsiveConfig::new();
        assert!(config.is_breakpoint_enabled(Breakpoint::Base));
        assert!(config.is_breakpoint_enabled(Breakpoint::Sm));
        assert!(config.is_breakpoint_enabled(Breakpoint::Md));
    }

    #[test]
    fn test_responsive_config_get_breakpoint_min_width() {
        let config = ResponsiveConfig::new();
        assert_eq!(config.get_breakpoint_min_width(Breakpoint::Base), 0);
        assert_eq!(config.get_breakpoint_min_width(Breakpoint::Sm), 640);
        assert_eq!(config.get_breakpoint_min_width(Breakpoint::Md), 768);
    }

    #[test]
    fn test_responsive_config_get_breakpoint_media_query() {
        let config = ResponsiveConfig::new();
        assert_eq!(config.get_breakpoint_media_query(Breakpoint::Base), "");
        assert_eq!(config.get_breakpoint_media_query(Breakpoint::Sm), "(min-width: 640px)");
        assert_eq!(config.get_breakpoint_media_query(Breakpoint::Md), "(min-width: 768px)");
    }

    #[test]
    fn test_responsive_config_get_enabled_breakpoints() {
        let config = ResponsiveConfig::new();
        let enabled = config.get_enabled_breakpoints();
        assert_eq!(enabled.len(), 6);
        assert!(enabled.contains(&Breakpoint::Base));
        assert!(enabled.contains(&Breakpoint::Sm));
        assert!(enabled.contains(&Breakpoint::Md));
    }

    #[test]
    fn test_responsive_config_validate() {
        let config = ResponsiveConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_responsive_new() {
        let responsive = Responsive::new();
        assert_eq!(responsive.get_current_breakpoint(), Breakpoint::Base);
        assert_eq!(responsive.get_config().breakpoints.len(), 6);
    }

    #[test]
    fn test_responsive_set_current_breakpoint() {
        let mut responsive = Responsive::new();
        responsive.set_current_breakpoint(Breakpoint::Md);
        assert_eq!(responsive.get_current_breakpoint(), Breakpoint::Md);
    }

    #[test]
    fn test_responsive_is_breakpoint_active() {
        let responsive = Responsive::new();
        assert!(responsive.is_breakpoint_active(Breakpoint::Base, 0));
        assert!(responsive.is_breakpoint_active(Breakpoint::Sm, 640));
        assert!(responsive.is_breakpoint_active(Breakpoint::Md, 768));
        assert!(!responsive.is_breakpoint_active(Breakpoint::Sm, 639));
    }

    #[test]
    fn test_responsive_get_breakpoint_for_width() {
        let responsive = Responsive::new();
        assert_eq!(responsive.get_breakpoint_for_width(0), Breakpoint::Base);
        assert_eq!(responsive.get_breakpoint_for_width(640), Breakpoint::Sm);
        assert_eq!(responsive.get_breakpoint_for_width(768), Breakpoint::Md);
        assert_eq!(responsive.get_breakpoint_for_width(1024), Breakpoint::Lg);
        assert_eq!(responsive.get_breakpoint_for_width(1280), Breakpoint::Xl);
        assert_eq!(responsive.get_breakpoint_for_width(1536), Breakpoint::Xl2);
    }
}
