# Tailwind-RS v2.0 API Specifications

## Overview

This document defines the complete API specifications for Tailwind-RS v2.0, ensuring backward compatibility with v0.9.1 while introducing powerful new features for PostCSS integration, content scanning, and enhanced plugin support.

## Core API Architecture

### Primary APIs

```rust
// Re-exported from tailwind-rs-core
pub use tailwind_rs_core::{
    ClassBuilder,
    TailwindConfig, 
    TailwindEngine,
    PluginManager,
    ContentScanner,
};

// Framework-specific integrations
pub use tailwind_rs_leptos::*;
pub use tailwind_rs_yew::*;
pub use tailwind_rs_dioxus::*;
```

## 1. Enhanced ClassBuilder API

### Current API (v0.9.1 - Maintained for Compatibility)
```rust
impl ClassBuilder {
    pub fn new() -> Self;
    pub fn padding(self, value: impl Into<SpacingValue>) -> Self;
    pub fn margin(self, value: impl Into<SpacingValue>) -> Self;
    pub fn background_color(self, color: Color) -> Self;
    pub fn text_color(self, color: Color) -> Self;
    pub fn hover<F>(self, builder: F) -> Self where F: FnOnce(ClassBuilder) -> ClassBuilder;
    pub fn build(self) -> Vec<String>;
    pub fn to_string(&self) -> String;
}
```

### New Enhanced API (v2.0)
```rust
impl ClassBuilder {
    // Existing methods remain unchanged for backward compatibility
    
    /// Enhanced responsive design with fluid breakpoints
    pub fn responsive<F>(self, config: ResponsiveConfig<F>) -> Self
    where
        F: FnOnce(ResponsiveBuilder) -> ResponsiveBuilder,
    {
        // Implementation details...
    }
    
    /// Advanced variant system with composition
    pub fn variant<F>(self, variant: impl Into<Variant>, builder: F) -> Self
    where
        F: FnOnce(ClassBuilder) -> ClassBuilder,
    {
        // Implementation details...
    }
    
    /// Multi-variant composition for complex states
    pub fn variants<F>(self, variants: &[Variant], builder: F) -> Self
    where
        F: FnOnce(ClassBuilder) -> ClassBuilder,
    {
        // Implementation details...
    }
    
    /// Group interaction variants
    pub fn group<F>(self, state: GroupState, builder: F) -> Self
    where
        F: FnOnce(ClassBuilder) -> ClassBuilder,
    {
        self.variant(Variant::Group(state), builder)
    }
    
    /// Peer interaction variants  
    pub fn peer<F>(self, state: PeerState, builder: F) -> Self
    where
        F: FnOnce(ClassBuilder) -> ClassBuilder,
    {
        self.variant(Variant::Peer(state), builder)
    }
    
    /// Container query variants
    pub fn container<F>(self, query: ContainerQuery, builder: F) -> Self
    where
        F: FnOnce(ClassBuilder) -> ClassBuilder,
    {
        self.variant(Variant::Container(query), builder)
    }
    
    /// Arbitrary values with validation
    pub fn arbitrary<T>(self, property: &str, value: T) -> Self
    where
        T: Into<ArbitraryValue>,
    {
        // Implementation with runtime validation
    }
    
    /// Plugin-defined utilities
    pub fn plugin_utility<P, V>(self, plugin: P, utility: &str, value: Option<V>) -> Self
    where
        P: Into<PluginName>,
        V: Into<UtilityValue>,
    {
        // Implementation details...
    }
    
    /// Conditional class application
    pub fn when(self, condition: bool, classes: &str) -> Self {
        if condition {
            self.add_classes(classes)
        } else {
            self
        }
    }
    
    /// Merge with another ClassBuilder
    pub fn merge(self, other: ClassBuilder) -> Self {
        // Implementation details...
    }
    
    /// Convert to optimized CSS string
    pub fn to_css(&self) -> Result<String, CSSGenerationError> {
        // Generate actual CSS rather than just class names
    }
    
    /// Get performance metrics
    pub fn metrics(&self) -> ClassBuilderMetrics {
        // Return compilation time, memory usage, etc.
    }
}
```

