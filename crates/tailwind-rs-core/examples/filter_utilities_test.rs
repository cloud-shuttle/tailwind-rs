use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Filter Utilities");
    println!("===========================");
    
    let test_classes = vec![
        "filter-none",
        "blur-sm",
        "blur-lg", 
        "brightness-50",
        "brightness-150",
        "contrast-75",
        "contrast-125",
        "drop-shadow-md",
        "drop-shadow-xl",
        "grayscale",
        "grayscale-50",
        "hue-rotate-90",
        "hue-rotate-180",
        "invert",
        "invert-20",
        "saturate-50",
        "saturate-150",
        "sepia",
        "sepia-75"
    ];
    
    let mut generator = CssGenerator::new();
    
    for class in &test_classes {
        match generator.add_class(class) {
            Ok(_) => {
                println!("✅ {} - Added", class);
            }
            Err(e) => {
                println!("❌ {} - Failed: {}", class, e);
            }
        }
    }
    
    let css = generator.generate_css();
    println!("\n📝 Generated CSS:");
    for line in css.lines() {
        if line.contains("filter") {
            println!("  {}", line);
        }
    }
}
