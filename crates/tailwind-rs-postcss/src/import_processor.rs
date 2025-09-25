//! @import Processing System
//! 
//! This module provides comprehensive @import statement processing functionality
//! for CSS imports, dependency resolution, and import optimization.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use regex::Regex;
use thiserror::Error;

/// Main import processor for handling @import statements
pub struct ImportProcessor {
    resolver: ImportResolver,
    cache: ImportCache,
    config: ImportConfig,
    dependency_graph: DependencyGraph,
}

impl ImportProcessor {
    /// Create a new import processor with default configuration
    pub fn new() -> Self {
        Self::with_config(ImportConfig::default())
    }
    
    /// Create a new import processor with custom configuration
    pub fn with_config(config: ImportConfig) -> Self {
        Self {
            resolver: ImportResolver::new(config.clone()),
            cache: ImportCache::new(),
            config,
            dependency_graph: DependencyGraph::new(),
        }
    }
    
    /// Process @import statements in CSS
    pub fn process_imports(&mut self, css: &str, base_path: &str) -> Result<String, ImportError> {
        let start_time = std::time::Instant::now();
        let mut processed_css = String::new();
        let mut processed_imports = Vec::new();
        let mut dependencies = Vec::new();
        
        for line in css.lines() {
            if line.trim().starts_with("@import") {
                let import_statement = self.parse_import_statement(line)?;
                let resolved_path = self.resolver.resolve(&import_statement.import_path, base_path)?;
                
                // Check for circular dependencies
                if let Err(circular) = self.dependency_graph.add_dependency(base_path, &resolved_path) {
                    match self.config.handle_circular {
                        CircularHandling::Error => return Err(circular),
                        CircularHandling::Warn => {
                            eprintln!("Warning: Circular dependency detected: {}", resolved_path);
                        },
                        CircularHandling::Ignore => {},
                    }
                }
                
                // Get import content
                let content = self.resolver.get_import_content(&import_statement.import_path, base_path)?;
                
                // Recursively process imports if enabled
                let processed_content = if self.config.inline_imports {
                    self.process_imports(&content, &resolved_path)?
                } else {
                    content
                };
                
                // Wrap in media query if needed
                let final_content = if let Some(media) = &import_statement.media_query {
                    format!("@media {} {{\n{}\n}}", media, processed_content)
                } else {
                    processed_content
                };
                
                processed_css.push_str(&final_content);
                processed_css.push('\n');
                
                let content_size = final_content.len();
                processed_imports.push(ImportInfo {
                    original_path: import_statement.import_path.clone(),
                    resolved_path: resolved_path.clone(),
                    content: final_content.clone(),
                    size: content_size,
                    processed: true,
                });
                
                dependencies.push(resolved_path);
            } else {
                processed_css.push_str(line);
                processed_css.push('\n');
            }
        }
        
        let _processing_time = start_time.elapsed();
        
        // Cache the result
        self.cache.cache_content(base_path.to_string(), processed_css.clone());
        
        Ok(processed_css)
    }
    
    /// Process imports with advanced options
    pub fn process_imports_advanced(&mut self, css: &str, options: &ImportOptions) -> Result<ImportResult, ImportError> {
        let start_time = std::time::Instant::now();
        let mut processed_css = String::new();
        let mut processed_imports = Vec::new();
        let mut dependencies = Vec::new();
        let mut circular_dependencies = Vec::new();
        
        for line in css.lines() {
            if line.trim().starts_with("@import") {
                let import_statement = self.parse_import_statement(line)?;
                let resolved_path = self.resolver.resolve(&import_statement.import_path, &options.search_paths[0])?;
                
                // Check for circular dependencies
                if let Err(circular) = self.dependency_graph.add_dependency(&options.search_paths[0], &resolved_path) {
                    circular_dependencies.push(circular.to_string());
                    match options.handle_circular {
                        CircularHandling::Error => return Err(circular),
                        CircularHandling::Warn => {
                            eprintln!("Warning: Circular dependency detected: {}", resolved_path);
                        },
                        CircularHandling::Ignore => {},
                    }
                }
                
                // Get import content
                let content = self.resolver.get_import_content(&import_statement.import_path, &options.search_paths[0])?;
                
                // Recursively process imports if enabled
                let processed_content = if options.inline_imports {
                    self.process_imports(&content, &resolved_path)?
                } else {
                    content
                };
                
                // Wrap in media query if needed
                let final_content = if let Some(media) = &import_statement.media_query {
                    format!("@media {} {{\n{}\n}}", media, processed_content)
                } else {
                    processed_content
                };
                
                processed_css.push_str(&final_content);
                processed_css.push('\n');
                
                let content_size = final_content.len();
                processed_imports.push(ImportInfo {
                    original_path: import_statement.import_path.clone(),
                    resolved_path: resolved_path.clone(),
                    content: final_content.clone(),
                    size: content_size,
                    processed: true,
                });
                
                dependencies.push(resolved_path);
            } else {
                processed_css.push_str(line);
                processed_css.push('\n');
            }
        }
        
        let processing_time = start_time.elapsed();
        let total_imports = processed_imports.len();
        let processed_imports_count = processed_imports.iter().filter(|i| i.processed).count();
        let skipped_imports = total_imports - processed_imports_count;
        let total_size = processed_css.len();
        
        Ok(ImportResult {
            processed_css,
            imports_processed: processed_imports,
            dependencies,
            circular_dependencies,
            statistics: ImportStatistics {
                total_imports,
                processed_imports: processed_imports_count,
                skipped_imports,
                total_size,
                processing_time,
            },
        })
    }
    
