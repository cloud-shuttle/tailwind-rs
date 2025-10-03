use tailwind_rs_core::css_generator::parsers::effects_utilities_modules::box_shadow::BoxShadowParser;

fn main() {
    let parser = BoxShadowParser::new();
    
    // Test basic shadow parsing
    let result = parser.parse_box_shadow_class("shadow-lg");
    match result {
        Some(properties) => {
            println!("✅ Shadow parsing works!");
            println!("Property: {} = {}", properties[0].name, properties[0].value);
        }
        None => {
            println!("❌ Shadow parsing failed");
        }
    }
    
    // Test another shadow
    let result2 = parser.parse_box_shadow_class("shadow-sm");
    match result2 {
        Some(properties) => {
            println!("✅ Shadow-sm parsing works!");
            println!("Property: {} = {}", properties[0].name, properties[0].value);
        }
        None => {
            println!("❌ Shadow-sm parsing failed");
        }
    }
}
