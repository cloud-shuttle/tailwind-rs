//! Prose Utilities Parser
//!
//! This module provides parsing logic for prose-related Tailwind CSS utilities,
//! including the prose plugin classes for beautiful typography.

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct ProseParser;

impl ProseParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse prose classes
    fn parse_prose_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "prose" => Some(vec![
                // Typography base styles
                CssProperty {
                    name: "color".to_string(),
                    value: "inherit".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "max-width".to_string(),
                    value: "65ch".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "line-height".to_string(),
                    value: "1.75".to_string(),
                    important: false,
                },
                // Headings
                CssProperty {
                    name: "--tw-prose-headings".to_string(),
                    value: "var(--tw-prose-headings-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-headings-color".to_string(),
                    value: "#111827".to_string(),
                    important: false,
                },
                // Body text
                CssProperty {
                    name: "--tw-prose-body".to_string(),
                    value: "var(--tw-prose-body-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-body-color".to_string(),
                    value: "#374151".to_string(),
                    important: false,
                },
                // Links
                CssProperty {
                    name: "--tw-prose-links".to_string(),
                    value: "var(--tw-prose-links-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-links-color".to_string(),
                    value: "#2563eb".to_string(),
                    important: false,
                },
                // Bold
                CssProperty {
                    name: "--tw-prose-bold".to_string(),
                    value: "var(--tw-prose-bold-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-bold-color".to_string(),
                    value: "#111827".to_string(),
                    important: false,
                },
                // Code
                CssProperty {
                    name: "--tw-prose-code".to_string(),
                    value: "var(--tw-prose-code-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-code-color".to_string(),
                    value: "#111827".to_string(),
                    important: false,
                },
                // Pre
                CssProperty {
                    name: "--tw-prose-pre-code".to_string(),
                    value: "var(--tw-prose-pre-code-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-pre-code-color".to_string(),
                    value: "#e5e7eb".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-pre-bg".to_string(),
                    value: "#1f2937".to_string(),
                    important: false,
                },
                // Quotes
                CssProperty {
                    name: "--tw-prose-quotes".to_string(),
                    value: "var(--tw-prose-quotes-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-quotes-color".to_string(),
                    value: "#111827".to_string(),
                    important: false,
                },
                // Quote borders
                CssProperty {
                    name: "--tw-prose-quote-borders".to_string(),
                    value: "#e5e7eb".to_string(),
                    important: false,
                },
                // Captions
                CssProperty {
                    name: "--tw-prose-captions".to_string(),
                    value: "var(--tw-prose-captions-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-captions-color".to_string(),
                    value: "#6b7280".to_string(),
                    important: false,
                },
                // Bullets
                CssProperty {
                    name: "--tw-prose-bullets".to_string(),
                    value: "var(--tw-prose-bullets-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-bullets-color".to_string(),
                    value: "#d1d5db".to_string(),
                    important: false,
                },
                // HR
                CssProperty {
                    name: "--tw-prose-hr".to_string(),
                    value: "var(--tw-prose-hr-color)".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-hr-color".to_string(),
                    value: "#e5e7eb".to_string(),
                    important: false,
                },
            ]),
            "prose-sm" => Some(vec![
                CssProperty {
                    name: "font-size".to_string(),
                    value: "0.875rem".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "line-height".to_string(),
                    value: "1.7142857".to_string(),
                    important: false,
                },
            ]),
            "prose-lg" => Some(vec![
                CssProperty {
                    name: "font-size".to_string(),
                    value: "1.125rem".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "line-height".to_string(),
                    value: "1.7777778".to_string(),
                    important: false,
                },
            ]),
            "prose-xl" => Some(vec![
                CssProperty {
                    name: "font-size".to_string(),
                    value: "1.25rem".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "line-height".to_string(),
                    value: "1.8".to_string(),
                    important: false,
                },
            ]),
            "prose-2xl" => Some(vec![
                CssProperty {
                    name: "font-size".to_string(),
                    value: "1.5rem".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "line-height".to_string(),
                    value: "1.6666667".to_string(),
                    important: false,
                },
            ]),
            "prose-invert" => Some(vec![
                // Dark mode prose styles
                CssProperty {
                    name: "--tw-prose-headings-color".to_string(),
                    value: "#f9fafb".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-body-color".to_string(),
                    value: "#d1d5db".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-links-color".to_string(),
                    value: "#60a5fa".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-bold-color".to_string(),
                    value: "#f9fafb".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-code-color".to_string(),
                    value: "#f9fafb".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-pre-code-color".to_string(),
                    value: "#d1d5db".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-pre-bg".to_string(),
                    value: "#0f172a".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-quotes-color".to_string(),
                    value: "#f9fafb".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-quote-borders".to_string(),
                    value: "#374151".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-captions-color".to_string(),
                    value: "#9ca3af".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-bullets-color".to_string(),
                    value: "#4b5563".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "--tw-prose-hr-color".to_string(),
                    value: "#374151".to_string(),
                    important: false,
                },
            ]),
            _ => None,
        }
    }
}

impl UtilityParser for ProseParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_prose_class(class)
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "prose",
            "prose-sm",
            "prose-lg",
            "prose-xl",
            "prose-2xl",
            "prose-invert",
        ]
    }

    fn get_priority(&self) -> u32 {
        95
    }
    fn get_category(&self) -> ParserCategory {
        ParserCategory::Typography
    }
}

impl Default for ProseParser {
    fn default() -> Self {
        Self::new()
    }
}