    /// Resolve import path
    pub fn resolve_import_path(&self, import_path: &str, base_path: &str) -> Result<String, ImportError> {
        self.resolver.resolve(import_path, base_path)
    }
    
    /// Parse import statement from CSS line
    fn parse_import_statement(&self, line: &str) -> Result<ImportStatement, ImportError> {
        let import_pattern = Regex::new(r#"@import\s+(?:url\()?["']?([^"')]+)["']?\)?(?:\s+([^;]+))?;"#).unwrap();
        
        if let Some(cap) = import_pattern.captures(line) {
            let import_path = cap.get(1).unwrap().as_str().to_string();
            let media_query = cap.get(2).map(|m| m.as_str().to_string());
            
            Ok(ImportStatement {
                line_number: 0, // Will be set by caller
                import_path,
                media_query,
            })
        } else {
            Err(ImportError::InvalidImportStatement { line: line.to_string() })
        }
    }
    
    /// Extract media query from import line
    fn extract_media_query(&self, line: &str) -> Option<String> {
        let media_pattern = Regex::new(r#"@import\s+[^;]+;\s*(.+)"#).unwrap();
        if let Some(cap) = media_pattern.captures(line) {
            Some(cap.get(1).unwrap().as_str().to_string())
        } else {
            None
        }
    }
    
    /// Optimize imports by removing duplicates and unused imports
    pub fn optimize_imports(&self, css: &str) -> Result<String, ImportError> {
        let mut optimized_css = String::new();
        let mut seen_imports = HashSet::new();
        
        for line in css.lines() {
            if line.trim().starts_with("@import") {
                let import_statement = self.parse_import_statement(line)?;
                if !seen_imports.contains(&import_statement.import_path) {
                    seen_imports.insert(import_statement.import_path);
                    optimized_css.push_str(line);
                    optimized_css.push('\n');
                }
            } else {
                optimized_css.push_str(line);
                optimized_css.push('\n');
            }
        }
        
        Ok(optimized_css)
    }
}

/// Import resolver for handling path resolution
pub struct ImportResolver {
    search_paths: Vec<String>,
    extensions: Vec<String>,
    config: ResolverConfig,
}

impl ImportResolver {
    /// Create new import resolver
    pub fn new(config: ImportConfig) -> Self {
        Self {
            search_paths: config.search_paths.clone(),
            extensions: config.extensions.clone(),
            config: ResolverConfig::default(),
        }
    }
    
    /// Resolve import path to file path
    pub fn resolve(&self, import_path: &str, base_path: &str) -> Result<String, ImportError> {
        // Handle different import path types
        if import_path.starts_with("http://") || import_path.starts_with("https://") {
            return Ok(import_path.to_string()); // External URL
        }
        
        if import_path.starts_with("//") {
            return Ok(format!("https:{}", import_path)); // Protocol-relative URL
        }
        
        if import_path.starts_with('/') {
            // Absolute path
            return Ok(import_path.to_string());
        }
        
        // Relative path
        let base_dir = Path::new(base_path).parent()
            .ok_or_else(|| ImportError::InvalidBasePath { path: base_path.to_string() })?;
        
        let resolved_path = base_dir.join(import_path);
        
        // Try different extensions
        for ext in &self.extensions {
            let path_with_ext = format!("{}{}", resolved_path.display(), ext);
            if Path::new(&path_with_ext).exists() {
                return Ok(path_with_ext);
            }
        }
        
        // Try without extension
        if Path::new(&resolved_path).exists() {
            return Ok(resolved_path.to_string_lossy().to_string());
        }
        
        // Try search paths
        for search_path in &self.search_paths {
            let full_path = Path::new(search_path).join(import_path);
            if Path::new(&full_path).exists() {
                return Ok(full_path.to_string_lossy().to_string());
            }
        }
        
        Err(ImportError::ImportNotFound { path: import_path.to_string() })
    }
    
    /// Check if import exists
    pub fn import_exists(&self, import_path: &str, base_path: &str) -> bool {
        self.resolve(import_path, base_path).is_ok()
    }
    
    /// Get import content
    pub fn get_import_content(&self, import_path: &str, base_path: &str) -> Result<String, ImportError> {
        let resolved_path = self.resolve(import_path, base_path)?;
        std::fs::read_to_string(&resolved_path)
            .map_err(|_| ImportError::FileReadError { path: resolved_path })
    }
}

/// Dependency graph for tracking import dependencies
pub struct DependencyGraph {
    nodes: HashMap<String, ImportNode>,
    edges: HashMap<String, Vec<String>>,
    visited: HashSet<String>,
}

impl DependencyGraph {
    /// Create new dependency graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            visited: HashSet::new(),
        }
    }
    