### Enhanced Type System
```rust
/// Comprehensive variant system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Variant {
    /// Responsive breakpoints (sm, md, lg, xl, 2xl)
    Responsive(Breakpoint),
    
    /// Pseudo-classes (:hover, :focus, :active, etc.)
    PseudoClass(PseudoClass),
    
    /// Pseudo-elements (::before, ::after, etc.) 
    PseudoElement(PseudoElement),
    
    /// Group states (group-hover, group-focus, etc.)
    Group(GroupState),
    
    /// Peer states (peer-checked, peer-invalid, etc.)
    Peer(PeerState),
    
    /// Dark mode
    Dark,
    
    /// Print styles
    Print,
    
    /// Motion preferences
    Motion(MotionPreference),
    
    /// Container queries
    Container(ContainerQuery),
    
    /// Supports queries
    Supports(SupportsQuery),
    
    /// Arbitrary selectors
    Arbitrary(ArbitrarySelector),
}

/// Enhanced responsive configuration
#[derive(Debug, Clone)]
pub struct ResponsiveConfig<F> {
    pub breakpoints: Vec<Breakpoint>,
    pub builder: F,
    pub strategy: ResponsiveStrategy,
}

/// Responsive generation strategies
#[derive(Debug, Clone)]
pub enum ResponsiveStrategy {
    MobileFirst,
    DesktopFirst,
    ContainerQueries,
}

/// Group interaction states
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupState {
    Hover,
    Focus, 
    Active,
    Disabled,
    Invalid,
    Valid,
    Checked,
    FocusWithin,
    FocusVisible,
}

/// Peer interaction states
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PeerState {
    Hover,
    Focus,
    Checked,
    Disabled,
    Invalid,
    Valid,
    Required,
    Optional,
    ReadOnly,
    ReadWrite,
}

/// Container query definitions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerQuery {
    pub name: Option<String>,
    pub condition: ContainerCondition,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContainerCondition {
    MinWidth(Length),
    MaxWidth(Length),
    Width(Length),
    Height(Length),
    AspectRatio(f32),
    Orientation(Orientation),
}

/// Motion preferences for accessibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MotionPreference {
    Safe,   // prefers-reduced-motion: no-preference
    Reduce, // prefers-reduced-motion: reduce
}
```

## 2. TailwindEngine API

### Main Engine Interface
```rust
/// The primary Tailwind-RS processing engine
pub struct TailwindEngine {
    config: Arc<TailwindConfig>,
    postcss_engine: Option<PostCSSEngine>,
    content_scanner: ContentScanner,
    plugin_manager: PluginManager,
    cache: Arc<RwLock<EngineCache>>,
}

impl TailwindEngine {
    /// Create a new engine with configuration
    pub fn new(config: TailwindConfig) -> Result<Self, EngineError> {
        // Implementation details...
    }
    
    /// Create engine from configuration file
    pub async fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self, EngineError> {
        let config = TailwindConfig::from_file(path).await?;
        Self::new(config)
    }
    
    /// Generate CSS from class list
    pub async fn generate_css_from_classes(&self, classes: &[String]) -> Result<String, CSSError> {
        // Implementation details...
    }
    
    /// Scan content and generate CSS
    pub async fn generate_css_from_content(&self) -> Result<GeneratedCSS, CSSError> {
        let classes = self.content_scanner.scan_for_classes().await?;
        let css = self.generate_css_from_classes(&classes.to_vec()).await?;
        
        Ok(GeneratedCSS {
            css,
            classes: classes.len(),
            size: css.len(),
            generation_time: /* ... */,
            source_map: /* ... */,
        })
    }
    
    /// Watch for content changes and regenerate CSS
    pub fn watch(&self) -> impl Stream<Item = Result<GeneratedCSS, CSSError>> {
        // Implementation details...
    }
    
    /// Get engine metrics and statistics
    pub fn metrics(&self) -> EngineMetrics {
        // Implementation details...
    }
    
    /// Validate class names
    pub fn validate_classes(&self, classes: &[String]) -> ValidationResult {
        // Implementation details...
    }
}

/// Generated CSS result with metadata
#[derive(Debug, Clone)]
pub struct GeneratedCSS {
    pub css: String,
    pub classes: usize,
    pub size: usize, 
    pub generation_time: Duration,
    pub source_map: Option<SourceMap>,
}

/// Engine performance metrics
#[derive(Debug, Clone)]
pub struct EngineMetrics {
    pub total_classes_processed: usize,
    pub cache_hit_rate: f32,
    pub average_generation_time: Duration,
    pub memory_usage: usize,
    pub plugins_loaded: usize,
}
```

