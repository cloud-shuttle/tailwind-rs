//! CSS Nesting utilities for tailwind-rs
//!
//! This module provides utilities for CSS nesting features.
//! It includes support for nested selectors, nested media queries, and nested pseudo-classes.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// CSS nesting selector types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NestingSelector {
    /// Direct child selector (>)
    DirectChild,
    /// Descendant selector (space)
    Descendant,
    /// Adjacent sibling selector (+)
    AdjacentSibling,
    /// General sibling selector (~)
    GeneralSibling,
    /// Custom selector
    Custom(String),
}

impl fmt::Display for NestingSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NestingSelector::DirectChild => write!(f, ">"),
            NestingSelector::Descendant => write!(f, " "),
            NestingSelector::AdjacentSibling => write!(f, "+"),
            NestingSelector::GeneralSibling => write!(f, "~"),
            NestingSelector::Custom(selector) => write!(f, "{}", selector),
        }
    }
}

impl NestingSelector {
    /// Get the CSS class name for this nesting selector
    pub fn to_class_name(&self) -> String {
        match self {
            NestingSelector::DirectChild => "nest-child".to_string(),
            NestingSelector::Descendant => "nest-descendant".to_string(),
            NestingSelector::AdjacentSibling => "nest-adjacent".to_string(),
            NestingSelector::GeneralSibling => "nest-sibling".to_string(),
            NestingSelector::Custom(selector) => format!(
                "nest-{}",
                selector
                    .replace(" ", "-")
                    .replace(">", "child")
                    .replace("+", "adjacent")
                    .replace("~", "sibling")
            ),
        }
    }

    /// Get the CSS value for this nesting selector
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// CSS nesting pseudo-class types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NestingPseudoClass {
    /// Hover pseudo-class
    Hover,
    /// Focus pseudo-class
    Focus,
    /// Active pseudo-class
    Active,
    /// Visited pseudo-class
    Visited,
    /// Link pseudo-class
    Link,
    /// First child pseudo-class
    FirstChild,
    /// Last child pseudo-class
    LastChild,
    /// Nth child pseudo-class
    NthChild(String),
    /// Custom pseudo-class
    Custom(String),
}

impl fmt::Display for NestingPseudoClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NestingPseudoClass::Hover => write!(f, ":hover"),
            NestingPseudoClass::Focus => write!(f, ":focus"),
            NestingPseudoClass::Active => write!(f, ":active"),
            NestingPseudoClass::Visited => write!(f, ":visited"),
            NestingPseudoClass::Link => write!(f, ":link"),
            NestingPseudoClass::FirstChild => write!(f, ":first-child"),
            NestingPseudoClass::LastChild => write!(f, ":last-child"),
            NestingPseudoClass::NthChild(n) => write!(f, ":nth-child({})", n),
            NestingPseudoClass::Custom(pseudo) => write!(f, ":{}", pseudo),
        }
    }
}

impl NestingPseudoClass {
    /// Get the CSS class name for this nesting pseudo-class
    pub fn to_class_name(&self) -> String {
        match self {
            NestingPseudoClass::Hover => "nest-hover".to_string(),
            NestingPseudoClass::Focus => "nest-focus".to_string(),
            NestingPseudoClass::Active => "nest-active".to_string(),
            NestingPseudoClass::Visited => "nest-visited".to_string(),
            NestingPseudoClass::Link => "nest-link".to_string(),
            NestingPseudoClass::FirstChild => "nest-first-child".to_string(),
            NestingPseudoClass::LastChild => "nest-last-child".to_string(),
            NestingPseudoClass::NthChild(n) => {
                format!("nest-nth-child-{}", n.replace("n", "n").replace(" ", "-"))
            }
            NestingPseudoClass::Custom(pseudo) => format!("nest-{}", pseudo),
        }
    }

    /// Get the CSS value for this nesting pseudo-class
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// CSS nesting media query types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NestingMediaQuery {
    /// Small screen media query
    Small,
    /// Medium screen media query
    Medium,
    /// Large screen media query
    Large,
    /// Extra large screen media query
    ExtraLarge,
    /// Dark mode media query
    Dark,
    /// Light mode media query
    Light,
    /// Print media query
    Print,
    /// Screen media query
    Screen,
    /// Custom media query
    Custom(String),
}

impl fmt::Display for NestingMediaQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NestingMediaQuery::Small => write!(f, "(min-width: 640px)"),
            NestingMediaQuery::Medium => write!(f, "(min-width: 768px)"),
            NestingMediaQuery::Large => write!(f, "(min-width: 1024px)"),
            NestingMediaQuery::ExtraLarge => write!(f, "(min-width: 1280px)"),
            NestingMediaQuery::Dark => write!(f, "(prefers-color-scheme: dark)"),
            NestingMediaQuery::Light => write!(f, "(prefers-color-scheme: light)"),
            NestingMediaQuery::Print => write!(f, "print"),
            NestingMediaQuery::Screen => write!(f, "screen"),
            NestingMediaQuery::Custom(query) => write!(f, "{}", query),
        }
    }
}

