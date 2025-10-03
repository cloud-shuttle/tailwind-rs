//! Boundary Classification System
//!
//! Context-aware boundary checking for accurate Tailwind class extraction.
//! Inspired by the official Tailwind Oxide implementation.

use std::fmt;

/// Boundary classification for characters around potential class names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryType {
    /// Valid boundary character (whitespace, quotes, etc.)
    Valid,
    /// Invalid boundary (continues parsing context)
    Invalid,
}

/// Boundary classification for different character types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryClass {
    /// Whitespace characters (tabs, newlines, spaces)
    /// Valid boundaries: `<div class="flex">`, `<div class="flex block">`
    Whitespace,

    /// Quote characters (single, double, backtick)
    /// Valid boundaries: `class="flex"`, `class='flex'`, `class=\`flex\``
    Quote,

    /// End of input
    /// Valid boundaries: `flex` (at end of content)
    EndOfInput,

    /// Valid characters before classes (dots, closing braces, etc.)
    /// Valid before: `.flex`, `}flex`, `>flex`
    ValidBefore,

    /// Valid characters after classes (opening braces, pipes, etc.)
    /// Valid after: `flex{`, `flex|`, `flex<`
    ValidAfter,

    /// Invalid boundary characters
    /// Examples: letters, numbers, most symbols
    Invalid,
}

impl BoundaryClass {
    /// Classify a byte character
    #[inline(always)]
    pub fn classify(c: u8) -> Self {
        match c {
            // Whitespace
            b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' => Self::Whitespace,

            // Quotes
            b'"' | b'\'' | b'`' => Self::Quote,

            // End of input
            0x00 => Self::EndOfInput,

            // Valid before boundaries
            b'.' | b'}' | b'>' => Self::ValidBefore,

            // Valid after boundaries
            b'{' | b'|' | b'<' | b'=' | b'\\' => Self::ValidAfter,

            // Everything else is invalid
            _ => Self::Invalid,
        }
    }

    /// Check if this boundary type allows a class to start
    #[inline(always)]
    pub fn is_valid_before(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Quote | Self::EndOfInput | Self::ValidBefore)
    }

    /// Check if this boundary type allows a class to end
    #[inline(always)]
    pub fn is_valid_after(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Quote | Self::EndOfInput | Self::ValidAfter)
    }
}

/// Boundary validator for checking class validity in context
#[derive(Debug, Clone)]
pub struct BoundaryValidator {
    input: Vec<u8>,
}

impl BoundaryValidator {
    /// Create a new boundary validator
    pub fn new(input: &[u8]) -> Self {
        Self {
            input: input.to_vec(),
        }
    }

    /// Check if a span has valid boundaries
    #[inline(always)]
    pub fn has_valid_boundaries(&self, start: usize, end: usize) -> bool {
        let before = self.get_boundary_before(start);
        let after = self.get_boundary_after(end);

        before.is_valid_before() && after.is_valid_after()
    }

    /// Get boundary classification before a position
    #[inline(always)]
    fn get_boundary_before(&self, pos: usize) -> BoundaryClass {
        if pos == 0 {
            // Start of input is always valid
            BoundaryClass::EndOfInput
        } else {
            BoundaryClass::classify(self.input[pos - 1])
        }
    }

    /// Get boundary classification after a position
    #[inline(always)]
    fn get_boundary_after(&self, pos: usize) -> BoundaryClass {
        if pos >= self.input.len() {
            // End of input is always valid
            BoundaryClass::EndOfInput
        } else {
            BoundaryClass::classify(self.input[pos])
        }
    }

    /// Check if a class name would be valid at a given position
    pub fn is_valid_class_position(&self, class_start: usize, class_end: usize) -> bool {
        // Quick bounds check
        if class_start >= self.input.len() || class_end > self.input.len() || class_start >= class_end {
            return false;
        }

        // Check boundaries
        self.has_valid_boundaries(class_start, class_end)
    }

    /// Extract valid class spans from input
    pub fn extract_valid_class_spans(&self, potential_spans: &[(usize, usize)]) -> Vec<(usize, usize)> {
        potential_spans
            .iter()
            .filter(|&&(start, end)| self.is_valid_class_position(start, end))
            .copied()
            .collect()
    }
}

