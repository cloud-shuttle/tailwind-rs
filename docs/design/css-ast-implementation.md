# CSS AST Implementation

## Overview

This document details the implementation of a comprehensive CSS Abstract Syntax Tree (AST) for `tailwind-rs-postcss`, enabling real CSS parsing, manipulation, and generation.

## AST Node Types

### Core Node Structure

```rust
// src/css_ast/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSSNode {
    pub node_type: NodeType,
    pub source: Option<SourceLocation>,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Root,
    AtRule(AtRule),
    Rule(Rule),
    Declaration(Declaration),
    Comment(Comment),
    Function(Function),
    Dimension(Dimension),
    Number(Number),
    String(String),
    Word(Word),
    Space(Space),
    Div(Div),
    Colon(Colon),
    Semicolon(Semicolon),
    Comma(Comma),
    LeftParenthesis(LeftParenthesis),
    RightParenthesis(RightParenthesis),
    LeftSquareBracket(LeftSquareBracket),
    RightSquareBracket(RightSquareBracket),
    LeftCurlyBracket(LeftCurlyBracket),
    RightCurlyBracket(RightCurlyBracket),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtRule {
    pub name: String,
    pub params: String,
    pub has_own_ending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub selector: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declaration {
    pub property: String,
    pub value: String,
    pub important: bool,
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub text: String,
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Number {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

pub type NodeId = usize;
```

## CSS Parser Implementation

### 1. Tokenizer

```rust
// src/css_ast/tokenizer.rs
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Function(String),
    AtKeyword(String),
    Hash(String),
    String(String),
    BadString,
    Url(String),
    BadUrl,
    Delim(char),
    Number(f64),
    Percentage(f64),
    Dimension(f64, String),
    UnicodeRange(String),
    IncludeMatch,
    DashMatch,
    PrefixMatch,
    SuffixMatch,
    SubstringMatch,
    Column,
    Whitespace,
    CDO,
    CDC,
    Colon,
    Semicolon,
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBracket,
    RightCurlyBracket,
}

pub struct Tokenizer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            self.consume_whitespace();
            
            if self.position >= self.input.len() {
                break;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token, ParseError> {
        let ch = self.current_char();
        
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                self.consume_whitespace();
                Ok(Token::Whitespace)
            }
            '"' | '\'' => self.consume_string(),
            '#' => self.consume_hash(),
            '@' => self.consume_at_keyword(),
            '(' => {
                self.advance();
                Ok(Token::LeftParenthesis)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParenthesis)
            }
            '[' => {
                self.advance();
                Ok(Token::LeftSquareBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RightSquareBracket)
            }
            '{' => {
                self.advance();
                Ok(Token::LeftCurlyBracket)
            }
            '}' => {
                self.advance();
                Ok(Token::RightCurlyBracket)
            }
            ':' => {
                self.advance();
                Ok(Token::Colon)
            }
            ';' => {
                self.advance();
                Ok(Token::Semicolon)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '~' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::IncludeMatch)
                } else {
                    Ok(Token::Delim('~'))
                }
            }
            '|' => {
                self.advance();
                match self.current_char() {
                    '=' => {
                        self.advance();
                        Ok(Token::DashMatch)
                    }
                    '~' => {
                        self.advance();
                        Ok(Token::PrefixMatch)
                    }
                    '$' => {
                        self.advance();
                        Ok(Token::SuffixMatch)
                    }
                    '*' => {
                        self.advance();
                        Ok(Token::SubstringMatch)
                    }
                    _ => Ok(Token::Delim('|'))
                }
            }
            '^' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::PrefixMatch)
                } else {
                    Ok(Token::Delim('^'))
                }
            }
            '$' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::SuffixMatch)
                } else {
                    Ok(Token::Delim('$'))
                }
            }
            '*' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::SubstringMatch)
                } else {
                    Ok(Token::Delim('*'))
                }
            }
            'u' | 'U' => {
                if self.lookahead(1) == '+' && self.lookahead(2).is_ascii_hexdigit() {
                    self.consume_unicode_range()
                } else {
                    self.consume_ident()
                }
            }
            _ if ch.is_ascii_digit() => self.consume_number(),
            _ if ch.is_ascii_alphabetic() || ch == '_' || ch == '-' => self.consume_ident(),
            _ => {
                self.advance();
                Ok(Token::Delim(ch))
            }
        }
    }
    
    fn consume_string(&mut self) -> Result<Token, ParseError> {
        let quote = self.current_char();
        self.advance(); // consume opening quote
        
        let mut value = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == quote {
                self.advance(); // consume closing quote
                return Ok(Token::String(value));
            } else if ch == '\\' {
                self.advance();
                if self.position < self.input.len() {
                    value.push(self.current_char());
                    self.advance();
                }
            } else if ch == '\n' || ch == '\r' {
                return Ok(Token::BadString);
            } else {
                value.push(ch);
                self.advance();
            }
        }
        
        Ok(Token::BadString)
    }
    
    fn consume_ident(&mut self) -> Result<Token, ParseError> {
        let mut value = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                value.push(ch);
                self.advance();
            } else if ch == '(' {
                self.advance();
                return Ok(Token::Function(value));
            } else {
                break;
            }
        }
        
        Ok(Token::Ident(value))
    }
    
    fn consume_number(&mut self) -> Result<Token, ParseError> {
        let mut value = String::new();
        let mut has_dot = false;
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                value.push(ch);
                self.advance();
            } else if ch.is_ascii_alphabetic() {
                let unit = self.consume_unit()?;
                let num: f64 = value.parse().map_err(|_| ParseError::InvalidNumber)?;
                return Ok(Token::Dimension(num, unit));
            } else if ch == '%' {
                self.advance();
                let num: f64 = value.parse().map_err(|_| ParseError::InvalidNumber)?;
                return Ok(Token::Percentage(num));
            } else {
                break;
            }
        }
        
        let num: f64 = value.parse().map_err(|_| ParseError::InvalidNumber)?;
        Ok(Token::Number(num))
    }
}
```

