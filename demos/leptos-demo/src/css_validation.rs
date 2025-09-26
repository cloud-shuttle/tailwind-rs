use std::fs;
use std::path::Path;
use std::collections::HashMap;

/// Comprehensive CSS validation system for Tailwind-RS v0.15.0
pub struct CssValidator {
    expected_classes: Vec<String>,
    generated_css: String,
    validation_results: HashMap<String, ValidationResult>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub class: String,
    pub found: bool,
    pub css_rule: Option<String>,
    pub error_message: Option<String>,
}

impl CssValidator {
    pub fn new() -> Self {
        Self {
            expected_classes: Vec::new(),
            generated_css: String::new(),
            validation_results: HashMap::new(),
        }
    }

    /// Load CSS from file and validate it
    pub fn validate_css_file(&mut self, file_path: &str) -> Result<ValidationReport, Box<dyn std::error::Error>> {
        // Read the generated CSS file
        self.generated_css = fs::read_to_string(file_path)?;
        
        // Get expected classes for v0.15.0
        self.expected_classes = self.get_expected_v0_15_0_classes();
        
        // Validate each class
        for class in &self.expected_classes {
            let result = self.validate_class(class);
            self.validation_results.insert(class.clone(), result);
        }
        
        Ok(self.generate_report())
    }

    /// Validate a specific class
    fn validate_class(&self, class: &str) -> ValidationResult {
        // Convert Tailwind class to CSS selector
        let selector = self.class_to_selector(class);
        
        // Check if the CSS rule exists in the generated CSS
        let found = self.generated_css.contains(&selector);
        
        let css_rule = if found {
            self.extract_css_rule(&selector)
        } else {
            None
        };
        
        ValidationResult {
            class: class.to_string(),
            found,
            css_rule,
            error_message: if !found { Some(format!("CSS rule for '{}' not found", class)) } else { None },
        }
    }

    /// Convert Tailwind class to CSS selector
    fn class_to_selector(&self, class: &str) -> String {
        format!(".{}", class)
    }

    /// Extract the full CSS rule for a selector
    fn extract_css_rule(&self, selector: &str) -> Option<String> {
        let lines: Vec<&str> = self.generated_css.lines().collect();
        let mut in_rule = false;
        let mut rule_lines = Vec::new();
        
        for line in lines {
            if line.trim().starts_with(selector) {
                in_rule = true;
                rule_lines.push(line);
            } else if in_rule {
                if line.trim().ends_with('}') {
                    rule_lines.push(line);
                    break;
                } else {
                    rule_lines.push(line);
                }
            }
        }
        
        if !rule_lines.is_empty() {
            Some(rule_lines.join("\n"))
        } else {
            None
        }
    }

    /// Get expected classes for v0.15.0 features
    fn get_expected_v0_15_0_classes(&self) -> Vec<String> {
        vec![
            // Basic Layout
            "block".to_string(), "flex".to_string(), "grid".to_string(), "hidden".to_string(),
            
            // Spacing
            "p-4".to_string(), "m-2".to_string(), "px-4".to_string(), "py-2".to_string(),
            
            // Colors
            "bg-white".to_string(), "bg-gray-100".to_string(), "text-white".to_string(), "text-gray-800".to_string(),
            
            // Typography
            "text-4xl".to_string(), "text-2xl".to_string(), "font-bold".to_string(), "text-center".to_string(),
            
            // Borders & Effects
            "rounded-lg".to_string(), "shadow-lg".to_string(), "border".to_string(),
            
            // Flexbox
            "items-center".to_string(), "justify-center".to_string(), "gap-2".to_string(),
            
            // Sizing
            "w-full".to_string(), "h-screen".to_string(), "min-h-screen".to_string(),
            
            // Interactive
            "hover:bg-blue-600".to_string(), "transition-colors".to_string(),
            
            // Dark mode
            "dark:bg-gray-800".to_string(), "dark:text-white".to_string(),
            
            // Responsive
            "sm:text-lg".to_string(), "md:text-xl".to_string(),
            
            // v0.15.0 New Features - Filter Utilities
            "blur-sm".to_string(), "brightness-50".to_string(), "grayscale".to_string(),
            "backdrop-blur-sm".to_string(), "backdrop-brightness-50".to_string(),
            
            // v0.15.0 New Features - Table Utilities
            "table-auto".to_string(), "border-collapse".to_string(), "caption-top".to_string(),
            
            // v0.15.0 New Features - SVG Utilities
            "fill-none".to_string(), "stroke-current".to_string(), "stroke-width-2".to_string(),
            
            // v0.15.0 New Features - Accessibility
            "forced-color-adjust-auto".to_string(),
            
            // v0.15.0 New Features - Enhanced Transforms
            "transform".to_string(), "scale-75".to_string(), "rotate-45".to_string(),
            "perspective-1000".to_string(), "backface-visible".to_string(),
            
            // v0.15.0 New Features - Touch Actions
            "touch-auto".to_string(), "touch-pan-x".to_string(), "touch-pinch-zoom".to_string(),
            
            // v0.15.0 New Features - Enhanced Borders
            "rounded-t".to_string(), "rounded-tl".to_string(), "rounded-tl-lg".to_string(),
            
            // v0.15.0 New Features - Background Utilities
            "bg-gradient-to-r".to_string(), "bg-size-cover".to_string(), "bg-position-center".to_string(),
        ]
    }

