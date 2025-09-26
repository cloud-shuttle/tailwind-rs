//! CSS Linting System
//!
//! This module provides comprehensive CSS linting functionality with
//! rule-based analysis, auto-fixing, and suggestions.

use super::types::*;
use regex::Regex;
use std::collections::HashMap;

/// CSS linter with comprehensive rules
pub struct CSSLinter {
    rules: Vec<LintRule>,
    config: LinterConfig,
    reporter: LintReporter,
    fixer: LintFixer,
}

impl CSSLinter {
    /// Create new CSS linter
    pub fn new(config: LinterConfig) -> Self {
        Self {
            rules: Self::get_default_rules(),
            config,
            reporter: LintReporter::new(),
            fixer: LintFixer::new(),
        }
    }

    /// Lint CSS with comprehensive rules
    pub fn lint_css(
        &self,
        css: &str,
        options: &LintOptions,
    ) -> Result<LintResult, AdvancedFeatureError> {
        // Parse CSS
        let ast = self.parse_css(css)?;

        // Apply linting rules
        let mut issues = Vec::new();
        let mut fixes = Vec::new();

        for rule in &self.rules {
            if rule.enabled {
                let rule_issues = self.apply_rule(&ast, rule)?;
                issues.extend(rule_issues);
            }
        }

        // Generate fixes if requested
        if options.auto_fix {
            fixes = self.generate_fixes(&issues)?;
        }

        // Generate suggestions
        let suggestions = self.generate_suggestions(&ast)?;

        // Calculate statistics
        let statistics = self.calculate_statistics(&issues);

        Ok(LintResult {
            issues,
            fixes,
            statistics,
            suggestions,
        })
    }

    /// Auto-fix CSS issues
    pub fn fix_css(&self, css: &str, fixes: &[LintFix]) -> Result<String, AdvancedFeatureError> {
        let mut fixed_css = css.to_string();

        // Apply fixes in order
        for fix in fixes {
            fixed_css = self.apply_fix(&fixed_css, fix)?;
        }

        Ok(fixed_css)
    }

    /// Get linting suggestions
    pub fn get_suggestions(&self, css: &str) -> Result<Vec<LintSuggestion>, AdvancedFeatureError> {
        let ast = self.parse_css(css)?;
        self.generate_suggestions(&ast)
    }

    /// Parse CSS into AST
    fn parse_css(&self, css: &str) -> Result<CSSAST, AdvancedFeatureError> {
        // Simplified CSS parsing - would use proper CSS parser
        Ok(CSSAST {
            rules: self.parse_rules(css)?,
            comments: self.parse_comments(css)?,
        })
    }

    /// Parse CSS rules
    fn parse_rules(&self, css: &str) -> Result<Vec<CSSRule>, AdvancedFeatureError> {
        let mut rules = Vec::new();
        let rule_pattern = Regex::new(r"([^{]+)\s*\{([^}]+)\}").unwrap();

        for cap in rule_pattern.captures_iter(css) {
            let selector = cap[1].trim().to_string();
            let properties = cap[2].trim().to_string();

            rules.push(CSSRule {
                selector,
                properties: self.parse_properties(&properties)?,
                line: 1, // Simplified
                column: 1,
            });
        }

        Ok(rules)
    }

    /// Parse CSS properties
    fn parse_properties(
        &self,
        properties_str: &str,
    ) -> Result<Vec<CSSProperty>, AdvancedFeatureError> {
        let mut properties = Vec::new();
        let property_pattern = Regex::new(r"([^:]+):\s*([^;]+);").unwrap();

        for cap in property_pattern.captures_iter(properties_str) {
            properties.push(CSSProperty {
                name: cap[1].trim().to_string(),
                value: cap[2].trim().to_string(),
                important: cap[2].contains("!important"),
            });
        }

        Ok(properties)
    }

    /// Parse CSS comments
    fn parse_comments(&self, css: &str) -> Result<Vec<CSSComment>, AdvancedFeatureError> {
        let mut comments = Vec::new();
        let comment_pattern = Regex::new(r"/\*([^*]|\*[^/])*\*/").unwrap();

        for cap in comment_pattern.captures_iter(css) {
            comments.push(CSSComment {
                content: cap[0].to_string(),
                line: 1, // Simplified
                column: 1,
            });
        }

        Ok(comments)
    }

