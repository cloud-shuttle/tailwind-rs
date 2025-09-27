//! Spacing value definitions and implementations
//!
//! This module contains the `SpacingValue` enum and its associated methods.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Spacing utility values for padding and margin
///
/// # Examples
///
/// ```rust
/// use tailwind_rs_core::utilities::spacing::{SpacingValue, PaddingUtilities};
/// use tailwind_rs_core::classes::ClassBuilder;
///
/// let classes = ClassBuilder::new()
///     .padding(SpacingValue::Integer(4))
///     .padding_x(SpacingValue::Integer(6))
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpacingValue {
    /// Zero spacing (0)
    Zero,
    /// 1px spacing
    Px,
    /// Fractional spacing (0.5, 1.5, 2.5, 3.5)
    Fractional(f32),
    /// Integer spacing (1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64, 72, 80, 96)
    Integer(u32),
    /// Auto spacing
    Auto,
    /// Full spacing (100%)
    Full,
    /// Screen spacing (100vh)
    Screen,
    /// Min content spacing
    Min,
    /// Max content spacing
    Max,
    /// Fit content spacing
    Fit,
}

impl std::hash::Hash for SpacingValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SpacingValue::Zero => 0u8.hash(state),
            SpacingValue::Px => 1u8.hash(state),
            SpacingValue::Fractional(f) => {
                2u8.hash(state);
                // Convert f32 to u32 for hashing (multiply by 1000 to preserve 3 decimal places)
                ((f * 1000.0) as u32).hash(state);
            }
            SpacingValue::Integer(i) => {
                3u8.hash(state);
                i.hash(state);
            }
            SpacingValue::Auto => 4u8.hash(state),
            SpacingValue::Full => 5u8.hash(state),
            SpacingValue::Screen => 6u8.hash(state),
            SpacingValue::Min => 7u8.hash(state),
            SpacingValue::Max => 8u8.hash(state),
            SpacingValue::Fit => 9u8.hash(state),
        }
    }
}

impl std::cmp::Eq for SpacingValue {}

impl SpacingValue {
    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "1px".to_string(),
            SpacingValue::Fractional(f) => format!("{}rem", f * 0.25), // 0.25rem per unit
            SpacingValue::Integer(i) => format!("{}rem", *i as f32 * 0.25),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "100%".to_string(),
            SpacingValue::Screen => "100vh".to_string(),
            SpacingValue::Min => "min-content".to_string(),
            SpacingValue::Max => "max-content".to_string(),
            SpacingValue::Fit => "fit-content".to_string(),
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            SpacingValue::Zero => "0".to_string(),
            SpacingValue::Px => "px".to_string(),
            SpacingValue::Fractional(f) => format!("{}", f),
            SpacingValue::Integer(i) => i.to_string(),
            SpacingValue::Auto => "auto".to_string(),
            SpacingValue::Full => "full".to_string(),
            SpacingValue::Screen => "screen".to_string(),
            SpacingValue::Min => "min".to_string(),
            SpacingValue::Max => "max".to_string(),
            SpacingValue::Fit => "fit".to_string(),
        }
    }

    /// Get all valid spacing values
    pub fn all_values() -> Vec<SpacingValue> {
        vec![
            SpacingValue::Zero,
            SpacingValue::Px,
            SpacingValue::Fractional(0.5),
            SpacingValue::Integer(1),
            SpacingValue::Fractional(1.5),
            SpacingValue::Integer(2),
            SpacingValue::Fractional(2.5),
            SpacingValue::Integer(3),
            SpacingValue::Fractional(3.5),
            SpacingValue::Integer(4),
            SpacingValue::Integer(5),
            SpacingValue::Integer(6),
            SpacingValue::Integer(7),
            SpacingValue::Integer(8),
            SpacingValue::Integer(9),
            SpacingValue::Integer(10),
            SpacingValue::Integer(11),
            SpacingValue::Integer(12),
            SpacingValue::Integer(14),
            SpacingValue::Integer(16),
            SpacingValue::Integer(20),
            SpacingValue::Integer(24),
            SpacingValue::Integer(28),
            SpacingValue::Integer(32),
            SpacingValue::Integer(36),
            SpacingValue::Integer(40),
            SpacingValue::Integer(44),
            SpacingValue::Integer(48),
            SpacingValue::Integer(52),
            SpacingValue::Integer(56),
            SpacingValue::Integer(60),
            SpacingValue::Integer(64),
            SpacingValue::Integer(72),
            SpacingValue::Integer(80),
            SpacingValue::Integer(96),
            SpacingValue::Auto,
            SpacingValue::Full,
            SpacingValue::Screen,
            SpacingValue::Min,
            SpacingValue::Max,
            SpacingValue::Fit,
        ]
    }
}

impl fmt::Display for SpacingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}
