use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();
    
    // Test transition classes
    let test_classes = vec![
        "transition",
        "transition-all", 
        "transition-colors",
        "duration-300",
        "ease-in-out",
        "delay-100"
    ];
    
    println!("Testing transition classes:");
    for class in test_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("✅ {} -> {:?}", class, properties);
            }
            Err(e) => {
                println!("❌ {} -> Error: {}", class, e);
            }
        }
    }
}