    /// Add import dependency
    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<(), ImportError> {
        // Check for circular dependency
        if self.has_circular_dependency(to, from) {
            return Err(ImportError::CircularDependency { path: to.to_string() });
        }
        
        self.edges.entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
        
        Ok(())
    }
    
    /// Check for circular dependencies
    pub fn has_circular_dependency(&self, start: &str, target: &str) -> bool {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        
        self.dfs_circular_detection(start, target, &mut visited, &mut recursion_stack)
    }
    
    /// DFS for circular dependency detection
    fn dfs_circular_detection(
        &self,
        node: &str,
        target: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
    ) -> bool {
        if node == target {
            return true;
        }
        
        if recursion_stack.contains(node) {
            return false;
        }
        
        if visited.contains(node) {
            return false;
        }
        
        visited.insert(node.to_string());
        recursion_stack.insert(node.to_string());
        
        if let Some(dependencies) = self.edges.get(node) {
            for dependency in dependencies {
                if self.dfs_circular_detection(dependency, target, visited, recursion_stack) {
                    return true;
                }
            }
        }
        
        recursion_stack.remove(node);
        false
    }
    
    /// Get import order (topological sort)
    pub fn get_import_order(&self, start: &str) -> Result<Vec<String>, ImportError> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut order = Vec::new();
        
        self.dfs_topological_sort(start, &mut visited, &mut recursion_stack, &mut order)?;
        
        Ok(order)
    }
    
    /// DFS for topological sort
    fn dfs_topological_sort(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), ImportError> {
        if recursion_stack.contains(node) {
            return Err(ImportError::CircularDependency { path: node.to_string() });
        }
        
        if visited.contains(node) {
            return Ok(());
        }
        
        visited.insert(node.to_string());
        recursion_stack.insert(node.to_string());
        
        if let Some(dependencies) = self.edges.get(node) {
            for dependency in dependencies {
                self.dfs_topological_sort(dependency, visited, recursion_stack, order)?;
            }
        }
        
        recursion_stack.remove(node);
        order.push(node.to_string());
        Ok(())
    }
}

/// Import cache for performance optimization
pub struct ImportCache {
    file_cache: HashMap<String, String>,
    dependency_cache: HashMap<String, Vec<String>>,
    processed_cache: HashMap<String, String>,
}

impl ImportCache {
    /// Create new import cache
    pub fn new() -> Self {
        Self {
            file_cache: HashMap::new(),
            dependency_cache: HashMap::new(),
            processed_cache: HashMap::new(),
        }
    }
    
    /// Get cached content
    pub fn get_cached_content(&self, path: &str) -> Option<&String> {
        self.file_cache.get(path)
    }
    
    /// Cache content
    pub fn cache_content(&mut self, path: String, content: String) {
        self.file_cache.insert(path, content);
    }
    
    /// Get cached dependencies
    pub fn get_cached_dependencies(&self, path: &str) -> Option<&Vec<String>> {
        self.dependency_cache.get(path)
    }
    
    /// Cache dependencies
    pub fn cache_dependencies(&mut self, path: String, dependencies: Vec<String>) {
        self.dependency_cache.insert(path, dependencies);
    }
    
    /// Get cached processed content
    pub fn get_cached_processed(&self, path: &str) -> Option<&String> {
        self.processed_cache.get(path)
    }
    
    /// Cache processed content
    pub fn cache_processed(&mut self, path: String, content: String) {
        self.processed_cache.insert(path, content);
    }
}

