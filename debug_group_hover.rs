use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();
    
    // Test group-hover class
    let test_class = "group-hover:opacity-100";
    match generator.class_to_css_rule(test_class) {
        Ok(rule) => {
            println!("✅ Successfully parsed: {}", test_class);
            println!("   Selector: {}", rule.selector);
            println!("   Properties: {:?}", rule.properties);
            println!("   Media query: {:?}", rule.media_query);
        }
        Err(e) => {
            println!("❌ Failed to parse: {}", test_class);
            println!("   Error: {}", e);
        }
    }
    
    // Test regular opacity class
    let test_class2 = "opacity-100";
    match generator.class_to_css_rule(test_class2) {
        Ok(rule) => {
            println!("✅ Successfully parsed: {}", test_class2);
            println!("   Selector: {}", rule.selector);
            println!("   Properties: {:?}", rule.properties);
            println!("   Media query: {:?}", rule.media_query);
        }
        Err(e) => {
            println!("❌ Failed to parse: {}", test_class2);
            println!("   Error: {}", e);
        }
    }
}
