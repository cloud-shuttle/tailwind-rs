//! PostCSS Plugin Implementation
//!
//! Main plugin interface that integrates with PostCSS build pipelines.

use crate::{
    CssAst, CssDeclaration, CssGeneratorFn, CssRule, ContentExtractorFn, PostCssConfig, PostCssError, PostCssOptions, Result,
};
use std::sync::Arc;

/// Main Tailwind-RS PostCSS plugin
pub struct TailwindRsPlugin {
    config: PostCssConfig,
    css_generator: CssGeneratorFn,
    content_extractor: ContentExtractorFn,
}

impl TailwindRsPlugin {
    /// Create a new plugin instance with callbacks
    pub fn new_with_callbacks(
        options: PostCssOptions,
        css_generator: CssGeneratorFn,
        content_extractor: ContentExtractorFn,
    ) -> Result<Self> {
        let config = PostCssConfig::from_options(options)?;

        Ok(Self {
            config,
            css_generator,
            content_extractor,
        })
    }

    /// Create a new plugin instance (for backwards compatibility - requires callbacks to be set later)
    pub fn new(options: PostCssOptions) -> Result<Self> {
        Err(PostCssError::Config("Plugin must be created with callbacks. Use new_with_callbacks()".to_string()))
    }

    /// Process CSS input and return processed output
    pub async fn process(&self, input_css: &str) -> Result<String> {
        // Parse input CSS into AST
        let mut ast = crate::utils::parse_css_to_ast(input_css)?;

        // Extract content from configured file patterns
        // For now, we'll create a simple content extraction
        // In practice, this would be handled by the build tool
        let content_files = self.get_content_files().await?;

        // Find Tailwind classes in content using callback
        let classes = (self.content_extractor)(&content_files)?;

        // Process CSS with found classes
        self.process_css_with_classes(&mut ast, &classes)?;

        Ok(crate::utils::css_ast_to_string(&ast))
    }

    /// Process CSS AST directly (for advanced usage)
    pub async fn process_ast(&self, ast: &mut CssAst) -> Result<()> {
        // Extract content from configured file patterns
        let content_files = self.get_content_files().await?;

        // Find Tailwind classes in content using callback
        let classes = (self.content_extractor)(&content_files)?;

        // Process AST with found classes
        self.process_css_with_classes(ast, &classes)?;

        Ok(())
    }

    /// Get plugin configuration
    pub fn config(&self) -> &PostCssConfig {
        &self.config
    }

    /// Update plugin configuration
    pub fn update_config(&mut self, options: PostCssOptions) -> Result<()> {
        self.config = PostCssConfig::from_options(options)?;
        Ok(())
    }

    /// Get content files from configured patterns (simplified implementation)
    async fn get_content_files(&self) -> Result<Vec<String>> {
        // In a real implementation, this would scan the filesystem
        // For now, return empty vec - content would be passed by build tool
        Ok(Vec::new())
    }

    /// Process CSS with found classes
    fn process_css_with_classes(&self, ast: &mut CssAst, classes: &std::collections::HashSet<String>) -> Result<()> {
        // Process @tailwind directives and expand them
        self.process_tailwind_directives(ast, classes)?;

        Ok(())
    }

    /// Process @tailwind directives and expand them
    fn process_tailwind_directives(&self, ast: &mut CssAst, classes: &std::collections::HashSet<String>) -> Result<()> {
        let mut new_rules = Vec::new();

        // Collect tailwind directives to process
        let tailwind_directives: Vec<String> = ast.at_rules.iter()
            .filter(|rule| rule.name == "tailwind")
            .map(|rule| rule.params.clone())
            .collect();

        // Process each directive
        for params in tailwind_directives {
            match params.as_str() {
                "base" => {
                    // Add base styles
                    new_rules.extend(self.generate_base_styles());
                }
                "components" => {
                    // Components layer (currently empty)
                }
                "utilities" => {
                    // Generate utilities from found classes
                    for class in classes {
                        if let Ok(css) = (self.css_generator)(class) {
                            // Parse the generated CSS and add to rules
                            // This is a simplified implementation
                            if let Ok(css_ast) = crate::utils::parse_css_to_ast(&css) {
                                new_rules.extend(css_ast.rules);
                            }
                        }
                    }
                }
                _ => {
                    // Unknown @tailwind directive, leave as-is
                }
            }
        }

        // Remove all @tailwind directives and add generated rules
        ast.at_rules.retain(|rule| rule.name != "tailwind");
        ast.rules.extend(new_rules);

        Ok(())
    }