## 3. Configuration API

### Enhanced Configuration System
```rust
/// Comprehensive Tailwind configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailwindConfig {
    /// Content scanning configuration
    pub content: ContentConfig,
    
    /// Theme customization
    pub theme: ThemeConfig,
    
    /// Plugin configuration
    pub plugins: Vec<PluginConfig>,
    
    /// CSS generation options
    pub css_options: CSSOptions,
    
    /// Development options
    pub dev_options: DevOptions,
    
    /// Compatibility options
    pub compatibility: CompatibilityOptions,
}

impl TailwindConfig {
    /// Load configuration from various sources
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let extension = path.as_ref().extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "js" | "mjs" | "cjs" => Self::from_js_config(path).await,
            "ts" => Self::from_ts_config(path).await,
            "toml" => Self::from_toml_config(path).await,
            "json" => Self::from_json_config(path).await,
            _ => Err(ConfigError::UnsupportedFormat(extension.to_string())),
        }
    }
    
    /// JavaScript configuration support
    pub async fn from_js_config<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        // Use V8 or Node.js to execute JavaScript config
    }
    
    /// Default configuration with sensible defaults
    pub fn default() -> Self {
        Self {
            content: ContentConfig::default(),
            theme: ThemeConfig::default(), 
            plugins: vec![],
            css_options: CSSOptions::default(),
            dev_options: DevOptions::default(),
            compatibility: CompatibilityOptions::default(),
        }
    }
    
    /// Merge configurations (for extending presets)
    pub fn merge(self, other: Self) -> Self {
        // Deep merge implementation
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<ConfigError>> {
        // Validation logic
    }
}

/// Content scanning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    pub files: Vec<String>,
    pub extract: ExtractConfig,
    pub transform: HashMap<String, String>,
}

/// Class extraction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractConfig {
    pub default_extractors: Vec<ExtractorConfig>,
    pub custom_extractors: HashMap<String, ExtractorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct ExtractorConfig {
    pub extensions: Vec<String>,
    pub patterns: Vec<String>,
    pub transform: Option<String>,
}
```

## 4. Plugin API