impl NestingMediaQuery {
    /// Get the CSS class name for this nesting media query
    pub fn to_class_name(&self) -> String {
        match self {
            NestingMediaQuery::Small => "nest-sm".to_string(),
            NestingMediaQuery::Medium => "nest-md".to_string(),
            NestingMediaQuery::Large => "nest-lg".to_string(),
            NestingMediaQuery::ExtraLarge => "nest-xl".to_string(),
            NestingMediaQuery::Dark => "nest-dark".to_string(),
            NestingMediaQuery::Light => "nest-light".to_string(),
            NestingMediaQuery::Print => "nest-print".to_string(),
            NestingMediaQuery::Screen => "nest-screen".to_string(),
            NestingMediaQuery::Custom(query) => format!(
                "nest-{}",
                query
                    .replace("(", "")
                    .replace(")", "")
                    .replace(" ", "-")
                    .replace(":", "-")
                    .replace("--", "-")
            ),
        }
    }

    /// Get the CSS value for this nesting media query
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Trait for adding CSS nesting to ClassBuilder
pub trait CssNestingUtilities {
    /// Set nesting selector
    fn nesting_selector(self, selector: NestingSelector) -> Self;
    /// Set nesting pseudo-class
    fn nesting_pseudo_class(self, pseudo_class: NestingPseudoClass) -> Self;
    /// Set nesting media query
    fn nesting_media_query(self, media_query: NestingMediaQuery) -> Self;
    /// Set nested class with selector
    fn nested_class(self, selector: NestingSelector, class: &str) -> Self;
    /// Set nested class with pseudo-class
    fn nested_pseudo_class(self, pseudo_class: NestingPseudoClass, class: &str) -> Self;
    /// Set nested class with media query
    fn nested_media_query(self, media_query: NestingMediaQuery, class: &str) -> Self;
}

impl CssNestingUtilities for ClassBuilder {
    fn nesting_selector(self, selector: NestingSelector) -> Self {
        self.class(&selector.to_class_name())
    }

    fn nesting_pseudo_class(self, pseudo_class: NestingPseudoClass) -> Self {
        self.class(&pseudo_class.to_class_name())
    }

    fn nesting_media_query(self, media_query: NestingMediaQuery) -> Self {
        self.class(&media_query.to_class_name())
    }

    fn nested_class(self, selector: NestingSelector, class: &str) -> Self {
        let nested_class = format!("{}-{}", selector.to_class_name(), class);
        self.class(&nested_class)
    }

    fn nested_pseudo_class(self, pseudo_class: NestingPseudoClass, class: &str) -> Self {
        let nested_class = format!("{}-{}", pseudo_class.to_class_name(), class);
        self.class(&nested_class)
    }

    fn nested_media_query(self, media_query: NestingMediaQuery, class: &str) -> Self {
        let nested_class = format!("{}-{}", media_query.to_class_name(), class);
        self.class(&nested_class)
    }
}

/// Convenience methods for common nesting patterns
pub trait CssNestingConvenience {
    /// Set nested hover class
    fn nested_hover(self, class: &str) -> Self;
    /// Set nested focus class
    fn nested_focus(self, class: &str) -> Self;
    /// Set nested active class
    fn nested_active(self, class: &str) -> Self;
    /// Set nested first child class
    fn nested_first_child(self, class: &str) -> Self;
    /// Set nested last child class
    fn nested_last_child(self, class: &str) -> Self;
    /// Set nested small screen class
    fn nested_sm(self, class: &str) -> Self;
    /// Set nested medium screen class
    fn nested_md(self, class: &str) -> Self;
    /// Set nested large screen class
    fn nested_lg(self, class: &str) -> Self;
    /// Set nested dark mode class
    fn nested_dark(self, class: &str) -> Self;
    /// Set nested light mode class
    fn nested_light(self, class: &str) -> Self;
}

impl CssNestingConvenience for ClassBuilder {
    fn nested_hover(self, class: &str) -> Self {
        self.nested_pseudo_class(NestingPseudoClass::Hover, class)
    }

    fn nested_focus(self, class: &str) -> Self {
        self.nested_pseudo_class(NestingPseudoClass::Focus, class)
    }

    fn nested_active(self, class: &str) -> Self {
        self.nested_pseudo_class(NestingPseudoClass::Active, class)
    }

    fn nested_first_child(self, class: &str) -> Self {
        self.nested_pseudo_class(NestingPseudoClass::FirstChild, class)
    }

