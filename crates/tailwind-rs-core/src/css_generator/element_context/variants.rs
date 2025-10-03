//! Variant Context Module
//!
//! Handles variant classes like hover:, focus:, md:, dark:, group-hover:, etc.

/// Context for managing variant state (hover, focus, responsive, etc.)
#[derive(Debug, Clone, Default)]
pub struct VariantContext {
    pub hover: bool,
    pub focus: bool,
    pub active: bool,
    pub visited: bool,
    pub disabled: bool,
    pub first: bool,
    pub last: bool,
    pub odd: bool,
    pub even: bool,
    pub responsive: Option<String>, // e.g., "md", "lg", etc.
    pub dark: bool,
    pub group_hover: bool,
    pub group_focus: bool,
    pub peer_hover: bool,
    pub peer_focus: bool,
}

impl VariantContext {
    /// Update variant context from a variant class
    pub fn update_from_class(&mut self, class: &str) {
        match class {
            "hover" => self.hover = true,
            "focus" => self.focus = true,
            "active" => self.active = true,
            "visited" => self.visited = true,
            "disabled" => self.disabled = true,
            "first" => self.first = true,
            "last" => self.last = true,
            "odd" => self.odd = true,
            "even" => self.even = true,
            "dark" => self.dark = true,
            "group-hover" => self.group_hover = true,
            "group-focus" => self.group_focus = true,
            "peer-hover" => self.peer_hover = true,
            "peer-focus" => self.peer_focus = true,
            // Handle responsive variants
            responsive if Self::is_responsive_variant(responsive) => {
                self.responsive = Some(responsive.to_string());
            }
            _ => {} // Not a recognized variant
        }
    }

    /// Check if a string is a responsive variant
    fn is_responsive_variant(variant: &str) -> bool {
        matches!(variant, "sm" | "md" | "lg" | "xl" | "2xl")
    }

    /// Generate CSS selector for the variants
    pub fn to_css_selector(&self, base_class: &str) -> String {
        let mut selector = format!(".{}", base_class.replace(":", "\\:"));

        // Add pseudo-selectors
        if self.hover { selector.push_str(":hover"); }
        if self.focus { selector.push_str(":focus"); }
        if self.active { selector.push_str(":active"); }
        if self.visited { selector.push_str(":visited"); }
        if self.disabled { selector.push_str(":disabled"); }
        if self.first { selector.push_str(":first-child"); }
        if self.last { selector.push_str(":last-child"); }
        if self.odd { selector.push_str(":nth-child(odd)"); }
        if self.even { selector.push_str(":nth-child(even)"); }

        // Group and peer selectors
        if self.group_hover { selector = format!(".group:hover {}", selector); }
        if self.group_focus { selector = format!(".group:focus {}", selector); }
        if self.peer_hover { selector = format!(".peer:hover ~ {}", selector); }
        if self.peer_focus { selector = format!(".peer:focus ~ {}", selector); }

        selector
    }

    /// Get media query for responsive variants
    pub fn to_media_query(&self) -> Option<String> {
        self.responsive.as_ref().map(|bp| match bp.as_str() {
            "sm" => "@media (min-width: 640px)",
            "md" => "@media (min-width: 768px)",
            "lg" => "@media (min-width: 1024px)",
            "xl" => "@media (min-width: 1280px)",
            "2xl" => "@media (min-width: 1536px)",
            _ => "",
        }).filter(|mq| !mq.is_empty()).map(|s| s.to_string())
    }

    /// Check if this variant context has any variants
    pub fn has_variants(&self) -> bool {
        self.hover || self.focus || self.active || self.visited || self.disabled ||
        self.first || self.last || self.odd || self.even || self.dark ||
        self.group_hover || self.group_focus || self.peer_hover || self.peer_focus ||
        self.responsive.is_some()
    }

    /// Parse variants from a class string like "hover:md:shadow-lg"
    pub fn parse_variants_from_class(class: &str) -> (Vec<String>, String) {
        let parts: Vec<&str> = class.split(':').collect();
        if parts.len() <= 1 {
            return (Vec::new(), class.to_string());
        }

        let variants = parts[..parts.len()-1].iter().map(|s| s.to_string()).collect();
        let base_class = parts.last().unwrap().to_string();

        (variants, base_class)
    }