/// Import node for dependency graph
#[derive(Debug, Clone)]
pub struct ImportNode {
    pub path: String,
    pub dependencies: Vec<String>,
    pub processed: bool,
}

/// Import statement representation
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub line_number: usize,
    pub import_path: String,
    pub media_query: Option<String>,
}

/// Configuration for import processing
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub search_paths: Vec<String>,
    pub extensions: Vec<String>,
    pub inline_imports: bool,
    pub preserve_imports: bool,
    pub optimize_imports: bool,
    pub handle_circular: CircularHandling,
    pub max_depth: usize,
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![".".to_string()],
            extensions: vec![".css".to_string(), ".scss".to_string(), ".sass".to_string()],
            inline_imports: true,
            preserve_imports: false,
            optimize_imports: true,
            handle_circular: CircularHandling::Warn,
            max_depth: 10,
        }
    }
}

/// Circular dependency handling strategy
#[derive(Debug, Clone)]
pub enum CircularHandling {
    Error,
    Warn,
    Ignore,
}

/// Advanced import processing options
#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub search_paths: Vec<String>,
    pub extensions: Vec<String>,
    pub inline_imports: bool,
    pub preserve_imports: bool,
    pub optimize_imports: bool,
    pub handle_circular: CircularHandling,
    pub max_depth: usize,
    pub source_map: bool,
}

/// Result of import processing
#[derive(Debug, Clone)]
pub struct ImportResult {
    pub processed_css: String,
    pub imports_processed: Vec<ImportInfo>,
    pub dependencies: Vec<String>,
    pub circular_dependencies: Vec<String>,
    pub statistics: ImportStatistics,
}

/// Information about processed import
#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub original_path: String,
    pub resolved_path: String,
    pub content: String,
    pub size: usize,
    pub processed: bool,
}

/// Statistics for import processing
#[derive(Debug, Clone)]
pub struct ImportStatistics {
    pub total_imports: usize,
    pub processed_imports: usize,
    pub skipped_imports: usize,
    pub total_size: usize,
    pub processing_time: std::time::Duration,
}

/// Resolver configuration
#[derive(Debug, Clone)]
pub struct ResolverConfig {
    pub follow_symlinks: bool,
    pub case_sensitive: bool,
    pub allow_external: bool,
}

impl Default for ResolverConfig {
    fn default() -> Self {
        Self {
            follow_symlinks: true,
            case_sensitive: false,
            allow_external: true,
        }
    }
}

/// Error types for import processing
#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Import not found: {path}")]
    ImportNotFound { path: String },
    
    #[error("Circular dependency detected: {path}")]
    CircularDependency { path: String },
    
    #[error("Invalid base path: {path}")]
    InvalidBasePath { path: String },
    
    #[error("Import depth exceeded: {depth}")]
    ImportDepthExceeded { depth: usize },
    
    #[error("Failed to read import file: {path}")]
    FileReadError { path: String },
    
    #[error("Invalid import statement: {line}")]
    InvalidImportStatement { line: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_import_processing() {
        let mut processor = ImportProcessor::new();
        let css = "@import 'styles.css';";
        let result = processor.process_imports(css, "/path/to/base");
        // This will fail in test environment due to missing files, but tests the structure
        assert!(result.is_err());
    }
    
    #[test]
    fn test_import_statement_parsing() {
        let processor = ImportProcessor::new();
        let line = "@import 'styles.css' screen and (max-width: 768px);";
        let statement = processor.parse_import_statement(line);
        assert!(statement.is_ok());
        
        let statement = statement.unwrap();
        assert_eq!(statement.import_path, "styles.css");
        assert_eq!(statement.media_query, Some("screen and (max-width: 768px)".to_string()));
    }
    
    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        // Add dependencies
        assert!(graph.add_dependency("a.css", "b.css").is_ok());
        assert!(graph.add_dependency("b.css", "c.css").is_ok());
        
        // Test circular dependency
        assert!(graph.add_dependency("c.css", "a.css").is_err());
    }
    
    #[test]
    fn test_import_cache() {
        let mut cache = ImportCache::new();
        cache.cache_content("test.css".to_string(), "body { color: red; }".to_string());
        
        let cached = cache.get_cached_content("test.css");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), "body { color: red; }");
    }
    
    #[test]
    fn test_import_optimization() {
        let processor = ImportProcessor::new();
        let css = r#"
            @import 'styles.css';
            @import 'styles.css';
            @import 'components.css';
            .custom { color: red; }
        "#;
        
        let result = processor.optimize_imports(css);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        let import_count = optimized.matches("@import").count();
        assert_eq!(import_count, 2); // Duplicate removed
    }
}
