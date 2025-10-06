//! CSS Processing Engine
//!
//! Handles CSS AST processing, Tailwind directive expansion,
//! and integration with the Tailwind-RS core generator.

use crate::{
    CssAst, CssDeclaration, CssRule, AtRule, PostCssConfig, PostCssError, Result,
};
use tailwind_rs_core::CssGenerator;
use std::collections::HashSet;

/// CSS processing engine for PostCSS integration
pub struct CssProcessor {
    generator: CssGenerator,
}

impl CssProcessor {
    /// Create a new CSS processor
    pub fn new() -> Result<Self> {
        let generator = CssGenerator::new();

        Ok(Self { generator })
    }

    /// Process CSS AST with extracted classes
    pub async fn process_ast(
        &self,
        ast: &mut CssAst,
        classes: &HashSet<String>,
        config: &PostCssConfig,
    ) -> Result<()> {
        // Process @tailwind directives
        self.process_tailwind_directives(ast, classes, config).await?;

        // Apply any additional processing
        self.apply_optimizations(ast, config)?;

        Ok(())
    }

    /// Process CSS string with extracted classes
    pub async fn process_css(
        &self,
        ast: &CssAst,
        classes: &HashSet<String>,
        config: &PostCssConfig,
    ) -> Result<String> {
        let mut processed_ast = ast.clone();
        self.process_ast(&mut processed_ast, classes, config).await?;
        Ok(crate::utils::css_ast_to_string(&processed_ast))
    }

    /// Process @tailwind directives and expand them
    async fn process_tailwind_directives(
        &self,
        ast: &mut CssAst,
        classes: &HashSet<String>,
        _config: &PostCssConfig,
    ) -> Result<()> {
        let mut new_rules = Vec::new();

        for at_rule in &mut ast.at_rules {
            if at_rule.name == "tailwind" {
                match at_rule.params.as_str() {
                    "base" => {
                        // Add base styles
                        new_rules.extend(self.generate_base_styles());
                    }
                    "components" => {
                        // Components layer (currently empty)
                        // In future, this could include component classes
                    }
                    "utilities" => {
                        // Generate utilities from found classes
                        let utilities = self.generate_utilities(classes).await?;
                        new_rules.extend(utilities);
                    }
                    _ => {
                        // Unknown @tailwind directive, leave as-is
                        continue;
                    }
                }

                // Mark this at-rule for removal by clearing it
                at_rule.rules.clear();
            }
        }

        // Remove processed @tailwind directives and add generated rules
        ast.at_rules.retain(|rule| !rule.rules.is_empty() || rule.name != "tailwind");
        ast.rules.extend(new_rules);

        Ok(())
    }

    /// Generate base styles for @tailwind base
    fn generate_base_styles(&self) -> Vec<CssRule> {
        vec![
            CssRule {
                selectors: vec!["*, ::before, ::after".to_string()],
                declarations: vec![
                    CssDeclaration {
                        property: "box-sizing".to_string(),
                        value: "border-box".to_string(),
                        important: false,
                    }
                ],
            },
            CssRule {
                selectors: vec!["html".to_string()],
                declarations: vec![
                    CssDeclaration {
                        property: "line-height".to_string(),
                        value: "1.15".to_string(),
                        important: false,
                    },
                    CssDeclaration {
                        property: "-webkit-text-size-adjust".to_string(),
                        value: "100%".to_string(),
                        important: false,
                    }
                ],
            },
            CssRule {
                selectors: vec!["body".to_string()],
                declarations: vec![
                    CssDeclaration {
                        property: "margin".to_string(),
                        value: "0".to_string(),
                        important: false,
                    }
                ],
            }
        ]
    }

    /// Generate utility classes from found classes
    async fn generate_utilities(&self, classes: &HashSet<String>) -> Result<Vec<CssRule>> {
        let mut rules = Vec::new();

        for class in classes {
            // Use the core generator to create CSS rules
            if let Ok(css_rule) = self.generator.generate_individual_css_rule(class) {
                // Convert from core CssRule to PostCSS CssRule
                let postcss_rule = CssRule {
                    selectors: vec![css_rule.selector],
                    declarations: css_rule.properties.into_iter().map(|prop| {
                        CssDeclaration {
                            property: prop.name,
                            value: prop.value,
                            important: prop.important,
                        }
                    }).collect(),
                };

                rules.push(postcss_rule);
            }
        }

        // Sort rules for consistent output
        rules.sort_by(|a, b| a.selectors.cmp(&b.selectors));

        Ok(rules)
    }

    /// Apply optimizations to the CSS AST
    fn apply_optimizations(&self, ast: &mut CssAst, config: &PostCssConfig) -> Result<()> {
        if config.minify {
            self.minify_css(ast)?;
        }

        // Additional optimizations could be added here:
        // - Deduplication
        // - Selector optimization
        // - Unused rule removal

        Ok(())
    }

    /// Minify CSS output
    fn minify_css(&self, ast: &mut CssAst) -> Result<()> {
        // Basic minification - remove unnecessary whitespace
        // In a full implementation, this would be much more sophisticated

        for rule in &mut ast.rules {
            // Remove extra spaces from declarations
            for decl in &mut rule.declarations {
                decl.property = decl.property.trim().to_string();
                decl.value = decl.value.trim().to_string();
            }
        }

        Ok(())
    }

    /// Get statistics about the processing
    pub fn get_stats(&self) -> ProcessingStats {
        ProcessingStats {
            rules_processed: 0, // Would track in real implementation
            classes_found: 0,
            utilities_generated: 0,
            processing_time_ms: 0,
        }
    }
}

/// Processing statistics
#[derive(Debug, Clone)]
pub struct ProcessingStats {
    pub rules_processed: usize,
    pub classes_found: usize,
    pub utilities_generated: usize,
    pub processing_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_base_styles_generation() {
        let processor = CssProcessor::new().unwrap();
        let base_styles = processor.generate_base_styles();

        assert!(!base_styles.is_empty());
        assert!(base_styles.iter().any(|rule| rule.selectors.contains(&"*, ::before, ::after".to_string())));
    }

    #[tokio::test]
    async fn test_utility_generation() {
        let processor = CssProcessor::new().unwrap();

        let classes = ["bg-blue-500", "text-center", "flex"]
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();

        let utilities = processor.generate_utilities(&classes).await.unwrap();

        // Should generate some utilities (exact number depends on what classes are valid)
        assert!(!utilities.is_empty());
    }

    #[tokio::test]
    async fn test_css_minification() {
        let processor = CssProcessor::new().unwrap();

        let mut ast = CssAst {
            rules: vec![
                CssRule {
                    selectors: vec![".test".to_string()],
                    declarations: vec![
                        CssDeclaration {
                            property: " color ".to_string(),
                            value: " red ".to_string(),
                            important: false,
                        }
                    ],
                }
            ],
            at_rules: vec![],
        };

        processor.minify_css(&mut ast).unwrap();

        assert_eq!(ast.rules[0].declarations[0].property, "color");
        assert_eq!(ast.rules[0].declarations[0].value, "red");
    }
}
