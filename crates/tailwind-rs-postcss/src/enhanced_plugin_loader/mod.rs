//! Enhanced Plugin System
//!
//! This module provides comprehensive plugin system functionality including
//! NPM plugin execution, advanced configuration, and ecosystem compatibility.

pub mod core;
pub mod npm_loader;
pub mod native_loader;
pub mod plugin_registry;
pub mod config_manager;
pub mod performance_monitor;
pub mod plugin_cache;
pub mod plugin_sandbox;
pub mod errors;

pub use core::EnhancedPluginLoader;
pub use npm_loader::NPMPluginLoader;
pub use native_loader::NativePluginLoader;
pub use plugin_registry::PluginRegistry;
pub use config_manager::PluginConfigManager;
pub use performance_monitor::PluginPerformanceMonitor;
pub use plugin_cache::PluginCache;
pub use plugin_sandbox::PluginSandbox;
pub use errors::PluginError;