### Plugin Development Interface
```rust
/// Core plugin trait that all plugins must implement
pub trait TailwindPlugin: Send + Sync + Debug {
    /// Plugin identification
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str { "" }
    
    /// Plugin capabilities
    fn add_utilities(&self) -> Result<Vec<Utility>, PluginError> { Ok(vec![]) }
    fn add_components(&self) -> Result<Vec<Component>, PluginError> { Ok(vec![]) }
    fn add_variants(&self) -> Result<Vec<VariantDefinition>, PluginError> { Ok(vec![]) }
    fn add_base(&self) -> Result<Vec<BaseStyle>, PluginError> { Ok(vec![]) }
    
    /// Configuration hooks
    fn configure_theme(&self, theme: &mut ThemeConfig) -> Result<(), PluginError> { Ok(()) }
    fn configure_variants(&self, variants: &mut VariantConfig) -> Result<(), PluginError> { Ok(()) }
    
    /// CSS processing hooks
    fn process_css(&self, css: &str) -> Result<String, PluginError> { Ok(css.to_string()) }
    
    /// Lifecycle hooks
    fn initialize(&mut self, config: &TailwindConfig) -> Result<(), PluginError> { Ok(()) }
    fn finalize(&mut self) -> Result<(), PluginError> { Ok(()) }
}

/// Utility definition for plugins
#[derive(Debug, Clone)]
pub struct Utility {
    pub name: String,
    pub css_properties: HashMap<String, String>,
    pub responsive: bool,
    pub variants: Vec<String>,
    pub arbitrary_value: Option<ArbitraryValueHandler>,
}

/// Component definition for plugins
#[derive(Debug, Clone)]
pub struct Component {
    pub selector: String,
    pub styles: ComponentStyles,
    pub variants: Vec<String>,
    pub responsive: bool,
}

/// Enhanced plugin manager
pub struct PluginManager {
    plugins: Vec<Box<dyn TailwindPlugin>>,
    npm_bridge: Option<NPMBridge>,
    plugin_cache: HashMap<String, PluginResult>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            npm_bridge: None,
            plugin_cache: HashMap::new(),
        }
    }
    
    /// Register a native Rust plugin
    pub fn register(&mut self, plugin: Box<dyn TailwindPlugin>) -> Result<(), PluginError> {
        // Validate and register plugin
    }
    
    /// Load plugin from NPM package
    pub async fn load_npm_plugin(&mut self, package: &str, version: Option<&str>) -> Result<(), PluginError> {
        let npm_bridge = self.npm_bridge.get_or_insert_with(|| NPMBridge::new());
        let plugin = npm_bridge.load_plugin(package, version).await?;
        self.register(plugin)
    }
    
    /// Load all plugins from configuration
    pub async fn load_from_config(&mut self, plugins: &[PluginConfig]) -> Result<(), PluginError> {
        for plugin_config in plugins {
            match &plugin_config.source {
                PluginSource::NPM { package, version } => {
                    self.load_npm_plugin(package, version.as_deref()).await?;
                }
                PluginSource::Path(path) => {
                    self.load_plugin_from_path(path).await?;
                }
                PluginSource::Registry(name) => {
                    self.load_plugin_from_registry(name).await?;
                }
            }
        }
        Ok(())
    }
}
```

## 5. Content Scanning API

