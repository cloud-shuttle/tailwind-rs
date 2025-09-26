//! Arbitrary Values Parser
//!
//! This module provides parsing logic for arbitrary value classes in Tailwind CSS,
//! such as `size-[38px]`, `top-[4px]`, `left-[7px]`, `drop-shadow-[0_3px_1px_rgba(0,0,0,.15)]`.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct ArbitraryParser;

impl ArbitraryParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse arbitrary size classes (size-[38px], w-[100px], h-[50px])
    fn parse_arbitrary_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    fn parse_standard_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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

    /// Parse arbitrary positioning classes (top-[4px], left-[7px])
    fn parse_arbitrary_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    fn parse_arbitrary_background_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    fn parse_arbitrary_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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

    /// Parse arbitrary transform classes (-translate-x-1/2)
    fn parse_arbitrary_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
    fn parse_arbitrary_mask_class(&self, class: &str) -> Option<Vec<CssProperty>> {
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
}

impl UtilityParser for ArbitraryParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(properties) = self.parse_arbitrary_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_standard_size_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_position_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_background_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_filter_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_transform_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_arbitrary_mask_class(class) {
            return Some(properties);
        }
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "size-[*]",
            "size-0",
            "size-1",
            "size-2",
            "size-3",
            "size-4",
            "size-5",
            "size-6",
            "size-8",
            "size-10",
            "size-12",
            "size-16",
            "size-20",
            "size-24",
            "size-32",
            "size-40",
            "size-48",
            "size-56",
            "size-64",
            "size-72",
            "size-80",
            "size-96",
            "size-auto",
            "size-full",
            "size-screen",
            "size-min",
            "size-max",
            "size-fit",
            "w-[*]",
            "h-[*]",
            "top-[*]",
            "left-[*]",
            "right-[*]",
            "bottom-[*]",
            "bg-[*]",
            "drop-shadow-[*]",
            "blur-[*]",
            "brightness-[*]",
            "contrast-[*]",
            "grayscale-[*]",
            "hue-rotate-[*]",
            "invert-[*]",
            "opacity-[*]",
            "saturate-[*]",
            "sepia-[*]",
            "translate-x-[*]",
            "translate-y-[*]",
            "-translate-x-[*]",
            "-translate-y-[*]",
            "rotate-[*]",
            "scale-[*]",
            "mask-[*]",
        ]
    }

    fn get_priority(&self) -> u32 {
        95
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }
}

impl Default for ArbitraryParser {
    fn default() -> Self {
        Self::new()
    }
}
