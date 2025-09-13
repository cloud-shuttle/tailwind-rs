//! # Tailwind-RS WASM
//! 
//! A WASM-optimized implementation of Tailwind CSS for Rust web applications.
//! This crate provides a subset of the full Tailwind-RS functionality that's
//! optimized for WebAssembly and browser environments.

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(target_arch = "wasm32")]
extern crate alloc;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use alloc::{string::String, vec::Vec, format, vec, string::ToString};

#[cfg(not(target_arch = "wasm32"))]
use std::{string::String, vec::Vec, format, vec, string::ToString};

#[cfg(target_arch = "wasm32")]
use core::fmt;

#[cfg(not(target_arch = "wasm32"))]
use std::fmt;

/// Initialize WASM-specific functionality
#[cfg(target_arch = "wasm32")]
pub fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logging");
}

/// WASM-optimized class builder
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmClassBuilder {
    classes: Vec<String>,
}

#[wasm_bindgen]
impl WasmClassBuilder {
    /// Create a new WASM class builder
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmClassBuilder {
        WasmClassBuilder {
            classes: Vec::new(),
        }
    }
    
    /// Add a class to the builder
    pub fn class(&mut self, class: &str) {
        self.classes.push(class.to_string());
    }
    
    /// Add multiple classes at once
    pub fn add_classes(&mut self, classes: &str) {
        for class in classes.split_whitespace() {
            if !class.is_empty() {
                self.classes.push(class.to_string());
            }
        }
    }
    
    /// Build the final class string
    pub fn build(&self) -> String {
        self.classes.join(" ")
    }
    
    /// Get the number of classes
    pub fn len(&self) -> usize {
        self.classes.len()
    }
    
    /// Check if the builder is empty
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
}

impl Default for WasmClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// WASM-optimized responsive breakpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmBreakpoint {
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
}

impl WasmBreakpoint {
    /// Get the CSS media query for this breakpoint
    pub fn media_query(&self) -> &'static str {
        match self {
            WasmBreakpoint::Sm => "(min-width: 640px)",
            WasmBreakpoint::Md => "(min-width: 768px)",
            WasmBreakpoint::Lg => "(min-width: 1024px)",
            WasmBreakpoint::Xl => "(min-width: 1280px)",
            WasmBreakpoint::Xl2 => "(min-width: 1536px)",
        }
    }
    
    /// Get the prefix for this breakpoint
    pub fn prefix(&self) -> &'static str {
        match self {
            WasmBreakpoint::Sm => "sm:",
            WasmBreakpoint::Md => "md:",
            WasmBreakpoint::Lg => "lg:",
            WasmBreakpoint::Xl => "xl:",
            WasmBreakpoint::Xl2 => "2xl:",
        }
    }
}

/// WASM-optimized spacing system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmSpacing {
    Px,
    Zero,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P8,
    P10,
    P12,
    P16,
    P20,
    P24,
    P32,
    P40,
    P48,
    P56,
    P64,
}

