//! CSS minifier for compression and minification

use regex::Regex;
use super::types::*;

/// CSS minifier for compression and minification
pub struct CSSMinifier {
    minification_strategies: Vec<Box<dyn MinificationStrategy>>,
}

impl CSSMinifier {
    /// Create new CSS minifier
    pub fn new() -> Self {
        Self {
            minification_strategies: Self::build_minification_strategies(),
        }
    }
    
    /// Minify CSS
    pub fn minify(&self, css: &str) -> Result<String, OptimizationError> {
        let mut minified = css.to_string();
        
        // Apply minification strategies
        for strategy in &self.minification_strategies {
            minified = strategy.apply(&minified)?;
        }
        
        Ok(minified)
    }
    
    /// Build minification strategies
    fn build_minification_strategies() -> Vec<Box<dyn MinificationStrategy>> {
        vec![
            Box::new(RemoveWhitespaceStrategy),
            Box::new(RemoveCommentsStrategy),
            Box::new(OptimizeColorsStrategy),
            Box::new(RemoveSemicolonsStrategy),
        ]
    }
}

/// Minification strategy trait
pub trait MinificationStrategy {
    fn apply(&self, css: &str) -> Result<String, OptimizationError>;
    fn name(&self) -> &str;
}

/// Remove whitespace strategy
pub struct RemoveWhitespaceStrategy;

impl MinificationStrategy for RemoveWhitespaceStrategy {
    fn apply(&self, css: &str) -> Result<String, OptimizationError> {
        let mut result = String::new();
        let mut in_string = false;
        let mut string_char = '\0';
        
        for ch in css.chars() {
            match ch {
                '"' | '\'' => {
                    if !in_string {
                        in_string = true;
                        string_char = ch;
                    } else if ch == string_char {
                        in_string = false;
                    }
                    result.push(ch);
                }
                ' ' | '\t' | '\n' | '\r' => {
                    if !in_string {
                        // Only add space if necessary
                        if !result.is_empty() && !result.ends_with(';') && !result.ends_with('{') && !result.ends_with('}') {
                            if let Some(next_char) = css.chars().nth(result.len()) {
                                if !matches!(next_char, ' ' | '\t' | '\n' | '\r' | ';' | '{' | '}') {
                                    result.push(' ');
                                }
                            }
                        }
                    } else {
                        result.push(ch);
                    }
                }
                _ => result.push(ch),
            }
        }
        
        Ok(result)
    }
    
    fn name(&self) -> &str {
        "remove_whitespace"
    }
}

/// Remove comments strategy
pub struct RemoveCommentsStrategy;

impl MinificationStrategy for RemoveCommentsStrategy {
    fn apply(&self, css: &str) -> Result<String, OptimizationError> {
        let mut result = String::new();
        let mut chars = css.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '/' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '*' {
                        // Skip comment
                        chars.next(); // consume *
                        while let Some(ch) = chars.next() {
                            if ch == '*' {
                                if let Some(&next_ch) = chars.peek() {
                                    if next_ch == '/' {
                                        chars.next(); // consume /
                                        break;
                                    }
                                }
                            }
                        }
                        continue;
                    }
                }
            }
            result.push(ch);
        }
        
        Ok(result)
    }
    
    fn name(&self) -> &str {
        "remove_comments"
    }
}

/// Optimize colors strategy
pub struct OptimizeColorsStrategy;

impl MinificationStrategy for OptimizeColorsStrategy {
    fn apply(&self, css: &str) -> Result<String, OptimizationError> {
        let mut result = css.to_string();
        
        // Convert #rrggbb to #rgb where possible
        let hex_pattern = Regex::new(r"#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();
        result = hex_pattern.replace_all(&result, |caps: &regex::Captures| {
            let r = &caps[1];
            let g = &caps[2];
            let b = &caps[3];
            
            if r.chars().nth(0) == r.chars().nth(1) &&
               g.chars().nth(0) == g.chars().nth(1) &&
               b.chars().nth(0) == b.chars().nth(1) {
                format!("#{}{}{}", r.chars().nth(0).unwrap(), g.chars().nth(0).unwrap(), b.chars().nth(0).unwrap())
            } else {
                caps[0].to_string()
            }
        }).to_string();
        
        Ok(result)
    }
    
    fn name(&self) -> &str {
        "optimize_colors"
    }
}

/// Remove unnecessary semicolons strategy
pub struct RemoveSemicolonsStrategy;

impl MinificationStrategy for RemoveSemicolonsStrategy {
    fn apply(&self, css: &str) -> Result<String, OptimizationError> {
        let mut result = css.to_string();
        
        // Remove semicolons before closing braces
        result = result.replace(";}", "}");
        
        Ok(result)
    }
    
    fn name(&self) -> &str {
        "remove_semicolons"
    }
}
