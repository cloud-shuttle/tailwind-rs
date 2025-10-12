use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🔍 Testing Tailwind-RS v0.12.0 Fixes");
    println!("=" .repeat(50));
    
    let mut generator = CssGenerator::new();
    
    // Test 1: Transition classes (should now work after fix)
    println!("\n1️⃣ Testing Transition Classes (FIXED):");
    let transition_tests = vec![
        ("transition", "Should work"),
        ("transition-all", "Should work"), 
        ("duration-300", "Should work"),
        ("ease-in-out", "Should work"),
    ];
    
    for (class, description) in transition_tests {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {} -> {:?}", class, description, properties),
            Err(e) => println!("❌ {} -> {} -> Error: {}", class, description, e),
        }
    }
    
    // Test 2: Device variants (should now work after fix)
    println!("\n2️⃣ Testing Device Variants (FIXED):");
    let device_tests = vec![
        ("pointer-coarse:bg-blue-500", "Should work"),
        ("motion-reduce:transition-none", "Should work"),
        ("light:bg-white", "Should work"),
    ];
    
    for (class, description) in device_tests {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {} -> {:?}", class, description, properties),
            Err(e) => println!("❌ {} -> {} -> Error: {}", class, description, e),
        }
    }
    
    // Test 3: Hover/Focus states (should already work)
    println!("\n3️⃣ Testing Hover/Focus States (VERIFIED):");
    let state_tests = vec![
        ("hover:scale-105", "Should work"),
        ("hover:bg-blue-600", "Should work"),
        ("focus:ring-4", "Should work"),
    ];
    
    for (class, description) in state_tests {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {} -> {:?}", class, description, properties),
            Err(e) => println!("❌ {} -> {} -> Error: {}", class, description, e),
        }
    }
    
    // Test 4: Responsive variants (should already work)
    println!("\n4️⃣ Testing Responsive Variants (VERIFIED):");
    let responsive_tests = vec![
        ("sm:flex", "Should work"),
        ("md:grid", "Should work"),
        ("lg:hidden", "Should work"),
    ];
    
    for (class, description) in responsive_tests {
        match generator.class_to_properties(class) {
            Ok(properties) => println!("✅ {} -> {} -> {:?}", class, description, properties),
            Err(e) => println!("❌ {} -> {} -> Error: {}", class, description, e),
        }
    }
    
    println!("\n" + "=".repeat(50));
    println!("🏁 Fix Verification Complete");
    println!("Expected: All tests should show ✅ (success)");
    println!("If any show ❌, the fixes need additional work");
}
