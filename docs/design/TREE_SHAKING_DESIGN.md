# 🌳 **Tree Shaking System Design Document**

**Document**: Tree Shaking Engine v2.0  
**Version**: 1.0  
**Date**: December 2024  
**Status**: 📋 **DESIGN PHASE**  

---

## 🎯 **OVERVIEW**

### **Problem Statement**
The current tree shaking system is completely broken. All removal metrics return hardcoded zeros, making it impossible to measure or validate any unused code removal.

### **Current Issues**
```rust
responsive_removed: 0, // TODO: Track responsive removals
conditional_removed: 0, // TODO: Track conditional removals
custom_removed: 0, // TODO: Track custom property removals
```

### **Solution Goals**
- ✅ **Real tree shaking algorithms** with actual unused code removal
- ✅ **Accurate removal metrics** and tracking
- ✅ **Comprehensive dependency analysis** and dead code detection
- ✅ **Configurable tree shaking levels** for different use cases
- ✅ **Performance benchmarking** and validation

---

## 🏗️ **ARCHITECTURE DESIGN**

### **Core Components**

```rust
// crates/tailwind-rs-core/src/tree_shaker/v2/mod.rs
pub mod dependency_analyzer;  // Dependency analysis and graph building
pub mod usage_scanner;        // Source code scanning for used classes
pub mod dead_code_eliminator; // Dead code identification and removal
pub mod metrics;              // Performance metrics and tracking
pub mod analyzer;             // Tree shaking analysis and reporting
pub mod optimizer;            // Tree shaking optimization strategies
```

### **Tree Shaking Pipeline**

```
Source Files
    ↓
Usage Scanner (identify used classes)
    ↓
Dependency Analyzer (build dependency graph)
    ↓
Dead Code Eliminator (identify unused code)
    ↓
CSS Generator (remove unused rules)
    ↓
Optimized CSS + Removal Metrics
```

---

## 📋 **DETAILED COMPONENT DESIGN**

### **1. Usage Scanner**