    /// Generate variant CSS selector from a complex class
    pub fn to_variant_css_selector(class: &str) -> String {
        let (variants, base_class) = Self::parse_variants_from_class(class);
        let mut selector = format!(".{}", class.replace(":", "\\:"));

        // Apply variants in reverse order (innermost first)
        for variant in variants.iter().rev() {
            match variant.as_str() {
                "hover" => selector.push_str(":hover"),
                "focus" => selector.push_str(":focus"),
                "active" => selector.push_str(":active"),
                "visited" => selector.push_str(":visited"),
                "disabled" => selector.push_str(":disabled"),
                "first" => selector.push_str(":first-child"),
                "last" => selector.push_str(":last-child"),
                "odd" => selector.push_str(":nth-child(odd)"),
                "even" => selector.push_str(":nth-child(even)"),
                "group-hover" => selector = format!(".group:hover {}", selector),
                "group-focus" => selector = format!(".group:focus {}", selector),
                "peer-hover" => selector = format!(".peer:hover ~ {}", selector),
                "peer-focus" => selector = format!(".peer:focus ~ {}", selector),
                _ => {} // Unknown variant
            }
        }

        selector
    }

    /// Get media query from a variant string
    pub fn get_media_query_from_variant(variant: &str) -> Option<String> {
        match variant {
            "sm" => Some("@media (min-width: 640px)".to_string()),
            "md" => Some("@media (min-width: 768px)".to_string()),
            "lg" => Some("@media (min-width: 1024px)".to_string()),
            "xl" => Some("@media (min-width: 1280px)".to_string()),
            "2xl" => Some("@media (min-width: 1536px)".to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_parsing() {
        let mut context = VariantContext::default();

        context.update_from_class("hover");
        context.update_from_class("focus");
        context.update_from_class("md");
        context.update_from_class("dark");

        assert!(context.hover);
        assert!(context.focus);
        assert_eq!(context.responsive, Some("md".to_string()));
        assert!(context.dark);
        assert!(context.has_variants());
    }

    #[test]
    fn css_selector_generation() {
        let mut context = VariantContext::default();
        context.hover = true;
        context.focus = true;

        let selector = context.to_css_selector("shadow-lg");
        assert_eq!(selector, ".shadow-lg:hover:focus");
    }

    #[test]
    fn media_query_generation() {
        let mut context = VariantContext::default();
        context.responsive = Some("md".to_string());

        let mq = context.to_media_query();
        assert_eq!(mq, Some("@media (min-width: 768px)".to_string()));
    }

    #[test]
    fn complex_variant_parsing() {
        let (variants, base) = VariantContext::parse_variants_from_class("hover:md:shadow-lg");
        assert_eq!(variants, vec!["hover", "md"]);
        assert_eq!(base, "shadow-lg");

        let selector = VariantContext::to_variant_css_selector("hover:md:shadow-lg");
        assert_eq!(selector, ".hover\\:md\\:shadow-lg:hover");

        let mq = VariantContext::get_media_query_from_variant("md");
        assert_eq!(mq, Some("@media (min-width: 768px)".to_string()));
    }

    #[test]
    fn group_and_peer_selectors() {
        let mut context = VariantContext::default();
        context.group_hover = true;

        let selector = context.to_css_selector("text-blue-500");
        assert_eq!(selector, ".group:hover .text-blue-500");

        let mut peer_context = VariantContext::default();
        peer_context.peer_focus = true;

        let peer_selector = peer_context.to_css_selector("border-red-500");
        assert_eq!(peer_selector, ".peer:focus ~ .border-red-500");
    }

    #[test]
    fn empty_variants() {
        let context = VariantContext::default();
        assert!(!context.has_variants());

        let empty_selector = context.to_css_selector("bg-blue-500");
        assert_eq!(empty_selector, ".bg-blue-500");
    }
}
