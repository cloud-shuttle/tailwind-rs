use std::fs;
use std::path::Path;
use std::process::Command;

/// Comprehensive validation system for Tailwind-RS v0.15.0 integration
pub struct ComprehensiveValidator {
    validation_results: Vec<ValidationStep>,
}

#[derive(Debug, Clone)]
pub struct ValidationStep {
    pub name: String,
    pub status: ValidationStatus,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ValidationStatus {
    Success,
    Warning,
    Error,
    Info,
}

impl ComprehensiveValidator {
    pub fn new() -> Self {
        Self {
            validation_results: Vec::new(),
        }
    }

    /// Run comprehensive validation of the entire CSS generation pipeline
    pub fn validate_all(&mut self) -> Result<ValidationReport, Box<dyn std::error::Error>> {
        println!("🔍 Starting comprehensive Tailwind-RS v0.15.0 validation...");
        
        // Step 1: Validate dependencies
        self.validate_dependencies()?;
        
        // Step 2: Validate CSS generation
        self.validate_css_generation()?;
        
        // Step 3: Validate CSS files
        self.validate_css_files()?;
        
        // Step 4: Validate v0.15.0 features
        self.validate_v0_15_0_features()?;
        
        // Step 5: Validate integration
        self.validate_integration()?;
        
        // Step 6: Validate performance
        self.validate_performance()?;
        
        Ok(self.generate_report())
    }

    fn validate_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_step("Dependencies", ValidationStatus::Info, "Checking Tailwind-RS v0.15.0 dependencies");
        
        // Check if Cargo.toml has correct dependencies
        let cargo_toml = fs::read_to_string("Cargo.toml")?;
        
        let required_deps = [
            "tailwind-rs-core = \"0.15.0\"",
            "tailwind-rs-postcss = \"0.15.0\"",
            "tailwind-rs-scanner = \"0.15.0\"",
        ];
        
        let mut deps_found = 0;
        for dep in &required_deps {
            if cargo_toml.contains(dep) {
                deps_found += 1;
            }
        }
        
        if deps_found == required_deps.len() {
            self.add_step("Dependencies", ValidationStatus::Success, "All v0.15.0 dependencies found");
        } else {
            self.add_step("Dependencies", ValidationStatus::Error, 
                &format!("Only {}/{} dependencies found", deps_found, required_deps.len()));
        }
        
        Ok(())
    }

    fn validate_css_generation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_step("CSS Generation", ValidationStatus::Info, "Testing CSS generation process");
        
