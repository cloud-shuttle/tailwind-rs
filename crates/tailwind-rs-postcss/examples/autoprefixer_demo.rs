//! Example: Enhanced Autoprefixer System
//! 
//! This example demonstrates how to use the Autoprefixer to add vendor prefixes
//! to CSS properties based on browser compatibility data.

use tailwind_rs_postcss::{Autoprefixer, AutoprefixerConfig, PrefixOptions, FlexboxMode, GridMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced Autoprefixer System Demo");
    println!("=====================================");
    
    // Example 1: Basic vendor prefixing
    println!("\n📝 Example 1: Basic vendor prefixing");
    let autoprefixer = Autoprefixer::new();
    
    let css = r#"
        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
        }
        
        .item {
            flex: 1;
            transform: translateX(10px);
        }
    "#;
    
    let browsers = vec![
        "chrome 30".to_string(),
        "firefox 25".to_string(),
        "safari 7".to_string(),
    ];
    
    let result = autoprefixer.add_prefixes(css, &browsers)?;
    println!("✅ Basic prefixing completed");
    println!("   - Original CSS: {} bytes", css.len());
    println!("   - Prefixed CSS: {} bytes", result.len());
    println!("   - Contains webkit prefixes: {}", result.contains("-webkit-"));
    println!("   - Contains moz prefixes: {}", result.contains("-moz-"));
    
    // Example 2: Advanced prefixing with options
    println!("\n📝 Example 2: Advanced prefixing with options");
    let mut config = AutoprefixerConfig::default();
    config.flexbox = FlexboxMode::All;
    config.grid = GridMode::Autoplace;
    let advanced_autoprefixer = Autoprefixer::with_config(config);
    
    let complex_css = r#"
        .flexbox-container {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            flex-wrap: wrap;
        }
        
        .flexbox-item {
            flex: 1 1 200px;
            order: 2;
        }
        
        .grid-container {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            grid-gap: 20px;
        }
        
        .grid-item {
            grid-column: span 2;
            grid-row: span 1;
        }
        
        .transform-item {
            transform: rotate(45deg) scale(1.2);
            transform-origin: center;
        }
        
        .transition-item {
            transition: all 0.3s ease;
            transition-property: transform, opacity;
        }
        
        .animation-item {
            animation: slideIn 1s ease-in-out;
            animation-name: slideIn;
            animation-duration: 1s;
        }
        
        @keyframes slideIn {
            from { transform: translateX(-100%); }
            to { transform: translateX(0); }
        }
    "#;
    
    let options = PrefixOptions {
        browsers: vec![
            "chrome 30".to_string(),
            "firefox 25".to_string(),
            "safari 7".to_string(),
            "ie 10".to_string(),
            "edge 12".to_string(),
        ],
        cascade: true,
        add: true,
        remove: false,
        supports: true,
        flexbox: FlexboxMode::All,
        grid: GridMode::Autoplace,
        stats: None,
    };
    
    let advanced_result = advanced_autoprefixer.add_prefixes_advanced(complex_css, &options)?;
    println!("✅ Advanced prefixing completed");
    println!("   - Original size: {} bytes", advanced_result.statistics.original_size);
    println!("   - Prefixed size: {} bytes", advanced_result.statistics.prefixed_size);
    println!("   - Prefixes added: {}", advanced_result.statistics.prefixes_added);
    println!("   - Properties processed: {}", advanced_result.statistics.properties_processed);
    println!("   - Processing time: {}ms", advanced_result.statistics.processing_time_ms);
    
    // Example 3: Browser-specific prefixing
    println!("\n📝 Example 3: Browser-specific prefixing");
    let ie_css = r#"
        .ie-flexbox {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }
        
        .ie-transform {
            transform: translateX(50px);
        }
    "#;
    
    let ie_browsers = vec!["ie 10".to_string()];
    let ie_result = autoprefixer.add_prefixes(ie_css, &ie_browsers)?;
    println!("✅ IE-specific prefixing completed");
    println!("   - Contains MS prefixes: {}", ie_result.contains("-ms-"));
    println!("   - Contains webkit prefixes: {}", ie_result.contains("-webkit-"));
    
    // Example 4: Flexbox mode testing
    println!("\n📝 Example 4: Flexbox mode testing");
    let flexbox_css = r#"
        .flexbox-2009 {
            display: -webkit-box;
            -webkit-box-orient: vertical;
            -webkit-box-pack: center;
        }
        
        .flexbox-2012 {
            display: -webkit-flex;
            -webkit-flex-direction: column;
            -webkit-justify-content: center;
        }
        
        .flexbox-modern {
            display: flex;
            flex-direction: column;
            justify-content: center;
        }
    "#;
    
    // Test with No2009 mode
    let mut no2009_config = AutoprefixerConfig::default();
    no2009_config.flexbox = FlexboxMode::No2009;
    let no2009_autoprefixer = Autoprefixer::with_config(no2009_config);
    let no2009_result = no2009_autoprefixer.add_prefixes(flexbox_css, &browsers)?;
    println!("✅ No2009 flexbox mode completed");
    println!("   - Removed 2009 prefixes: {}", !no2009_result.contains("-webkit-box"));
    
    // Test with No2012 mode
    let mut no2012_config = AutoprefixerConfig::default();
    no2012_config.flexbox = FlexboxMode::No2012;
    let no2012_autoprefixer = Autoprefixer::with_config(no2012_config);
    let no2012_result = no2012_autoprefixer.add_prefixes(flexbox_css, &browsers)?;
    println!("✅ No2012 flexbox mode completed");
    println!("   - Removed 2012 prefixes: {}", !no2012_result.contains("-webkit-flex"));
    
    // Example 5: Performance testing
    println!("\n📝 Example 5: Performance testing");
    let large_css = generate_large_css();
    let start_time = std::time::Instant::now();
    let performance_result = autoprefixer.add_prefixes(&large_css, &browsers)?;
    let processing_time = start_time.elapsed().as_millis();
    
    println!("✅ Performance test completed");
    println!("   - Large CSS size: {} bytes", large_css.len());
    println!("   - Processing time: {}ms", processing_time);
    println!("   - Prefixed size: {} bytes", performance_result.len());
    println!("   - Size increase: {:.1}%", 
        ((performance_result.len() as f64 - large_css.len() as f64) / large_css.len() as f64) * 100.0);
    
    // Example 6: Property-specific prefixing
    println!("\n📝 Example 6: Property-specific prefixing");
    let property_css = r#"
        .transform-properties {
            transform: rotate(45deg) scale(1.2);
            transform-origin: center;
            transform-style: preserve-3d;
        }
        
        .transition-properties {
            transition: all 0.3s ease;
            transition-property: transform, opacity;
            transition-duration: 0.3s;
            transition-timing-function: ease;
            transition-delay: 0s;
        }
        
        .animation-properties {
            animation: slideIn 1s ease-in-out;
            animation-name: slideIn;
            animation-duration: 1s;
            animation-timing-function: ease-in-out;
            animation-delay: 0s;
            animation-iteration-count: 1;
            animation-direction: normal;
            animation-fill-mode: both;
            animation-play-state: running;
        }
        
        .filter-properties {
            filter: blur(5px) brightness(1.2);
            backdrop-filter: blur(10px);
        }
        
        .mask-properties {
            mask: url(#mask);
            mask-image: url(#mask);
            mask-size: cover;
            mask-position: center;
            mask-repeat: no-repeat;
            mask-origin: border-box;
            mask-clip: border-box;
            mask-composite: add;
        }
        
        .clip-path-properties {
            clip-path: circle(50%);
            shape-outside: circle(50%);
            shape-margin: 10px;
        }
    "#;
    
    let property_result = autoprefixer.add_prefixes(property_css, &browsers)?;
    println!("✅ Property-specific prefixing completed");
    println!("   - Contains transform prefixes: {}", property_result.contains("-webkit-transform"));
    println!("   - Contains transition prefixes: {}", property_result.contains("-webkit-transition"));
    println!("   - Contains animation prefixes: {}", property_result.contains("-webkit-animation"));
    println!("   - Contains filter prefixes: {}", property_result.contains("-webkit-filter"));
    println!("   - Contains mask prefixes: {}", property_result.contains("-webkit-mask"));
    println!("   - Contains clip-path prefixes: {}", property_result.contains("-webkit-clip-path"));
    
    println!("\n🎉 All autoprefixer examples completed successfully!");
    println!("==================================================");
    
    Ok(())
}

/// Generate large CSS for performance testing
fn generate_large_css() -> String {
    let mut css = String::new();
    
    // Generate 500 CSS rules with various properties
    for i in 0..500 {
        css.push_str(&format!(
            r#".rule-{} {{
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                transform: translateX({}px);
                transition: all 0.3s ease;
                animation: slideIn 1s ease-in-out;
                filter: blur({}px);
                mask: url(#mask-{});
                clip-path: circle({}%);
            }}
            "#,
            i, i * 2, i % 10, i, i % 100
        ));
    }
    
    css
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_prefixing() {
        let autoprefixer = Autoprefixer::new();
        let css = ".test { display: flex; }";
        let result = autoprefixer.add_prefixes(css, &["chrome 30".to_string()]);
        assert!(result.is_ok());
        let prefixed = result.unwrap();
        assert!(prefixed.contains("-webkit-"));
    }
    
    #[test]
    fn test_flexbox_prefixing() {
        let mut config = AutoprefixerConfig::default();
        config.flexbox = FlexboxMode::All;
        let autoprefixer = Autoprefixer::with_config(config);
        
        let css = ".container { display: flex; }";
        let result = autoprefixer.add_prefixes(css, &["ie 10".to_string()]);
        assert!(result.is_ok());
        let prefixed = result.unwrap();
        assert!(prefixed.contains("-ms-"));
    }
    
    #[test]
    fn test_complex_prefixing() {
        let autoprefixer = Autoprefixer::new();
        
        let css = r#"
            .container {
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            
            .item {
                flex: 1;
                transform: translateX(10px);
            }
        "#;
        
        let browsers = vec![
            "chrome 30".to_string(),
            "firefox 25".to_string(),
            "safari 7".to_string(),
        ];
        
        let result = autoprefixer.add_prefixes(css, &browsers);
        assert!(result.is_ok());
        
        let prefixed = result.unwrap();
        assert!(prefixed.contains("-webkit-"));
        assert!(prefixed.contains("-moz-"));
    }
    
    #[test]
    fn test_advanced_prefixing() {
        let autoprefixer = Autoprefixer::new();
        
        let css = r#"
            .container {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                grid-gap: 20px;
            }
        "#;
        
        let options = PrefixOptions {
            browsers: vec!["chrome 30".to_string(), "ie 10".to_string()],
            cascade: true,
            add: true,
            remove: false,
            supports: true,
            flexbox: FlexboxMode::All,
            grid: GridMode::Autoplace,
            stats: None,
        };
        
        let result = autoprefixer.add_prefixes_advanced(css, &options);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.statistics.properties_processed > 0);
        assert!(result.statistics.processing_time_ms > 0);
    }
}