    /// Apply linting rule
    fn apply_rule(
        &self,
        ast: &CSSAST,
        rule: &LintRule,
    ) -> Result<Vec<LintIssue>, AdvancedFeatureError> {
        let mut issues = Vec::new();

        match rule.name.as_str() {
            "no-duplicate-selectors" => {
                issues.extend(self.check_duplicate_selectors(ast)?);
            }
            "no-empty-rules" => {
                issues.extend(self.check_empty_rules(ast)?);
            }
            "no-important" => {
                issues.extend(self.check_important_declarations(ast)?);
            }
            "selector-max-specificity" => {
                issues.extend(self.check_selector_specificity(ast, rule)?);
            }
            _ => {
                // Apply custom rule - simplified implementation
                // Custom rules would need to implement a trait
            }
        }

        Ok(issues)
    }

    /// Check for duplicate selectors
    fn check_duplicate_selectors(
        &self,
        ast: &CSSAST,
    ) -> Result<Vec<LintIssue>, AdvancedFeatureError> {
        let mut issues = Vec::new();
        let mut selector_map: HashMap<String, Vec<&CSSRule>> = HashMap::new();

        for rule in &ast.rules {
            selector_map
                .entry(rule.selector.clone())
                .or_insert_with(Vec::new)
                .push(rule);
        }

        for (selector, rules) in selector_map {
            if rules.len() > 1 {
                for (i, rule) in rules.iter().enumerate() {
                    if i > 0 {
                        issues.push(LintIssue {
                            rule: "no-duplicate-selectors".to_string(),
                            severity: SeverityLevel::Warning,
                            message: format!("Duplicate selector '{}'", selector),
                            line: rule.line,
                            column: rule.column,
                            end_line: None,
                            end_column: None,
                            fix: Some(LintFix {
                                rule: "no-duplicate-selectors".to_string(),
                                message: "Remove duplicate selector".to_string(),
                                fix_type: FixType::Delete,
                                replacement: String::new(),
                                range: TextRange::new(0, 0),
                            }),
                        });
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Check for empty rules
    fn check_empty_rules(&self, ast: &CSSAST) -> Result<Vec<LintIssue>, AdvancedFeatureError> {
        let mut issues = Vec::new();

        for rule in &ast.rules {
            if rule.properties.is_empty() {
                issues.push(LintIssue {
                    rule: "no-empty-rules".to_string(),
                    severity: SeverityLevel::Warning,
                    message: format!("Empty rule '{}'", rule.selector),
                    line: rule.line,
                    column: rule.column,
                    end_line: None,
                    end_column: None,
                    fix: Some(LintFix {
                        rule: "no-empty-rules".to_string(),
                        message: "Remove empty rule".to_string(),
                        fix_type: FixType::Delete,
                        replacement: String::new(),
                        range: TextRange::new(0, 0),
                    }),
                });
            }
        }

        Ok(issues)
    }

    /// Check for important declarations
    fn check_important_declarations(
        &self,
        ast: &CSSAST,
    ) -> Result<Vec<LintIssue>, AdvancedFeatureError> {
        let mut issues = Vec::new();

        for rule in &ast.rules {
            for property in &rule.properties {
                if property.important {
                    issues.push(LintIssue {
                        rule: "no-important".to_string(),
                        severity: SeverityLevel::Warning,
                        message: format!("Avoid using !important in '{}'", property.name),
                        line: rule.line,
                        column: rule.column,
                        end_line: None,
                        end_column: None,
                        fix: Some(LintFix {
                            rule: "no-important".to_string(),
                            message: "Remove !important".to_string(),
                            fix_type: FixType::Replace,
                            replacement: property
                                .value
                                .replace("!important", "")
                                .trim()
                                .to_string(),
                            range: TextRange::new(0, 0),
                        }),
                    });
                }
            }
        }

        Ok(issues)
    }

    /// Check selector specificity
    fn check_selector_specificity(
        &self,
        ast: &CSSAST,
        rule: &LintRule,
    ) -> Result<Vec<LintIssue>, AdvancedFeatureError> {
        let mut issues = Vec::new();
        let max_specificity = rule
            .options
            .get("max")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as usize;

        for rule in &ast.rules {
            let specificity = self.calculate_specificity(&rule.selector);
            if specificity > max_specificity {
                issues.push(LintIssue {
                    rule: "selector-max-specificity".to_string(),
                    severity: SeverityLevel::Warning,
                    message: format!(
                        "Selector '{}' has specificity {} (max: {})",
                        rule.selector, specificity, max_specificity
                    ),
                    line: rule.line,
                    column: rule.column,
                    end_line: None,
                    end_column: None,
                    fix: None,
                });
            }
        }

        Ok(issues)
    }

    /// Calculate selector specificity
    fn calculate_specificity(&self, selector: &str) -> usize {
        let mut specificity = 0;

        // Count IDs
        specificity += selector.matches('#').count() * 100;

        // Count classes and attributes
        specificity += selector.matches('.').count() * 10;
        specificity += selector.matches('[').count() * 10;

        // Count elements
        specificity += selector.split_whitespace().count();

        specificity
    }

    /// Generate fixes for issues
    fn generate_fixes(&self, issues: &[LintIssue]) -> Result<Vec<LintFix>, AdvancedFeatureError> {
        let mut fixes = Vec::new();

        for issue in issues {
            if let Some(fix) = &issue.fix {
                fixes.push(fix.clone());
            }
        }

        Ok(fixes)
    }

    /// Generate suggestions
    fn generate_suggestions(
        &self,
        ast: &CSSAST,
    ) -> Result<Vec<LintSuggestion>, AdvancedFeatureError> {
        let mut suggestions = Vec::new();

        // Generate suggestions for optimization
        for rule in &ast.rules {
            if rule.properties.len() > 10 {
                suggestions.push(LintSuggestion {
                    rule: "rule-too-long".to_string(),
                    message: "Consider splitting this rule into smaller rules".to_string(),
                    severity: SeverityLevel::Info,
                    line: rule.line,
                    column: rule.column,
                    fix: None,
                });
            }
        }

        Ok(suggestions)
    }

    /// Calculate linting statistics
    fn calculate_statistics(&self, issues: &[LintIssue]) -> LintStatistics {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;
        let mut fixable_count = 0;

        for issue in issues {
            match issue.severity {
                SeverityLevel::Error => error_count += 1,
                SeverityLevel::Warning => warning_count += 1,
                SeverityLevel::Info => info_count += 1,
                SeverityLevel::Off => {}
            }

            if issue.fix.is_some() {
                fixable_count += 1;
            }
        }

        LintStatistics {
            total_issues: issues.len(),
            error_count,
            warning_count,
            info_count,
            fixable_count,
        }
    }

    /// Apply fix to CSS
    fn apply_fix(&self, css: &str, fix: &LintFix) -> Result<String, AdvancedFeatureError> {
        match fix.fix_type {
            FixType::Replace => Ok(css.replace(&fix.replacement, "")),
            FixType::Insert => Ok(format!("{}{}", css, fix.replacement)),
            FixType::Delete => Ok(css.replace(&fix.replacement, "")),
            FixType::Reorder => {
                // Simplified reordering
                Ok(css.to_string())
            }
        }
    }

    /// Get custom rule
    fn get_custom_rule(&self, rule_name: &str) -> Option<&CustomRule> {
        self.config
            .custom_rules
            .iter()
            .find(|rule| rule.name == rule_name)
    }

    /// Get default linting rules
    fn get_default_rules() -> Vec<LintRule> {
        vec![
            LintRule {
                name: "no-duplicate-selectors".to_string(),
                description: "Disallow duplicate selectors".to_string(),
                severity: SeverityLevel::Warning,
                enabled: true,
                options: HashMap::new(),
            },
            LintRule {
                name: "no-empty-rules".to_string(),
                description: "Disallow empty rules".to_string(),
                severity: SeverityLevel::Warning,
                enabled: true,
                options: HashMap::new(),
            },
            LintRule {
                name: "no-important".to_string(),
                description: "Disallow !important declarations".to_string(),
                severity: SeverityLevel::Warning,
                enabled: true,
                options: HashMap::new(),
            },
            LintRule {
                name: "selector-max-specificity".to_string(),
                description: "Limit selector specificity".to_string(),
                severity: SeverityLevel::Warning,
                enabled: true,
                options: {
                    let mut opts = HashMap::new();
                    opts.insert(
                        "max".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(3)),
                    );
                    opts
                },
            },
        ]
    }
}

/// Lint rule
#[derive(Debug, Clone)]
pub struct LintRule {
    pub name: String,
    pub description: String,
    pub severity: SeverityLevel,
    pub enabled: bool,
    pub options: HashMap<String, serde_json::Value>,
}

/// Lint reporter
pub struct LintReporter;

impl LintReporter {
    pub fn new() -> Self {
        Self
    }
}

/// Lint fixer
pub struct LintFixer;

impl LintFixer {
    pub fn new() -> Self {
        Self
    }
}

/// CSS AST structures
#[derive(Debug, Clone)]
pub struct CSSAST {
    pub rules: Vec<CSSRule>,
    pub comments: Vec<CSSComment>,
}

#[derive(Debug, Clone)]
pub struct CSSRule {
    pub selector: String,
    pub properties: Vec<CSSProperty>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct CSSProperty {
    pub name: String,
    pub value: String,
    pub important: bool,
}

#[derive(Debug, Clone)]
pub struct CSSComment {
    pub content: String,
    pub line: usize,
    pub column: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_linting() {
        let config = LinterConfig::default();
        let linter = CSSLinter::new(config);
        let css = ".test { color: red; } .test { color: blue; }"; // Duplicate selector
        let result = linter.lint_css(css, &LintOptions::default());
        assert!(result.is_ok());

        let lint_result = result.unwrap();
        assert!(!lint_result.issues.is_empty());
        assert!(lint_result
            .issues
            .iter()
            .any(|issue| issue.rule == "no-duplicate-selectors"));
    }

    #[test]
    fn test_empty_rules_linting() {
        let config = LinterConfig::default();
        let linter = CSSLinter::new(config);
        let css = ".empty { }"; // Empty rule
        let result = linter.lint_css(css, &LintOptions::default());
        assert!(result.is_ok());

        let lint_result = result.unwrap();
        assert!(!lint_result.issues.is_empty());
        assert!(lint_result
            .issues
            .iter()
            .any(|issue| issue.rule == "no-empty-rules"));
    }

    #[test]
    fn test_important_linting() {
        let config = LinterConfig::default();
        let linter = CSSLinter::new(config);
        let css = ".test { color: red !important; }"; // Important declaration
        let result = linter.lint_css(css, &LintOptions::default());
        assert!(result.is_ok());

        let lint_result = result.unwrap();
        assert!(!lint_result.issues.is_empty());
        assert!(lint_result
            .issues
            .iter()
            .any(|issue| issue.rule == "no-important"));
    }

    #[test]
    fn test_specificity_linting() {
        let config = LinterConfig::default();
        let linter = CSSLinter::new(config);
        let css = "#id .class .class .class { color: red; }"; // High specificity
        let result = linter.lint_css(css, &LintOptions::default());
        assert!(result.is_ok());

        let lint_result = result.unwrap();
        assert!(!lint_result.issues.is_empty());
        assert!(lint_result
            .issues
            .iter()
            .any(|issue| issue.rule == "selector-max-specificity"));
    }

    #[test]
    fn test_lint_statistics() {
        let config = LinterConfig::default();
        let linter = CSSLinter::new(config);
        let css = ".test { color: red !important; } .empty { }";
        let result = linter.lint_css(css, &LintOptions::default());
        assert!(result.is_ok());

        let lint_result = result.unwrap();
        assert!(lint_result.statistics.total_issues > 0);
        assert!(lint_result.statistics.warning_count > 0);
    }
}
