//! JIT Compilation System for Tailwind-RS
//!
//! This module provides Just-In-Time compilation for CSS generation:
//! - Pattern recognition and optimization
//! - Compiled CSS templates for repeated patterns
//! - Runtime optimization based on usage patterns
//! - Precompiled common class combinations

use crate::css_generator::types::CssProperty;
use crate::error::{Result, TailwindError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// JIT-compiled CSS template
#[derive(Debug, Clone)]
pub struct CssTemplate {
    /// Template ID for caching
    pub id: String,
    /// Input class pattern that this template matches
    pub pattern: Vec<String>,
    /// Pre-compiled CSS output (simplified as properties)
    pub css_output: Vec<CssProperty>,
    /// Usage statistics
    pub stats: TemplateStats,
    /// Compilation timestamp
    pub compiled_at: Instant,
}

#[derive(Debug, Clone, Default)]
pub struct TemplateStats {
    pub usage_count: u64,
    pub total_compile_time: Duration,
    pub average_generation_time: Duration,
    pub memory_usage: usize,
}

/// JIT Compiler for CSS generation
#[derive(Debug)]
pub struct JitCompiler {
    /// Compiled templates cache
    templates: Mutex<HashMap<String, CssTemplate>>,
    /// Pattern recognition engine
    pattern_engine: PatternEngine,
    /// Compilation statistics
    stats: Mutex<CompilerStats>,
    /// Maximum templates to cache
    max_templates: usize,
}

#[derive(Debug, Clone, Default)]
pub struct CompilerStats {
    pub templates_compiled: usize,
    pub templates_used: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub total_compile_time: Duration,
    pub average_compile_time: Duration,
}

impl JitCompiler {
    /// Create a new JIT compiler
    pub fn new(max_templates: usize) -> Self {
        Self {
            templates: Mutex::new(HashMap::new()),
            pattern_engine: PatternEngine::new(),
            stats: Mutex::new(CompilerStats::default()),
            max_templates,
        }
    }

    /// Compile CSS for a class pattern
    pub fn compile_pattern(&self, classes: &[String]) -> Result<CssTemplate> {
        let start_time = Instant::now();
        let pattern_id = self.generate_pattern_id(classes);

        // Check if already compiled
        if let Some(template) = self.get_template(&pattern_id) {
            let mut stats = self.stats.lock().unwrap();
            stats.cache_hits += 1;
            return Ok(template);
        }

        // Compile new template
        let css_output = self.compile_css_from_classes(classes)?;
        let compile_time = start_time.elapsed();

        let template = CssTemplate {
            id: pattern_id.clone(),
            pattern: classes.to_vec(),
            css_output,
            stats: TemplateStats {
                usage_count: 1,
                total_compile_time: compile_time,
                average_generation_time: Duration::from_nanos(0),
                memory_usage: self.estimate_memory_usage(&css_output),
            },
            compiled_at: Instant::now(),
        };

        // Store template
        self.store_template(template.clone());

        let mut stats = self.stats.lock().unwrap();
        stats.templates_compiled += 1;
        stats.cache_misses += 1;
        stats.total_compile_time += compile_time;

        if stats.templates_compiled > 0 {
            stats.average_compile_time = stats.total_compile_time / stats.templates_compiled as u32;
        }

        Ok(template)
    }

    /// Generate CSS from a compiled template
    pub fn generate_from_template(&self, template_id: &str) -> Option<Vec<CssProperty>> {
        if let Some(mut template) = self.get_template(template_id) {
            // Update usage statistics
            template.stats.usage_count += 1;
            let generation_start = Instant::now();

            // In a real implementation, this might involve additional runtime optimization
            let result = template.css_output.clone();

            let generation_time = generation_start.elapsed();
            template.stats.average_generation_time =
                (template.stats.average_generation_time + generation_time) / 2;

            // Update template in cache
            self.update_template(template);

            let mut stats = self.stats.lock().unwrap();
            stats.templates_used += 1;

            Some(result)
        } else {
            None
        }
    }

    /// Optimize based on usage patterns
    pub fn optimize_patterns(&self) {
        let mut templates = self.templates.lock().unwrap();

        // Remove least-used templates if over limit
        if templates.len() > self.max_templates {
            let mut template_vec: Vec<_> = templates.drain().collect();
            template_vec.sort_by(|a, b| b.1.stats.usage_count.cmp(&a.1.stats.usage_count));

            // Keep only the most used templates
            templates.clear();
            for (id, template) in template_vec.into_iter().take(self.max_templates) {
                templates.insert(id, template);
            }
        }

        // Identify frequently co-occurring patterns for optimization
        self.pattern_engine.analyze_patterns(&templates);
    }

    /// Get compiler statistics
    pub fn stats(&self) -> CompilerStats {
        self.stats.lock().unwrap().clone()
    }

    /// Generate a unique ID for a class pattern
    fn generate_pattern_id(&self, classes: &[String]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        classes.hash(&mut hasher);
        format!("pattern_{}", hasher.finish())
    }

    /// Compile CSS from class names (simplified implementation)
    fn compile_css_from_classes(&self, classes: &[String]) -> Result<Vec<CssProperty>> {
        // This is a placeholder - in a real implementation, this would
        // use the actual CSS generator to compile the classes
        let mut properties = Vec::new();

        for class in classes {
            // Create a simple property for demonstration
            properties.push(CssProperty {
                name: "/* compiled */".to_string(),
                value: format!("/* {} */", class),
                important: false,
            });
        }

        Ok(properties)
    }

    /// Estimate memory usage of CSS properties
    fn estimate_memory_usage(&self, properties: &[CssProperty]) -> usize {
        properties.iter().map(|prop| {
            prop.name.len() + prop.value.len() + std::mem::size_of::<CssProperty>()
        }).sum()
    }

    /// Get a template from cache
    fn get_template(&self, id: &str) -> Option<CssTemplate> {
        self.templates.lock().unwrap().get(id).cloned()
    }

    /// Store a template in cache
    fn store_template(&self, template: CssTemplate) {
        let mut templates = self.templates.lock().unwrap();
        templates.insert(template.id.clone(), template);
    }

    /// Update a template in cache
    fn update_template(&self, template: CssTemplate) {
        let mut templates = self.templates.lock().unwrap();
        templates.insert(template.id.clone(), template);
    }
}

/// Pattern recognition engine for optimization
#[derive(Debug)]
pub struct PatternEngine {
    /// Recognized patterns
    patterns: Mutex<HashMap<String, PatternInfo>>,
}

#[derive(Debug, Clone)]
pub struct PatternInfo {
    pub frequency: usize,
    pub co_occurring_classes: Vec<String>,
    pub optimization_potential: f64,
}

impl PatternEngine {
    pub fn new() -> Self {
        Self {
            patterns: Mutex::new(HashMap::new()),
        }
    }

    /// Analyze patterns for optimization opportunities
    pub fn analyze_patterns(&self, templates: &HashMap<String, CssTemplate>) {
        let mut patterns = self.patterns.lock().unwrap();

        // Simple co-occurrence analysis
        for template in templates.values() {
            for class in &template.pattern {
                let entry = patterns.entry(class.clone()).or_insert(PatternInfo {
                    frequency: 0,
                    co_occurring_classes: Vec::new(),
                    optimization_potential: 0.0,
                });

                entry.frequency += 1;

                // Add co-occurring classes
                for other_class in &template.pattern {
                    if other_class != class && !entry.co_occurring_classes.contains(other_class) {
                        entry.co_occurring_classes.push(other_class.clone());
                    }
                }
            }
        }

        // Calculate optimization potential
        for pattern in patterns.values_mut() {
            pattern.optimization_potential = pattern.frequency as f64 * pattern.co_occurring_classes.len() as f64;
        }
    }

    /// Get pattern recommendations
    pub fn get_recommendations(&self) -> Vec<(String, PatternInfo)> {
        let patterns = self.patterns.lock().unwrap();
        let mut recommendations: Vec<_> = patterns.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        recommendations.sort_by(|a, b| b.1.optimization_potential.partial_cmp(&a.1.optimization_potential).unwrap());
        recommendations
    }
}

/// Precompiled CSS for common patterns
#[derive(Debug)]
pub struct PrecompiledCss {
    /// Precompiled button styles
    pub buttons: HashMap<String, Vec<CssProperty>>,
    /// Precompiled layout patterns
    pub layouts: HashMap<String, Vec<CssProperty>>,
    /// Precompiled form styles
    pub forms: HashMap<String, Vec<CssProperty>>,
}

impl PrecompiledCss {
    /// Create precompiled CSS with common patterns
    pub fn new() -> Self {
        let mut buttons = HashMap::new();
        let mut layouts = HashMap::new();
        let mut forms = HashMap::new();

        // Precompile common button patterns
        buttons.insert("primary".to_string(), Self::compile_button_primary());
        buttons.insert("secondary".to_string(), Self::compile_button_secondary());
        buttons.insert("outline".to_string(), Self::compile_button_outline());

        // Precompile common layout patterns
        layouts.insert("container".to_string(), Self::compile_container());
        layouts.insert("flex-center".to_string(), Self::compile_flex_center());
        layouts.insert("grid-responsive".to_string(), Self::compile_grid_responsive());

        // Precompile common form patterns
        forms.insert("input".to_string(), Self::compile_input());
        forms.insert("textarea".to_string(), Self::compile_textarea());
        forms.insert("select".to_string(), Self::compile_select());

        Self {
            buttons,
            layouts,
            forms,
        }
    }

    /// Get precompiled CSS for a pattern
    pub fn get(&self, category: &str, pattern: &str) -> Option<&Vec<CssProperty>> {
        match category {
            "button" => self.buttons.get(pattern),
            "layout" => self.layouts.get(pattern),
            "form" => self.forms.get(pattern),
            _ => None,
        }
    }

    // Precompiled pattern implementations (simplified)
    fn compile_button_primary() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "background-color".to_string(), value: "#3b82f6".to_string(), important: false },
            CssProperty { name: "color".to_string(), value: "white".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0.5rem 1rem".to_string(), important: false },
            CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false },
        ]
    }

    fn compile_button_secondary() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "background-color".to_string(), value: "#6b7280".to_string(), important: false },
            CssProperty { name: "color".to_string(), value: "white".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0.5rem 1rem".to_string(), important: false },
            CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false },
        ]
    }

    fn compile_button_outline() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "background-color".to_string(), value: "transparent".to_string(), important: false },
            CssProperty { name: "color".to_string(), value: "#3b82f6".to_string(), important: false },
            CssProperty { name: "border".to_string(), value: "1px solid #3b82f6".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0.5rem 1rem".to_string(), important: false },
            CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false },
        ]
    }

    fn compile_container() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "width".to_string(), value: "100%".to_string(), important: false },
            CssProperty { name: "max-width".to_string(), value: "1200px".to_string(), important: false },
            CssProperty { name: "margin".to_string(), value: "0 auto".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0 1rem".to_string(), important: false },
        ]
    }

    fn compile_flex_center() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false },
            CssProperty { name: "justify-content".to_string(), value: "center".to_string(), important: false },
            CssProperty { name: "align-items".to_string(), value: "center".to_string(), important: false },
        ]
    }

    fn compile_grid_responsive() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "display".to_string(), value: "grid".to_string(), important: false },
            CssProperty { name: "grid-template-columns".to_string(), value: "repeat(auto-fit, minmax(250px, 1fr))".to_string(), important: false },
            CssProperty { name: "gap".to_string(), value: "1rem".to_string(), important: false },
        ]
    }

    fn compile_input() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "width".to_string(), value: "100%".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0.5rem 0.75rem".to_string(), important: false },
            CssProperty { name: "border".to_string(), value: "1px solid #d1d5db".to_string(), important: false },
            CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false },
            CssProperty { name: "font-size".to_string(), value: "0.875rem".to_string(), important: false },
        ]
    }

    fn compile_textarea() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "width".to_string(), value: "100%".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0.5rem 0.75rem".to_string(), important: false },
            CssProperty { name: "border".to_string(), value: "1px solid #d1d5db".to_string(), important: false },
            CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false },
            CssProperty { name: "font-size".to_string(), value: "0.875rem".to_string(), important: false },
            CssProperty { name: "min-height".to_string(), value: "5rem".to_string(), important: false },
            CssProperty { name: "resize".to_string(), value: "vertical".to_string(), important: false },
        ]
    }

    fn compile_select() -> Vec<CssProperty> {
        vec![
            CssProperty { name: "width".to_string(), value: "100%".to_string(), important: false },
            CssProperty { name: "padding".to_string(), value: "0.5rem 0.75rem".to_string(), important: false },
            CssProperty { name: "border".to_string(), value: "1px solid #d1d5db".to_string(), important: false },
            CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false },
            CssProperty { name: "font-size".to_string(), value: "0.875rem".to_string(), important: false },
            CssProperty { name: "background-color".to_string(), value: "white".to_string(), important: false },
            CssProperty { name: "cursor".to_string(), value: "pointer".to_string(), important: false },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_compiler_creation() {
        let compiler = JitCompiler::new(100);
        let stats = compiler.stats();
        assert_eq!(stats.templates_compiled, 0);
        assert_eq!(stats.cache_hits, 0);
    }

    #[test]
    fn test_css_template_creation() {
        let template = CssTemplate {
            id: "test_template".to_string(),
            pattern: vec!["text-red-500".to_string()],
            css_output: CssRules::new(),
            stats: TemplateStats::default(),
            compiled_at: Instant::now(),
        };

        assert_eq!(template.id, "test_template");
        assert_eq!(template.pattern.len(), 1);
        assert_eq!(template.stats.usage_count, 0);
    }

    #[test]
    fn test_pattern_compilation() {
        let compiler = JitCompiler::new(100);
        let classes = vec!["text-red-500".to_string(), "bg-blue-500".to_string()];

        let result = compiler.compile_pattern(&classes);
        assert!(result.is_ok());

        let template = result.unwrap();
        assert_eq!(template.pattern, classes);
        assert!(!template.id.is_empty());

        let stats = compiler.stats();
        assert_eq!(stats.templates_compiled, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[test]
    fn test_template_caching() {
        let compiler = JitCompiler::new(100);
        let classes = vec!["text-red-500".to_string()];

        // First compilation
        let template1 = compiler.compile_pattern(&classes).unwrap();
        let stats1 = compiler.stats();
        assert_eq!(stats1.cache_misses, 1);

        // Second compilation (should hit cache)
        let template2 = compiler.compile_pattern(&classes).unwrap();
        let stats2 = compiler.stats();
        assert_eq!(stats2.cache_hits, 1);
        assert_eq!(template1.id, template2.id);
    }

    #[test]
    fn test_pattern_engine() {
        let engine = PatternEngine::new();

        // Create mock templates
        let mut templates = HashMap::new();
        templates.insert("template1".to_string(), CssTemplate {
            id: "template1".to_string(),
            pattern: vec!["text-red-500".to_string(), "bg-blue-500".to_string()],
            css_output: CssRules::new(),
            stats: TemplateStats { usage_count: 5, ..Default::default() },
            compiled_at: Instant::now(),
        });

        engine.analyze_patterns(&templates);

        let recommendations = engine.get_recommendations();
        assert!(!recommendations.is_empty());
    }

    #[test]
    fn test_precompiled_css() {
        let precompiled = PrecompiledCss::new();

        // Test button patterns
        let primary_button = precompiled.get("button", "primary");
        assert!(primary_button.is_some());

        let secondary_button = precompiled.get("button", "secondary");
        assert!(secondary_button.is_some());

        // Test layout patterns
        let container = precompiled.get("layout", "container");
        assert!(container.is_some());

        let flex_center = precompiled.get("layout", "flex-center");
        assert!(flex_center.is_some());

        // Test form patterns
        let input = precompiled.get("form", "input");
        assert!(input.is_some());

        let textarea = precompiled.get("form", "textarea");
        assert!(textarea.is_some());
    }

    #[test]
    fn test_precompiled_css_content() {
        let precompiled = PrecompiledCss::new();

        if let Some(button) = precompiled.get("button", "primary") {
            assert!(!button.is_empty());
            // Check that it contains expected properties
            let rule = &button[0];
            assert!(rule.properties.iter().any(|p| p.name == "background-color"));
            assert!(rule.properties.iter().any(|p| p.value.contains("#3b82f6")));
        }
    }

    #[test]
    fn test_template_generation() {
        let compiler = JitCompiler::new(100);
        let classes = vec!["test-class".to_string()];

        let template = compiler.compile_pattern(&classes).unwrap();
        let generated = compiler.generate_from_template(&template.id);

        assert!(generated.is_some());

        let stats = compiler.stats();
        assert_eq!(stats.templates_used, 1);
    }

    #[test]
    fn test_compiler_optimization() {
        let compiler = JitCompiler::new(5); // Small cache

        // Fill cache beyond limit
        for i in 0..10 {
            let classes = vec![format!("class-{}", i)];
            let _ = compiler.compile_pattern(&classes);
        }

        // Compiler should optimize (reduce templates)
        compiler.optimize_patterns();

        // Cache should be limited
        let templates = compiler.templates.lock().unwrap();
        assert!(templates.len() <= 5);
    }
}