```rust
// crates/tailwind-rs-core/src/tree_shaker/v2/usage_scanner.rs

/// Advanced usage scanning engine
pub struct UsageScanner {
    ast_parser: AstParser,
    class_extractor: ClassExtractor,
    pattern_matcher: PatternMatcher,
    metrics: UsageScanMetrics,
}

impl UsageScanner {
    pub fn new() -> Self {
        Self {
            ast_parser: AstParser::new(),
            class_extractor: ClassExtractor::new(),
            pattern_matcher: PatternMatcher::new(),
            metrics: UsageScanMetrics::new(),
        }
    }
    
    /// Scan source files for used Tailwind classes
    pub fn scan_sources(&mut self, source_paths: &[&Path]) -> Result<HashSet<String>> {
        let start_time = std::time::Instant::now();
        let mut used_classes = HashSet::new();
        
        for source_path in source_paths {
            let file_classes = self.scan_file(source_path)?;
            used_classes.extend(file_classes);
        }
        
        self.metrics.files_scanned = source_paths.len();
        self.metrics.classes_found = used_classes.len();
        self.metrics.scan_time = start_time.elapsed();
        
        Ok(used_classes)
    }
    
    /// Scan individual file for used classes
    fn scan_file(&mut self, file_path: &Path) -> Result<HashSet<String>> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| TailwindError::build(format!("Failed to read file: {}", e)))?;
        
        // Parse file content
        let ast = self.ast_parser.parse_file(&content)?;
        
        // Extract classes from AST
        let classes = self.class_extractor.extract_classes(&ast)?;
        
        // Match patterns to identify Tailwind classes
        let tailwind_classes = self.pattern_matcher.match_tailwind_classes(&classes)?;
        
        Ok(tailwind_classes)
    }
}

/// Class extraction from AST
pub struct ClassExtractor {
    visitor: ClassVisitor,
    class_patterns: Vec<ClassPattern>,
}

impl ClassExtractor {
    pub fn new() -> Self {
        Self {
            visitor: ClassVisitor::new(),
            class_patterns: Self::build_class_patterns(),
        }
    }
    
    /// Extract classes from AST
    pub fn extract_classes(&self, ast: &Ast) -> Result<Vec<String>> {
        let mut classes = Vec::new();
        
        // Visit all nodes in AST
        self.visitor.visit_ast(ast, &mut |node| {
            if let Some(class) = self.extract_class_from_node(node)? {
                classes.push(class);
            }
            Ok(())
        })?;
        
        Ok(classes)
    }
    
    /// Extract class from AST node
    fn extract_class_from_node(&self, node: &AstNode) -> Result<Option<String>> {
        match node {
            AstNode::StringLiteral(s) => {
                // Check if string contains Tailwind classes
                if self.is_tailwind_class_string(s)? {
                    return Ok(Some(s.clone()));
                }
            }
            AstNode::MethodCall(call) => {
                // Check for class builder methods
                if self.is_class_builder_method(call)? {
                    if let Some(class) = self.extract_class_from_method_call(call)? {
                        return Ok(Some(class));
                    }
                }
            }
            AstNode::FunctionCall(call) => {
                // Check for class generation functions
                if self.is_class_generation_function(call)? {
                    if let Some(class) = self.extract_class_from_function_call(call)? {
                        return Ok(Some(class));
                    }
                }
            }
            _ => {}
        }
        
        Ok(None)
    }
    
    /// Check if string contains Tailwind classes
    fn is_tailwind_class_string(&self, s: &str) -> Result<bool> {
        // Check for common Tailwind class patterns
        for pattern in &self.class_patterns {
            if pattern.matches(s)? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Build class patterns for matching
    fn build_class_patterns() -> Vec<ClassPattern> {
        vec![
            // Spacing patterns
            ClassPattern::new(r"p-\d+", "padding"),
            ClassPattern::new(r"m-\d+", "margin"),
            ClassPattern::new(r"px-\d+", "padding-x"),
            ClassPattern::new(r"py-\d+", "padding-y"),
            ClassPattern::new(r"mx-\d+", "margin-x"),
            ClassPattern::new(r"my-\d+", "margin-y"),
            
            // Color patterns
            ClassPattern::new(r"bg-\w+-\d+", "background-color"),
            ClassPattern::new(r"text-\w+-\d+", "text-color"),
            ClassPattern::new(r"border-\w+-\d+", "border-color"),
            
            // Size patterns
            ClassPattern::new(r"w-\d+", "width"),
            ClassPattern::new(r"h-\d+", "height"),
            ClassPattern::new(r"w-1/\d+", "width-fraction"),
            ClassPattern::new(r"h-1/\d+", "height-fraction"),
            
            // Typography patterns
            ClassPattern::new(r"text-\w+", "font-size"),
            ClassPattern::new(r"font-\w+", "font-weight"),
            ClassPattern::new(r"leading-\d+", "line-height"),
            
            // Layout patterns
            ClassPattern::new(r"flex", "display"),
            ClassPattern::new(r"grid", "display"),
            ClassPattern::new(r"block", "display"),
            ClassPattern::new(r"inline", "display"),
            
            // Responsive patterns
            ClassPattern::new(r"sm:\w+", "responsive"),
            ClassPattern::new(r"md:\w+", "responsive"),
            ClassPattern::new(r"lg:\w+", "responsive"),
            ClassPattern::new(r"xl:\w+", "responsive"),
            ClassPattern::new(r"2xl:\w+", "responsive"),
        ]
    }
}

/// Pattern matching for Tailwind classes
pub struct PatternMatcher {
    regex_patterns: HashMap<String, Regex>,
    class_categories: HashMap<String, Vec<String>>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            regex_patterns: Self::build_regex_patterns(),
            class_categories: Self::build_class_categories(),
        }
    }
    
    /// Match Tailwind classes from extracted classes
    pub fn match_tailwind_classes(&self, classes: &[String]) -> Result<HashSet<String>> {
        let mut tailwind_classes = HashSet::new();
        
        for class in classes {
            // Check if class matches any Tailwind pattern
            for (pattern_name, regex) in &self.regex_patterns {
                if regex.is_match(class) {
                    tailwind_classes.insert(class.clone());
                    break;
                }
            }
        }
        
        Ok(tailwind_classes)
    }
    
    /// Build regex patterns for class matching
    fn build_regex_patterns() -> HashMap<String, Regex> {
        let mut patterns = HashMap::new();
        
        // Spacing patterns
        patterns.insert("padding".to_string(), Regex::new(r"p-\d+").unwrap());
        patterns.insert("margin".to_string(), Regex::new(r"m-\d+").unwrap());
        patterns.insert("padding-x".to_string(), Regex::new(r"px-\d+").unwrap());
        patterns.insert("padding-y".to_string(), Regex::new(r"py-\d+").unwrap());
        patterns.insert("margin-x".to_string(), Regex::new(r"mx-\d+").unwrap());
        patterns.insert("margin-y".to_string(), Regex::new(r"my-\d+").unwrap());
        
        // Color patterns
        patterns.insert("background-color".to_string(), Regex::new(r"bg-\w+-\d+").unwrap());
        patterns.insert("text-color".to_string(), Regex::new(r"text-\w+-\d+").unwrap());
        patterns.insert("border-color".to_string(), Regex::new(r"border-\w+-\d+").unwrap());
        
        // Size patterns
        patterns.insert("width".to_string(), Regex::new(r"w-\d+").unwrap());
        patterns.insert("height".to_string(), Regex::new(r"h-\d+").unwrap());
        patterns.insert("width-fraction".to_string(), Regex::new(r"w-1/\d+").unwrap());
        patterns.insert("height-fraction".to_string(), Regex::new(r"h-1/\d+").unwrap());
        
        // Typography patterns
        patterns.insert("font-size".to_string(), Regex::new(r"text-\w+").unwrap());
        patterns.insert("font-weight".to_string(), Regex::new(r"font-\w+").unwrap());
        patterns.insert("line-height".to_string(), Regex::new(r"leading-\d+").unwrap());
        
        // Layout patterns
        patterns.insert("display".to_string(), Regex::new(r"(flex|grid|block|inline)").unwrap());
        
        // Responsive patterns
        patterns.insert("responsive".to_string(), Regex::new(r"(sm|md|lg|xl|2xl):\w+").unwrap());
        
        patterns
    }
    
    /// Build class categories for analysis
    fn build_class_categories() -> HashMap<String, Vec<String>> {
        let mut categories = HashMap::new();
        
        categories.insert("spacing".to_string(), vec![
            "p-".to_string(), "m-".to_string(), "px-".to_string(), "py-".to_string(),
            "mx-".to_string(), "my-".to_string(), "pt-".to_string(), "pb-".to_string(),
            "pl-".to_string(), "pr-".to_string(), "mt-".to_string(), "mb-".to_string(),
            "ml-".to_string(), "mr-".to_string(),
        ]);
        
        categories.insert("colors".to_string(), vec![
            "bg-".to_string(), "text-".to_string(), "border-".to_string(), "ring-".to_string(),
        ]);
        
        categories.insert("sizing".to_string(), vec![
            "w-".to_string(), "h-".to_string(), "min-w-".to_string(), "min-h-".to_string(),
            "max-w-".to_string(), "max-h-".to_string(),
        ]);
        
        categories.insert("typography".to_string(), vec![
            "text-".to_string(), "font-".to_string(), "leading-".to_string(), "tracking-".to_string(),
        ]);
        
        categories.insert("layout".to_string(), vec![
            "flex".to_string(), "grid".to_string(), "block".to_string(), "inline".to_string(),
            "hidden".to_string(), "visible".to_string(),
        ]);
        
        categories.insert("responsive".to_string(), vec![
            "sm:".to_string(), "md:".to_string(), "lg:".to_string(), "xl:".to_string(), "2xl:".to_string(),
        ]);
        
        categories
    }
}
```

