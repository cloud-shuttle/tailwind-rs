use tailwind_rs_core::css_generator::parsers::effects_utilities_modules::box_shadow::BoxShadowParser;
use tailwind_rs_core::css_generator::types::CssProperty;

fn main() {
    let parser = BoxShadowParser::new();
    
    match parser.parse_box_shadow_class("shadow-lg") {
        Some(properties) => {
            println!("✅ Shadow parsing works!");
            for prop in properties {
                println!("  {}: {}", prop.name, prop.value);
            }
        }
        None => println!("❌ Shadow parsing failed"),
    }
}
