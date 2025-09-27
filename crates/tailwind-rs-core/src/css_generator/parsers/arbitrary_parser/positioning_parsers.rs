//! Positioning and background parsing logic for arbitrary values
//!
//! This module contains parsing logic for positioning and background-related
//! arbitrary values including top, left, right, bottom, and background utilities.

use crate::css_generator::types::CssProperty;

/// Parse arbitrary positioning classes (top-[4px], left-[7px])
pub fn parse_arbitrary_position_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(value) = class.strip_prefix("top-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "top".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("left-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "left".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("right-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "right".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("bottom-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "bottom".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    None
}

/// Parse arbitrary background classes (bg-[url(/map.png)])
pub fn parse_arbitrary_background_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(value) = class.strip_prefix("bg-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "background-image".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    None
}

/// Parse arbitrary filter classes (drop-shadow-[0_3px_1px_rgba(0,0,0,.15)])
pub fn parse_arbitrary_filter_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(value) = class.strip_prefix("drop-shadow-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("drop-shadow({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("blur-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("blur({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("brightness-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("brightness({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("contrast-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("contrast({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("grayscale-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("grayscale({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("hue-rotate-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("hue-rotate({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("invert-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("invert({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("opacity-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("opacity({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("saturate-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("saturate({})", value),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("sepia-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "filter".to_string(),
                value: format!("sepia({})", value),
                important: false,
            }]);
        }
    }
    None
}
