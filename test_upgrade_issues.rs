use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();
    
    println!("🔍 Testing Tailwind-RS v0.12.0 Upgrade Issues");
    println!("=" .repeat(50));
    
    // Test 1: Transition classes (should now work after fix)
    println!("\n1️⃣ Testing Transition Classes:");
    let transition_classes = vec![
        "transition",
        "transition-all", 
        "transition-colors",
        "duration-300",
        "ease-in-out",
        "delay-100"
    ];
    
    for class in transition_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {:?}", class, properties),
            Err(e) => println!("❌ {} -> Error: {}", class, e),
        }
    }
    
    // Test 2: Hover states (reported as failing)
    println!("\n2️⃣ Testing Hover States:");
    let hover_classes = vec![
        "hover:scale-105",
        "hover:bg-blue-600", 
        "hover:text-white",
        "hover:shadow-lg"
    ];
    
    for class in hover_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {:?}", class, properties),
            Err(e) => println!("❌ {} -> Error: {}", class, e),
        }
    }
    
    // Test 3: Focus states (reported as failing)
    println!("\n3️⃣ Testing Focus States:");
    let focus_classes = vec![
        "focus:ring-4",
        "focus:ring-blue-500",
        "focus:outline-none",
        "focus:bg-blue-100"
    ];
    
    for class in focus_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {:?}", class, properties),
            Err(e) => println!("❌ {} -> Error: {}", class, e),
        }
    }
    
    // Test 4: Device variants (reported as failing)
    println!("\n4️⃣ Testing Device Variants:");
    let device_classes = vec![
        "pointer-coarse:bg-blue-500",
        "motion-reduce:transition-none",
        "motion-safe:animate-spin",
        "light:bg-white",
        "dark:bg-gray-900"
    ];
    
    for class in device_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {:?}", class, properties),
            Err(e) => println!("❌ {} -> Error: {}", class, e),
        }
    }
    
    // Test 5: Responsive variants (reported as limited)
    println!("\n5️⃣ Testing Responsive Variants:");
    let responsive_classes = vec![
        "sm:flex",
        "md:grid", 
        "lg:hidden",
        "xl:block"
    ];
    
    for class in responsive_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {:?}", class, properties),
            Err(e) => println!("❌ {} -> Error: {}", class, e),
        }
    }
    
    // Test 6: Basic classes (should work)
    println!("\n6️⃣ Testing Basic Classes (Control Group):");
    let basic_classes = vec![
        "block",
        "flex", 
        "p-4",
        "bg-blue-500",
        "text-white"
    ];
    
    for class in basic_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {:?}", class, properties),
            Err(e) => println!("❌ {} -> Error: {}", class, e),
        }
    }
    
    println!("\n" + "=".repeat(50));
    println!("🏁 Testing Complete");
}