        // Try to compile the CSS generator
        let output = Command::new("cargo")
            .args(&["check", "--quiet"])
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    self.add_step("CSS Generation", ValidationStatus::Success, "CSS generator compiles successfully");
                } else {
                    let error = String::from_utf8_lossy(&result.stderr);
                    self.add_step("CSS Generation", ValidationStatus::Error, 
                        &format!("CSS generator compilation failed: {}", error));
                }
            },
            Err(e) => {
                self.add_step("CSS Generation", ValidationStatus::Error, 
                    &format!("Failed to run cargo check: {}", e));
            }
        }
        
        Ok(())
    }

    fn validate_css_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_step("CSS Files", ValidationStatus::Info, "Checking generated CSS files");
        
        let css_files = ["comprehensive-styles.css", "custom-styles.css", "generated-styles.css"];
        let mut files_found = 0;
        let mut total_size = 0;
        
        for file in &css_files {
            if Path::new(file).exists() {
                files_found += 1;
                let metadata = fs::metadata(file)?;
                total_size += metadata.len();
                
                if metadata.len() > 100 {
                    self.add_step("CSS Files", ValidationStatus::Success, 
                        &format!("{} exists and has content ({} bytes)", file, metadata.len()));
                } else {
                    self.add_step("CSS Files", ValidationStatus::Warning, 
                        &format!("{} exists but is very small ({} bytes)", file, metadata.len()));
                }
            } else {
                self.add_step("CSS Files", ValidationStatus::Error, 
                    &format!("{} not found", file));
            }
        }
        
        if files_found == css_files.len() && total_size > 1000 {
            self.add_step("CSS Files", ValidationStatus::Success, 
                &format!("All CSS files generated successfully ({} total bytes)", total_size));
        } else if files_found > 0 {
            self.add_step("CSS Files", ValidationStatus::Warning, 
                &format!("Partial CSS generation: {}/{} files, {} bytes", files_found, css_files.len(), total_size));
        } else {
            self.add_step("CSS Files", ValidationStatus::Error, "No CSS files generated");
        }
        
        Ok(())
    }

    fn validate_v0_15_0_features(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_step("v0.15.0 Features", ValidationStatus::Info, "Checking v0.15.0 specific features");
        
        let v0_15_0_features = [
            // Filter Utilities
            "blur-sm", "brightness-50", "grayscale", "backdrop-blur-sm",
            // Table Utilities
            "table-auto", "border-collapse", "caption-top",
            // SVG Utilities
            "fill-none", "stroke-current", "stroke-width-2",
            // Accessibility
            "forced-color-adjust-auto",
            // Enhanced Transforms
            "transform", "scale-75", "rotate-45", "perspective-1000",
            // Touch Actions
            "touch-auto", "touch-pan-x", "touch-pinch-zoom",
            // Enhanced Borders
            "rounded-t", "rounded-tl", "rounded-tl-lg",
            // Background Utilities
            "bg-gradient-to-r", "bg-size-cover", "bg-position-center",
        ];
        
        let mut features_found = 0;
        let css_files = ["comprehensive-styles.css", "custom-styles.css", "generated-styles.css"];
        
        for file in &css_files {
            if Path::new(file).exists() {
                let content = fs::read_to_string(file)?;
                for feature in &v0_15_0_features {
                    if content.contains(&format!(".{}", feature)) {
                        features_found += 1;
                        self.add_step("v0.15.0 Features", ValidationStatus::Success, 
                            &format!("Found {} in {}", feature, file));
                    }
                }
            }
        }
        
        let feature_percentage = (features_found as f32 / v0_15_0_features.len() as f32) * 100.0;
        
        if feature_percentage >= 80.0 {
            self.add_step("v0.15.0 Features", ValidationStatus::Success, 
                &format!("{}/{} v0.15.0 features found ({:.1}%)", features_found, v0_15_0_features.len(), feature_percentage));
        } else if feature_percentage >= 50.0 {
            self.add_step("v0.15.0 Features", ValidationStatus::Warning, 
                &format!("{}/{} v0.15.0 features found ({:.1}%)", features_found, v0_15_0_features.len(), feature_percentage));
        } else {
            self.add_step("v0.15.0 Features", ValidationStatus::Error, 
                &format!("Only {}/{} v0.15.0 features found ({:.1}%)", features_found, v0_15_0_features.len(), feature_percentage));
        }
        
        Ok(())
    }

    fn validate_integration(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_step("Integration", ValidationStatus::Info, "Validating Leptos integration");
        
        // Check if the main lib.rs uses the correct CSS generator
        let lib_rs = fs::read_to_string("src/lib.rs")?;
        
        if lib_rs.contains("DemoCssGenerator") && lib_rs.contains("generate_all_css_files") {
            self.add_step("Integration", ValidationStatus::Success, "Leptos integration properly configured");
        } else {
            self.add_step("Integration", ValidationStatus::Error, "Leptos integration not properly configured");
        }
        
        // Check if index.html references the generated CSS files
        let index_html = fs::read_to_string("index.html")?;
        
        if index_html.contains("comprehensive-styles.css") && 
           index_html.contains("custom-styles.css") && 
           index_html.contains("generated-styles.css") {
            self.add_step("Integration", ValidationStatus::Success, "HTML properly references generated CSS files");
        } else {
            self.add_step("Integration", ValidationStatus::Warning, "HTML may not properly reference all CSS files");
        }
        
        Ok(())
    }

    fn validate_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_step("Performance", ValidationStatus::Info, "Validating CSS generation performance");
        
        let css_files = ["comprehensive-styles.css", "custom-styles.css", "generated-styles.css"];
        let mut total_size = 0;
        
        for file in &css_files {
            if Path::new(file).exists() {
                let metadata = fs::metadata(file)?;
                total_size += metadata.len();
            }
        }
        
        if total_size > 50000 {
            self.add_step("Performance", ValidationStatus::Success, 
                &format!("Generated substantial CSS ({} bytes)", total_size));
        } else if total_size > 10000 {
            self.add_step("Performance", ValidationStatus::Warning, 
                &format!("Generated moderate CSS ({} bytes)", total_size));
        } else {
            self.add_step("Performance", ValidationStatus::Error, 
                &format!("Generated minimal CSS ({} bytes)", total_size));
        }
        
        Ok(())
    }

    fn add_step(&mut self, name: &str, status: ValidationStatus, message: &str) {
        self.validation_results.push(ValidationStep {
            name: name.to_string(),
            status,
            message: message.to_string(),
            details: None,
        });
    }

    fn generate_report(&self) -> ValidationReport {
        let total_steps = self.validation_results.len();
        let success_steps = self.validation_results.iter().filter(|s| matches!(s.status, ValidationStatus::Success)).count();
        let warning_steps = self.validation_results.iter().filter(|s| matches!(s.status, ValidationStatus::Warning)).count();
        let error_steps = self.validation_results.iter().filter(|s| matches!(s.status, ValidationStatus::Error)).count();
        
        let success_rate = (success_steps as f32 / total_steps as f32) * 100.0;
        
        ValidationReport {
            total_steps,
            success_steps,
            warning_steps,
            error_steps,
            success_rate,
            validation_results: self.validation_results.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub total_steps: usize,
    pub success_steps: usize,
    pub warning_steps: usize,
    pub error_steps: usize,
    pub success_rate: f32,
    pub validation_results: Vec<ValidationStep>,
}

impl ValidationReport {
    pub fn print_report(&self) {
        println!("\n📊 Comprehensive Validation Report");
        println!("{}", "=".repeat(50));
        println!("Total Steps: {}", self.total_steps);
        println!("✅ Success: {}", self.success_steps);
        println!("⚠️  Warnings: {}", self.warning_steps);
        println!("❌ Errors: {}", self.error_steps);
        println!("📈 Success Rate: {:.1}%", self.success_rate);
        println!();
        
        for result in &self.validation_results {
            let status_icon = match result.status {
                ValidationStatus::Success => "✅",
                ValidationStatus::Warning => "⚠️ ",
                ValidationStatus::Error => "❌",
                ValidationStatus::Info => "ℹ️ ",
            };
            
            println!("{} {}: {}", status_icon, result.name, result.message);
        }
        
        println!();
        if self.success_rate >= 90.0 {
            println!("🎉 EXCELLENT: Tailwind-RS v0.15.0 integration is working perfectly!");
        } else if self.success_rate >= 70.0 {
            println!("✅ GOOD: Tailwind-RS v0.15.0 integration is working well with minor issues");
        } else if self.success_rate >= 50.0 {
            println!("⚠️  FAIR: Tailwind-RS v0.15.0 integration has some issues that need attention");
        } else {
            println!("❌ POOR: Tailwind-RS v0.15.0 integration has significant issues");
        }
    }
    
    pub fn is_working(&self) -> bool {
        self.success_rate >= 70.0
    }
}

impl Default for ComprehensiveValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick comprehensive validation function
pub fn quick_comprehensive_validation() -> Result<ValidationReport, Box<dyn std::error::Error>> {
    let mut validator = ComprehensiveValidator::new();
    validator.validate_all()
}
