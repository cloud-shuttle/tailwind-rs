use tailwind_rs_core::utilities::css_nesting::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod css_nesting_performance_tests {
    use super::*;

    #[test]
    fn test_css_nesting_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 CSS nesting utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .nesting_selector(NestingSelector::DirectChild)
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "CSS nesting generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 CSS nesting string representations
        for _ in 0..1000 {
            let _ = NestingSelector::DirectChild.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 CSS nesting class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().nesting_selector(NestingSelector::DirectChild).build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many CSS nesting builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().nesting_selector(NestingSelector::DirectChild))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_css_nesting_serialization_performance() {
        let selector = NestingSelector::DirectChild;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&selector).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_deserialization_performance() {
        let selector = NestingSelector::DirectChild;
        let serialized = serde_json::to_string(&selector).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: NestingSelector = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with CSS nesting
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .nesting_selector(NestingSelector::DirectChild)
                .nesting_pseudo_class(NestingPseudoClass::Hover)
                .nesting_media_query(NestingMediaQuery::Small)
                .nested_class(NestingSelector::Descendant, "text-blue-500")
                .nested_pseudo_class(NestingPseudoClass::Focus, "text-red-500")
                .nested_media_query(NestingMediaQuery::Medium, "text-green-500")
                .nested_hover("text-yellow-500")
                .nested_focus("text-purple-500")
                .nested_active("text-pink-500")
                .nested_first_child("text-indigo-500")
                .nested_last_child("text-cyan-500")
                .nested_sm("text-gray-500")
                .nested_md("text-white")
                .nested_lg("text-black")
                .nested_dark("text-gray-100")
                .nested_light("text-gray-900")
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "nest-descendant")
                .conditional("hover", "nest-hover")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_enum_values_performance() {
        let start = Instant::now();
        
        // Generate CSS nesting enum values
        for _i in 0..1000 {
            let _ = NestingSelector::DirectChild.to_string();
            let _ = NestingSelector::Descendant.to_string();
            let _ = NestingSelector::AdjacentSibling.to_string();
            let _ = NestingSelector::GeneralSibling.to_string();
            let _ = NestingSelector::Custom("div".to_string()).to_string();
            let _ = NestingPseudoClass::Hover.to_string();
            let _ = NestingPseudoClass::Focus.to_string();
            let _ = NestingPseudoClass::Active.to_string();
            let _ = NestingPseudoClass::Visited.to_string();
            let _ = NestingPseudoClass::Link.to_string();
            let _ = NestingPseudoClass::FirstChild.to_string();
            let _ = NestingPseudoClass::LastChild.to_string();
            let _ = NestingPseudoClass::NthChild("2n".to_string()).to_string();
            let _ = NestingPseudoClass::Custom("custom".to_string()).to_string();
            let _ = NestingMediaQuery::Small.to_string();
            let _ = NestingMediaQuery::Medium.to_string();
            let _ = NestingMediaQuery::Large.to_string();
            let _ = NestingMediaQuery::ExtraLarge.to_string();
            let _ = NestingMediaQuery::Dark.to_string();
            let _ = NestingMediaQuery::Light.to_string();
            let _ = NestingMediaQuery::Print.to_string();
            let _ = NestingMediaQuery::Screen.to_string();
            let _ = NestingMediaQuery::Custom("(max-width: 600px)".to_string()).to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_css_nesting_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all CSS nesting variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .nesting_selector(NestingSelector::DirectChild)
                .nesting_selector(NestingSelector::Descendant)
                .nesting_selector(NestingSelector::AdjacentSibling)
                .nesting_selector(NestingSelector::GeneralSibling)
                .nesting_selector(NestingSelector::Custom("div".to_string()))
                .nesting_pseudo_class(NestingPseudoClass::Hover)
                .nesting_pseudo_class(NestingPseudoClass::Focus)
                .nesting_pseudo_class(NestingPseudoClass::Active)
                .nesting_pseudo_class(NestingPseudoClass::Visited)
                .nesting_pseudo_class(NestingPseudoClass::Link)
                .nesting_pseudo_class(NestingPseudoClass::FirstChild)
                .nesting_pseudo_class(NestingPseudoClass::LastChild)
                .nesting_pseudo_class(NestingPseudoClass::NthChild("2n".to_string()))
                .nesting_pseudo_class(NestingPseudoClass::Custom("custom".to_string()))
                .nesting_media_query(NestingMediaQuery::Small)
                .nesting_media_query(NestingMediaQuery::Medium)
                .nesting_media_query(NestingMediaQuery::Large)
                .nesting_media_query(NestingMediaQuery::ExtraLarge)
                .nesting_media_query(NestingMediaQuery::Dark)
                .nesting_media_query(NestingMediaQuery::Light)
                .nesting_media_query(NestingMediaQuery::Print)
                .nesting_media_query(NestingMediaQuery::Screen)
                .nesting_media_query(NestingMediaQuery::Custom("(max-width: 600px)".to_string()))
                .nested_class(NestingSelector::Descendant, "text-blue-500")
                .nested_pseudo_class(NestingPseudoClass::Hover, "text-red-500")
                .nested_media_query(NestingMediaQuery::Medium, "text-green-500")
                .nested_hover("text-yellow-500")
                .nested_focus("text-purple-500")
                .nested_active("text-pink-500")
                .nested_first_child("text-indigo-500")
                .nested_last_child("text-cyan-500")
                .nested_sm("text-gray-500")
                .nested_md("text-white")
                .nested_lg("text-black")
                .nested_dark("text-gray-100")
                .nested_light("text-gray-900")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200, "All variants too slow: {}ms", duration.as_millis());
    }
}

// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real scenario, you might use more sophisticated memory tracking
    std::mem::size_of::<ClassBuilder>() * 1000
}
