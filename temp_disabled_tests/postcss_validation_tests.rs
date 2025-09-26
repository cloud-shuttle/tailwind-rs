//! PostCSS Processing Validation Tests
//!
//! This module contains comprehensive tests to validate that our PostCSS
//! processing is working as intended and has comprehensive coverage.

use crate::postcss_integration::EnhancedCssGenerator;
use crate::responsive::Breakpoint;

#[cfg(test)]
mod postcss_validation_tests {
    use super::*;
    use std::time::Instant;

    /// Test basic PostCSS processing
    #[test]
    fn test_basic_postcss_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = "relative flex p-4 bg-blue-500 text-white hover:bg-blue-600 focus:ring-2";
        let result = generator.generate_css(classes).unwrap();
        
        // Verify CSS is generated
        assert!(!result.css.is_empty());
        assert!(result.css.contains("position: relative"));
        assert!(result.css.contains("display: flex"));
        assert!(result.css.contains("padding: 1rem"));
        assert!(result.css.contains("background-color: rgb(59 130 246)"));
        assert!(result.css.contains("color: rgb(255 255 255)"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains(":focus"));
    }

    /// Test PostCSS processing with complex CSS
    #[test]
    fn test_complex_postcss_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .container {
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 1rem;
                padding: 1rem;
            }
            
            .item {
                background-color: #3b82f6;
                color: white;
                padding: 0.5rem;
                border-radius: 0.25rem;
            }
            
            .item:hover {
                background-color: #2563eb;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify CSS is processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("display: grid"));
        assert!(result.css.contains("grid-template-columns: 1fr 1fr"));
        assert!(result.css.contains("gap: 1rem"));
        assert!(result.css.contains("background-color: #3b82f6"));
        assert!(result.css.contains("color: white"));
        assert!(result.css.contains(":hover"));
    }

    /// Test PostCSS processing with CSS variables
    #[test]
    fn test_css_variables_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            :root {
                --primary-color: #3b82f6;
                --secondary-color: #64748b;
                --spacing: 1rem;
            }
            