### Advanced Content Scanner
```rust
/// Content scanning and class extraction engine
pub struct ContentScanner {
    config: ContentConfig,
    file_watcher: Option<FileWatcher>,
    extractors: HashMap<String, Box<dyn ClassExtractor>>,
    cache: Arc<RwLock<ScanCache>>,
}

impl ContentScanner {
    pub fn new(config: ContentConfig) -> Self {
        // Implementation details...
    }
    
    /// Scan all configured content sources
    pub async fn scan_for_classes(&self) -> Result<ClassSet, ScanError> {
        let mut all_classes = HashSet::new();
        
        for file_pattern in &self.config.files {
            let matching_files = glob::glob(file_pattern)?;
            
            for file_path in matching_files {
                let file_path = file_path?;
                let classes = self.extract_classes_from_file(&file_path).await?;
                all_classes.extend(classes);
            }
        }
        
        Ok(ClassSet::new(all_classes))
    }
    
    /// Extract classes from a specific file
    pub async fn extract_classes_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<String>, ScanError> {
        let content = tokio::fs::read_to_string(&path).await?;
        let extension = path.as_ref().extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let extractor = self.extractors.get(extension)
            .or_else(|| self.extractors.get("default"))
            .ok_or_else(|| ScanError::NoExtractorFound(extension.to_string()))?;
        
        extractor.extract(&content)
    }
    
    /// Watch for file changes and emit class updates
    pub fn watch(&mut self) -> Result<impl Stream<Item = ClassSet>, ScanError> {
        let watcher = FileWatcher::new(self.config.files.clone())?;
        
        Ok(watcher.watch()
            .map(|change_event| async move {
                match change_event {
                    ChangeEvent::FileChanged(path) => {
                        self.extract_classes_from_file(&path).await
                    }
                    ChangeEvent::FileDeleted(_) => {
                        // Trigger full rescan
                        self.scan_for_classes().await
                    }
                    ChangeEvent::FileAdded(path) => {
                        self.extract_classes_from_file(&path).await
                    }
                }
            })
            .buffer_unordered(10)
            .filter_map(|result| async { result.ok() })
            .map(|classes| ClassSet::new(classes.into_iter().collect())))
    }
}

/// Class extraction trait for different file types
pub trait ClassExtractor: Send + Sync + Debug {
    fn extract(&self, content: &str) -> Result<Vec<String>, ExtractionError>;
    fn supported_extensions(&self) -> &[&str];
}

/// Set of extracted classes with metadata
#[derive(Debug, Clone)]
pub struct ClassSet {
    classes: HashSet<String>,
    metadata: ClassSetMetadata,
}

impl ClassSet {
    pub fn new(classes: HashSet<String>) -> Self {
        Self {
            metadata: ClassSetMetadata {
                total_classes: classes.len(),
                extraction_time: Instant::now(),
            },
            classes,
        }
    }
    
    pub fn classes(&self) -> &HashSet<String> { &self.classes }
    pub fn len(&self) -> usize { self.classes.len() }
    pub fn is_empty(&self) -> bool { self.classes.is_empty() }
    pub fn contains(&self, class: &str) -> bool { self.classes.contains(class) }
    
    pub fn to_vec(self) -> Vec<String> {
        self.classes.into_iter().collect()
    }
}
```

## 6. Framework Integration APIs

### Leptos Integration Enhanced
```rust
/// Enhanced Leptos integration with reactive features
pub mod leptos {
    use leptos::prelude::*;
    use crate::*;
    
    /// Reactive class builder for Leptos
    pub struct ReactiveClassBuilder {
        inner: ClassBuilder,
        signals: Vec<Box<dyn Fn() -> String>>,
    }
    
    impl ReactiveClassBuilder {
        pub fn new() -> Self { /* ... */ }
        
        /// Add reactive classes based on signals
        pub fn reactive<F, T>(mut self, signal: ReadSignal<T>, mapper: F) -> Self
        where
            F: Fn(&T) -> String + 'static,
            T: Clone + 'static,
        {
            let signal_fn = move || mapper(&signal.get());
            self.signals.push(Box::new(signal_fn));
            self
        }
        
        /// Conditional reactive classes
        pub fn reactive_when<F, T>(self, signal: ReadSignal<T>, condition: F, classes: &str) -> Self
        where
            F: Fn(&T) -> bool + 'static,
            T: Clone + 'static,
        {
            let classes = classes.to_string();
            self.reactive(signal, move |value| {
                if condition(value) { classes.clone() } else { String::new() }
            })
        }
        
        /// Build into a reactive signal
        pub fn build_signal(self) -> ReadSignal<String> {
            let (getter, setter) = create_signal(String::new());
            
            create_effect(move |_| {
                let mut class_string = self.inner.to_string();
                for signal_fn in &self.signals {
                    let reactive_classes = signal_fn();
                    if !reactive_classes.is_empty() {
                        if !class_string.is_empty() {
                            class_string.push(' ');
                        }
                        class_string.push_str(&reactive_classes);
                    }
                }
                setter.set(class_string);
            });
            
            getter
        }
    }
    
    /// Convenient macro for reactive classes
    #[macro_export]
    macro_rules! tw_reactive {
        ($($tokens:tt)*) => {
            ReactiveClassBuilder::new()
                .$($tokens)*
                .build_signal()
        };
    }
}
```