impl WasmSpacing {
    /// Get the CSS value for this spacing
    pub fn css_value(&self) -> &'static str {
        match self {
            WasmSpacing::Px => "1px",
            WasmSpacing::Zero => "0px",
            WasmSpacing::P1 => "0.25rem",
            WasmSpacing::P2 => "0.5rem",
            WasmSpacing::P3 => "0.75rem",
            WasmSpacing::P4 => "1rem",
            WasmSpacing::P5 => "1.25rem",
            WasmSpacing::P6 => "1.5rem",
            WasmSpacing::P8 => "2rem",
            WasmSpacing::P10 => "2.5rem",
            WasmSpacing::P12 => "3rem",
            WasmSpacing::P16 => "4rem",
            WasmSpacing::P20 => "5rem",
            WasmSpacing::P24 => "6rem",
            WasmSpacing::P32 => "8rem",
            WasmSpacing::P40 => "10rem",
            WasmSpacing::P48 => "12rem",
            WasmSpacing::P56 => "14rem",
            WasmSpacing::P64 => "16rem",
        }
    }
    
    /// Get the Tailwind class name for padding
    pub fn padding_class(&self) -> String {
        match self {
            WasmSpacing::Px => "p-px".to_string(),
            WasmSpacing::Zero => "p-0".to_string(),
            WasmSpacing::P1 => "p-1".to_string(),
            WasmSpacing::P2 => "p-2".to_string(),
            WasmSpacing::P3 => "p-3".to_string(),
            WasmSpacing::P4 => "p-4".to_string(),
            WasmSpacing::P5 => "p-5".to_string(),
            WasmSpacing::P6 => "p-6".to_string(),
            WasmSpacing::P8 => "p-8".to_string(),
            WasmSpacing::P10 => "p-10".to_string(),
            WasmSpacing::P12 => "p-12".to_string(),
            WasmSpacing::P16 => "p-16".to_string(),
            WasmSpacing::P20 => "p-20".to_string(),
            WasmSpacing::P24 => "p-24".to_string(),
            WasmSpacing::P32 => "p-32".to_string(),
            WasmSpacing::P40 => "p-40".to_string(),
            WasmSpacing::P48 => "p-48".to_string(),
            WasmSpacing::P56 => "p-56".to_string(),
            WasmSpacing::P64 => "p-64".to_string(),
        }
    }
    
    /// Get the Tailwind class name for margin
    pub fn margin_class(&self) -> String {
        match self {
            WasmSpacing::Px => "m-px".to_string(),
            WasmSpacing::Zero => "m-0".to_string(),
            WasmSpacing::P1 => "m-1".to_string(),
            WasmSpacing::P2 => "m-2".to_string(),
            WasmSpacing::P3 => "m-3".to_string(),
            WasmSpacing::P4 => "m-4".to_string(),
            WasmSpacing::P5 => "m-5".to_string(),
            WasmSpacing::P6 => "m-6".to_string(),
            WasmSpacing::P8 => "m-8".to_string(),
            WasmSpacing::P10 => "m-10".to_string(),
            WasmSpacing::P12 => "m-12".to_string(),
            WasmSpacing::P16 => "m-16".to_string(),
            WasmSpacing::P20 => "m-20".to_string(),
            WasmSpacing::P24 => "m-24".to_string(),
            WasmSpacing::P32 => "m-32".to_string(),
            WasmSpacing::P40 => "m-40".to_string(),
            WasmSpacing::P48 => "m-48".to_string(),
            WasmSpacing::P56 => "m-56".to_string(),
            WasmSpacing::P64 => "m-64".to_string(),
        }
    }
}

/// WASM-optimized color system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmColor {
    // Grays
    Gray50,
    Gray100,
    Gray200,
    Gray300,
    Gray400,
    Gray500,
    Gray600,
    Gray700,
    Gray800,
    Gray900,
    // Blues
    Blue50,
    Blue100,
    Blue200,
    Blue300,
    Blue400,
    Blue500,
    Blue600,
    Blue700,
    Blue800,
    Blue900,
    // Reds
    Red50,
    Red100,
    Red200,
    Red300,
    Red400,
    Red500,
    Red600,
    Red700,
    Red800,
    Red900,
    // Greens
    Green50,
    Green100,
    Green200,
    Green300,
    Green400,
    Green500,
    Green600,
    Green700,
    Green800,
    Green900,
    // Special
    White,
    Black,
    Transparent,
}

impl WasmColor {
    /// Get the Tailwind class name for text color
    pub fn text_class(&self) -> String {
        match self {
            WasmColor::Gray50 => "text-gray-50".to_string(),
            WasmColor::Gray100 => "text-gray-100".to_string(),
            WasmColor::Gray200 => "text-gray-200".to_string(),
            WasmColor::Gray300 => "text-gray-300".to_string(),
            WasmColor::Gray400 => "text-gray-400".to_string(),
            WasmColor::Gray500 => "text-gray-500".to_string(),
            WasmColor::Gray600 => "text-gray-600".to_string(),
            WasmColor::Gray700 => "text-gray-700".to_string(),
            WasmColor::Gray800 => "text-gray-800".to_string(),
            WasmColor::Gray900 => "text-gray-900".to_string(),
            WasmColor::Blue50 => "text-blue-50".to_string(),
            WasmColor::Blue100 => "text-blue-100".to_string(),
            WasmColor::Blue200 => "text-blue-200".to_string(),
            WasmColor::Blue300 => "text-blue-300".to_string(),
            WasmColor::Blue400 => "text-blue-400".to_string(),
            WasmColor::Blue500 => "text-blue-500".to_string(),
            WasmColor::Blue600 => "text-blue-600".to_string(),
            WasmColor::Blue700 => "text-blue-700".to_string(),
            WasmColor::Blue800 => "text-blue-800".to_string(),
            WasmColor::Blue900 => "text-blue-900".to_string(),
            WasmColor::Red50 => "text-red-50".to_string(),
            WasmColor::Red100 => "text-red-100".to_string(),
            WasmColor::Red200 => "text-red-200".to_string(),
            WasmColor::Red300 => "text-red-300".to_string(),
            WasmColor::Red400 => "text-red-400".to_string(),
            WasmColor::Red500 => "text-red-500".to_string(),
            WasmColor::Red600 => "text-red-600".to_string(),
            WasmColor::Red700 => "text-red-700".to_string(),
            WasmColor::Red800 => "text-red-800".to_string(),
            WasmColor::Red900 => "text-red-900".to_string(),
            WasmColor::Green50 => "text-green-50".to_string(),
            WasmColor::Green100 => "text-green-100".to_string(),
            WasmColor::Green200 => "text-green-200".to_string(),
            WasmColor::Green300 => "text-green-300".to_string(),
            WasmColor::Green400 => "text-green-400".to_string(),
            WasmColor::Green500 => "text-green-500".to_string(),
            WasmColor::Green600 => "text-green-600".to_string(),
            WasmColor::Green700 => "text-green-700".to_string(),
            WasmColor::Green800 => "text-green-800".to_string(),
            WasmColor::Green900 => "text-green-900".to_string(),
            WasmColor::White => "text-white".to_string(),
            WasmColor::Black => "text-black".to_string(),
            WasmColor::Transparent => "text-transparent".to_string(),
        }
    }
    
