//! CSS AST (Abstract Syntax Tree) definitions
//!
//! This module provides the core AST structures for representing CSS
//! in a structured, manipulatable format.

use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

/// Root CSS AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CSSNode {
    /// Stylesheet containing multiple rules
    Stylesheet(Vec<CSSRule>),
    /// Single CSS rule
    Rule(CSSRule),
    /// CSS declaration
    Declaration(CSSDeclaration),
    /// At-rule (e.g., @media, @keyframes)
    AtRule(CSSAtRule),
    /// Comment
    Comment(String),
}

/// CSS rule representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSSRule {
    /// Rule selector (e.g., ".class", "#id", "div")
    pub selector: String,
    /// CSS declarations
    pub declarations: Vec<CSSDeclaration>,
    /// Nested rules (for complex selectors)
    pub nested_rules: Vec<CSSRule>,
    /// Media query (for responsive rules)
    pub media_query: Option<String>,
    /// Rule specificity
    pub specificity: u32,
    /// Source position
    pub position: Option<SourcePosition>,
}

/// CSS declaration (property-value pair)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSSDeclaration {
    /// CSS property name
    pub property: String,
    /// CSS property value
    pub value: String,
    /// Whether the declaration is marked as !important
    pub important: bool,
    /// Source position
    pub position: Option<SourcePosition>,
}

/// CSS at-rule (e.g., @media, @keyframes, @import)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSSAtRule {
    /// At-rule name (e.g., "media", "keyframes", "import")
    pub name: String,
    /// At-rule parameters
    pub params: String,
    /// Nested rules or declarations
    pub body: Vec<CSSNode>,
    /// Source position
    pub position: Option<SourcePosition>,
}

/// Source position information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourcePosition {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Source file path
    pub source: Option<String>,
}

/// CSS selector component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectorComponent {
    /// Class selector (.class)
    Class(String),
    /// ID selector (#id)
    Id(String),
    /// Element selector (div, span, etc.)
    Element(String),
    /// Attribute selector ([attr="value"])
    Attribute(AttributeSelector),
    /// Pseudo-class (:hover, :focus)
    PseudoClass(String),
    /// Pseudo-element (::before, ::after)
    PseudoElement(String),
    /// Universal selector (*)
    Universal,
    /// Combinator (>, +, ~, space)
    Combinator(CombinatorType),
    /// Group of selectors
    Group(Vec<SelectorComponent>),
}

/// Attribute selector
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeSelector {
    pub name: String,
    pub operator: AttributeOperator,
    pub value: Option<String>,
    pub case_sensitive: bool,
}

/// Attribute operator types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeOperator {
    /// [attr]
    Exists,
    /// [attr="value"]
    Equals,
    /// [attr~="value"]
    ContainsWord,
    /// [attr|="value"]
    StartsWith,
    /// [attr^="value"]
    StartsWithPrefix,
    /// [attr$="value"]
    EndsWith,
    /// [attr*="value"]
    Contains,
}

/// Combinator types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CombinatorType {
    /// Descendant combinator (space)
    Descendant,
    /// Child combinator (>)
    Child,
    /// Adjacent sibling combinator (+)
    AdjacentSibling,
    /// General sibling combinator (~)
    GeneralSibling,
}

/// CSS specificity calculation
impl CSSRule {
    /// Calculate CSS specificity
    pub fn calculate_specificity(&self) -> u32 {
        let mut specificity = 0u32;

        // Count ID selectors (100 points each)
        let id_count = self.selector.matches('#').count();
        specificity += (id_count as u32) * 100;

        // Count class selectors, attribute selectors, and pseudo-classes (10 points each)
        let class_count = self.selector.matches('.').count();
        let attribute_count = self.selector.matches('[').count();
        let pseudo_class_count =
            self.selector.matches(':').count() - self.selector.matches("::").count();
        specificity += ((class_count + attribute_count + pseudo_class_count) as u32) * 10;

        // Count element selectors (1 point each)
        let element_count = self
            .selector
            .split_whitespace()
            .filter(|s| {
                !s.starts_with('.')
                    && !s.starts_with('#')
                    && !s.starts_with('[')
                    && !s.starts_with(':')
            })
            .count();
        specificity += element_count as u32;

        specificity
    }

    /// Check if rule matches a selector
    pub fn matches_selector(&self, target_selector: &str) -> bool {
        self.selector == target_selector
    }

    /// Add a declaration to the rule
    pub fn add_declaration(&mut self, property: String, value: String, important: bool) {
        let declaration = CSSDeclaration {
            property,
            value,
            important,
            position: None,
        };
        self.declarations.push(declaration);
    }