### **2. Dependency Analyzer**

```rust
// crates/tailwind-rs-core/src/tree_shaker/v2/dependency_analyzer.rs

/// Advanced dependency analysis engine
pub struct DependencyAnalyzer {
    dependency_graph: DependencyGraph,
    class_dependencies: HashMap<String, HashSet<String>>,
    reverse_dependencies: HashMap<String, HashSet<String>>,
    metrics: DependencyAnalysisMetrics,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self {
            dependency_graph: DependencyGraph::new(),
            class_dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
            metrics: DependencyAnalysisMetrics::new(),
        }
    }
    
    /// Analyze dependencies between CSS classes
    pub fn analyze_dependencies(&mut self, generator: &CssGenerator) -> Result<DependencyGraph> {
        let start_time = std::time::Instant::now();
        
        // Build dependency graph
        self.build_dependency_graph(generator)?;
        
        // Analyze class relationships
        self.analyze_class_relationships(generator)?;
        
        // Build reverse dependencies
        self.build_reverse_dependencies()?;
        
        self.metrics.analysis_time = start_time.elapsed();
        self.metrics.classes_analyzed = self.class_dependencies.len();
        
        Ok(self.dependency_graph.clone())
    }
    
    /// Build dependency graph from CSS generator
    fn build_dependency_graph(&mut self, generator: &CssGenerator) -> Result<()> {
        for (class_name, rule) in generator.get_rules() {
            let dependencies = self.extract_dependencies(rule)?;
            self.class_dependencies.insert(class_name.clone(), dependencies);
        }
        
        Ok(())
    }
    
    /// Extract dependencies from CSS rule
    fn extract_dependencies(&self, rule: &CssRule) -> Result<HashSet<String>> {
        let mut dependencies = HashSet::new();
        
        for property in &rule.properties {
            // Look for class references in CSS values
            if let Some(class_refs) = self.find_class_references(&property.value)? {
                dependencies.extend(class_refs);
            }
            
            // Look for CSS variable references
            if let Some(var_refs) = self.find_css_variable_references(&property.value)? {
                dependencies.extend(var_refs);
            }
        }
        
        Ok(dependencies)
    }
    
    /// Find class references in CSS value
    fn find_class_references(&self, value: &str) -> Result<Option<HashSet<String>>> {
        let mut references = HashSet::new();
        
        // Look for class references in CSS values
        // This is a simplified implementation - in reality, this would be more sophisticated
        if value.contains("var(--") {
            // Extract CSS variable references
            let var_pattern = Regex::new(r"var\(--([^)]+)\)")?;
            for cap in var_pattern.captures_iter(value) {
                if let Some(var_name) = cap.get(1) {
                    references.insert(format!("--{}", var_name.as_str()));
                }
            }
        }
        
        if references.is_empty() {
            Ok(None)
        } else {
            Ok(Some(references))
        }
    }
    
    /// Find CSS variable references
    fn find_css_variable_references(&self, value: &str) -> Result<Option<HashSet<String>>> {
        let mut references = HashSet::new();
        
        let var_pattern = Regex::new(r"var\(--([^)]+)\)")?;
        for cap in var_pattern.captures_iter(value) {
            if let Some(var_name) = cap.get(1) {
                references.insert(format!("--{}", var_name.as_str()));
            }
        }
        
        if references.is_empty() {
            Ok(None)
        } else {
            Ok(Some(references))
        }
    }
    
    /// Analyze class relationships
    fn analyze_class_relationships(&mut self, generator: &CssGenerator) -> Result<()> {
        for (class_name, rule) in generator.get_rules() {
            // Analyze responsive relationships
            if self.is_responsive_class(class_name)? {
                self.analyze_responsive_relationships(class_name, rule)?;
            }
            
            // Analyze conditional relationships
            if self.is_conditional_class(class_name)? {
                self.analyze_conditional_relationships(class_name, rule)?;
            }
            
            // Analyze custom property relationships
            if self.uses_custom_properties(rule)? {
                self.analyze_custom_property_relationships(class_name, rule)?;
            }
        }
        
        Ok(())
    }
    
    /// Check if class is responsive
    fn is_responsive_class(&self, class_name: &str) -> Result<bool> {
        let responsive_pattern = Regex::new(r"(sm|md|lg|xl|2xl):")?;
        Ok(responsive_pattern.is_match(class_name))
    }
    
    /// Check if class is conditional
    fn is_conditional_class(&self, class_name: &str) -> Result<bool> {
        let conditional_pattern = Regex::new(r"(hover|focus|active|disabled):")?;
        Ok(conditional_pattern.is_match(class_name))
    }
    
    /// Check if rule uses custom properties
    fn uses_custom_properties(&self, rule: &CssRule) -> Result<bool> {
        for property in &rule.properties {
            if property.value.contains("var(--") {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /// Build reverse dependencies
    fn build_reverse_dependencies(&mut self) -> Result<()> {
        for (class_name, dependencies) in &self.class_dependencies {
            for dependency in dependencies {
                self.reverse_dependencies
                    .entry(dependency.clone())
                    .or_insert_with(HashSet::new)
                    .insert(class_name.clone());
            }
        }
        
        Ok(())
    }
}

/// Dependency graph structure
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    nodes: HashMap<String, DependencyNode>,
    edges: Vec<DependencyEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
    
    /// Add node to graph
    pub fn add_node(&mut self, name: String, dependencies: HashSet<String>) {
        let node = DependencyNode {
            name: name.clone(),
            dependencies: dependencies.clone(),
            dependents: HashSet::new(),
        };
        
        self.nodes.insert(name, node);
        
        // Add edges
        for dependency in dependencies {
            self.edges.push(DependencyEdge {
                from: dependency,
                to: name.clone(),
            });
        }
    }
    
    /// Get node dependencies
    pub fn get_dependencies(&self, node_name: &str) -> Option<&HashSet<String>> {
        self.nodes.get(node_name).map(|node| &node.dependencies)
    }
    
    /// Get node dependents
    pub fn get_dependents(&self, node_name: &str) -> Option<&HashSet<String>> {
        self.nodes.get(node_name).map(|node| &node.dependents)
    }
    
    /// Check if node is used
    pub fn is_node_used(&self, node_name: &str) -> bool {
        if let Some(node) = self.nodes.get(node_name) {
            !node.dependents.is_empty()
        } else {
            false
        }
    }
}

/// Dependency node
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub name: String,
    pub dependencies: HashSet<String>,
    pub dependents: HashSet<String>,
}

/// Dependency edge
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
}
```

