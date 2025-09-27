//! Mask property parsing logic for mask utilities
//!
//! This module contains parsing logic for mask property-related utilities
//! including mask-mode, mask-origin, mask-position, mask-repeat, and mask-size.

use crate::css_generator::types::CssProperty;

/// Parse mask-mode classes
pub fn parse_mask_mode_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-alpha" => Some(vec![CssProperty {
            name: "mask-mode".to_string(),
            value: "alpha".to_string(),
            important: false,
        }]),
        "mask-luminance" => Some(vec![CssProperty {
            name: "mask-mode".to_string(),
            value: "luminance".to_string(),
            important: false,
        }]),
        "mask-match" => Some(vec![CssProperty {
            name: "mask-mode".to_string(),
            value: "match-source".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse mask-origin classes
pub fn parse_mask_origin_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-origin-border" => Some(vec![CssProperty {
            name: "mask-origin".to_string(),
            value: "border-box".to_string(),
            important: false,
        }]),
        "mask-origin-padding" => Some(vec![CssProperty {
            name: "mask-origin".to_string(),
            value: "padding-box".to_string(),
            important: false,
        }]),
        "mask-origin-content" => Some(vec![CssProperty {
            name: "mask-origin".to_string(),
            value: "content-box".to_string(),
            important: false,
        }]),
        "mask-origin-fill" => Some(vec![CssProperty {
            name: "mask-origin".to_string(),
            value: "fill-box".to_string(),
            important: false,
        }]),
        "mask-origin-stroke" => Some(vec![CssProperty {
            name: "mask-origin".to_string(),
            value: "stroke-box".to_string(),
            important: false,
        }]),
        "mask-origin-view" => Some(vec![CssProperty {
            name: "mask-origin".to_string(),
            value: "view-box".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse mask-position classes
pub fn parse_mask_position_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-top-left" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "top left".to_string(),
            important: false,
        }]),
        "mask-top" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "top".to_string(),
            important: false,
        }]),
        "mask-top-right" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "top right".to_string(),
            important: false,
        }]),
        "mask-left" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "left".to_string(),
            important: false,
        }]),
        "mask-center" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "center".to_string(),
            important: false,
        }]),
        "mask-right" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "right".to_string(),
            important: false,
        }]),
        "mask-bottom-left" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "bottom left".to_string(),
            important: false,
        }]),
        "mask-bottom" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "bottom".to_string(),
            important: false,
        }]),
        "mask-bottom-right" => Some(vec![CssProperty {
            name: "mask-position".to_string(),
            value: "bottom right".to_string(),
            important: false,
        }]),
        _ => {
            // Custom properties for mask position
            if let Some(value) = class.strip_prefix("mask-position-(") {
                if let Some(value) = value.strip_suffix(")") {
                    return Some(vec![CssProperty {
                        name: "mask-position".to_string(),
                        value: format!("var({})", value),
                        important: false,
                    }]);
                }
            }

            // Arbitrary values for mask position
            if let Some(value) = class.strip_prefix("mask-position-[") {
                if let Some(value) = value.strip_suffix("]") {
                    return Some(vec![CssProperty {
                        name: "mask-position".to_string(),
                        value: value.to_string(),
                        important: false,
                    }]);
                }
            }

            None
        }
    }
}

/// Parse mask-repeat classes
pub fn parse_mask_repeat_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-repeat" => Some(vec![CssProperty {
            name: "mask-repeat".to_string(),
            value: "repeat".to_string(),
            important: false,
        }]),
        "mask-no-repeat" => Some(vec![CssProperty {
            name: "mask-repeat".to_string(),
            value: "no-repeat".to_string(),
            important: false,
        }]),
        "mask-repeat-x" => Some(vec![CssProperty {
            name: "mask-repeat".to_string(),
            value: "repeat-x".to_string(),
            important: false,
        }]),
        "mask-repeat-y" => Some(vec![CssProperty {
            name: "mask-repeat".to_string(),
            value: "repeat-y".to_string(),
            important: false,
        }]),
        "mask-repeat-space" => Some(vec![CssProperty {
            name: "mask-repeat".to_string(),
            value: "space".to_string(),
            important: false,
        }]),
        "mask-repeat-round" => Some(vec![CssProperty {
            name: "mask-repeat".to_string(),
            value: "round".to_string(),
            important: false,
        }]),
        _ => None,
    }
}

/// Parse mask-size classes
pub fn parse_mask_size_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-auto" => Some(vec![CssProperty {
            name: "mask-size".to_string(),
            value: "auto".to_string(),
            important: false,
        }]),
        "mask-cover" => Some(vec![CssProperty {
            name: "mask-size".to_string(),
            value: "cover".to_string(),
            important: false,
        }]),
        "mask-contain" => Some(vec![CssProperty {
            name: "mask-size".to_string(),
            value: "contain".to_string(),
            important: false,
        }]),
        _ => {
            // Custom properties for mask size
            if let Some(value) = class.strip_prefix("mask-size-(") {
                if let Some(value) = value.strip_suffix(")") {
                    return Some(vec![CssProperty {
                        name: "mask-size".to_string(),
                        value: format!("var({})", value),
                        important: false,
                    }]);
                }
            }

            // Arbitrary values for mask size
            if let Some(value) = class.strip_prefix("mask-size-[") {
                if let Some(value) = value.strip_suffix("]") {
                    return Some(vec![CssProperty {
                        name: "mask-size".to_string(),
                        value: value.to_string(),
                        important: false,
                    }]);
                }
            }

            None
        }
    }
}

/// Parse mask-type classes
pub fn parse_mask_type_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "mask-type-alpha" => Some(vec![CssProperty {
            name: "mask-type".to_string(),
            value: "alpha".to_string(),
            important: false,
        }]),
        "mask-type-luminance" => Some(vec![CssProperty {
            name: "mask-type".to_string(),
            value: "luminance".to_string(),
            important: false,
        }]),
        _ => None,
    }
}
