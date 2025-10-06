//! CSS Processing Engine
//!
//! Handles content file scanning, class extraction, and CSS generation
//! using the Tailwind-RS core engine.

use std::collections::HashSet;
use std::path::Path;
use tailwind_rs_core::CssGenerator;
use walkdir::WalkDir;
use glob::Pattern;
use tokio::fs;
use regex::Regex;

/// CSS processing engine
pub struct CssProcessor {
    /// CSS generator instance
    generator: CssGenerator,
    /// Regex for extracting class names from content
    class_regex: Regex,
}

impl CssProcessor {
    /// Create a new CSS processor
    pub fn new() -> Result<Self, CssProcessorError> {
        let generator = CssGenerator::new();

        // Regex to match Tailwind classes in various contexts
        // This captures class attributes in HTML and className in JS/TS
        let class_regex = Regex::new(r#"(?:class|className)=["']([^"']*)["']"#)?;

        Ok(Self {
            generator,
            class_regex,
        })
    }

    /// Scan content files and extract Tailwind classes
    pub async fn scan_content_files(&self, patterns: &[String]) -> Result<HashSet<String>, CssProcessorError> {
        let mut all_classes = HashSet::new();

        for pattern in patterns {
            let files = self.find_files(pattern)?;
            println!("📁 Scanning {} files matching '{}'", files.len(), pattern);

            for file_path in files {
                let classes = self.extract_classes_from_file(&file_path).await?;
                all_classes.extend(classes);
            }
        }

        Ok(all_classes)
    }

    /// Find files matching a glob pattern
    fn find_files(&self, pattern: &str) -> Result<Vec<std::path::PathBuf>, CssProcessorError> {
        let mut files = Vec::new();

        // Handle different pattern types
        if pattern.contains('*') {
            // Glob pattern
            let glob_pattern = Pattern::new(pattern)?;
            for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    let path_str = path.to_string_lossy();
                    if glob_pattern.matches(&path_str) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        } else {
            // Direct file path
            let path = Path::new(pattern);
            if path.exists() && path.is_file() {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// Extract Tailwind classes from a single file
    async fn extract_classes_from_file(&self, file_path: &Path) -> Result<HashSet<String>, CssProcessorError> {
        let content = fs::read_to_string(file_path).await?;
        let mut classes = HashSet::new();

        // Extract classes using regex
        for cap in self.class_regex.captures_iter(&content) {
            if let Some(class_attr) = cap.get(1) {
                let class_list = class_attr.as_str();

                // Split by whitespace and filter out empty strings
                for class in class_list.split_whitespace() {
                    let class = class.trim();
                    if !class.is_empty() {
                        classes.insert(class.to_string());
                    }
                }
            }
        }

        // Also handle dynamic class construction in JS/TS files
        self.extract_dynamic_classes(&content, &mut classes);

        Ok(classes)
    }

    /// Extract dynamically constructed classes (basic implementation)
    fn extract_dynamic_classes(&self, content: &str, classes: &mut HashSet<String>) {
        // Look for common patterns like:
        // `className: \`bg-${color}-${intensity}\``
        // `className: 'bg-' + color + '-' + intensity`

        // Simple template literal extraction
        let template_regex = Regex::new(r#"`([^`]*\$\{[^}]+\}[^`]*)`"#).unwrap();
        for cap in template_regex.captures_iter(content) {
            if let Some(template) = cap.get(1) {
                // For now, just extract static parts around variables
                // A more sophisticated implementation would evaluate the expressions
                let template_str = template.as_str();
                self.extract_static_classes_from_template(template_str, classes);
            }
        }
    }

    /// Extract static class parts from template literals
    fn extract_static_classes_from_template(&self, template: &str, classes: &mut HashSet<String>) {
        // Split on variable interpolation and extract static parts
        for part in template.split("${").flat_map(|s| s.split("}")) {
            for class in part.split_whitespace() {
                let class = class.trim().trim_matches(|c: char| !c.is_alphanumeric() && c != '-');
                if !class.is_empty() && class.contains('-') {
                    classes.insert(class.to_string());
                }
            }
        }
    }

    /// Generate CSS from input CSS and extracted classes
    pub async fn generate_css(&mut self, input_css: &str, classes: &HashSet<String>, minify: bool) -> Result<String, CssProcessorError> {
        let mut css_output = String::new();

        // Add input CSS (with Tailwind directives processed)
        css_output.push_str(&self.process_input_css(input_css, classes)?);

        if minify {
            // Basic minification (remove extra whitespace and newlines)
            css_output = css_output
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("");
        }

        Ok(css_output)
    }

    /// Process input CSS with Tailwind directives
    fn process_input_css(&mut self, input_css: &str, classes: &HashSet<String>) -> Result<String, CssProcessorError> {
        let mut output = String::new();
        let mut in_tailwind_block = false;

        for line in input_css.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("@tailwind") {
                // Process Tailwind directives
                if trimmed == "@tailwind base;" {
                    // Add base styles
                    output.push_str("/* Tailwind Base Styles */\n");
                    output.push_str("*, ::before, ::after {\n  box-sizing: border-box;\n}\n\n");
                } else if trimmed == "@tailwind components;" {
                    // Components layer - currently empty
                    output.push_str("/* Tailwind Components */\n\n");
                } else if trimmed == "@tailwind utilities;" {
                    // Generate utilities from found classes
                    output.push_str("/* Tailwind Utilities */\n");
                    for class in classes {
                        if let Ok(rule) = self.generator.generate_individual_css_rule(class) {
                            // Format the rule as CSS
                            output.push_str(&format!("{} {{\n", rule.selector));
                            for property in &rule.properties {
                                output.push_str(&format!("  {}: {};\n",
                                    property.name,
                                    property.value
                                ));
                            }
                            output.push_str("}\n\n");
                        }
                    }
                }
            } else {
                output.push_str(line);
                output.push('\n');
            }
        }

        Ok(output)
    }
}

/// CSS processor error types
#[derive(Debug, thiserror::Error)]
pub enum CssProcessorError {
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Glob pattern error: {0}")]
    Glob(#[from] glob::PatternError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSS generation error: {0}")]
    CssGeneration(String),

    #[error("File scanning error: {0}")]
    FileScan(String),
}
