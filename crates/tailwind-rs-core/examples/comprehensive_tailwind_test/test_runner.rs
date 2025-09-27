//! Test runner and results reporting for comprehensive Tailwind CSS testing
//!
//! This module contains the test execution logic, results collection,
//! and comprehensive reporting functionality.

use crate::css_generator::CssGenerator;

/// Test results for a category
#[derive(Debug, Clone)]
pub struct CategoryResult {
    pub name: String,
    pub working: usize,
    pub broken: usize,
    pub coverage: f32,
}

/// Overall test results
#[derive(Debug, Clone)]
pub struct TestResults {
    pub total_working: usize,
    pub total_broken: usize,
    pub overall_coverage: f32,
    pub category_results: Vec<CategoryResult>,
}

/// Run comprehensive tests on all categories
pub fn run_comprehensive_tests() -> TestResults {
    println!("🔍 Comprehensive Tailwind CSS Coverage Test");
    println!("=============================================");

    let generator = CssGenerator::new();
    let mut total_working = 0;
    let mut total_broken = 0;
    let mut category_results = Vec::new();

    // Import test categories
    use super::test_categories::get_test_categories;
    use super::advanced_categories::get_advanced_categories;
    use super::special_categories::get_special_categories;

    // Combine all test categories
    let mut all_categories = get_test_categories();
    all_categories.extend(get_advanced_categories());
    all_categories.extend(get_special_categories());

    for (category_name, classes) in &all_categories {
        let mut working = 0;
        let mut broken = 0;

        println!(
            "\n📋 Testing {} ({} classes):",
            category_name,
            classes.len()
        );

        for class in classes {
            match generator.class_to_properties(class) {
                Ok(_) => {
                    println!("  ✅ {}", class);
                    working += 1;
                }
                Err(_) => {
                    println!("  ❌ {}", class);
                    broken += 1;
                }
            }
        }

        let coverage = (working as f32 / classes.len() as f32) * 100.0;
        category_results.push(CategoryResult {
            name: category_name.clone(),
            working,
            broken,
            coverage,
        });

        total_working += working;
        total_broken += broken;

        println!(
            "  📊 {}: {}/{} ({:.1}%)",
            category_name,
            working,
            classes.len(),
            coverage
        );
    }

    let total_classes = total_working + total_broken;
    let overall_coverage = (total_working as f32 / total_classes as f32) * 100.0;

    TestResults {
        total_working,
        total_broken,
        overall_coverage,
        category_results,
    }
}

/// Print comprehensive test results
pub fn print_test_results(results: &TestResults) {
    println!("\n📊 Overall Results:");
    println!("==================");
    println!("  ✅ Working: {}/{}", results.total_working, results.total_working + results.total_broken);
    println!("  ❌ Broken: {}/{}", results.total_broken, results.total_working + results.total_broken);
    println!("  📈 Overall Coverage: {:.1}%", results.overall_coverage);

    println!("\n📋 Category Breakdown:");
    println!("=====================");
    for category in &results.category_results {
        println!(
            "  {}: {}/{} ({:.1}%)",
            category.name,
            category.working,
            category.working + category.broken,
            category.coverage
        );
    }

    // Print coverage assessment
    if results.overall_coverage >= 95.0 {
        println!("\n🎉 Excellent coverage! Ready for production!");
    } else if results.overall_coverage >= 90.0 {
        println!("\n🚀 Good coverage! Consider release candidate.");
    } else if results.overall_coverage >= 80.0 {
        println!("\n⚠️  Coverage needs improvement before release.");
    } else {
        println!("\n❌ Coverage needs significant improvement.");
    }
}

/// Run tests for a specific category
pub fn run_category_test(generator: &CssGenerator, category_name: &str, classes: &[&str]) -> CategoryResult {
    let mut working = 0;
    let mut broken = 0;

    println!("\n📋 Testing {} ({} classes):", category_name, classes.len());

    for class in classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
                broken += 1;
            }
        }
    }

    let coverage = (working as f32 / classes.len() as f32) * 100.0;
    
    println!(
        "  📊 {}: {}/{} ({:.1}%)",
        category_name,
        working,
        classes.len(),
        coverage
    );

    CategoryResult {
        name: category_name.to_string(),
        working,
        broken,
        coverage,
    }
}
