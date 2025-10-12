use tailwind_rs_core::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();
    
    // Test a few basic classes
    let classes = vec![
        "min-h-screen",
        "text-center", 
        "text-white",
        "bg-gradient-to-br",
        "from-slate-900",
        "to-slate-900",
        "shadow-lg",
        "rounded-xl",
        "p-8",
        "text-4xl",
        "font-bold"
    ];
    
    for class in &classes {
        println!("Adding class: {}", class);
        match generator.add_class(class) {
            Ok(_) => println!("  ✓ Success"),
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }
    
    let css = generator.generate_css();
    println!("\nGenerated CSS ({} chars):", css.len());
    println!("{}", css);
}