### **3. Dead Code Eliminator**

```rust
// crates/tailwind-rs-core/src/tree_shaker/v2/dead_code_eliminator.rs

/// Advanced dead code elimination engine
pub struct DeadCodeEliminator {
    usage_analyzer: UsageAnalyzer,
    dependency_analyzer: DependencyAnalyzer,
    removal_strategies: Vec<RemovalStrategy>,
    metrics: DeadCodeEliminationMetrics,
}

impl DeadCodeEliminator {
    pub fn new() -> Self {
        Self {
            usage_analyzer: UsageAnalyzer::new(),
            dependency_analyzer: DependencyAnalyzer::new(),
            removal_strategies: Self::build_removal_strategies(),
            metrics: DeadCodeEliminationMetrics::new(),
        }
    }
    
    /// Remove dead code from CSS generator
    pub fn remove_dead_code(&mut self, used_classes: &HashSet<String>, generator: &mut CssGenerator) -> Result<DeadCodeEliminationResults> {
        let start_time = std::time::Instant::now();
        let mut removed_classes = Vec::new();
        let mut responsive_removed = 0;
        let mut conditional_removed = 0;
        let mut custom_removed = 0;
        
        // Analyze usage patterns
        let usage_analysis = self.usage_analyzer.analyze_usage(used_classes, generator)?;
        
        // Analyze dependencies
        let dependency_graph = self.dependency_analyzer.analyze_dependencies(generator)?;
        
        // Identify dead code
        let dead_classes = self.identify_dead_code(used_classes, &dependency_graph, generator)?;
        
        // Remove dead code
        for dead_class in dead_classes {
            if generator.remove_rule(&dead_class).is_ok() {
                removed_classes.push(dead_class.clone());
                
                // Categorize removal
                if self.is_responsive_class(&dead_class)? {
                    responsive_removed += 1;
                } else if self.is_conditional_class(&dead_class)? {
                    conditional_removed += 1;
                } else if self.is_custom_property_class(&dead_class)? {
                    custom_removed += 1;
                }
            }
        }
        
        // Update metrics
        self.metrics.classes_removed = removed_classes.len();
        self.metrics.responsive_removed = responsive_removed;
        self.metrics.conditional_removed = conditional_removed;
        self.metrics.custom_removed = custom_removed;
        self.metrics.elimination_time = start_time.elapsed();
        
        Ok(DeadCodeEliminationResults {
            removed_classes,
            responsive_removed,
            conditional_removed,
            custom_removed,
            elimination_time: start_time.elapsed(),
            metrics: self.metrics.clone(),
        })
    }
    
    /// Identify dead code
    fn identify_dead_code(&self, used_classes: &HashSet<String>, dependency_graph: &DependencyGraph, generator: &CssGenerator) -> Result<Vec<String>> {
        let mut dead_classes = Vec::new();
        
        for (class_name, _) in generator.get_rules() {
            // Check if class is directly used
            if used_classes.contains(class_name) {
                continue;
            }
            
            // Check if class is used through dependencies
            if dependency_graph.is_node_used(class_name) {
                continue;
            }
            
            // Check if class is used through responsive variants
            if self.is_used_through_responsive_variants(class_name, used_classes)? {
                continue;
            }
            
            // Check if class is used through conditional variants
            if self.is_used_through_conditional_variants(class_name, used_classes)? {
                continue;
            }
            
            // Class is dead code
            dead_classes.push(class_name.clone());
        }
        
        Ok(dead_classes)
    }
    
    /// Check if class is used through responsive variants
    fn is_used_through_responsive_variants(&self, class_name: &str, used_classes: &HashSet<String>) -> Result<bool> {
        // Extract base class name (remove responsive prefix)
        if let Some(base_class) = self.extract_base_class(class_name)? {
            // Check if base class is used
            if used_classes.contains(&base_class) {
                return Ok(true);
            }
            
            // Check if any responsive variants are used
            for breakpoint in &["sm", "md", "lg", "xl", "2xl"] {
                let responsive_class = format!("{}:{}", breakpoint, base_class);
                if used_classes.contains(&responsive_class) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Check if class is used through conditional variants
    fn is_used_through_conditional_variants(&self, class_name: &str, used_classes: &HashSet<String>) -> Result<bool> {
        // Extract base class name (remove conditional prefix)
        if let Some(base_class) = self.extract_base_class(class_name)? {
            // Check if base class is used
            if used_classes.contains(&base_class) {
                return Ok(true);
            }
            
            // Check if any conditional variants are used
            for condition in &["hover", "focus", "active", "disabled"] {
                let conditional_class = format!("{}:{}", condition, base_class);
                if used_classes.contains(&conditional_class) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Extract base class name from prefixed class
    fn extract_base_class(&self, class_name: &str) -> Result<Option<String>> {
        // Remove responsive prefixes
        for breakpoint in &["sm", "md", "lg", "xl", "2xl"] {
            if class_name.starts_with(&format!("{}:", breakpoint)) {
                return Ok(Some(class_name.strip_prefix(&format!("{}:", breakpoint)).unwrap().to_string()));
            }
        }
        
        // Remove conditional prefixes
        for condition in &["hover", "focus", "active", "disabled"] {
            if class_name.starts_with(&format!("{}:", condition)) {
                return Ok(Some(class_name.strip_prefix(&format!("{}:", condition)).unwrap().to_string()));
            }
        }
        
        Ok(None)
    }
    
    /// Check if class is responsive
    fn is_responsive_class(&self, class_name: &str) -> Result<bool> {
        let responsive_pattern = Regex::new(r"(sm|md|lg|xl|2xl):")?;
        Ok(responsive_pattern.is_match(class_name))
    }
    
    /// Check if class is conditional
    fn is_conditional_class(&self, class_name: &str) -> Result<bool> {
        let conditional_pattern = Regex::new(r"(hover|focus|active|disabled):")?;
        Ok(conditional_pattern.is_match(class_name))
    }
    
    /// Check if class is custom property
    fn is_custom_property_class(&self, class_name: &str) -> Result<bool> {
        Ok(class_name.starts_with("--"))
    }
    
    /// Build removal strategies
    fn build_removal_strategies() -> Vec<RemovalStrategy> {
        vec![
            RemovalStrategy::DirectUnused,
            RemovalStrategy::DependencyUnused,
            RemovalStrategy::ResponsiveUnused,
            RemovalStrategy::ConditionalUnused,
            RemovalStrategy::CustomPropertyUnused,
        ]
    }
}

/// Usage analyzer
pub struct UsageAnalyzer {
    pattern_matcher: PatternMatcher,
    class_categorizer: ClassCategorizer,
}

impl UsageAnalyzer {
    pub fn new() -> Self {
        Self {
            pattern_matcher: PatternMatcher::new(),
            class_categorizer: ClassCategorizer::new(),
        }
    }
    
    /// Analyze usage patterns
    pub fn analyze_usage(&self, used_classes: &HashSet<String>, generator: &CssGenerator) -> Result<UsageAnalysis> {
        let mut analysis = UsageAnalysis::new();
        
        // Categorize used classes
        for class in used_classes {
            let category = self.class_categorizer.categorize_class(class)?;
            analysis.add_used_class(class.clone(), category);
        }
        
        // Analyze unused classes
        for (class_name, _) in generator.get_rules() {
            if !used_classes.contains(class_name) {
                let category = self.class_categorizer.categorize_class(class_name)?;
                analysis.add_unused_class(class_name.clone(), category);
            }
        }
        
        Ok(analysis)
    }
}

/// Class categorizer
pub struct ClassCategorizer {
    categories: HashMap<String, Regex>,
}

impl ClassCategorizer {
    pub fn new() -> Self {
        Self {
            categories: Self::build_categories(),
        }
    }
    
    /// Categorize class
    pub fn categorize_class(&self, class_name: &str) -> Result<ClassCategory> {
        for (category_name, regex) in &self.categories {
            if regex.is_match(class_name) {
                return Ok(ClassCategory::from_str(category_name)?);
            }
        }
        
        Ok(ClassCategory::Other)
    }
    
    /// Build category patterns
    fn build_categories() -> HashMap<String, Regex> {
        let mut categories = HashMap::new();
        
        categories.insert("spacing".to_string(), Regex::new(r"^(p|m|px|py|mx|my|pt|pb|pl|pr|mt|mb|ml|mr)-\d+").unwrap());
        categories.insert("colors".to_string(), Regex::new(r"^(bg|text|border|ring)-\w+-\d+").unwrap());
        categories.insert("sizing".to_string(), Regex::new(r"^(w|h|min-w|min-h|max-w|max-h)-\d+").unwrap());
        categories.insert("typography".to_string(), Regex::new(r"^(text|font|leading|tracking)-\w+").unwrap());
        categories.insert("layout".to_string(), Regex::new(r"^(flex|grid|block|inline|hidden|visible)").unwrap());
        categories.insert("responsive".to_string(), Regex::new(r"^(sm|md|lg|xl|2xl):").unwrap());
        categories.insert("conditional".to_string(), Regex::new(r"^(hover|focus|active|disabled):").unwrap());
        categories.insert("custom".to_string(), Regex::new(r"^--").unwrap());
        
        categories
    }
}

/// Class category enum
#[derive(Debug, Clone, PartialEq)]
pub enum ClassCategory {
    Spacing,
    Colors,
    Sizing,
    Typography,
    Layout,
    Responsive,
    Conditional,
    Custom,
    Other,
}

impl ClassCategory {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "spacing" => Ok(ClassCategory::Spacing),
            "colors" => Ok(ClassCategory::Colors),
            "sizing" => Ok(ClassCategory::Sizing),
            "typography" => Ok(ClassCategory::Typography),
            "layout" => Ok(ClassCategory::Layout),
            "responsive" => Ok(ClassCategory::Responsive),
            "conditional" => Ok(ClassCategory::Conditional),
            "custom" => Ok(ClassCategory::Custom),
            _ => Ok(ClassCategory::Other),
        }
    }
}
```

