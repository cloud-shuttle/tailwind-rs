//! CSS Parser implementation
//!
//! This module provides CSS parsing functionality with support for
//! modern CSS features and PostCSS compatibility.

use crate::ast::{CSSNode, CSSRule, CSSDeclaration, CSSAtRule, SourcePosition};
use crate::error::{PostCSSError, Result};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

/// CSS parser with configurable options
#[derive(Debug, Clone)]
pub struct CSSParser {
    options: ParseOptions,
}

/// Parser configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOptions {
    /// Enable source position tracking
    pub track_positions: bool,
    /// Enable strict parsing
    pub strict_mode: bool,
    /// Enable CSS custom properties support
    pub custom_properties: bool,
    /// Enable CSS nesting support
    pub nesting: bool,
    /// Enable CSS container queries support
    pub container_queries: bool,
    /// Enable CSS cascade layers support
    pub cascade_layers: bool,
    /// Maximum nesting depth
    pub max_nesting_depth: usize,
    /// Custom parser plugins
    pub plugins: Vec<String>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            track_positions: true,
            strict_mode: false,
            custom_properties: true,
            nesting: true,
            container_queries: true,
            cascade_layers: true,
            max_nesting_depth: 10,
            plugins: Vec::new(),
        }
    }
}

impl CSSParser {
    /// Create a new CSS parser with options
    pub fn new(options: ParseOptions) -> Self {
        Self { options }
    }
    
    /// Parse CSS input into AST
    pub fn parse(&self, input: &str) -> Result<CSSNode> {
        let mut parser_state = ParserState::new(input, &self.options);
        self.parse_stylesheet(&mut parser_state)
    }
    
    /// Parse a stylesheet (root level)
    fn parse_stylesheet(&self, state: &mut ParserState) -> Result<CSSNode> {
        let mut rules = Vec::new();
        
        while !state.is_eof() {
            state.skip_whitespace();
            
            if state.is_eof() {
                break;
            }
            
            // Handle at-rules
            if state.peek() == Some('@') {
                if let Ok(at_rule) = self.parse_at_rule(state) {
                    rules.push(CSSRule {
                        selector: format!("@{}", at_rule.name),
                        declarations: Vec::new(),
                        nested_rules: vec![CSSRule {
                            selector: at_rule.params.clone(),
                            declarations: Vec::new(),
                            nested_rules: Vec::new(),
                            media_query: None,
                            specificity: 0,
                            position: at_rule.position.clone(),
                        }],
                        media_query: None,
                        specificity: 0,
                        position: at_rule.position.clone(),
                    });
                }
            } else {
                // Parse regular rule
                if let Ok(rule) = self.parse_rule(state) {
                    rules.push(rule);
                }
            }
        }
        
        Ok(CSSNode::Stylesheet(rules))
    }
    
    /// Parse a CSS rule
    fn parse_rule(&self, state: &mut ParserState) -> Result<CSSRule> {
        let start_pos = state.position();
        
        // Parse selector
        let selector = self.parse_selector(state)?;
        state.skip_whitespace();
        
        // Expect opening brace
        if state.peek() != Some('{') {
            return Err(PostCSSError::ParseError {
                message: "Expected '{' after selector".to_string(),
                line: state.line(),
                column: state.column(),
            });
        }
        state.advance(); // consume '{'
        
        // Parse declarations
        let mut declarations = Vec::new();
        state.skip_whitespace();
        
        while !state.is_eof() && state.peek() != Some('}') {
            if let Ok(declaration) = self.parse_declaration(state) {
                declarations.push(declaration);
            }
            state.skip_whitespace();
        }
        
        // Expect closing brace
        if state.peek() != Some('}') {
            return Err(PostCSSError::ParseError {
                message: "Expected '}' after declarations".to_string(),
                line: state.line(),
                column: state.column(),
            });
        }
        state.advance(); // consume '}'
        
        Ok(CSSRule {
            selector,
            declarations,
            nested_rules: Vec::new(),
            media_query: None,
            specificity: 0,
            position: if self.options.track_positions {
                Some(SourcePosition {
                    line: start_pos.line,
                    column: start_pos.column,
                    source: None,
                })
            } else {
                None
            },
        })
    }
    