/// Language-specific boundary rules
#[derive(Debug, Clone)]
pub struct LanguageBoundaryRules {
    /// Language type
    pub language: TemplateLanguage,
    /// Custom boundary mappings (additional to defaults)
    pub custom_boundaries: std::collections::HashMap<u8, BoundaryClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateLanguage {
    HTML,
    JavaScript,
    Vue,
    Svelte,
    Angular,
    React,
    Haml,
    Pug,
    Slim,
    Clojure,
    Elixir,
    Unknown,
}

impl LanguageBoundaryRules {
    /// Get boundary rules for a specific language
    pub fn for_language(language: TemplateLanguage) -> Self {
        let mut rules = Self {
            language,
            custom_boundaries: std::collections::HashMap::new(),
        };

        match language {
            TemplateLanguage::Vue => {
                // Vue-specific boundaries
                rules.custom_boundaries.insert(b':', BoundaryClass::ValidBefore); // :class=
                rules.custom_boundaries.insert(b'v', BoundaryClass::ValidAfter); // v-bind:
            }
            TemplateLanguage::Svelte => {
                // Svelte-specific boundaries
                rules.custom_boundaries.insert(b'{', BoundaryClass::ValidBefore); // {variable}
                rules.custom_boundaries.insert(b'}', BoundaryClass::ValidAfter);
            }
            TemplateLanguage::Angular => {
                // Angular-specific boundaries
                rules.custom_boundaries.insert(b'[', BoundaryClass::ValidBefore); // [class]=
                rules.custom_boundaries.insert(b']', BoundaryClass::ValidAfter);
                rules.custom_boundaries.insert(b'(', BoundaryClass::ValidBefore); // (click)=
                rules.custom_boundaries.insert(b')', BoundaryClass::ValidAfter);
            }
            TemplateLanguage::Haml => {
                // Haml-specific boundaries
                rules.custom_boundaries.insert(b'%', BoundaryClass::ValidBefore); // %div
                rules.custom_boundaries.insert(b'#', BoundaryClass::ValidBefore); // #id
            }
            TemplateLanguage::Pug => {
                // Pug-specific boundaries
                rules.custom_boundaries.insert(b'#', BoundaryClass::ValidBefore); // #id
                rules.custom_boundaries.insert(b'.', BoundaryClass::ValidBefore); // .class
            }
            _ => {
                // Default rules for HTML/JavaScript
            }
        }

        rules
    }

    /// Classify a character with language-specific rules
    #[inline(always)]
    pub fn classify_with_language(&self, c: u8) -> BoundaryClass {
        // Check custom language rules first
        if let Some(&boundary) = self.custom_boundaries.get(&c) {
            return boundary;
        }

        // Fall back to default classification
        BoundaryClass::classify(c)
    }
}

/// Utility functions for boundary checking
pub mod utils {
    use super::*;

    /// Check if a string slice has valid boundaries in the given input
    pub fn has_valid_boundaries_in_input(input: &str, class_start: usize, class_len: usize) -> bool {
        let validator = BoundaryValidator::new(input.as_bytes());
        validator.has_valid_boundaries(class_start, class_start + class_len)
    }

    /// Extract all valid class names from a string with boundary checking
    pub fn extract_valid_classes(input: &str, potential_classes: &[&str]) -> Vec<String> {
        let validator = BoundaryValidator::new(input.as_bytes());
        let input_bytes = input.as_bytes();

        potential_classes
            .iter()
            .filter_map(|&class| {
                // Find all occurrences of this class
                let mut start = 0;
                while let Some(pos) = find_subsequence(&input_bytes[start..], class.as_bytes()) {
                    let absolute_start = start + pos;
                    let absolute_end = absolute_start + class.len();

                    if validator.is_valid_class_position(absolute_start, absolute_end) {
                        return Some(class.to_string());
                    }

                    start = absolute_end;
                }
                None
            })
            .collect()
    }

    /// Find the position of a subsequence in a byte slice
    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_classification() {
        assert!(BoundaryClass::classify(b' ').is_valid_before());
        assert!(BoundaryClass::classify(b' ').is_valid_after());

        assert!(BoundaryClass::classify(b'"').is_valid_before());
        assert!(BoundaryClass::classify(b'"').is_valid_after());

        assert!(BoundaryClass::classify(b'.').is_valid_before());
        assert!(!BoundaryClass::classify(b'.').is_valid_after());

        assert!(!BoundaryClass::classify(b'a').is_valid_before());
        assert!(!BoundaryClass::classify(b'a').is_valid_after());
    }

    #[test]
    fn test_boundary_validator() {
        let input = br#"<div class="flex items-center px-4"></div>"#;
        let validator = BoundaryValidator::new(input);

        // Valid class positions
        assert!(validator.is_valid_class_position(12, 16)); // "flex"
        assert!(validator.is_valid_class_position(17, 29)); // "items-center"
        assert!(validator.is_valid_class_position(30, 34)); // "px-4"

        // Invalid positions (no proper boundaries)
        assert!(!validator.is_valid_class_position(1, 4)); // "div" (no quotes around)
    }

    #[test]
    fn test_language_specific_rules() {
        let vue_rules = LanguageBoundaryRules::for_language(TemplateLanguage::Vue);

        // Vue-specific boundaries
        assert_eq!(vue_rules.classify_with_language(b':'), BoundaryClass::ValidBefore);

        let svelte_rules = LanguageBoundaryRules::for_language(TemplateLanguage::Svelte);

        // Svelte-specific boundaries
        assert_eq!(svelte_rules.classify_with_language(b'{'), BoundaryClass::ValidBefore);
        assert_eq!(svelte_rules.classify_with_language(b'}'), BoundaryClass::ValidAfter);
    }

    #[test]
    fn test_extract_valid_classes() {
        let input = r#"<div class="flex invalid items-center px-4"></div>"#;
        let potential_classes = vec!["flex", "invalid", "items-center", "px-4"];

        let valid_classes = utils::extract_valid_classes(input, &potential_classes);

        // Only classes with proper boundaries should be extracted
        assert_eq!(valid_classes.len(), 3);
        assert!(valid_classes.contains(&"flex".to_string()));
        assert!(valid_classes.contains(&"items-center".to_string()));
        assert!(valid_classes.contains(&"px-4".to_string()));
        assert!(!valid_classes.contains(&"invalid".to_string()));
    }
}
