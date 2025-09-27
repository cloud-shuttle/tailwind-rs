//! Transform and mask parsing logic for arbitrary values
//!
//! This module contains parsing logic for transform and mask-related
//! arbitrary values including translate, rotate, scale, and mask utilities.

use crate::css_generator::types::CssProperty;

/// Parse arbitrary transform classes (-translate-x-1/2)
pub fn parse_arbitrary_transform_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(value) = class.strip_prefix("-translate-x-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "transform".to_string(),
                value: format!("translateX(-{})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("-translate-y-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "transform".to_string(),
                value: format!("translateY(-{})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("translate-x-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "transform".to_string(),
                value: format!("translateX({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("translate-y-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "transform".to_string(),
                value: format!("translateY({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("rotate-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "transform".to_string(),
                value: format!("rotate({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("scale-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "transform".to_string(),
                value: format!("scale({})", value),
                important: false,
            }]);
        }
    }
    None
}

/// Parse arbitrary mask classes (mask-[linear-gradient(...)])
pub fn parse_arbitrary_mask_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(value) = class.strip_prefix("mask-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "mask-image".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    None
}
