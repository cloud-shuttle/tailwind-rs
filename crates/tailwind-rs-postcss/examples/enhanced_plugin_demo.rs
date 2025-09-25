//! Example: Enhanced Plugin System
//! 
//! This example demonstrates how to use the EnhancedPluginLoader to load and execute
//! NPM plugins, native plugins, and manage plugin configurations.

use tailwind_rs_postcss::{EnhancedPluginLoader, PluginConfig, ExecutionMode, SecurityLevel, PluginCapability};
use std::collections::HashMap;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced Plugin System Demo");
    println!("==============================");
    
    // Example 1: Basic plugin loading
    println!("\n📝 Example 1: Basic plugin loading");
    let mut loader = EnhancedPluginLoader::new();
    
    let config = PluginConfig {
        name: "autoprefixer".to_string(),
        version: Some("10.4.0".to_string()),
        options: HashMap::new(),
        dependencies: vec![],
        execution_mode: ExecutionMode::NPM,
        security_level: SecurityLevel::Sandboxed,
    };
    
    let plugin = loader.load_npm_plugin("autoprefixer", &config)?;
    println!("✅ Loaded plugin: {}", plugin.name);
    println!("   - Version: {}", plugin.version);
    println!("   - Execution mode: {:?}", plugin.execution_mode);
    println!("   - Security level: {:?}", plugin.security_level);
    
    // Example 2: Plugin execution
    println!("\n📝 Example 2: Plugin execution");
    let css = r#"
        .test {
            display: flex;
            flex-direction: column;
            align-items: center;
        }
    "#;
    
    let result = loader.execute_plugin(&plugin, css)?;
    println!("✅ Plugin execution completed");
    println!("   - Success: {}", result.success);
    println!("   - Output size: {} bytes", result.css.len());
    println!("   - Memory usage: {} bytes", result.memory_usage);
    println!("   - Warnings: {}", result.warnings.len());
    println!("   - Errors: {}", result.errors.len());
    
    // Example 3: Plugin pipeline
    println!("\n📝 Example 3: Plugin pipeline");
    let mut plugins = Vec::new();
    
    // Create multiple plugin configurations
    let autoprefixer_config = PluginConfig {
        name: "autoprefixer".to_string(),
        version: Some("10.4.0".to_string()),
        options: HashMap::new(),
        dependencies: vec![],
        execution_mode: ExecutionMode::NPM,
        security_level: SecurityLevel::Sandboxed,
    };
    
    let cssnano_config = PluginConfig {
        name: "cssnano".to_string(),
        version: Some("5.1.0".to_string()),
        options: HashMap::new(),
        dependencies: vec![],
        execution_mode: ExecutionMode::NPM,
        security_level: SecurityLevel::Sandboxed,
    };
    
    // Load plugins
    let autoprefixer_plugin = loader.load_npm_plugin("autoprefixer", &autoprefixer_config)?;
    let cssnano_plugin = loader.load_npm_plugin("cssnano", &cssnano_config)?;
    
    plugins.push(autoprefixer_plugin);
    plugins.push(cssnano_plugin);
    
    // Execute plugin pipeline
    let pipeline_css = r#"
        .container {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            padding: 20px;
            margin: 10px;
        }
        
        .item {
            flex: 1;
            background: #f0f0f0;
            border: 1px solid #ccc;
            border-radius: 4px;
        }
    "#;
    
    let pipeline_result = loader.execute_plugin_pipeline(&plugins, pipeline_css)?;
    println!("✅ Plugin pipeline executed");
    println!("   - Processed CSS length: {} bytes", pipeline_result.len());
    
    // Example 4: Plugin discovery
    println!("\n📝 Example 4: Plugin discovery");
    let search_paths = vec![
        "./node_modules".to_string(),
        "./plugins".to_string(),
    ];
    
    let discovered_plugins = loader.discover_plugins(&search_paths)?;
    println!("✅ Plugin discovery completed");
    println!("   - Found {} plugins", discovered_plugins.len());
    
    for plugin_info in discovered_plugins {
        println!("   - Plugin: {} v{}", plugin_info.name, plugin_info.version);
        println!("     Description: {}", plugin_info.description);
        println!("     Author: {}", plugin_info.author);
        println!("     License: {}", plugin_info.license);
        println!("     Capabilities: {:?}", plugin_info.capabilities);
    }
    
    // Example 5: Plugin statistics
    println!("\n📝 Example 5: Plugin statistics");
    let statistics = loader.get_statistics();
    println!("✅ Plugin statistics:");
    println!("   - Total plugins: {}", statistics.total_plugins);
    println!("   - Total executions: {}", statistics.total_executions);
    println!("   - Average execution time: {}ms", statistics.average_execution_time.as_millis());
    println!("   - Performance alerts: {}", statistics.performance_alerts);
    
    // Example 6: Advanced plugin configuration
    println!("\n📝 Example 6: Advanced plugin configuration");
    let mut advanced_options = HashMap::new();
    advanced_options.insert("browsers".to_string(), Value::String("last 2 versions".to_string()));
    advanced_options.insert("cascade".to_string(), Value::Bool(true));
    
    let advanced_config = PluginConfig {
        name: "autoprefixer".to_string(),
        version: Some("10.4.0".to_string()),
        options: advanced_options,
        dependencies: vec![],
        execution_mode: ExecutionMode::NPM,
        security_level: SecurityLevel::Trusted,
    };
    
    let advanced_plugin = loader.load_npm_plugin("autoprefixer", &advanced_config)?;
    println!("✅ Advanced plugin configuration loaded");
    println!("   - Plugin: {}", advanced_plugin.name);
    println!("   - Options: {:?}", advanced_plugin.config.options);
    println!("   - Security level: {:?}", advanced_plugin.security_level);
    
    // Example 7: Plugin performance monitoring
    println!("\n📝 Example 7: Plugin performance monitoring");
    let performance_css = r#"
        .performance-test {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 20px;
            padding: 20px;
        }
        
        .grid-item {
            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
            border-radius: 8px;
            padding: 20px;
            color: white;
            text-align: center;
        }
    "#;
    
    let start_time = std::time::Instant::now();
    let performance_result = loader.execute_plugin(&advanced_plugin, performance_css)?;
    let execution_time = start_time.elapsed();
    
    println!("✅ Performance monitoring completed");
    println!("   - Execution time: {}ms", execution_time.as_millis());
    println!("   - Success: {}", performance_result.success);
    println!("   - Memory usage: {} bytes", performance_result.memory_usage);
    println!("   - Output size: {} bytes", performance_result.css.len());
    
    // Example 8: Plugin error handling
    println!("\n📝 Example 8: Plugin error handling");
    let invalid_config = PluginConfig {
        name: "nonexistent-plugin".to_string(),
        version: Some("999.999.999".to_string()),
        options: HashMap::new(),
        dependencies: vec![],
        execution_mode: ExecutionMode::NPM,
        security_level: SecurityLevel::Sandboxed,
    };
    
    match loader.load_npm_plugin("nonexistent-plugin", &invalid_config) {
        Ok(_) => println!("✅ Plugin loaded successfully (unexpected)"),
        Err(e) => println!("❌ Plugin loading failed as expected: {}", e),
    }
    
    // Example 9: Plugin capabilities
    println!("\n📝 Example 9: Plugin capabilities");
    let capabilities = vec![
        PluginCapability::Transform,
        PluginCapability::Optimization,
        PluginCapability::SourceMap,
    ];
    
    println!("✅ Available plugin capabilities:");
    for capability in capabilities {
        println!("   - {:?}", capability);
    }
    
    // Example 10: Plugin security levels
    println!("\n📝 Example 10: Plugin security levels");
    let security_levels = vec![
        SecurityLevel::Trusted,
        SecurityLevel::Sandboxed,
        SecurityLevel::Restricted,
    ];
    
    println!("✅ Available security levels:");
    for level in security_levels {
        println!("   - {:?}", level);
    }
    
    println!("\n🎉 All enhanced plugin system examples completed successfully!");
    println!("=============================================================");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_plugin_loader_creation() {
        let loader = EnhancedPluginLoader::new();
        let stats = loader.get_statistics();
        assert_eq!(stats.total_plugins, 0);
    }
    
    #[test]
    fn test_plugin_config_creation() {
        let config = PluginConfig {
            name: "test-plugin".to_string(),
            version: Some("1.0.0".to_string()),
            options: HashMap::new(),
            dependencies: vec![],
            execution_mode: ExecutionMode::NPM,
            security_level: SecurityLevel::Sandboxed,
        };
        
        assert_eq!(config.name, "test-plugin");
        assert_eq!(config.version, Some("1.0.0".to_string()));
        assert_eq!(config.execution_mode, ExecutionMode::NPM);
        assert_eq!(config.security_level, SecurityLevel::Sandboxed);
    }
    
    #[test]
    fn test_plugin_execution_modes() {
        let modes = vec![
            ExecutionMode::Native,
            ExecutionMode::NPM,
            ExecutionMode::WebAssembly,
            ExecutionMode::Sandboxed,
        ];
        
        for mode in modes {
            match mode {
                ExecutionMode::Native => assert!(true),
                ExecutionMode::NPM => assert!(true),
                ExecutionMode::WebAssembly => assert!(true),
                ExecutionMode::Sandboxed => assert!(true),
            }
        }
    }
    
    #[test]
    fn test_plugin_security_levels() {
        let levels = vec![
            SecurityLevel::Trusted,
            SecurityLevel::Sandboxed,
            SecurityLevel::Restricted,
        ];
        
        for level in levels {
            match level {
                SecurityLevel::Trusted => assert!(true),
                SecurityLevel::Sandboxed => assert!(true),
                SecurityLevel::Restricted => assert!(true),
            }
        }
    }
    
    #[test]
    fn test_plugin_capabilities() {
        let capabilities = vec![
            PluginCapability::Transform,
            PluginCapability::Parse,
            PluginCapability::Stringify,
            PluginCapability::SourceMap,
            PluginCapability::Optimization,
            PluginCapability::Linting,
            PluginCapability::Validation,
        ];
        
        for capability in capabilities {
            match capability {
                PluginCapability::Transform => assert!(true),
                PluginCapability::Parse => assert!(true),
                PluginCapability::Stringify => assert!(true),
                PluginCapability::SourceMap => assert!(true),
                PluginCapability::Optimization => assert!(true),
                PluginCapability::Linting => assert!(true),
                PluginCapability::Validation => assert!(true),
            }
        }
    }
}