    /// Generate comprehensive validation report
    fn generate_report(&self) -> ValidationReport {
        let total_classes = self.expected_classes.len();
        let found_classes = self.validation_results.values().filter(|r| r.found).count();
        let missing_classes = total_classes - found_classes;
        let success_rate = (found_classes as f32 / total_classes as f32) * 100.0;
        
        let missing_classes_list: Vec<String> = self.validation_results
            .values()
            .filter(|r| !r.found)
            .map(|r| r.class.clone())
            .collect();
        
        ValidationReport {
            total_classes,
            found_classes,
            missing_classes,
            success_rate,
            missing_classes_list,
            validation_results: self.validation_results.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub total_classes: usize,
    pub found_classes: usize,
    pub missing_classes: usize,
    pub success_rate: f32,
    pub missing_classes_list: Vec<String>,
    pub validation_results: HashMap<String, ValidationResult>,
}

impl ValidationReport {
    /// Print a detailed validation report
    pub fn print_report(&self) {
        println!("🎨 Tailwind-RS v0.15.0 CSS Validation Report");
        println!("{}", "=".repeat(50));
        println!("📊 Overall Statistics:");
        println!("   Total Classes: {}", self.total_classes);
        println!("   Found Classes: {}", self.found_classes);
        println!("   Missing Classes: {}", self.missing_classes);
        println!("   Success Rate: {:.1}%", self.success_rate);
        println!();
        
        if self.success_rate >= 95.0 {
            println!("✅ EXCELLENT: CSS generation is working properly!");
        } else if self.success_rate >= 80.0 {
            println!("⚠️  GOOD: CSS generation is mostly working with minor issues");
        } else if self.success_rate >= 60.0 {
            println!("🔶 FAIR: CSS generation has some issues that need attention");
        } else {
            println!("❌ POOR: CSS generation has significant issues");
        }
        
        if !self.missing_classes_list.is_empty() {
            println!();
            println!("❌ Missing Classes:");
            for class in &self.missing_classes_list {
                println!("   - {}", class);
            }
        }
        
        println!();
        println!("🔍 Detailed Results:");
        for (class, result) in &self.validation_results {
            let status = if result.found { "✅" } else { "❌" };
            println!("   {} {}", status, class);
            if let Some(rule) = &result.css_rule {
                println!("      CSS: {}", rule.lines().next().unwrap_or(""));
            }
        }
    }
    
    /// Check if CSS generation is working properly
    pub fn is_working(&self) -> bool {
        self.success_rate >= 80.0
    }
    
    /// Get CSS generation status
    pub fn get_status(&self) -> &'static str {
        if self.success_rate >= 95.0 {
            "EXCELLENT"
        } else if self.success_rate >= 80.0 {
            "GOOD"
        } else if self.success_rate >= 60.0 {
            "FAIR"
        } else {
            "POOR"
        }
    }
}

impl Default for CssValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick validation function for testing
pub fn quick_validate_css(file_path: &str) -> Result<ValidationReport, Box<dyn std::error::Error>> {
    let mut validator = CssValidator::new();
    validator.validate_css_file(file_path)
}

/// Comprehensive validation function that tests all aspects
pub fn comprehensive_validate_css_generation() -> Result<ValidationReport, Box<dyn std::error::Error>> {
    println!("🔍 Starting comprehensive CSS validation...");
    
    // Test all three CSS files
    let files = ["comprehensive-styles.css", "custom-styles.css", "generated-styles.css"];
    let mut overall_report = ValidationReport {
        total_classes: 0,
        found_classes: 0,
        missing_classes: 0,
        success_rate: 0.0,
        missing_classes_list: Vec::new(),
        validation_results: HashMap::new(),
    };
    
    for file in &files {
        if Path::new(file).exists() {
            println!("📄 Validating {}...", file);
            let mut validator = CssValidator::new();
            let report = validator.validate_css_file(file)?;
            
            println!("   Status: {} ({:.1}%)", report.get_status(), report.success_rate);
            
            // Aggregate results
            overall_report.total_classes += report.total_classes;
            overall_report.found_classes += report.found_classes;
            overall_report.missing_classes += report.missing_classes;
        } else {
            println!("⚠️  File {} not found", file);
        }
    }
    
    // Calculate overall success rate
    if overall_report.total_classes > 0 {
        overall_report.success_rate = (overall_report.found_classes as f32 / overall_report.total_classes as f32) * 100.0;
    }
    
    println!();
    println!("📊 Overall CSS Generation Status:");
    println!("   Total Classes Tested: {}", overall_report.total_classes);
    println!("   Successfully Generated: {}", overall_report.found_classes);
    println!("   Success Rate: {:.1}%", overall_report.success_rate);
    
    if overall_report.is_working() {
        println!("✅ CSS generation is working properly!");
    } else {
        println!("❌ CSS generation needs attention!");
    }
    
    Ok(overall_report)
}