    fn nested_last_child(self, class: &str) -> Self {
        self.nested_pseudo_class(NestingPseudoClass::LastChild, class)
    }

    fn nested_sm(self, class: &str) -> Self {
        self.nested_media_query(NestingMediaQuery::Small, class)
    }

    fn nested_md(self, class: &str) -> Self {
        self.nested_media_query(NestingMediaQuery::Medium, class)
    }

    fn nested_lg(self, class: &str) -> Self {
        self.nested_media_query(NestingMediaQuery::Large, class)
    }

    fn nested_dark(self, class: &str) -> Self {
        self.nested_media_query(NestingMediaQuery::Dark, class)
    }

    fn nested_light(self, class: &str) -> Self {
        self.nested_media_query(NestingMediaQuery::Light, class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_nesting_selector_enum_values() {
        assert_eq!(NestingSelector::DirectChild.to_string(), ">");
        assert_eq!(NestingSelector::Descendant.to_string(), " ");
        assert_eq!(NestingSelector::AdjacentSibling.to_string(), "+");
        assert_eq!(NestingSelector::GeneralSibling.to_string(), "~");
        assert_eq!(
            NestingSelector::Custom("div".to_string()).to_string(),
            "div"
        );
    }

    #[test]
    fn test_nesting_selector_class_names() {
        assert_eq!(NestingSelector::DirectChild.to_class_name(), "nest-child");
        assert_eq!(
            NestingSelector::Descendant.to_class_name(),
            "nest-descendant"
        );
        assert_eq!(
            NestingSelector::AdjacentSibling.to_class_name(),
            "nest-adjacent"
        );
        assert_eq!(
            NestingSelector::GeneralSibling.to_class_name(),
            "nest-sibling"
        );
        assert_eq!(
            NestingSelector::Custom("div".to_string()).to_class_name(),
            "nest-div"
        );
    }

    #[test]
    fn test_nesting_pseudo_class_enum_values() {
        assert_eq!(NestingPseudoClass::Hover.to_string(), ":hover");
        assert_eq!(NestingPseudoClass::Focus.to_string(), ":focus");
        assert_eq!(NestingPseudoClass::Active.to_string(), ":active");
        assert_eq!(NestingPseudoClass::FirstChild.to_string(), ":first-child");
        assert_eq!(
            NestingPseudoClass::NthChild("2n".to_string()).to_string(),
            ":nth-child(2n)"
        );
        assert_eq!(
            NestingPseudoClass::Custom("custom".to_string()).to_string(),
            ":custom"
        );
    }

    #[test]
    fn test_nesting_pseudo_class_class_names() {
        assert_eq!(NestingPseudoClass::Hover.to_class_name(), "nest-hover");
        assert_eq!(NestingPseudoClass::Focus.to_class_name(), "nest-focus");
        assert_eq!(NestingPseudoClass::Active.to_class_name(), "nest-active");
        assert_eq!(
            NestingPseudoClass::FirstChild.to_class_name(),
            "nest-first-child"
        );
        assert_eq!(
            NestingPseudoClass::NthChild("2n".to_string()).to_class_name(),
            "nest-nth-child-2n"
        );
        assert_eq!(
            NestingPseudoClass::Custom("custom".to_string()).to_class_name(),
            "nest-custom"
        );
    }

    #[test]
    fn test_nesting_media_query_enum_values() {
        assert_eq!(NestingMediaQuery::Small.to_string(), "(min-width: 640px)");
        assert_eq!(NestingMediaQuery::Medium.to_string(), "(min-width: 768px)");
        assert_eq!(NestingMediaQuery::Large.to_string(), "(min-width: 1024px)");
        assert_eq!(
            NestingMediaQuery::Dark.to_string(),
            "(prefers-color-scheme: dark)"
        );
        assert_eq!(NestingMediaQuery::Print.to_string(), "print");
        assert_eq!(
            NestingMediaQuery::Custom("(max-width: 600px)".to_string()).to_string(),
            "(max-width: 600px)"
        );
    }

    #[test]
    fn test_nesting_media_query_class_names() {
        assert_eq!(NestingMediaQuery::Small.to_class_name(), "nest-sm");
        assert_eq!(NestingMediaQuery::Medium.to_class_name(), "nest-md");
        assert_eq!(NestingMediaQuery::Large.to_class_name(), "nest-lg");
        assert_eq!(NestingMediaQuery::Dark.to_class_name(), "nest-dark");
        assert_eq!(NestingMediaQuery::Print.to_class_name(), "nest-print");
        assert_eq!(
            NestingMediaQuery::Custom("(max-width: 600px)".to_string()).to_class_name(),
            "nest-max-width-600px"
        );
    }

    #[test]
    fn test_css_nesting_utilities() {
        let classes = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .nesting_pseudo_class(NestingPseudoClass::Hover)
            .nesting_media_query(NestingMediaQuery::Small)
            .nested_class(NestingSelector::Descendant, "text-blue-500")
            .nested_pseudo_class(NestingPseudoClass::Focus, "text-red-500")
            .nested_media_query(NestingMediaQuery::Medium, "text-green-500");

        let result = classes.build();
        assert!(result.classes.contains("nest-child"));
        assert!(result.classes.contains("nest-hover"));
        assert!(result.classes.contains("nest-sm"));
        assert!(result.classes.contains("nest-descendant-text-blue-500"));
        assert!(result.classes.contains("nest-focus-text-red-500"));
        assert!(result.classes.contains("nest-md-text-green-500"));
    }

    #[test]
    fn test_css_nesting_convenience() {
        let classes = ClassBuilder::new()
            .nested_hover("text-blue-500")
            .nested_focus("text-red-500")
            .nested_active("text-green-500")
            .nested_first_child("text-yellow-500")
            .nested_last_child("text-purple-500")
            .nested_sm("text-pink-500")
            .nested_md("text-indigo-500")
            .nested_lg("text-cyan-500")
            .nested_dark("text-gray-500")
            .nested_light("text-white");

        let result = classes.build();
        assert!(result.classes.contains("nest-hover-text-blue-500"));
        assert!(result.classes.contains("nest-focus-text-red-500"));
        assert!(result.classes.contains("nest-active-text-green-500"));
        assert!(result.classes.contains("nest-first-child-text-yellow-500"));
        assert!(result.classes.contains("nest-last-child-text-purple-500"));
        assert!(result.classes.contains("nest-sm-text-pink-500"));
        assert!(result.classes.contains("nest-md-text-indigo-500"));
        assert!(result.classes.contains("nest-lg-text-cyan-500"));
        assert!(result.classes.contains("nest-dark-text-gray-500"));
        assert!(result.classes.contains("nest-light-text-white"));
    }

    #[test]
    fn test_css_nesting_serialization() {
        let selector = NestingSelector::DirectChild;
        let serialized = serde_json::to_string(&selector).unwrap();
        let deserialized: NestingSelector = serde_json::from_str(&serialized).unwrap();
        assert_eq!(selector, deserialized);

        let pseudo_class = NestingPseudoClass::Hover;
        let serialized = serde_json::to_string(&pseudo_class).unwrap();
        let deserialized: NestingPseudoClass = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pseudo_class, deserialized);

        let media_query = NestingMediaQuery::Small;
        let serialized = serde_json::to_string(&media_query).unwrap();
        let deserialized: NestingMediaQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(media_query, deserialized);
    }

    #[test]
    fn test_css_nesting_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .nesting_selector(NestingSelector::DirectChild)
            .nesting_pseudo_class(NestingPseudoClass::Hover)
            .nesting_media_query(NestingMediaQuery::Small)
            .nested_class(NestingSelector::Descendant, "text-blue-500")
            .nested_pseudo_class(NestingPseudoClass::Focus, "text-red-500")
            .nested_media_query(NestingMediaQuery::Medium, "text-green-500")
            .nested_hover("text-yellow-500")
            .nested_focus("text-purple-500")
            .nested_active("text-pink-500")
            .nested_first_child("text-indigo-500")
            .nested_last_child("text-cyan-500")
            .nested_sm("text-gray-500")
            .nested_md("text-white")
            .nested_lg("text-black")
            .nested_dark("text-gray-100")
            .nested_light("text-gray-900");

        let result = classes.build();
        assert!(result.classes.contains("nest-child"));
        assert!(result.classes.contains("nest-hover"));
        assert!(result.classes.contains("nest-sm"));
        assert!(result.classes.contains("nest-descendant-text-blue-500"));
        assert!(result.classes.contains("nest-focus-text-red-500"));
        assert!(result.classes.contains("nest-md-text-green-500"));
        assert!(result.classes.contains("nest-hover-text-yellow-500"));
        assert!(result.classes.contains("nest-focus-text-purple-500"));
        assert!(result.classes.contains("nest-active-text-pink-500"));
        assert!(result.classes.contains("nest-first-child-text-indigo-500"));
        assert!(result.classes.contains("nest-last-child-text-cyan-500"));
        assert!(result.classes.contains("nest-sm-text-gray-500"));
        assert!(result.classes.contains("nest-md-text-white"));
        assert!(result.classes.contains("nest-lg-text-black"));
        assert!(result.classes.contains("nest-dark-text-gray-100"));
        assert!(result.classes.contains("nest-light-text-gray-900"));
    }
}
