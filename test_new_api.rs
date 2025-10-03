use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🧪 Testing new process_element_classes API...");

    let mut generator = CssGenerator::new();

    // Test the new element-based processing
    let classes = vec![
        "bg-gradient-to-r",
        "from-pink-400",
        "via-purple-500",
        "to-cyan-400"
    ];

    let css = generator.process_element_classes(&classes);
    println!("🎨 Generated CSS:\n{}", css);

    // Check if colors are properly resolved
    if css.contains("#f472b6") { // pink-400
        println!("✅ Pink-400 resolved correctly!");
    } else {
        println!("❌ Pink-400 not resolved");
    }

    if css.contains("#a855f7") { // purple-500
        println!("✅ Purple-500 resolved correctly!");
    } else {
        println!("❌ Purple-500 not resolved");
    }

    if css.contains("#06b6d4") { // cyan-400
        println!("✅ Cyan-400 resolved correctly!");
    } else {
        println!("❌ Cyan-400 not resolved");
    }
}
