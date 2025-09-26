use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Table Utilities");
    println!("==========================");

    let test_classes = vec![
        // Table layout
        "table-auto",
        "table-fixed",
        // Border collapse
        "border-collapse",
        "border-separate",
        // Border spacing
        "border-spacing-0",
        "border-spacing-1",
        "border-spacing-2",
        "border-spacing-4",
        "border-spacing-8",
        // Border spacing X
        "border-spacing-x-1",
        "border-spacing-x-2",
        "border-spacing-x-4",
        // Border spacing Y
        "border-spacing-y-1",
        "border-spacing-y-2",
        "border-spacing-y-4",
        // Caption side
        "caption-top",
        "caption-bottom",
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
        if line.contains("table-layout")
            || line.contains("border-collapse")
            || line.contains("border-spacing")
            || line.contains("caption-side")
        {
            println!("  {}", line);
        }
    }
}