### 2. Parser

```rust
// src/css_ast/parser.rs
use crate::css_ast::tokenizer::{Tokenizer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<CSSNode, ParseError> {
        let mut root = CSSNode {
            node_type: NodeType::Root,
            source: None,
            parent: None,
            children: Vec::new(),
            raw: None,
        };
        
        while self.position < self.tokens.len() {
            match self.current_token() {
                Some(Token::AtKeyword(name)) => {
                    let at_rule = self.parse_at_rule(name)?;
                    root.children.push(at_rule);
                }
                Some(Token::Ident(_)) => {
                    let rule = self.parse_rule()?;
                    root.children.push(rule);
                }
                Some(Token::Whitespace) => {
                    self.advance();
                }
                _ => {
                    return Err(ParseError::UnexpectedToken);
                }
            }
        }
        
        Ok(root)
    }
    
    fn parse_at_rule(&mut self, name: String) -> Result<CSSNode, ParseError> {
        self.advance(); // consume @keyword
        
        let mut params = String::new();
        
        // Parse parameters until we hit a semicolon or left curly bracket
        while self.position < self.tokens.len() {
            match self.current_token() {
                Some(Token::Semicolon) => {
                    self.advance();
                    break;
                }
                Some(Token::LeftCurlyBracket) => {
                    break;
                }
                Some(token) => {
                    params.push_str(&format!("{}", token));
                    self.advance();
                }
                None => break,
            }
        }
        
        let at_rule = AtRule {
            name,
            params,
            has_own_ending: false,
        };
        
        Ok(CSSNode {
            node_type: NodeType::AtRule(at_rule),
            source: None,
            parent: None,
            children: Vec::new(),
            raw: None,
        })
    }
    
    fn parse_rule(&mut self) -> Result<CSSNode, ParseError> {
        let mut selector = String::new();
        
        // Parse selector until we hit a left curly bracket
        while self.position < self.tokens.len() {
            match self.current_token() {
                Some(Token::LeftCurlyBracket) => {
                    self.advance();
                    break;
                }
                Some(token) => {
                    selector.push_str(&format!("{}", token));
                    self.advance();
                }
                None => return Err(ParseError::UnexpectedEndOfInput),
            }
        }
        
        let mut declarations = Vec::new();
        
        // Parse declarations until we hit a right curly bracket
        while self.position < self.tokens.len() {
            match self.current_token() {
                Some(Token::RightCurlyBracket) => {
                    self.advance();
                    break;
                }
                Some(Token::Ident(_)) => {
                    let declaration = self.parse_declaration()?;
                    declarations.push(declaration);
                }
                Some(Token::Whitespace) => {
                    self.advance();
                }
                _ => {
                    return Err(ParseError::UnexpectedToken);
                }
            }
        }
        
        let rule = Rule {
            selector,
            declarations,
        };
        
        Ok(CSSNode {
            node_type: NodeType::Rule(rule),
            source: None,
            parent: None,
            children: Vec::new(),
            raw: None,
        })
    }
    
    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        let property = match self.current_token() {
            Some(Token::Ident(name)) => {
                self.advance();
                name
            }
            _ => return Err(ParseError::UnexpectedToken),
        };
        
        // Consume colon
        match self.current_token() {
            Some(Token::Colon) => {
                self.advance();
            }
            _ => return Err(ParseError::UnexpectedToken),
        }
        
        let mut value = String::new();
        let mut important = false;
        
        // Parse value until we hit a semicolon
        while self.position < self.tokens.len() {
            match self.current_token() {
                Some(Token::Semicolon) => {
                    self.advance();
                    break;
                }
                Some(Token::Ident(name)) if name == "important" => {
                    important = true;
                    self.advance();
                }
                Some(token) => {
                    value.push_str(&format!("{}", token));
                    self.advance();
                }
                None => break,
            }
        }
        
        Ok(Declaration {
            property,
            value,
            important,
            raw: None,
        })
    }
}
```

