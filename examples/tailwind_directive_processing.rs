//! Example: @tailwind Directive Processing
//! 
//! This example demonstrates how to use the TailwindProcessor to process
//! @tailwind directives, @apply directives, and @layer directives.

use tailwind_rs_postcss::{TailwindProcessor, TailwindConfig, ProcessingResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Tailwind Directive Processing Example");
    println!("==========================================");
    
    // Create a new TailwindProcessor with default configuration
    let mut processor = TailwindProcessor::new();
    
    // Example 1: Basic @tailwind directives
    println!("\n📝 Example 1: Basic @tailwind directives");
    let css_with_tailwind = r#"
        @tailwind base;
        @tailwind components;
        @tailwind utilities;
        
        .custom-class {
            color: red;
        }
    "#;
    
    let result = processor.process_directives(css_with_tailwind)?;
    println!("✅ Processed {} directives", result.statistics.total_directives);
    println!("   - Base directives: {}", result.statistics.base_directives);
    println!("   - Component directives: {}", result.statistics.component_directives);
    println!("   - Utility directives: {}", result.statistics.utility_directives);
    println!("   - Processing time: {}ms", result.statistics.processing_time_ms);
    
    // Example 2: @apply directives
    println!("\n📝 Example 2: @apply directives");
    let css_with_apply = r#"
        .btn {
            @apply bg-blue-500 text-white px-4 py-2 rounded;
        }
        
        .btn-primary {
            @apply bg-blue-600 hover:bg-blue-700;
        }
    "#;
    
    let result = processor.process_apply(css_with_apply)?;
    println!("✅ Processed @apply directives");
    println!("   - Apply directives: {}", result.statistics.apply_directives);
    
    // Example 3: @layer directives
    println!("\n📝 Example 3: @layer directives");
    let css_with_layers = r#"
        @layer base {
            html, body {
                margin: 0;
                padding: 0;
            }
        }
        
        @layer components {
            .container {
                max-width: 1200px;
                margin: 0 auto;
            }
        }
        
        @layer utilities {
            .text-center {
                text-align: center;
            }
        }
    "#;
    
    let result = processor.process_layer(css_with_layers)?;
    println!("✅ Processed @layer directives");
    println!("   - Layer directives: {}", result.statistics.layer_directives);
    
    // Example 4: Complex CSS with all directive types
    println!("\n📝 Example 4: Complex CSS with all directive types");
    let complex_css = r#"
        @tailwind base;
        @tailwind components;
        @tailwind utilities;
        
        @layer base {
            html, body {
                margin: 0;
                padding: 0;
            }
        }
        
        @layer components {
            .btn {
                @apply bg-blue-500 text-white px-4 py-2 rounded;
            }
            
            .btn-primary {
                @apply bg-blue-600 hover:bg-blue-700;
            }
        }
        
        @layer utilities {
            .text-center {
                text-align: center;
            }
        }
        
        .custom-class {
            color: red;
        }
    "#;
    
    let result = processor.process_directives(complex_css)?;
    println!("✅ Processed complex CSS");
    println!("   - Total directives: {}", result.statistics.total_directives);
    println!("   - Base directives: {}", result.statistics.base_directives);
    println!("   - Component directives: {}", result.statistics.component_directives);
    println!("   - Utility directives: {}", result.statistics.utility_directives);
    println!("   - Apply directives: {}", result.statistics.apply_directives);
    println!("   - Layer directives: {}", result.statistics.layer_directives);
    println!("   - Processing time: {}ms", result.statistics.processing_time_ms);
    
    // Example 5: Custom configuration
    println!("\n📝 Example 5: Custom configuration");
    let custom_config = TailwindConfig {
        base_styles: true,
        component_styles: true,
        utility_styles: true,
        apply_processing: true,
        layer_processing: true,
        custom_directives: vec![],
    };
    
    let mut custom_processor = TailwindProcessor::with_config(custom_config);
    let result = custom_processor.process_directives(complex_css)?;
    println!("✅ Processed with custom configuration");
    println!("   - Total directives: {}", result.statistics.total_directives);
    println!("   - Processing time: {}ms", result.statistics.processing_time_ms);
    
    println!("\n🎉 All examples completed successfully!");
    println!("==========================================");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_processing() {
        let mut processor = TailwindProcessor::new();
        let css = "@tailwind base; @tailwind components; @tailwind utilities;";
        let result = processor.process_directives(css);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.statistics.total_directives, 3);
        assert_eq!(result.statistics.base_directives, 1);
        assert_eq!(result.statistics.component_directives, 1);
        assert_eq!(result.statistics.utility_directives, 1);
    }
    
    #[test]
    fn test_apply_processing() {
        let mut processor = TailwindProcessor::new();
        let css = ".btn { @apply bg-blue-500 text-white px-4 py-2; }";
        let result = processor.process_apply(css);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_layer_processing() {
        let mut processor = TailwindProcessor::new();
        let css = "@layer base; @layer components; @layer utilities;";
        let result = processor.process_layer(css);
        assert!(result.is_ok());
    }
}