    /// Parse a CSS selector
    fn parse_selector(&self, state: &mut ParserState) -> Result<String> {
        let mut selector = String::new();
        
        while !state.is_eof() && state.peek() != Some('{') {
            let ch = state.peek().unwrap();
            if ch == ';' || ch == '}' {
                break;
            }
            selector.push(ch);
            state.advance();
        }
        
        Ok(selector.trim().to_string())
    }
    
    /// Parse a CSS declaration
    fn parse_declaration(&self, state: &mut ParserState) -> Result<CSSDeclaration> {
        let start_pos = state.position();
        
        // Parse property name
        let property = self.parse_property_name(state)?;
        state.skip_whitespace();
        
        // Expect colon
        if state.peek() != Some(':') {
            return Err(PostCSSError::ParseError {
                message: "Expected ':' after property name".to_string(),
                line: state.line(),
                column: state.column(),
            });
        }
        state.advance(); // consume ':'
        state.skip_whitespace();
        
        // Parse value
        let value = self.parse_property_value(state)?;
        state.skip_whitespace();
        
        // Check for !important
        let mut important = false;
        if state.peek() == Some('!') {
            state.advance(); // consume '!'
            if state.peek() == Some('i') || state.peek() == Some('I') {
                let important_str = state.read_while(|ch| ch.is_alphabetic());
                if important_str.to_lowercase() == "important" {
                    important = true;
                }
            }
        }
        
        // Expect semicolon or end of rule
        if state.peek() == Some(';') {
            state.advance(); // consume ';'
        }
        
        Ok(CSSDeclaration {
            property,
            value,
            important,
            position: if self.options.track_positions {
                Some(SourcePosition {
                    line: start_pos.line,
                    column: start_pos.column,
                    source: None,
                })
            } else {
                None
            },
        })
    }
    
    /// Parse property name
    fn parse_property_name(&self, state: &mut ParserState) -> Result<String> {
        let name = state.read_while(|ch| ch.is_alphanumeric() || ch == '-');
        if name.is_empty() {
            return Err(PostCSSError::ParseError {
                message: "Expected property name".to_string(),
                line: state.line(),
                column: state.column(),
            });
        }
        Ok(name)
    }
    
    /// Parse property value
    fn parse_property_value(&self, state: &mut ParserState) -> Result<String> {
        let mut value = String::new();
        let mut depth = 0;
        
        while !state.is_eof() {
            let ch = state.peek().unwrap();
            
            match ch {
                '(' | '[' | '{' => {
                    depth += 1;
                    value.push(ch);
                    state.advance();
                }
                ')' | ']' | '}' => {
                    if depth > 0 {
                        depth -= 1;
                        value.push(ch);
                        state.advance();
                    } else {
                        break;
                    }
                }
                ';' | '!' => {
                    if depth == 0 {
                        break;
                    }
                    value.push(ch);
                    state.advance();
                }
                _ => {
                    value.push(ch);
                    state.advance();
                }
            }
        }
        
        Ok(value.trim().to_string())
    }
    
    /// Parse an at-rule
    fn parse_at_rule(&self, state: &mut ParserState) -> Result<CSSAtRule> {
        let start_pos = state.position();
        
        // Consume '@'
        state.advance();
        
        // Parse at-rule name
        let name = state.read_while(|ch| ch.is_alphanumeric() || ch == '-');
        if name.is_empty() {
            return Err(PostCSSError::ParseError {
                message: "Expected at-rule name".to_string(),
                line: state.line(),
                column: state.column(),
            });
        }
        
        state.skip_whitespace();
        
        // Parse parameters
        let params = if state.peek() == Some('{') {
            String::new()
        } else {
            self.parse_at_rule_params(state)?
        };
        
        // Parse body if present
        let mut body = Vec::new();
        if state.peek() == Some('{') {
            state.advance(); // consume '{'
            state.skip_whitespace();
            
            while !state.is_eof() && state.peek() != Some('}') {
                if let Ok(rule) = self.parse_rule(state) {
                    body.push(CSSNode::Rule(rule));
                }
                state.skip_whitespace();
            }
            
            if state.peek() == Some('}') {
                state.advance(); // consume '}'
            }
        }
        
        Ok(CSSAtRule {
            name,
            params,
            body,
            position: if self.options.track_positions {
                Some(SourcePosition {
                    line: start_pos.line,
                    column: start_pos.column,
                    source: None,
                })
            } else {
                None
            },
        })
    }
    