    /// Generate base styles for @tailwind base
    fn generate_base_styles(&self) -> Vec<CssRule> {
        use crate::CssDeclaration;

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
}

// Note: No Default implementation since plugin requires callbacks

/// PostCSS plugin trait for JavaScript integration
pub trait PostCssPlugin {
    fn process(&self, css: &str, options: PostCssOptions) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + '_>>;
}

impl PostCssPlugin for TailwindRsPlugin {
    fn process(&self, css: &str, _options: PostCssOptions) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + '_>> {
        let css = css.to_string();
        let plugin = self.clone();

        Box::pin(async move {
            plugin.process(&css).await
        })
    }
}

impl Clone for TailwindRsPlugin {
    fn clone(&self) -> Self {
        // Cloning callbacks is not straightforward, so we return an error for now
        // In practice, plugins should be created fresh with callbacks
        panic!("TailwindRsPlugin cannot be cloned. Create new instances with callbacks.");
    }
}

#[cfg(feature = "js-bridge")]
mod js_bridge {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl TailwindRsPlugin {
        /// Create plugin from JavaScript options
        #[wasm_bindgen(constructor)]
        pub fn from_js_options(options: &JsValue) -> Result<TailwindRsPlugin, JsValue> {
            let options: PostCssOptions = options.into_serde()
                .map_err(|e| JsValue::from_str(&format!("Failed to parse options: {}", e)))?;

            Self::new(options)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        /// Process CSS from JavaScript
        #[wasm_bindgen]
        pub async fn process_js(&self, css: &str) -> Result<String, JsValue> {
            self.process(css)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    #[wasm_bindgen]
    pub fn tailwind_rs_plugin(options: &JsValue) -> Result<TailwindRsPlugin, JsValue> {
        TailwindRsPlugin::from_js_options(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_css_generator(class: &str) -> Result<String> {
        // Simple mock that returns CSS for known classes
        match class {
            "bg-blue-500" => Ok(".bg-blue-500 { background-color: rgb(59, 130, 246); }".to_string()),
            "text-center" => Ok(".text-center { text-align: center; }".to_string()),
            _ => Ok(format!(".{} {{ /* generated */ }}", class)),
        }
    }

    fn mock_content_extractor(_content: &[String]) -> Result<std::collections::HashSet<String>> {
        // Return some mock classes
        let mut classes = std::collections::HashSet::new();
        classes.insert("bg-blue-500".to_string());
        classes.insert("text-center".to_string());
        Ok(classes)
    }

    #[tokio::test]
    async fn test_plugin_creation_with_callbacks() {
        let options = PostCssOptions::default();
        let css_gen: CssGeneratorFn = Box::new(mock_css_generator);
        let content_ext: ContentExtractorFn = Box::new(mock_content_extractor);

        let plugin = TailwindRsPlugin::new_with_callbacks(options, css_gen, content_ext)
            .expect("Failed to create plugin");

        assert!(!plugin.config().content.is_empty());
    }

    #[tokio::test]
    async fn test_basic_css_processing() {
        let options = PostCssOptions::default();
        let css_gen: CssGeneratorFn = Box::new(mock_css_generator);
        let content_ext: ContentExtractorFn = Box::new(mock_content_extractor);

        let plugin = TailwindRsPlugin::new_with_callbacks(options, css_gen, content_ext)
            .expect("Failed to create plugin");

        let input_css = r#"
        @tailwind base;
        @tailwind utilities;
        "#;

        let result = plugin.process(input_css).await;
        assert!(result.is_ok());

        let output = result.unwrap();
        // Should contain base styles
        assert!(output.contains("box-sizing"));
        assert!(output.contains("margin: 0"));
    }

    #[test]
    fn test_base_styles_generation() {
        let options = PostCssOptions::default();
        let css_gen: CssGeneratorFn = Box::new(mock_css_generator);
        let content_ext: ContentExtractorFn = Box::new(mock_content_extractor);

        let plugin = TailwindRsPlugin::new_with_callbacks(options, css_gen, content_ext)
            .expect("Failed to create plugin");

        let base_styles = plugin.generate_base_styles();
        assert!(!base_styles.is_empty());
        assert!(base_styles.iter().any(|rule| rule.selectors.contains(&"*, ::before, ::after".to_string())));
    }
}