---

## 🧪 **TESTING STRATEGY**

### **Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_usage_scanning() {
        let mut scanner = UsageScanner::new();
        let source_paths = vec![Path::new("test.rs")];
        
        // Create test file with Tailwind classes
        std::fs::write("test.rs", r#"
            let classes = "px-4 py-2 bg-blue-500 text-white";
            let button = ClassBuilder::new()
                .padding_x(SpacingValue::Integer(4))
                .padding_y(SpacingValue::Integer(2))
                .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
                .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
                .build_string();
        "#).unwrap();
        
        let used_classes = scanner.scan_sources(&source_paths).unwrap();
        
        assert!(used_classes.contains("px-4"));
        assert!(used_classes.contains("py-2"));
        assert!(used_classes.contains("bg-blue-500"));
        assert!(used_classes.contains("text-white"));
        
        // Clean up
        std::fs::remove_file("test.rs").unwrap();
    }
    
    #[test]
    fn test_dependency_analysis() {
        let mut generator = CssGenerator::new();
        
        // Add rules with dependencies
        generator.add_rule(".btn".to_string(), CssRule {
            properties: vec![
                CssProperty { name: "padding".to_string(), value: "var(--spacing-md)".to_string() },
                CssProperty { name: "background".to_string(), value: "var(--color-primary)".to_string() },
            ],
        }).unwrap();
        
        generator.add_rule("--spacing-md".to_string(), CssRule {
            properties: vec![
                CssProperty { name: "value".to_string(), value: "1rem".to_string() },
            ],
        }).unwrap();
        
        generator.add_rule("--color-primary".to_string(), CssRule {
            properties: vec![
                CssProperty { name: "value".to_string(), value: "#3b82f6".to_string() },
            ],
        }).unwrap();
        
        let mut analyzer = DependencyAnalyzer::new();
        let graph = analyzer.analyze_dependencies(&generator).unwrap();
        
        // Check dependencies
        let btn_deps = graph.get_dependencies(".btn").unwrap();
        assert!(btn_deps.contains("--spacing-md"));
        assert!(btn_deps.contains("--color-primary"));
        
        // Check dependents
        let spacing_deps = graph.get_dependents("--spacing-md").unwrap();
        assert!(spacing_deps.contains(".btn"));
    }
    
    #[test]
    fn test_dead_code_elimination() {
        let mut generator = CssGenerator::new();
        
        // Add used and unused rules
        generator.add_rule(".btn".to_string(), CssRule {
            properties: vec![
                CssProperty { name: "padding".to_string(), value: "8px".to_string() },
            ],
        }).unwrap();
        
        generator.add_rule(".unused".to_string(), CssRule {
            properties: vec![
                CssProperty { name: "margin".to_string(), value: "16px".to_string() },
            ],
        }).unwrap();
        
        let used_classes = HashSet::from([".btn".to_string()]);
        
        let mut eliminator = DeadCodeEliminator::new();
        let result = eliminator.remove_dead_code(&used_classes, &mut generator).unwrap();
        
        assert_eq!(result.removed_classes.len(), 1);
        assert!(result.removed_classes.contains(".unused"));
        
        // Check that unused rule was removed
        assert!(generator.get_rule(".unused").is_none());
        assert!(generator.get_rule(".btn").is_some());
    }
}
```

---

## 📊 **PERFORMANCE CONSIDERATIONS**

### **Tree Shaking Performance**
- **Parallel processing**: Use rayon for parallel file scanning
- **Incremental analysis**: Only re-analyze changed files
- **Caching**: Cache analysis results to avoid recomputation
- **Streaming processing**: Process large files in chunks

### **Memory Usage**
- **Efficient data structures**: Use appropriate HashMap and HashSet types
- **Memory pooling**: Reuse objects to minimize allocations
- **Garbage collection**: Minimize allocations during analysis

### **Error Handling**
- **Graceful degradation**: Continue analysis even if some files fail
- **Detailed error reporting**: Provide specific error locations and context
- **Recovery strategies**: Attempt to recover from analysis errors

---

## 🎯 **SUCCESS METRICS**

### **Functionality**
- ✅ **Real tree shaking algorithms** with actual unused code removal
- ✅ **Accurate removal metrics** and tracking
- ✅ **Comprehensive dependency analysis** and dead code detection
- ✅ **Configurable tree shaking levels** for different use cases

### **Performance**
- ✅ **< 500ms analysis time** for typical projects
- ✅ **> 30% size reduction** for unused code removal
- ✅ **< 50MB memory usage** for large projects
- ✅ **> 95% accuracy** for dead code detection

### **Reliability**
- ✅ **100% test coverage** for tree shaking logic
- ✅ **Zero panics** in error conditions
- ✅ **Comprehensive error handling**

---

## 🚀 **IMPLEMENTATION PLAN**

### **Week 1: Core Tree Shaking Engine**
- [ ] Implement usage scanner
- [ ] Implement dependency analyzer
- [ ] Implement dead code eliminator
- [ ] Add basic unit tests

### **Week 2: Advanced Analysis**
- [ ] Implement pattern matching
- [ ] Implement class categorization
- [ ] Add comprehensive error handling
- [ ] Add integration tests

### **Week 3: Performance & Validation**
- [ ] Add performance benchmarks
- [ ] Add tree shaking metrics
- [ ] Add comprehensive test suite
- [ ] Performance optimization

---

## 🎯 **CONCLUSION**

This design provides a complete, real tree shaking system that addresses all the issues in the current implementation. The system is designed for performance, reliability, and maintainability, with comprehensive testing and validation.

**Key Benefits:**
- ✅ **Real tree shaking algorithms** with actual unused code removal
- ✅ **Accurate removal metrics** and tracking
- ✅ **Comprehensive dependency analysis** and dead code detection
- ✅ **Configurable tree shaking levels** for different use cases
- ✅ **Performance optimized** processing and validation
