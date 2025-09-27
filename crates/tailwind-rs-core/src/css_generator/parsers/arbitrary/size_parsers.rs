//! Size parsing logic for arbitrary values
//!
//! This module contains parsing logic for size-related arbitrary values
//! including width, height, and size utilities.

use crate::css_generator::types::CssProperty;

/// Parse arbitrary size classes (size-[38px], w-[100px], h-[50px])
pub fn parse_arbitrary_size_class(class: &str) -> Option<Vec<CssProperty>> {
    if let Some(value) = class.strip_prefix("size-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![
                CssProperty {
                    name: "width".to_string(),
                    value: value.to_string(),
                    important: false,
                },
                CssProperty {
                    name: "height".to_string(),
                    value: value.to_string(),
                    important: false,
                },
            ]);
        }
    }
    if let Some(value) = class.strip_prefix("w-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "width".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    if let Some(value) = class.strip_prefix("h-[") {
        if let Some(value) = value.strip_suffix("]") {
            return Some(vec![CssProperty {
                name: "height".to_string(),
                value: value.to_string(),
                important: false,
            }]);
        }
    }
    None
}

/// Parse standard size classes (size-12, size-6)
pub fn parse_standard_size_class(class: &str) -> Option<Vec<CssProperty>> {
    match class {
        "size-0" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0px".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0px".to_string(),
                important: false,
            },
        ]),
        "size-1" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.25rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.25rem".to_string(),
                important: false,
            },
        ]),
        "size-2" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.5rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.5rem".to_string(),
                important: false,
            },
        ]),
        "size-3" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.75rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.75rem".to_string(),
                important: false,
            },
        ]),
        "size-4" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "1rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "1rem".to_string(),
                important: false,
            },
        ]),
        "size-5" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "1.25rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "1.25rem".to_string(),
                important: false,
            },
        ]),
        "size-6" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "1.5rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "1.5rem".to_string(),
                important: false,
            },
        ]),
        "size-8" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "2rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "2rem".to_string(),
                important: false,
            },
        ]),
        "size-10" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "2.5rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "2.5rem".to_string(),
                important: false,
            },
        ]),
        "size-12" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "3rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "3rem".to_string(),
                important: false,
            },
        ]),
        "size-16" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "4rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "4rem".to_string(),
                important: false,
            },
        ]),
        "size-20" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "5rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "5rem".to_string(),
                important: false,
            },
        ]),
        "size-24" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "6rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "6rem".to_string(),
                important: false,
            },
        ]),
        "size-32" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "8rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "8rem".to_string(),
                important: false,
            },
        ]),
        "size-40" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "10rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "10rem".to_string(),
                important: false,
            },
        ]),
        "size-48" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "12rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "12rem".to_string(),
                important: false,
            },
        ]),
        "size-56" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "14rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "14rem".to_string(),
                important: false,
            },
        ]),
        "size-64" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "16rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "16rem".to_string(),
                important: false,
            },
        ]),
        "size-72" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "18rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "18rem".to_string(),
                important: false,
            },
        ]),
        "size-80" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "20rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "20rem".to_string(),
                important: false,
            },
        ]),
        "size-96" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "24rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "24rem".to_string(),
                important: false,
            },
        ]),
        "size-auto" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "auto".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "auto".to_string(),
                important: false,
            },
        ]),
        "size-full" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "100%".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "100%".to_string(),
                important: false,
            },
        ]),
        "size-screen" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "100vw".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "100vh".to_string(),
                important: false,
            },
        ]),
        "size-min" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "min-content".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "min-content".to_string(),
                important: false,
            },
        ]),
        "size-max" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "max-content".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "max-content".to_string(),
                important: false,
            },
        ]),
        "size-fit" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "fit-content".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "fit-content".to_string(),
                important: false,
            },
        ]),
        // Size utilities with decimal values
        "size-px" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "1px".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "1px".to_string(),
                important: false,
            },
        ]),
        "size-0.5" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.125rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.125rem".to_string(),
                important: false,
            },
        ]),
        "size-1.5" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.375rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.375rem".to_string(),
                important: false,
            },
        ]),
        "size-2.5" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.625rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.625rem".to_string(),
                important: false,
            },
        ]),
        "size-3.5" => Some(vec![
            CssProperty {
                name: "width".to_string(),
                value: "0.875rem".to_string(),
                important: false,
            },
            CssProperty {
                name: "height".to_string(),
                value: "0.875rem".to_string(),
                important: false,
            },
        ]),
        _ => None,
    }
}
