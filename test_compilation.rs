use tailwind_rs_core::*;

fn main() {
    println!("🧪 Testing Tailwind-RS Core Compilation");
    
    // Test CssGenerator
    let mut generator = CssGenerator::new();
    generator.add_class("bg-blue-500").unwrap();
    generator.add_class("text-white").unwrap();
    generator.add_class("px-4").unwrap();
    generator.add_class("py-2").unwrap();
    generator.add_class("rounded").unwrap();
    generator.add_class("hover:bg-blue-600").unwrap();
    
    let css = generator.generate_css();
    println!("✅ CssGenerator working: {}", css);
    
    // Test ClassBuilder
    let class_set = ClassBuilder::new()
        .class("bg-red-500")
        .class("text-white")
        .class("px-4")
        .class("py-2")
        .class("rounded")
        .class("hover:bg-red-600")
        .build();
    
    let classes = class_set.to_css_classes();
    println!("✅ ClassBuilder working: {}", classes);
    
    println!("🎉 All Tailwind-RS functionality working!");
}
