//! Plugin Errors
//!
//! This module provides error types for plugin functionality.

use thiserror::Error;

/// Errors that can occur during plugin operations
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    
    #[error("Plugin load error: {0}")]
    PluginLoadError(String),
    
    #[error("Plugin execution error: {0}")]
    PluginExecutionError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Dependency error: {0}")]
    DependencyError(String),
    
    #[error("Security error: {0}")]
    SecurityError(String),
    
    #[error("Performance error: {0}")]
    PerformanceError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