    /// Get the Tailwind class name for background color
    pub fn bg_class(&self) -> String {
        match self {
            WasmColor::Gray50 => "bg-gray-50".to_string(),
            WasmColor::Gray100 => "bg-gray-100".to_string(),
            WasmColor::Gray200 => "bg-gray-200".to_string(),
            WasmColor::Gray300 => "bg-gray-300".to_string(),
            WasmColor::Gray400 => "bg-gray-400".to_string(),
            WasmColor::Gray500 => "bg-gray-500".to_string(),
            WasmColor::Gray600 => "bg-gray-600".to_string(),
            WasmColor::Gray700 => "bg-gray-700".to_string(),
            WasmColor::Gray800 => "bg-gray-800".to_string(),
            WasmColor::Gray900 => "bg-gray-900".to_string(),
            WasmColor::Blue50 => "bg-blue-50".to_string(),
            WasmColor::Blue100 => "bg-blue-100".to_string(),
            WasmColor::Blue200 => "bg-blue-200".to_string(),
            WasmColor::Blue300 => "bg-blue-300".to_string(),
            WasmColor::Blue400 => "bg-blue-400".to_string(),
            WasmColor::Blue500 => "bg-blue-500".to_string(),
            WasmColor::Blue600 => "bg-blue-600".to_string(),
            WasmColor::Blue700 => "bg-blue-700".to_string(),
            WasmColor::Blue800 => "bg-blue-800".to_string(),
            WasmColor::Blue900 => "bg-blue-900".to_string(),
            WasmColor::Red50 => "bg-red-50".to_string(),
            WasmColor::Red100 => "bg-red-100".to_string(),
            WasmColor::Red200 => "bg-red-200".to_string(),
            WasmColor::Red300 => "bg-red-300".to_string(),
            WasmColor::Red400 => "bg-red-400".to_string(),
            WasmColor::Red500 => "bg-red-500".to_string(),
            WasmColor::Red600 => "bg-red-600".to_string(),
            WasmColor::Red700 => "bg-red-700".to_string(),
            WasmColor::Red800 => "bg-red-800".to_string(),
            WasmColor::Red900 => "bg-red-900".to_string(),
            WasmColor::Green50 => "bg-green-50".to_string(),
            WasmColor::Green100 => "bg-green-100".to_string(),
            WasmColor::Green200 => "bg-green-200".to_string(),
            WasmColor::Green300 => "bg-green-300".to_string(),
            WasmColor::Green400 => "bg-green-400".to_string(),
            WasmColor::Green500 => "bg-green-500".to_string(),
            WasmColor::Green600 => "bg-green-600".to_string(),
            WasmColor::Green700 => "bg-green-700".to_string(),
            WasmColor::Green800 => "bg-green-800".to_string(),
            WasmColor::Green900 => "bg-green-900".to_string(),
            WasmColor::White => "bg-white".to_string(),
            WasmColor::Black => "bg-black".to_string(),
            WasmColor::Transparent => "bg-transparent".to_string(),
        }
    }
}

/// WASM-optimized utility functions
pub mod utils {
    use super::*;
    
    /// Validate a Tailwind class name
    pub fn validate_class(class: &str) -> bool {
        // Basic validation - check for common patterns
        if class.is_empty() {
            return false;
        }
        
        // Check for valid characters
        for ch in class.chars() {
            if !ch.is_alphanumeric() && ch != '-' && ch != ':' && ch != '/' && ch != '[' && ch != ']' {
                return false;
            }
        }
        
        true
    }
    
