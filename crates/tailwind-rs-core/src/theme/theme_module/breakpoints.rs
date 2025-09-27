//! Breakpoints system for tailwind-rs
//!
//! This module provides breakpoint utilities and definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Breakpoint definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    /// Small breakpoint (640px)
    Sm,
    /// Medium breakpoint (768px)
    Md,
    /// Large breakpoint (1024px)
    Lg,
    /// Extra large breakpoint (1280px)
    Xl,
    /// 2X large breakpoint (1536px)
    Xl2,
    /// Custom breakpoint
    Custom(u32),
}

impl Breakpoint {
    /// Get CSS media query for the breakpoint
    pub fn media_query(&self) -> String {
        match self {
            Breakpoint::Sm => "(min-width: 640px)".to_string(),
            Breakpoint::Md => "(min-width: 768px)".to_string(),
            Breakpoint::Lg => "(min-width: 1024px)".to_string(),
            Breakpoint::Xl => "(min-width: 1280px)".to_string(),
            Breakpoint::Xl2 => "(min-width: 1536px)".to_string(),
            Breakpoint::Custom(width) => format!("(min-width: {}px)", width),
        }
    }

    /// Get breakpoint name
    pub fn name(&self) -> &'static str {
        match self {
            Breakpoint::Sm => "sm",
            Breakpoint::Md => "md",
            Breakpoint::Lg => "lg",
            Breakpoint::Xl => "xl",
            Breakpoint::Xl2 => "2xl",
            Breakpoint::Custom(_) => "custom",
        }
    }

    /// Get breakpoint width in pixels
    pub fn width(&self) -> u32 {
        match self {
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
            Breakpoint::Xl2 => 1536,
            Breakpoint::Custom(width) => *width,
        }
    }
}

/// Breakpoint system configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakpointSystem {
    /// Breakpoint definitions
    pub breakpoints: HashMap<String, Breakpoint>,
    /// Default breakpoint
    pub default: String,
}

impl Default for BreakpointSystem {
    fn default() -> Self {
        let mut breakpoints = HashMap::new();
        breakpoints.insert("sm".to_string(), Breakpoint::Sm);
        breakpoints.insert("md".to_string(), Breakpoint::Md);
        breakpoints.insert("lg".to_string(), Breakpoint::Lg);
        breakpoints.insert("xl".to_string(), Breakpoint::Xl);
        breakpoints.insert("2xl".to_string(), Breakpoint::Xl2);

        Self {
            breakpoints,
            default: "sm".to_string(),
        }
    }
}

impl BreakpointSystem {
    /// Create a new breakpoint system
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, name: impl Into<String>, breakpoint: Breakpoint) {
        self.breakpoints.insert(name.into(), breakpoint);
    }

    /// Get a breakpoint
    pub fn get_breakpoint(&self, name: &str) -> Option<&Breakpoint> {
        self.breakpoints.get(name)
    }

    /// Set default breakpoint
    pub fn set_default(&mut self, name: impl Into<String>) {
        self.default = name.into();
    }

    /// Get default breakpoint
    pub fn get_default(&self) -> &str {
        &self.default
    }
}