## AST Manipulation

### 1. Node Traversal

```rust
// src/css_ast/traversal.rs
impl CSSNode {
    pub fn walk(&self, visitor: &mut dyn NodeVisitor) -> Result<(), TraversalError> {
        visitor.visit_node(self)?;
        
        match &self.node_type {
            NodeType::Rule(rule) => {
                for declaration in &rule.declarations {
                    let decl_node = CSSNode {
                        node_type: NodeType::Declaration(declaration.clone()),
                        source: None,
                        parent: None,
                        children: Vec::new(),
                        raw: None,
                    };
                    decl_node.walk(visitor)?;
                }
            }
            NodeType::AtRule(at_rule) => {
                // Handle at-rule children
                for child_id in &self.children {
                    // Traverse child nodes
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    pub fn find_nodes<F>(&self, predicate: F) -> Vec<&CSSNode>
    where
        F: Fn(&CSSNode) -> bool,
    {
        let mut results = Vec::new();
        
        if predicate(self) {
            results.push(self);
        }
        
        match &self.node_type {
            NodeType::Rule(rule) => {
                for declaration in &rule.declarations {
                    let decl_node = CSSNode {
                        node_type: NodeType::Declaration(declaration.clone()),
                        source: None,
                        parent: None,
                        children: Vec::new(),
                        raw: None,
                    };
                    results.extend(decl_node.find_nodes(&predicate));
                }
            }
            _ => {}
        }
        
        results
    }
}

pub trait NodeVisitor {
    fn visit_node(&mut self, node: &CSSNode) -> Result<(), TraversalError>;
    fn visit_rule(&mut self, rule: &Rule) -> Result<(), TraversalError>;
    fn visit_declaration(&mut self, declaration: &Declaration) -> Result<(), TraversalError>;
    fn visit_at_rule(&mut self, at_rule: &AtRule) -> Result<(), TraversalError>;
}
```

### 2. Node Transformation