    /// Parse responsive classes
    pub fn parse_responsive_class(class: &str) -> Option<(Option<WasmBreakpoint>, String)> {
        for breakpoint in [WasmBreakpoint::Sm, WasmBreakpoint::Md, WasmBreakpoint::Lg, WasmBreakpoint::Xl, WasmBreakpoint::Xl2] {
            if class.starts_with(breakpoint.prefix()) {
                let base_class = &class[breakpoint.prefix().len()..];
                return Some((Some(breakpoint), base_class.to_string()));
            }
        }
        
        Some((None, class.to_string()))
    }
    
    /// Generate responsive classes
    pub fn generate_responsive_classes(base_class: &str, breakpoints: &[WasmBreakpoint]) -> Vec<String> {
        let mut classes = vec![base_class.to_string()];
        
        for breakpoint in breakpoints {
            classes.push(format!("{}{}", breakpoint.prefix(), base_class));
        }
        
        classes
    }
}

/// WASM-specific error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasmError {
    InvalidClass(String),
    ValidationError(String),
    SerializationError(String),
}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmError::InvalidClass(msg) => write!(f, "Invalid class: {}", msg),
            WasmError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            WasmError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::error::Error for WasmError {}

/// WASM-optimized theme system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmTheme {
    pub name: String,
    pub colors: Vec<WasmColor>,
    pub spacing: Vec<WasmSpacing>,
    pub breakpoints: Vec<WasmBreakpoint>,
}

impl WasmTheme {
    /// Create a new WASM theme
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: vec![
                WasmColor::White,
                WasmColor::Black,
                WasmColor::Gray500,
                WasmColor::Blue500,
                WasmColor::Red500,
                WasmColor::Green500,
            ],
            spacing: vec![
                WasmSpacing::P1,
                WasmSpacing::P2,
                WasmSpacing::P4,
                WasmSpacing::P8,
                WasmSpacing::P16,
            ],
            breakpoints: vec![
                WasmBreakpoint::Sm,
                WasmBreakpoint::Md,
                WasmBreakpoint::Lg,
                WasmBreakpoint::Xl,
            ],
        }
    }
    
    /// Get the default theme
    pub fn default() -> Self {
        Self::new("default".to_string())
    }
}

impl Default for WasmTheme {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wasm_class_builder() {
        let builder = WasmClassBuilder::new()
            .class("bg-blue-500")
            .class("text-white")
            .class("p-4");
        
        assert_eq!(builder.clone().build(), "bg-blue-500 text-white p-4");
        assert_eq!(builder.len(), 3);
        assert!(!builder.is_empty());
    }
    
    #[test]
    fn test_wasm_spacing() {
        assert_eq!(WasmSpacing::P4.css_value(), "1rem");
        assert_eq!(WasmSpacing::P4.padding_class(), "p-4");
        assert_eq!(WasmSpacing::P4.margin_class(), "m-4");
    }
    
    #[test]
    fn test_wasm_color() {
        assert_eq!(WasmColor::Blue500.text_class(), "text-blue-500");
        assert_eq!(WasmColor::Blue500.bg_class(), "bg-blue-500");
    }
    
    #[test]
    fn test_wasm_breakpoint() {
        assert_eq!(WasmBreakpoint::Md.media_query(), "(min-width: 768px)");
        assert_eq!(WasmBreakpoint::Md.prefix(), "md:");
    }
    
    #[test]
    fn test_utils_validate_class() {
        assert!(utils::validate_class("bg-blue-500"));
        assert!(utils::validate_class("text-white"));
        assert!(!utils::validate_class(""));
        assert!(!utils::validate_class("invalid@class"));
    }
    
    #[test]
    fn test_utils_parse_responsive_class() {
        let (bp, class) = utils::parse_responsive_class("md:bg-blue-500").unwrap();
        assert_eq!(bp, Some(WasmBreakpoint::Md));
        assert_eq!(class, "bg-blue-500");
        
        let (bp, class) = utils::parse_responsive_class("bg-blue-500").unwrap();
        assert_eq!(bp, None);
        assert_eq!(class, "bg-blue-500");
    }
    
    #[test]
    fn test_wasm_theme() {
        let theme = WasmTheme::new("test".to_string());
        assert_eq!(theme.name, "test");
        assert!(!theme.colors.is_empty());
        assert!(!theme.spacing.is_empty());
        assert!(!theme.breakpoints.is_empty());
    }
}
