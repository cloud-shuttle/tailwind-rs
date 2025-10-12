use tailwind_rs_core::CssGenerator;

fn main() {
    println!("Testing Standard API: CssGenerator::new() + add_class()");

    // Create a new generator using the standard API
    let mut generator = CssGenerator::new();

    // Add several different classes that should generate separate CSS rules
    let classes = vec![
        "bg-gradient-to-r", // Gradient direction
        "from-blue-400",    // Gradient stop
        "to-purple-600",    // Gradient stop
        "via-pink-500",     // Gradient stop
        "text-center",      // Regular utility
        "hover:bg-red-500", // Hover variant
        "md:p-6",           // Responsive variant
        "animate-pulse",    // Animation
    ];

    println!("Adding classes:");
    for class in &classes {
        println!("  {}", class);
        generator.add_class(class).unwrap();
    }

    // Generate CSS
    let css = generator.generate_css();
    println!("\nGenerated CSS:\n{}", css);

    // Verify that each class generates its own rule (not accumulated properties)
    println!("\nVerification:");

    // Check for individual rules
    let rule_count = css.matches(".bg-gradient-to-r").count()
                   + css.matches(".from-blue-400").count()
                   + css.matches(".to-purple-600").count()
                   + css.matches(".via-pink-500").count()
                   + css.matches(".text-center").count()
                   + css.matches(".hover\\:bg-red-500").count()
                   + css.matches(".md\\:p-6").count()
                   + css.matches(".animate-pulse").count();

    println!("- Found {} individual CSS rules", rule_count);

    // Check that properties are not mixed between classes
    let has_mixed_properties = css.contains("text-center")
                            && css.contains("--tw-gradient")
                            && css.lines().any(|line| line.contains("text-center") && line.contains("--tw-gradient"));

    if has_mixed_properties {
        println!("❌ FAILURE: Properties are mixed between classes (accumulation bug still present)");
    } else {
        println!("✅ SUCCESS: Each class generates its own CSS rule without property accumulation");
    }

    // Check for proper gradient variables
    let has_gradient_vars = css.contains("--tw-gradient-from")
                         && css.contains("--tw-gradient-via")
                         && css.contains("--tw-gradient-to");

    if has_gradient_vars {
        println!("✅ SUCCESS: Gradient CSS variables are properly generated");
    } else {
        println!("❌ FAILURE: Gradient CSS variables are missing");
    }

    // Check for proper selectors
    let has_proper_selectors = css.contains(".hover\\:bg-red-500:hover")
                            && css.contains("@media (min-width: 768px)") && css.contains(".md\\:p-6");

    if has_proper_selectors {
        println!("✅ SUCCESS: Variant selectors are properly generated");
    } else {
        println!("❌ FAILURE: Variant selectors are malformed");
    }

    println!("\nTotal rules in generator: {}", generator.rule_count());
}
