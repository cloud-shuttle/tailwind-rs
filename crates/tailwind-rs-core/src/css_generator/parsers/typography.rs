//! Typography Utilities Parser
//!
//! This module provides parsing logic for typography-related Tailwind CSS utilities,
//! including font-family, font-size, font-smoothing, font-style, font-weight, font-stretch, and font-variant-numeric.

use super::{UtilityParser, ParserCategory};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct TypographyParser;

impl TypographyParser {
    pub fn new() -> Self { Self }

    /// Parse font-family classes
    fn parse_font_family_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "font-sans" => Some(vec![CssProperty {
                name: "font-family".to_string(),
                value: "var(--font-sans)".to_string(),
                important: false,
            }]),
            "font-serif" => Some(vec![CssProperty {
                name: "font-family".to_string(),
                value: "var(--font-serif)".to_string(),
                important: false,
            }]),
            "font-mono" => Some(vec![CssProperty {
                name: "font-family".to_string(),
                value: "var(--font-mono)".to_string(),
                important: false,
            }]),
            _ => {
                // Custom properties for font-family
                if let Some(value) = class.strip_prefix("font-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "font-family".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for font-family
                if let Some(value) = class.strip_prefix("font-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "font-family".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse font-size classes
    fn parse_font_size_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-xs" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-xs)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-xs--line-height)".to_string(), important: false },
            ]),
            "text-sm" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-sm)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-sm--line-height)".to_string(), important: false },
            ]),
            "text-base" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-base)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-base--line-height)".to_string(), important: false },
            ]),
            "text-lg" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-lg)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-lg--line-height)".to_string(), important: false },
            ]),
            "text-xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-xl--line-height)".to_string(), important: false },
            ]),
            "text-2xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-2xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-2xl--line-height)".to_string(), important: false },
            ]),
            "text-3xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-3xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-3xl--line-height)".to_string(), important: false },
            ]),
            "text-4xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-4xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-4xl--line-height)".to_string(), important: false },
            ]),
            "text-5xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-5xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-5xl--line-height)".to_string(), important: false },
            ]),
            "text-6xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-6xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-6xl--line-height)".to_string(), important: false },
            ]),
            "text-7xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-7xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-7xl--line-height)".to_string(), important: false },
            ]),
            "text-8xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-8xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-8xl--line-height)".to_string(), important: false },
            ]),
            "text-9xl" => Some(vec![
                CssProperty { name: "font-size".to_string(), value: "var(--text-9xl)".to_string(), important: false },
                CssProperty { name: "line-height".to_string(), value: "var(--text-9xl--line-height)".to_string(), important: false },
            ]),
            _ => {
                // Custom properties for font-size
                if let Some(value) = class.strip_prefix("text-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "font-size".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for font-size
                if let Some(value) = class.strip_prefix("text-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "font-size".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Font-size with line-height (e.g., text-sm/6, text-lg/7)
                if let Some(parts) = class.strip_prefix("text-") {
                    if let Some((size, line_height)) = self.parse_font_size_with_line_height(parts) {
                        return Some(vec![
                            CssProperty { name: "font-size".to_string(), value: size, important: false },
                            CssProperty { name: "line-height".to_string(), value: line_height, important: false },
                        ]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse font-size with line-height (e.g., text-sm/6, text-lg/7)
    fn parse_font_size_with_line_height(&self, parts: &str) -> Option<(String, String)> {
        if let Some((size_part, line_height_part)) = parts.split_once('/') {
            let font_size = match size_part {
                "xs" => "var(--text-xs)".to_string(),
                "sm" => "var(--text-sm)".to_string(),
                "base" => "var(--text-base)".to_string(),
                "lg" => "var(--text-lg)".to_string(),
                "xl" => "var(--text-xl)".to_string(),
                "2xl" => "var(--text-2xl)".to_string(),
                "3xl" => "var(--text-3xl)".to_string(),
                "4xl" => "var(--text-4xl)".to_string(),
                "5xl" => "var(--text-5xl)".to_string(),
                "6xl" => "var(--text-6xl)".to_string(),
                "7xl" => "var(--text-7xl)".to_string(),
                "8xl" => "var(--text-8xl)".to_string(),
                "9xl" => "var(--text-9xl)".to_string(),
                _ => return None,
            };
            
            let line_height = match line_height_part {
                "1" => "1".to_string(),
                "2" => "0.5rem".to_string(),
                "3" => "0.75rem".to_string(),
                "4" => "1rem".to_string(),
                "5" => "1.25rem".to_string(),
                "6" => "1.5rem".to_string(),
                "7" => "1.75rem".to_string(),
                "8" => "2rem".to_string(),
                "9" => "2.25rem".to_string(),
                "10" => "2.5rem".to_string(),
                "none" => "1".to_string(),
                "tight" => "1.25".to_string(),
                "snug" => "1.375".to_string(),
                "normal" => "1.5".to_string(),
                "relaxed" => "1.625".to_string(),
                "loose" => "2".to_string(),
                _ => return None,
            };
            
            Some((font_size, line_height))
        } else {
            None
        }
    }

    /// Parse font-smoothing classes
    fn parse_font_smoothing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "antialiased" => Some(vec![
                CssProperty { name: "-webkit-font-smoothing".to_string(), value: "antialiased".to_string(), important: false },
                CssProperty { name: "-moz-osx-font-smoothing".to_string(), value: "grayscale".to_string(), important: false },
            ]),
            "subpixel-antialiased" => Some(vec![
                CssProperty { name: "-webkit-font-smoothing".to_string(), value: "auto".to_string(), important: false },
                CssProperty { name: "-moz-osx-font-smoothing".to_string(), value: "auto".to_string(), important: false },
            ]),
            _ => None,
        }
    }

    /// Parse font-style classes
    fn parse_font_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "italic" => Some(vec![CssProperty {
                name: "font-style".to_string(),
                value: "italic".to_string(),
                important: false,
            }]),
            "not-italic" => Some(vec![CssProperty {
                name: "font-style".to_string(),
                value: "normal".to_string(),
                important: false,
            }]),
            _ => None,
        }
    }

    /// Parse font-weight classes
    fn parse_font_weight_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "font-thin" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "100".to_string(), important: false }]),
            "font-extralight" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "200".to_string(), important: false }]),
            "font-light" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "300".to_string(), important: false }]),
            "font-normal" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "400".to_string(), important: false }]),
            "font-medium" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "500".to_string(), important: false }]),
            "font-semibold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "600".to_string(), important: false }]),
            "font-bold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "700".to_string(), important: false }]),
            "font-extrabold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "800".to_string(), important: false }]),
            "font-black" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "900".to_string(), important: false }]),
            _ => {
                // Custom properties for font-weight
                if let Some(value) = class.strip_prefix("font-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "font-weight".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for font-weight
                if let Some(value) = class.strip_prefix("font-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "font-weight".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse font-stretch classes
    fn parse_font_stretch_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "font-stretch-ultra-condensed" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "ultra-condensed".to_string(), important: false }]),
            "font-stretch-extra-condensed" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "extra-condensed".to_string(), important: false }]),
            "font-stretch-condensed" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "condensed".to_string(), important: false }]),
            "font-stretch-semi-condensed" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "semi-condensed".to_string(), important: false }]),
            "font-stretch-normal" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "normal".to_string(), important: false }]),
            "font-stretch-semi-expanded" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "semi-expanded".to_string(), important: false }]),
            "font-stretch-expanded" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "expanded".to_string(), important: false }]),
            "font-stretch-extra-expanded" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "extra-expanded".to_string(), important: false }]),
            "font-stretch-ultra-expanded" => Some(vec![CssProperty { name: "font-stretch".to_string(), value: "ultra-expanded".to_string(), important: false }]),
            _ => {
                // Percentage values for font-stretch
                if let Some(value) = class.strip_prefix("font-stretch-") {
                    if value.ends_with('%') {
                        return Some(vec![CssProperty {
                            name: "font-stretch".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for font-stretch
                if let Some(value) = class.strip_prefix("font-stretch-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "font-stretch".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for font-stretch
                if let Some(value) = class.strip_prefix("font-stretch-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "font-stretch".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse font-variant-numeric classes
    fn parse_font_variant_numeric_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "normal-nums" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "normal".to_string(), important: false }]),
            "ordinal" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "ordinal".to_string(), important: false }]),
            "slashed-zero" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "slashed-zero".to_string(), important: false }]),
            "lining-nums" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "lining-nums".to_string(), important: false }]),
            "oldstyle-nums" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "oldstyle-nums".to_string(), important: false }]),
            "proportional-nums" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "proportional-nums".to_string(), important: false }]),
            "tabular-nums" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "tabular-nums".to_string(), important: false }]),
            "diagonal-fractions" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "diagonal-fractions".to_string(), important: false }]),
            "stacked-fractions" => Some(vec![CssProperty { name: "font-variant-numeric".to_string(), value: "stacked-fractions".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse letter-spacing (tracking) classes
    fn parse_letter_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "tracking-tighter" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "var(--tracking-tighter)".to_string(), important: false }]),
            "tracking-tight" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "var(--tracking-tight)".to_string(), important: false }]),
            "tracking-normal" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "var(--tracking-normal)".to_string(), important: false }]),
            "tracking-wide" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "var(--tracking-wide)".to_string(), important: false }]),
            "tracking-wider" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "var(--tracking-wider)".to_string(), important: false }]),
            "tracking-widest" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "var(--tracking-widest)".to_string(), important: false }]),
            _ => {
                // Custom properties for letter-spacing
                if let Some(value) = class.strip_prefix("tracking-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "letter-spacing".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for letter-spacing
                if let Some(value) = class.strip_prefix("tracking-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "letter-spacing".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Negative values for letter-spacing
                if let Some(value) = class.strip_prefix("-tracking-") {
                    if let Some(spacing) = self.get_spacing_value(value) {
                        return Some(vec![CssProperty {
                            name: "letter-spacing".to_string(),
                            value: format!("-{}", spacing),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse line-clamp classes
    fn parse_line_clamp_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "line-clamp-none" => Some(vec![
                CssProperty { name: "overflow".to_string(), value: "visible".to_string(), important: false },
                CssProperty { name: "display".to_string(), value: "block".to_string(), important: false },
                CssProperty { name: "-webkit-box-orient".to_string(), value: "horizontal".to_string(), important: false },
                CssProperty { name: "-webkit-line-clamp".to_string(), value: "unset".to_string(), important: false },
            ]),
            _ => {
                // Line-clamp with number
                if let Some(value) = class.strip_prefix("line-clamp-") {
                    if let Ok(_) = value.parse::<u32>() {
                        return Some(vec![
                            CssProperty { name: "overflow".to_string(), value: "hidden".to_string(), important: false },
                            CssProperty { name: "display".to_string(), value: "-webkit-box".to_string(), important: false },
                            CssProperty { name: "-webkit-box-orient".to_string(), value: "vertical".to_string(), important: false },
                            CssProperty { name: "-webkit-line-clamp".to_string(), value: value.to_string(), important: false },
                        ]);
                    }
                }
                
                // Custom properties for line-clamp
                if let Some(value) = class.strip_prefix("line-clamp-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![
                            CssProperty { name: "overflow".to_string(), value: "hidden".to_string(), important: false },
                            CssProperty { name: "display".to_string(), value: "-webkit-box".to_string(), important: false },
                            CssProperty { name: "-webkit-box-orient".to_string(), value: "vertical".to_string(), important: false },
                            CssProperty { name: "-webkit-line-clamp".to_string(), value: format!("var({})", value), important: false },
                        ]);
                    }
                }
                
                // Arbitrary values for line-clamp
                if let Some(value) = class.strip_prefix("line-clamp-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![
                            CssProperty { name: "overflow".to_string(), value: "hidden".to_string(), important: false },
                            CssProperty { name: "display".to_string(), value: "-webkit-box".to_string(), important: false },
                            CssProperty { name: "-webkit-box-orient".to_string(), value: "vertical".to_string(), important: false },
                            CssProperty { name: "-webkit-line-clamp".to_string(), value: value.to_string(), important: false },
                        ]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse line-height (leading) classes
    fn parse_line_height_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "leading-none" => Some(vec![CssProperty { name: "line-height".to_string(), value: "1".to_string(), important: false }]),
            _ => {
                // Leading with number
                if let Some(value) = class.strip_prefix("leading-") {
                    if let Some(spacing) = self.get_spacing_value(value) {
                        return Some(vec![CssProperty {
                            name: "line-height".to_string(),
                            value: format!("calc(var(--spacing) * {})", spacing),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for line-height
                if let Some(value) = class.strip_prefix("leading-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "line-height".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for line-height
                if let Some(value) = class.strip_prefix("leading-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "line-height".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse list-style-image classes
    fn parse_list_style_image_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "list-image-none" => Some(vec![CssProperty { name: "list-style-image".to_string(), value: "none".to_string(), important: false }]),
            _ => {
                // Custom properties for list-style-image
                if let Some(value) = class.strip_prefix("list-image-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "list-style-image".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for list-style-image
                if let Some(value) = class.strip_prefix("list-image-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "list-style-image".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse list-style-position classes
    fn parse_list_style_position_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "list-inside" => Some(vec![CssProperty { name: "list-style-position".to_string(), value: "inside".to_string(), important: false }]),
            "list-outside" => Some(vec![CssProperty { name: "list-style-position".to_string(), value: "outside".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse list-style-type classes
    fn parse_list_style_type_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "list-disc" => Some(vec![CssProperty { name: "list-style-type".to_string(), value: "disc".to_string(), important: false }]),
            "list-decimal" => Some(vec![CssProperty { name: "list-style-type".to_string(), value: "decimal".to_string(), important: false }]),
            "list-none" => Some(vec![CssProperty { name: "list-style-type".to_string(), value: "none".to_string(), important: false }]),
            _ => {
                // Custom properties for list-style-type
                if let Some(value) = class.strip_prefix("list-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "list-style-type".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for list-style-type
                if let Some(value) = class.strip_prefix("list-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "list-style-type".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse text-align classes
    fn parse_text_align_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-left" => Some(vec![CssProperty { name: "text-align".to_string(), value: "left".to_string(), important: false }]),
            "text-center" => Some(vec![CssProperty { name: "text-align".to_string(), value: "center".to_string(), important: false }]),
            "text-right" => Some(vec![CssProperty { name: "text-align".to_string(), value: "right".to_string(), important: false }]),
            "text-justify" => Some(vec![CssProperty { name: "text-align".to_string(), value: "justify".to_string(), important: false }]),
            "text-start" => Some(vec![CssProperty { name: "text-align".to_string(), value: "start".to_string(), important: false }]),
            "text-end" => Some(vec![CssProperty { name: "text-align".to_string(), value: "end".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse text color classes
    fn parse_text_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-inherit" => Some(vec![CssProperty { name: "color".to_string(), value: "inherit".to_string(), important: false }]),
            "text-current" => Some(vec![CssProperty { name: "color".to_string(), value: "currentColor".to_string(), important: false }]),
            "text-transparent" => Some(vec![CssProperty { name: "color".to_string(), value: "transparent".to_string(), important: false }]),
            "text-black" => Some(vec![CssProperty { name: "color".to_string(), value: "var(--color-black)".to_string(), important: false }]),
            "text-white" => Some(vec![CssProperty { name: "color".to_string(), value: "var(--color-white)".to_string(), important: false }]),
            _ => {
                // Custom properties for text color
                if let Some(value) = class.strip_prefix("text-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for text color
                if let Some(value) = class.strip_prefix("text-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Color with opacity modifier (e.g., text-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        if let Some(color_value) = self.get_color_value(base_color) {
                            return Some(vec![CssProperty {
                                name: "color".to_string(),
                                value: format!("{}/{}", color_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Standard color classes (text-red-500, text-blue-600, etc.)
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }
                
                None
            }
        }
    }

    /// Parse text-decoration-line classes
    fn parse_text_decoration_line_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "underline" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "underline".to_string(), important: false }]),
            "overline" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "overline".to_string(), important: false }]),
            "line-through" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "line-through".to_string(), important: false }]),
            "no-underline" => Some(vec![CssProperty { name: "text-decoration-line".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse text-decoration-color classes
    fn parse_text_decoration_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "decoration-inherit" => Some(vec![CssProperty { name: "text-decoration-color".to_string(), value: "inherit".to_string(), important: false }]),
            "decoration-current" => Some(vec![CssProperty { name: "text-decoration-color".to_string(), value: "currentColor".to_string(), important: false }]),
            "decoration-transparent" => Some(vec![CssProperty { name: "text-decoration-color".to_string(), value: "transparent".to_string(), important: false }]),
            "decoration-black" => Some(vec![CssProperty { name: "text-decoration-color".to_string(), value: "var(--color-black)".to_string(), important: false }]),
            "decoration-white" => Some(vec![CssProperty { name: "text-decoration-color".to_string(), value: "var(--color-white)".to_string(), important: false }]),
            _ => {
                // Custom properties for decoration color
                if let Some(value) = class.strip_prefix("decoration-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "text-decoration-color".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for decoration color
                if let Some(value) = class.strip_prefix("decoration-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "text-decoration-color".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                // Color with opacity modifier (e.g., decoration-blue-600/50)
                if class.contains("/") {
                    let parts: Vec<&str> = class.split("/").collect();
                    if parts.len() == 2 {
                        let base_color = parts[0];
                        let opacity = parts[1];
                        if let Some(color_value) = self.get_color_value(base_color) {
                            return Some(vec![CssProperty {
                                name: "text-decoration-color".to_string(),
                                value: format!("{}/{}", color_value, opacity),
                                important: false,
                            }]);
                        }
                    }
                }
                
                // Standard decoration color classes
                if let Some(color_value) = self.get_color_value(class) {
                    return Some(vec![CssProperty {
                        name: "text-decoration-color".to_string(),
                        value: color_value,
                        important: false,
                    }]);
                }
                
                None
            }
        }
    }

    /// Parse text-decoration-style classes
    fn parse_text_decoration_style_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "decoration-solid" => Some(vec![CssProperty { name: "text-decoration-style".to_string(), value: "solid".to_string(), important: false }]),
            "decoration-double" => Some(vec![CssProperty { name: "text-decoration-style".to_string(), value: "double".to_string(), important: false }]),
            "decoration-dotted" => Some(vec![CssProperty { name: "text-decoration-style".to_string(), value: "dotted".to_string(), important: false }]),
            "decoration-dashed" => Some(vec![CssProperty { name: "text-decoration-style".to_string(), value: "dashed".to_string(), important: false }]),
            "decoration-wavy" => Some(vec![CssProperty { name: "text-decoration-style".to_string(), value: "wavy".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse text-decoration-thickness classes
    fn parse_text_decoration_thickness_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "decoration-from-font" => Some(vec![CssProperty { name: "text-decoration-thickness".to_string(), value: "from-font".to_string(), important: false }]),
            "decoration-auto" => Some(vec![CssProperty { name: "text-decoration-thickness".to_string(), value: "auto".to_string(), important: false }]),
            _ => {
                // Decoration thickness with number
                if let Some(value) = class.strip_prefix("decoration-") {
                    if let Ok(thickness) = value.parse::<u32>() {
                        return Some(vec![CssProperty {
                            name: "text-decoration-thickness".to_string(),
                            value: format!("{}px", thickness),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for decoration thickness
                if let Some(value) = class.strip_prefix("decoration-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "text-decoration-thickness".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for decoration thickness
                if let Some(value) = class.strip_prefix("decoration-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "text-decoration-thickness".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse text-underline-offset classes
    fn parse_text_underline_offset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "underline-offset-auto" => Some(vec![CssProperty { name: "text-underline-offset".to_string(), value: "auto".to_string(), important: false }]),
            _ => {
                // Underline offset with number
                if let Some(value) = class.strip_prefix("underline-offset-") {
                    if let Ok(offset) = value.parse::<u32>() {
                        return Some(vec![CssProperty {
                            name: "text-underline-offset".to_string(),
                            value: format!("{}px", offset),
                            important: false,
                        }]);
                    }
                }
                
                // Negative underline offset
                if let Some(value) = class.strip_prefix("-underline-offset-") {
                    if let Ok(offset) = value.parse::<u32>() {
                        return Some(vec![CssProperty {
                            name: "text-underline-offset".to_string(),
                            value: format!("calc({}px * -1)", offset),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for underline offset
                if let Some(value) = class.strip_prefix("underline-offset-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "text-underline-offset".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for underline offset
                if let Some(value) = class.strip_prefix("underline-offset-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "text-underline-offset".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse text-transform classes
    fn parse_text_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "uppercase" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "uppercase".to_string(), important: false }]),
            "lowercase" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "lowercase".to_string(), important: false }]),
            "capitalize" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "capitalize".to_string(), important: false }]),
            "normal-case" => Some(vec![CssProperty { name: "text-transform".to_string(), value: "none".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse text-overflow classes
    fn parse_text_overflow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "truncate" => Some(vec![
                CssProperty { name: "overflow".to_string(), value: "hidden".to_string(), important: false },
                CssProperty { name: "text-overflow".to_string(), value: "ellipsis".to_string(), important: false },
                CssProperty { name: "white-space".to_string(), value: "nowrap".to_string(), important: false },
            ]),
            "text-ellipsis" => Some(vec![CssProperty { name: "text-overflow".to_string(), value: "ellipsis".to_string(), important: false }]),
            "text-clip" => Some(vec![CssProperty { name: "text-overflow".to_string(), value: "clip".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse text-wrap classes
    fn parse_text_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "text-wrap" => Some(vec![CssProperty { name: "text-wrap".to_string(), value: "wrap".to_string(), important: false }]),
            "text-nowrap" => Some(vec![CssProperty { name: "text-wrap".to_string(), value: "nowrap".to_string(), important: false }]),
            "text-balance" => Some(vec![CssProperty { name: "text-wrap".to_string(), value: "balance".to_string(), important: false }]),
            "text-pretty" => Some(vec![CssProperty { name: "text-wrap".to_string(), value: "pretty".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse text-indent classes
    fn parse_text_indent_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "indent-px" => Some(vec![CssProperty { name: "text-indent".to_string(), value: "1px".to_string(), important: false }]),
            "-indent-px" => Some(vec![CssProperty { name: "text-indent".to_string(), value: "-1px".to_string(), important: false }]),
            _ => {
                // Indent with number
                if let Some(value) = class.strip_prefix("indent-") {
                    if let Some(spacing) = self.get_spacing_value(value) {
                        return Some(vec![CssProperty {
                            name: "text-indent".to_string(),
                            value: format!("calc(var(--spacing) * {})", spacing),
                            important: false,
                        }]);
                    }
                }
                
                // Negative indent
                if let Some(value) = class.strip_prefix("-indent-") {
                    if let Some(spacing) = self.get_spacing_value(value) {
                        return Some(vec![CssProperty {
                            name: "text-indent".to_string(),
                            value: format!("calc(var(--spacing) * -{})", spacing),
                            important: false,
                        }]);
                    }
                }
                
                // Custom properties for text indent
                if let Some(value) = class.strip_prefix("indent-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "text-indent".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for text indent
                if let Some(value) = class.strip_prefix("indent-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "text-indent".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse vertical-align classes
    fn parse_vertical_align_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "align-baseline" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "baseline".to_string(), important: false }]),
            "align-top" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "top".to_string(), important: false }]),
            "align-middle" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "middle".to_string(), important: false }]),
            "align-bottom" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "bottom".to_string(), important: false }]),
            "align-text-top" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "text-top".to_string(), important: false }]),
            "align-text-bottom" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "text-bottom".to_string(), important: false }]),
            "align-sub" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "sub".to_string(), important: false }]),
            "align-super" => Some(vec![CssProperty { name: "vertical-align".to_string(), value: "super".to_string(), important: false }]),
            _ => {
                // Custom properties for vertical-align
                if let Some(value) = class.strip_prefix("align-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "vertical-align".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for vertical-align
                if let Some(value) = class.strip_prefix("align-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "vertical-align".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Parse white-space classes
    fn parse_white_space_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "whitespace-normal" => Some(vec![CssProperty { name: "white-space".to_string(), value: "normal".to_string(), important: false }]),
            "whitespace-nowrap" => Some(vec![CssProperty { name: "white-space".to_string(), value: "nowrap".to_string(), important: false }]),
            "whitespace-pre" => Some(vec![CssProperty { name: "white-space".to_string(), value: "pre".to_string(), important: false }]),
            "whitespace-pre-line" => Some(vec![CssProperty { name: "white-space".to_string(), value: "pre-line".to_string(), important: false }]),
            "whitespace-pre-wrap" => Some(vec![CssProperty { name: "white-space".to_string(), value: "pre-wrap".to_string(), important: false }]),
            "whitespace-break-spaces" => Some(vec![CssProperty { name: "white-space".to_string(), value: "break-spaces".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse word-break classes
    fn parse_word_break_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "break-normal" => Some(vec![CssProperty { name: "word-break".to_string(), value: "normal".to_string(), important: false }]),
            "break-all" => Some(vec![CssProperty { name: "word-break".to_string(), value: "break-all".to_string(), important: false }]),
            "break-keep" => Some(vec![CssProperty { name: "word-break".to_string(), value: "keep-all".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse overflow-wrap classes
    fn parse_overflow_wrap_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "wrap-break-word" => Some(vec![CssProperty { name: "overflow-wrap".to_string(), value: "break-word".to_string(), important: false }]),
            "wrap-anywhere" => Some(vec![CssProperty { name: "overflow-wrap".to_string(), value: "anywhere".to_string(), important: false }]),
            "wrap-normal" => Some(vec![CssProperty { name: "overflow-wrap".to_string(), value: "normal".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse hyphens classes
    fn parse_hyphens_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "hyphens-none" => Some(vec![CssProperty { name: "hyphens".to_string(), value: "none".to_string(), important: false }]),
            "hyphens-manual" => Some(vec![CssProperty { name: "hyphens".to_string(), value: "manual".to_string(), important: false }]),
            "hyphens-auto" => Some(vec![CssProperty { name: "hyphens".to_string(), value: "auto".to_string(), important: false }]),
            _ => None,
        }
    }

    /// Parse content classes
    fn parse_content_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "content-none" => Some(vec![CssProperty { name: "content".to_string(), value: "none".to_string(), important: false }]),
            _ => {
                // Custom properties for content
                if let Some(value) = class.strip_prefix("content-(") {
                    if let Some(value) = value.strip_suffix(")") {
                        return Some(vec![CssProperty {
                            name: "content".to_string(),
                            value: format!("var({})", value),
                            important: false,
                        }]);
                    }
                }
                
                // Arbitrary values for content
                if let Some(value) = class.strip_prefix("content-[") {
                    if let Some(value) = value.strip_suffix("]") {
                        return Some(vec![CssProperty {
                            name: "content".to_string(),
                            value: value.to_string(),
                            important: false,
                        }]);
                    }
                }
                
                None
            }
        }
    }

    /// Get color value for text and decoration colors
    fn get_color_value(&self, class: &str) -> Option<String> {
        // This is a simplified color mapping - in a real implementation,
        // you'd want a comprehensive color system
        match class {
            "text-red-50" => Some("var(--color-red-50)".to_string()),
            "text-red-100" => Some("var(--color-red-100)".to_string()),
            "text-red-200" => Some("var(--color-red-200)".to_string()),
            "text-red-300" => Some("var(--color-red-300)".to_string()),
            "text-red-400" => Some("var(--color-red-400)".to_string()),
            "text-red-500" => Some("var(--color-red-500)".to_string()),
            "text-red-600" => Some("var(--color-red-600)".to_string()),
            "text-red-700" => Some("var(--color-red-700)".to_string()),
            "text-red-800" => Some("var(--color-red-800)".to_string()),
            "text-red-900" => Some("var(--color-red-900)".to_string()),
            "text-red-950" => Some("var(--color-red-950)".to_string()),
            "text-blue-50" => Some("var(--color-blue-50)".to_string()),
            "text-blue-100" => Some("var(--color-blue-100)".to_string()),
            "text-blue-200" => Some("var(--color-blue-200)".to_string()),
            "text-blue-300" => Some("var(--color-blue-300)".to_string()),
            "text-blue-400" => Some("var(--color-blue-400)".to_string()),
            "text-blue-500" => Some("var(--color-blue-500)".to_string()),
            "text-blue-600" => Some("var(--color-blue-600)".to_string()),
            "text-blue-700" => Some("var(--color-blue-700)".to_string()),
            "text-blue-800" => Some("var(--color-blue-800)".to_string()),
            "text-blue-900" => Some("var(--color-blue-900)".to_string()),
            "text-blue-950" => Some("var(--color-blue-950)".to_string()),
            "text-green-50" => Some("var(--color-green-50)".to_string()),
            "text-green-100" => Some("var(--color-green-100)".to_string()),
            "text-green-200" => Some("var(--color-green-200)".to_string()),
            "text-green-300" => Some("var(--color-green-300)".to_string()),
            "text-green-400" => Some("var(--color-green-400)".to_string()),
            "text-green-500" => Some("var(--color-green-500)".to_string()),
            "text-green-600" => Some("var(--color-green-600)".to_string()),
            "text-green-700" => Some("var(--color-green-700)".to_string()),
            "text-green-800" => Some("var(--color-green-800)".to_string()),
            "text-green-900" => Some("var(--color-green-900)".to_string()),
            "text-green-950" => Some("var(--color-green-950)".to_string()),
            "decoration-red-50" => Some("var(--color-red-50)".to_string()),
            "decoration-red-100" => Some("var(--color-red-100)".to_string()),
            "decoration-red-200" => Some("var(--color-red-200)".to_string()),
            "decoration-red-300" => Some("var(--color-red-300)".to_string()),
            "decoration-red-400" => Some("var(--color-red-400)".to_string()),
            "decoration-red-500" => Some("var(--color-red-500)".to_string()),
            "decoration-red-600" => Some("var(--color-red-600)".to_string()),
            "decoration-red-700" => Some("var(--color-red-700)".to_string()),
            "decoration-red-800" => Some("var(--color-red-800)".to_string()),
            "decoration-red-900" => Some("var(--color-red-900)".to_string()),
            "decoration-red-950" => Some("var(--color-red-950)".to_string()),
            "decoration-blue-50" => Some("var(--color-blue-50)".to_string()),
            "decoration-blue-100" => Some("var(--color-blue-100)".to_string()),
            "decoration-blue-200" => Some("var(--color-blue-200)".to_string()),
            "decoration-blue-300" => Some("var(--color-blue-300)".to_string()),
            "decoration-blue-400" => Some("var(--color-blue-400)".to_string()),
            "decoration-blue-500" => Some("var(--color-blue-500)".to_string()),
            "decoration-blue-600" => Some("var(--color-blue-600)".to_string()),
            "decoration-blue-700" => Some("var(--color-blue-700)".to_string()),
            "decoration-blue-800" => Some("var(--color-blue-800)".to_string()),
            "decoration-blue-900" => Some("var(--color-blue-900)".to_string()),
            "decoration-blue-950" => Some("var(--color-blue-950)".to_string()),
            "decoration-green-50" => Some("var(--color-green-50)".to_string()),
            "decoration-green-100" => Some("var(--color-green-100)".to_string()),
            "decoration-green-200" => Some("var(--color-green-200)".to_string()),
            "decoration-green-300" => Some("var(--color-green-300)".to_string()),
            "decoration-green-400" => Some("var(--color-green-400)".to_string()),
            "decoration-green-500" => Some("var(--color-green-500)".to_string()),
            "decoration-green-600" => Some("var(--color-green-600)".to_string()),
            "decoration-green-700" => Some("var(--color-green-700)".to_string()),
            "decoration-green-800" => Some("var(--color-green-800)".to_string()),
            "decoration-green-900" => Some("var(--color-green-900)".to_string()),
            "decoration-green-950" => Some("var(--color-green-950)".to_string()),
            _ => None,
        }
    }

    /// Get spacing value for letter-spacing and line-height
    fn get_spacing_value(&self, value: &str) -> Option<String> {
        match value {
            "0" => Some("0".to_string()),
            "1" => Some("0.25rem".to_string()),
            "2" => Some("0.5rem".to_string()),
            "3" => Some("0.75rem".to_string()),
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
}

impl UtilityParser for TypographyParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Try each parser in order of specificity
        
        // Content (most specific)
        if let Some(properties) = self.parse_content_class(class) {
            return Some(properties);
        }
        
        // Vertical alignment
        if let Some(properties) = self.parse_vertical_align_class(class) {
            return Some(properties);
        }
        
        // White space and word breaking
        if let Some(properties) = self.parse_white_space_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_word_break_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_overflow_wrap_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_hyphens_class(class) {
            return Some(properties);
        }
        
        // Text color
        if let Some(properties) = self.parse_text_color_class(class) {
            return Some(properties);
        }
        
        // Text decoration utilities
        if let Some(properties) = self.parse_text_decoration_line_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_decoration_color_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_decoration_style_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_decoration_thickness_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_underline_offset_class(class) {
            return Some(properties);
        }
        
        // Text transform and overflow
        if let Some(properties) = self.parse_text_transform_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_overflow_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_wrap_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_text_indent_class(class) {
            return Some(properties);
        }
        
        // Text alignment
        if let Some(properties) = self.parse_text_align_class(class) {
            return Some(properties);
        }
        
        // List utilities
        if let Some(properties) = self.parse_list_style_type_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_list_style_position_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_list_style_image_class(class) {
            return Some(properties);
        }
        
        // Line utilities
        if let Some(properties) = self.parse_line_height_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_line_clamp_class(class) {
            return Some(properties);
        }
        
        // Letter spacing
        if let Some(properties) = self.parse_letter_spacing_class(class) {
            return Some(properties);
        }
        
        // Font utilities
        if let Some(properties) = self.parse_font_variant_numeric_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_font_stretch_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_font_weight_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_font_style_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_font_smoothing_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_font_size_class(class) {
            return Some(properties);
        }
        
        if let Some(properties) = self.parse_font_family_class(class) {
            return Some(properties);
        }
        
        None
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "font-*", "text-*", "antialiased", "subpixel-antialiased", 
            "italic", "not-italic", "font-stretch-*", "normal-nums", 
            "ordinal", "slashed-zero", "lining-nums", "oldstyle-nums", 
            "proportional-nums", "tabular-nums", "diagonal-fractions", "stacked-fractions",
            "tracking-*", "line-clamp-*", "leading-*", "list-image-*", 
            "list-inside", "list-outside", "list-*", "text-left", "text-center", 
            "text-right", "text-justify", "text-start", "text-end",
            "text-inherit", "text-current", "text-transparent", "text-black", "text-white",
            "underline", "overline", "line-through", "no-underline",
            "decoration-*", "underline-offset-*", "uppercase", "lowercase", "capitalize", "normal-case",
            "truncate", "text-ellipsis", "text-clip", "text-wrap", "text-nowrap", "text-balance", "text-pretty",
            "indent-*", "align-*", "whitespace-*", "break-*", "wrap-*", "hyphens-*", "content-*"
        ]
    }

    fn get_priority(&self) -> u32 { 70 }
    fn get_category(&self) -> ParserCategory { ParserCategory::Typography }
}