```rust
// src/css_ast/transformation.rs
impl CSSNode {
    pub fn transform(&mut self, transformer: &mut dyn NodeTransformer) -> Result<(), TransformError> {
        transformer.transform_node(self)?;
        
        match &mut self.node_type {
            NodeType::Rule(rule) => {
                for declaration in &mut rule.declarations {
                    let mut decl_node = CSSNode {
                        node_type: NodeType::Declaration(declaration.clone()),
                        source: None,
                        parent: None,
                        children: Vec::new(),
                        raw: None,
                    };
                    decl_node.transform(transformer)?;
                    *declaration = match decl_node.node_type {
                        NodeType::Declaration(decl) => decl,
                        _ => return Err(TransformError::InvalidNodeType),
                    };
                }
            }
            NodeType::AtRule(_) => {
                // Transform at-rule children
                for child_id in &self.children {
                    // Transform child nodes
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

pub trait NodeTransformer {
    fn transform_node(&mut self, node: &mut CSSNode) -> Result<(), TransformError>;
    fn transform_rule(&mut self, rule: &mut Rule) -> Result<(), TransformError>;
    fn transform_declaration(&mut self, declaration: &mut Declaration) -> Result<(), TransformError>;
    fn transform_at_rule(&mut self, at_rule: &mut AtRule) -> Result<(), TransformError>;
}
```

## CSS Generation

### 1. Stringifier

```rust
// src/css_ast/stringifier.rs
impl CSSNode {
    pub fn to_string(&self) -> String {
        let mut stringifier = Stringifier::new();
        stringifier.stringify_node(self)
    }
}

pub struct Stringifier {
    indent: String,
    indent_level: usize,
}

impl Stringifier {
    pub fn new() -> Self {
        Self {
            indent: "  ".to_string(),
            indent_level: 0,
        }
    }
    
    pub fn stringify_node(&mut self, node: &CSSNode) -> String {
        match &node.node_type {
            NodeType::Root => {
                let mut result = String::new();
                for child_id in &node.children {
                    // Stringify child nodes
                }
                result
            }
            NodeType::Rule(rule) => {
                let mut result = String::new();
                result.push_str(&rule.selector);
                result.push_str(" {\n");
                
                self.indent_level += 1;
                for declaration in &rule.declarations {
                    result.push_str(&self.indent());
                    result.push_str(&declaration.property);
                    result.push_str(": ");
                    result.push_str(&declaration.value);
                    if declaration.important {
                        result.push_str(" !important");
                    }
                    result.push_str(";\n");
                }
                self.indent_level -= 1;
                
                result.push_str("}");
                result
            }
            NodeType::Declaration(declaration) => {
                let mut result = String::new();
                result.push_str(&declaration.property);
                result.push_str(": ");
                result.push_str(&declaration.value);
                if declaration.important {
                    result.push_str(" !important");
                }
                result.push_str(";");
                result
            }
            NodeType::AtRule(at_rule) => {
                let mut result = String::new();
                result.push('@');
                result.push_str(&at_rule.name);
                if !at_rule.params.is_empty() {
                    result.push(' ');
                    result.push_str(&at_rule.params);
                }
                result.push_str(";");
                result
            }
            _ => String::new(),
        }
    }
    
    fn indent(&self) -> String {
        self.indent.repeat(self.indent_level)
    }
}
```

## Error Handling

```rust
// src/css_ast/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken,
    
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    
    #[error("Invalid string: {0}")]
    InvalidString(String),
    
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),
    
    #[error("Invalid declaration: {0}")]
    InvalidDeclaration(String),
}

#[derive(Error, Debug)]
pub enum TraversalError {
    #[error("Traversal error: {0}")]
    TraversalError(String),
}

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("Transform error: {0}")]
    TransformError(String),
    
    #[error("Invalid node type")]
    InvalidNodeType,
}
```

## Testing

```rust
// src/css_ast/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_rule() {
        let css = "body { color: red; }";
        let mut tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        match ast.node_type {
            NodeType::Root => {
                assert_eq!(ast.children.len(), 1);
            }
            _ => panic!("Expected root node"),
        }
    }
    
    #[test]
    fn test_stringify_rule() {
        let rule = Rule {
            selector: "body".to_string(),
            declarations: vec![
                Declaration {
                    property: "color".to_string(),
                    value: "red".to_string(),
                    important: false,
                    raw: None,
                }
            ],
        };
        
        let node = CSSNode {
            node_type: NodeType::Rule(rule),
            source: None,
            parent: None,
            children: Vec::new(),
            raw: None,
        };
        
        let css = node.to_string();
        assert!(css.contains("body"));
        assert!(css.contains("color: red"));
    }
}
```

This comprehensive CSS AST implementation provides the foundation for real CSS processing capabilities in `tailwind-rs-postcss`.
