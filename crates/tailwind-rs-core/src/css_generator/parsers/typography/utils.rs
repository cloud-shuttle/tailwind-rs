//! Typography Utilities
//!
//! This module provides utility functions for typography parsing.

/// Get color value from class name
pub fn get_color_value(class: &str) -> Option<String> {
    match class {
        "black" => Some("#000000".to_string()),
        "white" => Some("#ffffff".to_string()),
        "gray-500" => Some("#6b7280".to_string()),
        "blue-500" => Some("#3b82f6".to_string()),
        "red-500" => Some("#ef4444".to_string()),
        "green-500" => Some("#22c55e".to_string()),
        "yellow-500" => Some("#eab308".to_string()),
        "purple-500" => Some("#a855f7".to_string()),
        "pink-500" => Some("#ec4899".to_string()),
        "indigo-500" => Some("#6366f1".to_string()),
        _ => None,
    }
}

/// Get spacing value from class name
pub fn get_spacing_value(value: &str) -> Option<String> {
    match value {
        "0" => Some("0px".to_string()),
        "px" => Some("1px".to_string()),
        "0.5" => Some("0.125rem".to_string()),
        "1" => Some("0.25rem".to_string()),
        "1.5" => Some("0.375rem".to_string()),
        "2" => Some("0.5rem".to_string()),
        "2.5" => Some("0.625rem".to_string()),
        "3" => Some("0.75rem".to_string()),
        "3.5" => Some("0.875rem".to_string()),
        "4" => Some("1rem".to_string()),
        "5" => Some("1.25rem".to_string()),
        "6" => Some("1.5rem".to_string()),
        "7" => Some("1.75rem".to_string()),
        "8" => Some("2rem".to_string()),
        "9" => Some("2.25rem".to_string()),
        "10" => Some("2.5rem".to_string()),
        "11" => Some("2.75rem".to_string()),
        "12" => Some("3rem".to_string()),
        "14" => Some("3.5rem".to_string()),
        "16" => Some("4rem".to_string()),
        "20" => Some("5rem".to_string()),
        "24" => Some("6rem".to_string()),
        "28" => Some("7rem".to_string()),
        "32" => Some("8rem".to_string()),
        "36" => Some("9rem".to_string()),
        "40" => Some("10rem".to_string()),
        "44" => Some("11rem".to_string()),
        "48" => Some("12rem".to_string()),
        "52" => Some("13rem".to_string()),
        "56" => Some("14rem".to_string()),
        "60" => Some("15rem".to_string()),
        "64" => Some("16rem".to_string()),
        "72" => Some("18rem".to_string()),
        "80" => Some("20rem".to_string()),
        "96" => Some("24rem".to_string()),
        _ => None,
    }
}
