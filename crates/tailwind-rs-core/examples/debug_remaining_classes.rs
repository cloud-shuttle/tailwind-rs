use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();
    
    // Test the remaining broken classes
    let broken_classes = vec![
        "dark:hover:text-teal-400",
        "lg:-left-5", 
        "lg:-mt-2",
        "xl:-top-1.5",
        "dark:hover:border-zinc-700",
        "dark:hover:ring-white/20",
        "dark:group-hover:stroke-zinc-400",
    ];
    
    println!("🔍 Debugging Remaining Broken Classes");
    println!("=====================================");
    
    for class in broken_classes {
        println!("\n📝 Testing class: {}", class);
        
        match generator.add_class(class) {
            Ok(_) => {
                println!("  ✅ Successfully added class");
                // Check if it generated CSS
                let css = generator.generate_css();
                if css.contains(&format!(".\\{}", class.replace(":", "\\:"))) {
                    println!("  ✅ CSS rule generated");
                } else {
                    println!("  ❌ No CSS rule found in output");
                }
            }
            Err(e) => {
                println!("  ❌ Failed to add class: {}", e);
            }
        }
    }
    
    println!("\n🎨 Generated CSS:");
    println!("==================");
    let css = generator.generate_css();
    println!("{}", css);
}
