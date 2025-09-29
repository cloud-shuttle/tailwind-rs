use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();
    
    let test_classes = vec![
        "scale-110",
        "group-hover:scale-110",
        "hover:scale-110",
        "backdrop-blur-2xl",
        "animate-rainbow"
    ];
    
    for class in &test_classes {
        match generator.add_class(class) {
            Ok(_) => println!("✅ {} - ADDED", class),
            Err(e) => println!("❌ {} - ERROR: {}", class, e),
        }
    }
    
    let css = generator.generate_css();
    println!("\nCSS length: {} chars", css.len());
    println!("Contains scale-110: {}", css.contains("scale-110"));
    println!("Contains group:hover: {}", css.contains("group:hover"));
    println!("Contains hover: {}", css.contains(":hover"));
}