## 7. Error Handling

### Comprehensive Error Types
```rust
/// Main error type for all Tailwind-RS operations
#[derive(Debug, thiserror::Error)]
pub enum TailwindError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("CSS generation error: {0}")]
    CSSGeneration(#[from] CSSError),
    
    #[error("Plugin error: {0}")]
    Plugin(#[from] PluginError),
    
    #[error("Content scanning error: {0}")]
    Scan(#[from] ScanError),
    
    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),
}

/// Detailed error information with recovery suggestions
pub trait DetailedError {
    fn error_code(&self) -> &'static str;
    fn help_text(&self) -> Option<&'static str>;
    fn recovery_suggestions(&self) -> Vec<String>;
}

impl DetailedError for TailwindError {
    fn error_code(&self) -> &'static str {
        match self {
            TailwindError::Config(_) => "CONFIG_ERROR",
            TailwindError::CSSGeneration(_) => "CSS_ERROR", 
            TailwindError::Plugin(_) => "PLUGIN_ERROR",
            TailwindError::Scan(_) => "SCAN_ERROR",
            TailwindError::Engine(_) => "ENGINE_ERROR",
        }
    }
    
    fn help_text(&self) -> Option<&'static str> {
        match self {
            TailwindError::Config(_) => Some("Check your tailwind.config.js or tailwind-rs.toml file"),
            TailwindError::CSSGeneration(_) => Some("Verify your class names are valid"),
            TailwindError::Plugin(_) => Some("Check plugin configuration and compatibility"),
            TailwindError::Scan(_) => Some("Verify content paths and file permissions"),
            TailwindError::Engine(_) => Some("Check system resources and configuration"),
        }
    }
    
    fn recovery_suggestions(&self) -> Vec<String> {
        // Implementation with specific recovery steps
        vec![]
    }
}
```

## 8. Performance and Diagnostics

### Performance Monitoring API
```rust
/// Performance monitoring and diagnostics
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    enabled: bool,
}

impl PerformanceMonitor {
    pub fn new(enabled: bool) -> Self { /* ... */ }
    
    pub fn start_timer(&self, operation: &str) -> PerformanceTimer { /* ... */ }
    pub fn record_metric(&self, metric: PerformanceMetric) { /* ... */ }
    pub fn get_metrics(&self) -> PerformanceMetrics { /* ... */ }
    pub fn reset_metrics(&self) { /* ... */ }
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub css_generation_time: Duration,
    pub class_scanning_time: Duration,
    pub plugin_processing_time: Duration,
    pub cache_hit_rate: f32,
    pub memory_usage: usize,
    pub classes_processed: usize,
}

/// Diagnostic information for troubleshooting
pub struct Diagnostics {
    pub system_info: SystemInfo,
    pub config_validation: ConfigValidation,
    pub plugin_status: Vec<PluginStatus>,
    pub performance_metrics: PerformanceMetrics,
}

impl Diagnostics {
    pub fn collect() -> Self { /* ... */ }
    pub fn to_report(&self) -> String { /* ... */ }
    pub fn to_json(&self) -> Result<String, serde_json::Error> { /* ... */ }
}
```

This comprehensive API specification provides:

1. **Backward Compatibility**: All existing v0.9.1 APIs remain functional
2. **Enhanced Features**: New APIs for PostCSS, content scanning, and plugins
3. **Type Safety**: Rich type system with compile-time validation
4. **Performance**: Built-in monitoring and optimization hooks
5. **Framework Integration**: Seamless integration with Rust web frameworks
6. **Error Handling**: Comprehensive error types with recovery guidance
7. **Plugin Ecosystem**: Full plugin development and management APIs
8. **Developer Experience**: Rich tooling and diagnostic capabilities

The API design ensures a smooth migration path from v0.9.1 while providing powerful new capabilities that position Tailwind-RS as a leader in the CSS framework ecosystem.
