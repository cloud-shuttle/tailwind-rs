extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();
    let test_classes = vec!["bg-white/10", "dark:bg-gray-800/20", "bg-white/5", "bg-black/5"];
    
    for class in test_classes {
        match generator.add_class(class) {
            Ok(_) => println!("✅ {} works!", class),
            Err(e) => println!("❌ {} failed: {}", class, e),
        }
    }
}
