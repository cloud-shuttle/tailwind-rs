use tailwind_rs_core::CssGenerator;

fn main() {
    let generator = CssGenerator::new();
    
    // Test basic shadow class
    println!("Testing 'shadow-lg':");
    let (variants1, base1) = generator.parse_variants("shadow-lg");
    println!("  Variants: {:?}, Base: '{}'", variants1, base1);
    
    // Test variant shadow class
    println!("\nTesting 'hover:shadow-lg':");
    let (variants2, base2) = generator.parse_variants("hover:shadow-lg");
    println!("  Variants: {:?}, Base: '{}'", variants2, base2);
    
    // Test if class_to_properties works
    println!("\nTesting class_to_properties for 'shadow-lg':");
    match generator.class_to_properties("shadow-lg") {
        Ok(props) => println!("  Properties: {} items", props.len()),
        Err(e) => println!("  Error: {}", e),
    }
    
    // Test class_to_css_rule
    println!("\nTesting class_to_css_rule for 'hover:shadow-lg':");
    match generator.class_to_css_rule("hover:shadow-lg") {
        Ok(rule) => {
            println!("  Selector: '{}'", rule.selector);
            println!("  Properties: {} items", rule.properties.len());
            println!("  Media query: {:?}", rule.media_query);
        }
        Err(e) => println!("  Error: {}", e),
    }
}
