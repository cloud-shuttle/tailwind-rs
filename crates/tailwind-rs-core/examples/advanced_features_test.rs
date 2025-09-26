use tailwind_rs_core::*;

/// Test advanced Tailwind-RS features
fn main() {
    println!("🔍 Testing Advanced Tailwind-RS Features");
    println!("==========================================");
    
    let mut generator = CssGenerator::new();
    
    // Test arbitrary values
    let arbitrary_classes = vec![
        "size-[38px]",
        "top-[4px]", 
        "left-[7px]",
        "size-12",
        "size-6",
        "w-[100px]",
        "h-[50px]",
    ];
    
    println!("\n📏 Testing Arbitrary Values:");
    println!("============================");
    let mut arbitrary_working = 0;
    for class in &arbitrary_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                arbitrary_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Test complex calc() expressions
    let calc_classes = vec![
        "left-[calc(50%+var(--offset))]",
    ];
    
    println!("\n🧮 Testing Complex calc() Expressions:");
    println!("=====================================");
    let mut calc_working = 0;
    for class in &calc_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                calc_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Test complex drop-shadow
    let drop_shadow_classes = vec![
        "drop-shadow-[0_3px_1px_rgba(0,0,0,.15)]",
    ];
    
    println!("\n🎨 Testing Complex drop-shadow:");
    println!("===============================");
    let mut drop_shadow_working = 0;
    for class in &drop_shadow_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                drop_shadow_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Test complex background
    let background_classes = vec![
        "bg-[url(/map.png)]",
        "mask-[linear-gradient(to_bottom,black_50%,transparent)]",
        "bg-size-[530px_430px]",
        "bg-position-[center_-75px]",
        "bg-no-repeat",
    ];
    
    println!("\n🖼️ Testing Complex Background:");
    println!("===============================");
    let mut background_working = 0;
    for class in &background_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                background_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Test data attributes
    let data_attribute_classes = vec![
        "data-hover:bg-black/2.5",
        "data-hover:bg-black/5",
        "data-closed:opacity-0",
        "data-enter:ease-out",
        "data-leave:ease-in",
        "data-closed:scale-95",
    ];
    
    println!("\n📊 Testing Data Attributes:");
    println!("===========================");
    let mut data_working = 0;
    for class in &data_attribute_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                data_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Test complex transforms
    let transform_classes = vec![
        "-translate-x-1/2",
        "translate-x-[10px]",
        "translate-y-[20px]",
        "rotate-[45deg]",
        "scale-[1.2]",
    ];
    
    println!("\n🔄 Testing Complex Transforms:");
    println!("===============================");
    let mut transform_working = 0;
    for class in &transform_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                transform_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Test opacity with slash notation
    let opacity_classes = vec![
        "border-black/5",
        "bg-white/90",
        "text-black/50",
    ];
    
    println!("\n🎭 Testing Opacity with Slash Notation:");
    println!("======================================");
    let mut opacity_working = 0;
    for class in &opacity_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                opacity_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
            }
        }
    }
    
    // Generate comprehensive CSS
    let all_classes: Vec<String> = arbitrary_classes.iter()
        .chain(calc_classes.iter())
        .chain(drop_shadow_classes.iter())
        .chain(background_classes.iter())
        .chain(data_attribute_classes.iter())
        .chain(transform_classes.iter())
        .chain(opacity_classes.iter())
        .map(|s| s.to_string())
        .collect();
    
    let mut css = String::new();
    for class in &all_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                css.push_str(&format!(".{} {{\n", class.replace("[", "\\[").replace("]", "\\]")));
                for property in properties {
                    css.push_str(&format!("  {}: {};\n", property.name, property.value));
                }
                css.push_str("}\n\n");
            }
            Err(_) => {
                // Skip broken classes
            }
        }
    }
    
    // Write CSS to file
    std::fs::write("advanced-features-test.css", &css).expect("Failed to write CSS file");
    
    // Print summary
    let total_classes = arbitrary_classes.len() + calc_classes.len() + drop_shadow_classes.len() + 
                       background_classes.len() + data_attribute_classes.len() + 
                       transform_classes.len() + opacity_classes.len();
    let total_working = arbitrary_working + calc_working + drop_shadow_working + 
                       background_working + data_working + transform_working + opacity_working;
    
    println!("\n📊 Advanced Features Test Results:");
    println!("==================================");
    println!("  ✅ Working classes: {}", total_working);
    println!("  ❌ Broken classes: {}", total_classes - total_working);
    println!("  📊 Coverage: {:.1}%", (total_working as f32 / total_classes as f32) * 100.0);
    
    println!("\n🎨 Generated CSS:");
    println!("==================");
    println!("{}", css);
    
    println!("\n✅ CSS written to advanced-features-test.css");
    println!("\n🌐 To test the advanced features:");
    println!("1. Open advanced-features-test.css in a text editor");
    println!("2. Copy the CSS and use it in your HTML");
    println!("3. Test the advanced features functionality!");
}