    /// Remove a declaration by property name
    pub fn remove_declaration(&mut self, property: &str) {
        self.declarations.retain(|decl| decl.property != property);
    }

    /// Get a declaration by property name
    pub fn get_declaration(&self, property: &str) -> Option<&CSSDeclaration> {
        self.declarations
            .iter()
            .find(|decl| decl.property == property)
    }

    /// Check if rule has a specific property
    pub fn has_property(&self, property: &str) -> bool {
        self.declarations
            .iter()
            .any(|decl| decl.property == property)
    }
}

impl CSSDeclaration {
    /// Create a new declaration
    pub fn new(property: String, value: String) -> Self {
        Self {
            property,
            value,
            important: false,
            position: None,
        }
    }

    /// Create a new important declaration
    pub fn new_important(property: String, value: String) -> Self {
        Self {
            property,
            value,
            important: true,
            position: None,
        }
    }

    /// Set the declaration as important
    pub fn set_important(&mut self) {
        self.important = true;
    }

    /// Check if declaration is important
    pub fn is_important(&self) -> bool {
        self.important
    }
}

impl CSSAtRule {
    /// Create a new at-rule
    pub fn new(name: String, params: String) -> Self {
        Self {
            name,
            params,
            body: Vec::new(),
            position: None,
        }
    }

    /// Add a nested rule to the at-rule
    pub fn add_rule(&mut self, rule: CSSRule) {
        self.body.push(CSSNode::Rule(rule));
    }

    /// Add a declaration to the at-rule
    pub fn add_declaration(&mut self, declaration: CSSDeclaration) {
        self.body.push(CSSNode::Declaration(declaration));
    }
}

/// AST manipulation utilities
impl CSSNode {
    /// Get all rules from a stylesheet
    pub fn get_rules(&self) -> Vec<&CSSRule> {
        match self {
            CSSNode::Stylesheet(rules) => rules.iter().collect(),
            CSSNode::Rule(rule) => vec![rule],
            _ => Vec::new(),
        }
    }

    /// Get all declarations from a node
    pub fn get_declarations(&self) -> Vec<&CSSDeclaration> {
        match self {
            CSSNode::Rule(rule) => rule.declarations.iter().collect(),
            CSSNode::Declaration(decl) => vec![decl],
            _ => Vec::new(),
        }
    }

    /// Find rules by selector
    pub fn find_rules_by_selector(&self, selector: &str) -> Vec<&CSSRule> {
        self.get_rules()
            .into_iter()
            .filter(|rule| rule.matches_selector(selector))
            .collect()
    }

    /// Find rules by property
    pub fn find_rules_by_property(&self, property: &str) -> Vec<&CSSRule> {
        self.get_rules()
            .into_iter()
            .filter(|rule| rule.has_property(property))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_rule_creation() {
        let rule = CSSRule {
            selector: ".test".to_string(),
            declarations: vec![
                CSSDeclaration::new("color".to_string(), "red".to_string()),
                CSSDeclaration::new("font-size".to_string(), "16px".to_string()),
            ],
            nested_rules: Vec::new(),
            media_query: None,
            specificity: 0,
            position: None,
        };

        assert_eq!(rule.selector, ".test");
        assert_eq!(rule.declarations.len(), 2);
        assert!(rule.has_property("color"));
        assert!(!rule.has_property("background"));
    }

    #[test]
    fn test_specificity_calculation() {
        let rule = CSSRule {
            selector: "#id .class div".to_string(),
            declarations: Vec::new(),
            nested_rules: Vec::new(),
            media_query: None,
            specificity: 0,
            position: None,
        };

        let specificity = rule.calculate_specificity();
        // 1 ID (#id) = 100, 1 class (.class) = 10, 1 element (div) = 1
        assert_eq!(specificity, 111);
    }

    #[test]
    fn test_declaration_creation() {
        let decl = CSSDeclaration::new_important("color".to_string(), "red".to_string());
        assert_eq!(decl.property, "color");
        assert_eq!(decl.value, "red");
        assert!(decl.is_important());
    }

    #[test]
    fn test_at_rule_creation() {
        let mut at_rule = CSSAtRule::new("media".to_string(), "(max-width: 768px)".to_string());
        at_rule.add_rule(CSSRule {
            selector: ".mobile".to_string(),
            declarations: vec![CSSDeclaration::new(
                "display".to_string(),
                "block".to_string(),
            )],
            nested_rules: Vec::new(),
            media_query: None,
            specificity: 0,
            position: None,
        });

        assert_eq!(at_rule.name, "media");
        assert_eq!(at_rule.params, "(max-width: 768px)");
        assert_eq!(at_rule.body.len(), 1);
    }
}
