use tailwind_rs_core::*;

/// Test navbar classes from the React component
fn main() {
    println!("🔍 Testing Navbar Classes");
    println!("=========================");
    
    let generator = CssGenerator::new();
    
    // Navbar classes from the React component
    let navbar_classes = vec![
        "pt-12", "sm:pt-16", "max-w-4xl", "mx-auto", "px-8",
        "relative", "flex", "justify-between", "gap-6", "py-3",
        "text-2xl", "font-bold", "text-zinc-900", "dark:text-zinc-100",
        "hidden", "lg:flex", "items-center", "px-4", "py-3",
        "text-base", "font-medium", "text-gray-950", "bg-blend-multiply",
        "data-hover:bg-black/2.5", "data-hover:bg-black/5",
        "size-12", "size-6", "self-center", "rounded-lg",
        "lg:hidden", "aria-label", "Open main menu"
    ];
    
    let mut working_classes = 0;
    let mut css = String::new();
    
    for class in &navbar_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("  ✅ {}", class);
                working_classes += 1;
                
                // Generate CSS for this class
                css.push_str(&format!(".{} {{\n", class.replace(":", "\\:").replace("/", "\\/")));
                for property in properties {
                    css.push_str(&format!("  {}: {};\n", property.name, property.value));
                }
                css.push_str("}\n\n");
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Write CSS to file
    std::fs::write("navbar-test.css", &css).expect("Failed to write CSS file");
    
    println!("\n📊 Navbar Test Results:");
    println!("========================");
    println!("  ✅ Working classes: {}", working_classes);
    println!("  ❌ Broken classes: {}", navbar_classes.len() - working_classes);
    println!("  📊 Coverage: {:.1}%", (working_classes as f32 / navbar_classes.len() as f32) * 100.0);
    
    println!("\n🎨 Generated CSS:");
    println!("==================");
    println!("{}", css);
    
    println!("\n✅ CSS written to navbar-test.css");
}
