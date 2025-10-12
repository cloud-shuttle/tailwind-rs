use std::collections::HashMap;

// Simple test to verify our variant parsing logic works
fn parse_variants_from_class(class: &str) -> (Vec<String>, String) {
    let parts: Vec<&str> = class.split(':').collect();
    if parts.len() <= 1 {
        // No variants
        return (vec![], class.to_string());
    }

    let mut variants = Vec::new();
    let mut base_class = String::new();

    // Check each prefix to see if it's a variant
    for i in 0..parts.len() {
        let prefix = parts[i];

        // Check if this prefix is a variant
        if is_variant_prefix(prefix) {
            variants.push(prefix.to_string());

            // If we have more parts after this variant, continue
            if i + 1 < parts.len() {
                // Reconstruct base class from remaining parts
                base_class = parts[i + 1..].join(":");
            } else {
                // This was the last part, so base class is empty (invalid)
                base_class = String::new();
            }
        } else {
            // Not a variant, so this and remaining parts are the base class
            base_class = parts[i..].join(":");
            break;
        }
    }

    (variants, base_class)
}

fn is_variant_prefix(prefix: &str) -> bool {
    matches!(prefix,
        "hover" | "focus" | "active" | "visited" | "disabled" |
        "first" | "last" | "odd" | "even" |
        "sm" | "md" | "lg" | "xl" | "2xl" |
        "dark" | "group-hover" | "group-focus" | "peer-hover" | "peer-focus"
    )
}

fn main() {
    println!("Testing variant parsing...");

    // Test simple variant
    let (variants, base) = parse_variants_from_class("hover:bg-blue-500");
    println!("hover:bg-blue-500 -> variants: {:?}, base: {}", variants, base);
    assert_eq!(variants, vec!["hover"]);
    assert_eq!(base, "bg-blue-500");

    // Test multiple variants
    let (variants, base) = parse_variants_from_class("md:hover:shadow-lg");
    println!("md:hover:shadow-lg -> variants: {:?}, base: {}", variants, base);
    assert_eq!(variants, vec!["md", "hover"]);
    assert_eq!(base, "shadow-lg");

    // Test no variants
    let (variants, base) = parse_variants_from_class("bg-blue-500");
    println!("bg-blue-500 -> variants: {:?}, base: {}", variants, base);
    assert_eq!(variants, Vec::<String>::new());
    assert_eq!(base, "bg-blue-500");

    println!("✅ All variant parsing tests passed!");
}