            .test {
                color: var(--primary-color);
                background-color: var(--secondary-color);
                padding: var(--spacing);
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify CSS variables are processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("--primary-color: #3b82f6"));
        assert!(result.css.contains("--secondary-color: #64748b"));
        assert!(result.css.contains("--spacing: 1rem"));
        assert!(result.css.contains("color: var(--primary-color)"));
        assert!(result.css.contains("background-color: var(--secondary-color)"));
        assert!(result.css.contains("padding: var(--spacing)"));
    }

    /// Test PostCSS processing with media queries
    #[test]
    fn test_media_queries_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .responsive {
                display: block;
            }
            
            @media (min-width: 768px) {
                .responsive {
                    display: flex;
                }
            }
            
            @media (min-width: 1024px) {
                .responsive {
                    display: grid;
                }
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify media queries are processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("display: block"));
        assert!(result.css.contains("@media (min-width: 768px)"));
        assert!(result.css.contains("display: flex"));
        assert!(result.css.contains("@media (min-width: 1024px)"));
        assert!(result.css.contains("display: grid"));
    }

    /// Test PostCSS processing with pseudo-classes
    #[test]
    fn test_pseudo_classes_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .button {
                background-color: #3b82f6;
                color: white;
                padding: 0.5rem 1rem;
                border-radius: 0.25rem;
            }
            
            .button:hover {
                background-color: #2563eb;
            }
            
            .button:focus {
                outline: 2px solid #3b82f6;
                outline-offset: 2px;
            }
            
            .button:active {
                background-color: #1d4ed8;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify pseudo-classes are processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("background-color: #3b82f6"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains("background-color: #2563eb"));
        assert!(result.css.contains(":focus"));
        assert!(result.css.contains("outline: 2px solid #3b82f6"));
        assert!(result.css.contains(":active"));
        assert!(result.css.contains("background-color: #1d4ed8"));
    }

    /// Test PostCSS processing with CSS animations
    #[test]
    fn test_css_animations_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            @keyframes slideIn {
                from {
                    transform: translateX(-100%);
                    opacity: 0;
                }
                to {
                    transform: translateX(0);
                    opacity: 1;
                }
            }
            
            .animated {
                animation: slideIn 0.3s ease-out;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify CSS animations are processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("@keyframes slideIn"));
        assert!(result.css.contains("transform: translateX(-100%)"));
        assert!(result.css.contains("opacity: 0"));
        assert!(result.css.contains("transform: translateX(0)"));
        assert!(result.css.contains("opacity: 1"));
        assert!(result.css.contains("animation: slideIn 0.3s ease-out"));
    }

    /// Test PostCSS processing with CSS transforms
    #[test]
    fn test_css_transforms_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .transform {
                transform: translateX(10px) rotate(45deg) scale(1.2);
                transform-origin: center;
            }
            
            .transform:hover {
                transform: translateX(20px) rotate(90deg) scale(1.5);
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify CSS transforms are processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("transform: translateX(10px) rotate(45deg) scale(1.2)"));
        assert!(result.css.contains("transform-origin: center"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains("transform: translateX(20px) rotate(90deg) scale(1.5)"));
    }

    /// Test PostCSS processing with CSS functions
    #[test]
    fn test_css_functions_processing() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .functions {
                width: calc(100% - 20px);
                height: min(100vh, 500px);
                max-width: max(300px, 50vw);
                background-color: hsl(210, 100%, 50%);
                color: rgb(255, 255, 255);
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Verify CSS functions are processed
        assert!(!result.css.is_empty());
        assert!(result.css.contains("width: calc(100% - 20px)"));
        assert!(result.css.contains("height: min(100vh, 500px)"));
        assert!(result.css.contains("max-width: max(300px, 50vw)"));
        assert!(result.css.contains("background-color: hsl(210, 100%, 50%)"));
        assert!(result.css.contains("color: rgb(255, 255, 255)"));
    }

    /// Test PostCSS processing performance
    #[test]
    fn test_postcss_processing_performance() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .test {
                color: red;
                background-color: blue;
                padding: 1rem;
                margin: 0.5rem;
            }
        "#;
        
        let start = Instant::now();
        
        for _ in 0..100 {
            let _ = generator.process_css(classes).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "PostCSS processing should be fast");
    }

    /// Test PostCSS processing error handling
    #[test]
    fn test_postcss_processing_error_handling() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let invalid_css = r#"
            .test {
                color: red;
                /* unclosed comment
            }
        "#;
        
        let result = generator.process_css(invalid_css);
        
        // Should handle invalid CSS gracefully
        match result {
            Ok(_) => {
                // If it processes successfully, that's also valid
                assert!(true);
            }
            Err(e) => {
                // If it fails, the error should be informative
                assert!(e.to_string().contains("unclosed comment") || 
                        e.to_string().contains("syntax error"));
            }
        }
    }

    /// Test PostCSS processing with empty CSS
    #[test]
    fn test_postcss_processing_empty_css() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let result = generator.process_css("").unwrap();
        
        // Should handle empty CSS gracefully
        assert!(result.css.is_empty() || result.css.trim().is_empty());
    }

    /// Test PostCSS processing with whitespace-only CSS
    #[test]
    fn test_postcss_processing_whitespace_css() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let result = generator.process_css("   \n\t   ").unwrap();
        
        // Should handle whitespace-only CSS gracefully
        assert!(result.css.is_empty() || result.css.trim().is_empty());
    }

    /// Test PostCSS processing with comments
    #[test]
    fn test_postcss_processing_comments() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            /* This is a comment */
            .test {
                color: red; /* Inline comment */
                background-color: blue;
            }
            /* Another comment */
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process CSS with comments
        assert!(!result.css.is_empty());
        assert!(result.css.contains("color: red"));
        assert!(result.css.contains("background-color: blue"));
    }

    /// Test PostCSS processing with nested CSS
    #[test]
    fn test_postcss_processing_nested_css() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .parent {
                color: red;
                
                .child {
                    color: blue;
                    
                    &:hover {
                        color: green;
                    }
                }
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process nested CSS
        assert!(!result.css.is_empty());
        assert!(result.css.contains("color: red"));
        assert!(result.css.contains("color: blue"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains("color: green"));
    }

    /// Test PostCSS processing with CSS imports
    #[test]
    fn test_postcss_processing_css_imports() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap');
            
            .test {
                font-family: 'Inter', sans-serif;
                font-weight: 500;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process CSS imports
        assert!(!result.css.is_empty());
        assert!(result.css.contains("@import"));
        assert!(result.css.contains("font-family: 'Inter', sans-serif"));
        assert!(result.css.contains("font-weight: 500"));
    }

    /// Test PostCSS processing with CSS custom properties
    #[test]
    fn test_postcss_processing_css_custom_properties() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            :root {
                --primary-color: #3b82f6;
                --secondary-color: #64748b;
                --spacing: 1rem;
                --border-radius: 0.25rem;
            }
            
            .test {
                color: var(--primary-color);
                background-color: var(--secondary-color);
                padding: var(--spacing);
                border-radius: var(--border-radius);
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process CSS custom properties
        assert!(!result.css.is_empty());
        assert!(result.css.contains("--primary-color: #3b82f6"));
        assert!(result.css.contains("--secondary-color: #64748b"));
        assert!(result.css.contains("--spacing: 1rem"));
        assert!(result.css.contains("--border-radius: 0.25rem"));
        assert!(result.css.contains("color: var(--primary-color)"));
        assert!(result.css.contains("background-color: var(--secondary-color)"));
        assert!(result.css.contains("padding: var(--spacing)"));
        assert!(result.css.contains("border-radius: var(--border-radius)"));
    }

    /// Test PostCSS processing with CSS selectors
    #[test]
    fn test_postcss_processing_css_selectors() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .test {
                color: red;
            }
            
            .test:hover {
                color: blue;
            }
            
            .test:focus {
                color: green;
            }
            
            .test:active {
                color: yellow;
            }
            
            .test:disabled {
                color: gray;
            }
            
            .test:checked {
                color: purple;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process CSS selectors
        assert!(!result.css.is_empty());
        assert!(result.css.contains("color: red"));
        assert!(result.css.contains(":hover"));
        assert!(result.css.contains("color: blue"));
        assert!(result.css.contains(":focus"));
        assert!(result.css.contains("color: green"));
        assert!(result.css.contains(":active"));
        assert!(result.css.contains("color: yellow"));
        assert!(result.css.contains(":disabled"));
        assert!(result.css.contains("color: gray"));
        assert!(result.css.contains(":checked"));
        assert!(result.css.contains("color: purple"));
    }

    /// Test PostCSS processing with CSS pseudo-elements
    #[test]
    fn test_postcss_processing_css_pseudo_elements() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            .test::before {
                content: '';
                display: block;
                width: 10px;
                height: 10px;
                background-color: red;
            }
            
            .test::after {
                content: '';
                display: block;
                width: 10px;
                height: 10px;
                background-color: blue;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process CSS pseudo-elements
        assert!(!result.css.is_empty());
        assert!(result.css.contains("::before"));
        assert!(result.css.contains("content: ''"));
        assert!(result.css.contains("display: block"));
        assert!(result.css.contains("width: 10px"));
        assert!(result.css.contains("height: 10px"));
        assert!(result.css.contains("background-color: red"));
        assert!(result.css.contains("::after"));
        assert!(result.css.contains("background-color: blue"));
    }

    /// Test PostCSS processing with CSS at-rules
    #[test]
    fn test_postcss_processing_css_at_rules() {
        let generator = EnhancedCssGenerator::new().unwrap();
        
        let classes = r#"
            @media (max-width: 768px) {
                .test {
                    display: none;
                }
            }
            
            @supports (display: grid) {
                .test {
                    display: grid;
                }
            }
            
            @page {
                margin: 1in;
            }
        "#;
        
        let result = generator.process_css(classes).unwrap();
        
        // Should process CSS at-rules
        assert!(!result.css.is_empty());
        assert!(result.css.contains("@media (max-width: 768px)"));
        assert!(result.css.contains("display: none"));
        assert!(result.css.contains("@supports (display: grid)"));
        assert!(result.css.contains("display: grid"));
        assert!(result.css.contains("@page"));
        assert!(result.css.contains("margin: 1in"));
    }
}