    /// Parse at-rule parameters
    fn parse_at_rule_params(&self, state: &mut ParserState) -> Result<String> {
        let mut params = String::new();
        let mut depth = 0;
        
        while !state.is_eof() {
            let ch = state.peek().unwrap();
            
            match ch {
                '(' | '[' | '{' => {
                    depth += 1;
                    params.push(ch);
                    state.advance();
                }
                ')' | ']' | '}' => {
                    if depth > 0 {
                        depth -= 1;
                        params.push(ch);
                        state.advance();
                    } else {
                        break;
                    }
                }
                _ => {
                    params.push(ch);
                    state.advance();
                }
            }
        }
        
        Ok(params.trim().to_string())
    }
}

/// Parser state for tracking position and input
#[derive(Debug)]
struct ParserState<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
    options: &'a ParseOptions,
}

impl<'a> ParserState<'a> {
    fn new(input: &'a str, options: &'a ParseOptions) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
            options,
        }
    }
    
    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match self.peek() {
                Some(ch) if ch.is_whitespace() => {
                    self.advance();
                }
                Some('/') if self.peek_ahead(1) == Some('*') => {
                    // Skip CSS comments
                    self.advance(); // consume '/'
                    self.advance(); // consume '*'
                    while !self.is_eof() {
                        if self.peek() == Some('*') && self.peek_ahead(1) == Some('/') {
                            self.advance(); // consume '*'
                            self.advance(); // consume '/'
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }
    
    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.chars().nth(self.position + offset)
    }
    
    fn read_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.is_eof() {
            if let Some(ch) = self.peek() {
                if predicate(ch) {
                    result.push(ch);
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        result
    }
    
    fn position(&self) -> SourcePosition {
        SourcePosition {
            line: self.line,
            column: self.column,
            source: None,
        }
    }
    
    fn line(&self) -> usize {
        self.line
    }
    
    fn column(&self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_css_parsing() {
        let parser = CSSParser::new(ParseOptions::default());
        let input = ".test { color: red; font-size: 16px; }";
        let result = parser.parse(input);
        
        assert!(result.is_ok());
        
        if let Ok(CSSNode::Stylesheet(rules)) = result {
            assert_eq!(rules.len(), 1);
            assert_eq!(rules[0].selector, ".test");
            assert_eq!(rules[0].declarations.len(), 2);
            assert_eq!(rules[0].declarations[0].property, "color");
            assert_eq!(rules[0].declarations[0].value, "red");
        }
    }

    #[test]
    fn test_at_rule_parsing() {
        let parser = CSSParser::new(ParseOptions::default());
        let input = "@media (max-width: 768px) { .mobile { display: block; } }";
        let result = parser.parse(input);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_important_declaration() {
        let parser = CSSParser::new(ParseOptions::default());
        let input = ".test { color: red !important; }";
        let result = parser.parse(input);
        
        assert!(result.is_ok());
        
        if let Ok(CSSNode::Stylesheet(rules)) = result {
            assert!(rules[0].declarations[0].important);
        }
    }

    #[test]
    fn test_parser_state() {
        let options = ParseOptions::default();
        let mut state = ParserState::new("test input", &options);
        
        assert!(!state.is_eof());
        assert_eq!(state.peek(), Some('t'));
        
        state.advance();
        assert_eq!(state.peek(), Some('e'));
        assert_eq!(state.line(), 1);
        assert_eq!(state.column(), 2);
    }
}
