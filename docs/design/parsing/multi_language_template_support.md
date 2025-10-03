# Multi-Language Template Support Design

## Overview

Implement comprehensive support for extracting Tailwind classes from multiple templating languages, inspired by the official Tailwind Oxide implementation's extensive language support including Clojure, Elixir, Haml, Pug, Slim, and more.

## Problem Statement

Current extraction is limited to basic HTML and JavaScript contexts. Complex templating languages with unique syntax (Vue directives, Svelte class expressions, etc.) are not properly handled.

## Solution Architecture

### Core Components

#### 1. Language Detection System

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLanguage {
    HTML,
    XHTML,
    JavaScript,
    TypeScript,
    JSX,
    TSX,
    Vue,
    Svelte,
    Angular,
    React,
    Clojure,
    Elixir,
    Haml,
    Pug,
    Slim,
    Ruby,
    Python,
    PHP,
    Rust,
    Markdown,
    Unknown,
}

pub struct LanguageDetector {
    file_extensions: HashMap<String, TemplateLanguage>,
    content_patterns: Vec<(TemplateLanguage, Regex)>,
}

impl LanguageDetector {
    pub fn detect_from_path(&self, path: &Path) -> TemplateLanguage {
        if let Some(ext) = path.extension() {
            if let Some(&lang) = self.file_extensions.get(ext.to_str().unwrap_or("")) {
                return lang;
            }
        }
        TemplateLanguage::Unknown
    }

    pub fn detect_from_content(&self, content: &str) -> TemplateLanguage {
        for (lang, pattern) in &self.content_patterns {
            if pattern.is_match(content) {
                return *lang;
            }
        }
        TemplateLanguage::Unknown
    }
}
```

#### 2. Language-Specific Extractors

```rust
pub trait TemplateExtractor {
    fn extract_classes(&self, content: &str) -> Vec<ExtractedClass>;
    fn supports_language(&self, language: TemplateLanguage) -> bool;
    fn get_preprocessing_rules(&self) -> Vec<PreprocessingRule>;
}

pub struct VueExtractor;
pub struct SvelteExtractor;
pub struct HamlExtractor;
pub struct PugExtractor;

impl TemplateExtractor for VueExtractor {
    fn extract_classes(&self, content: &str) -> Vec<ExtractedClass> {
        // Handle Vue-specific syntax:
        // - :class="dynamicClass"
        // - class="static-class"
        // - v-bind:class="{ active: isActive }"
        // - class:class-name="condition"
    }

    fn get_preprocessing_rules(&self) -> Vec<PreprocessingRule> {
        vec![
            // Convert Vue class bindings to extractable format
            PreprocessingRule::new(
                r#":class="([^"]*)""#,
                |captures| format!("class=\"{}\"", captures[1])
            ),
            // Handle v-bind:class
            PreprocessingRule::new(
                r#"v-bind:class="([^"]*)""#,
                |captures| format!("class=\"{}\"", captures[1])
            ),
        ]
    }
}
```

#### 3. Preprocessing Pipeline

```rust
pub struct TemplatePreprocessor {
    extractors: HashMap<TemplateLanguage, Box<dyn TemplateExtractor>>,
    preprocessing_cache: LruCache<String, String>,
}

impl TemplatePreprocessor {
    pub fn preprocess(&mut self, content: &str, language: TemplateLanguage) -> String {
        if let Some(extractor) = self.extractors.get(&language) {
            let cache_key = format!("{:?}:{}", language, hash_content(content));

            if let Some(preprocessed) = self.preprocessing_cache.get(&cache_key) {
                return preprocessed.clone();
            }

            let preprocessed = self.apply_preprocessing_rules(content, extractor);
            self.preprocessing_cache.put(cache_key, preprocessed.clone());
            preprocessed
        } else {
            content.to_string()
        }
    }

    fn apply_preprocessing_rules(&self, content: &str, extractor: &dyn TemplateExtractor) -> String {
        let mut result = content.to_string();

        for rule in extractor.get_preprocessing_rules() {
            result = rule.regex.replace_all(&result, |caps: &regex::Captures| {
                rule.replacement_function(caps)
            }).to_string();
        }

        result
    }
}
```

### Language-Specific Implementations

#### Vue.js Support
```rust
impl TemplateExtractor for VueExtractor {
    fn extract_classes(&self, content: &str) -> Vec<ExtractedClass> {
        let mut classes = Vec::new();

        // Object syntax: :class="{ active: isActive, 'text-danger': hasError }"
        let object_regex = regex!(r#":class="\{([^}]+)\}""#);
        for cap in object_regex.captures_iter(content) {
            let object_content = &cap[1];
            // Parse object keys as potential classes
            for key in self.parse_object_keys(object_content) {
                classes.push(ExtractedClass::new(key, ExtractedClassType::Dynamic));
            }
        }

        // Array syntax: :class="[activeClass, errorClass]"
        let array_regex = regex!(r#":class="\[(.*?)\]""#);
        for cap in array_regex.captures_iter(content) {
            let array_content = &cap[1];
            for class in self.parse_array_classes(array_content) {
                classes.push(ExtractedClass::new(class, ExtractedClassType::Dynamic));
            }
        }

        classes
    }
}
```

#### Svelte Support
```rust
impl TemplateExtractor for SvelteExtractor {
    fn extract_classes(&self, content: &str) -> Vec<ExtractedClass> {
        let mut classes = Vec::new();

        // Class directive: class:active={condition}
        let directive_regex = regex!(r#"class:([^{]+)=\{([^}]+)\}"#);
        for cap in directive_regex.captures_iter(content) {
            let class_name = cap[1].trim();
            classes.push(ExtractedClass::new(class_name, ExtractedClassType::Conditional));
        }

        // Class attribute: class="static {dynamic} classes"
        let class_attr_regex = regex!(r#"class="([^"]*)""#);
        for cap in class_attr_regex.captures_iter(content) {
            let class_content = &cap[1];
            for class in self.parse_class_attribute(class_content) {
                classes.push(ExtractedClass::new(class, ExtractedClassType::Mixed));
            }
        }

        classes
    }
}
```

#### Haml Support
```rust
impl TemplateExtractor for HamlExtractor {
    fn extract_classes(&self, content: &str) -> Vec<ExtractedClass> {
        let mut classes = Vec::new();

        // Haml class syntax: .class-name#id
        let haml_class_regex = regex!(r#"\.([^{#\s]+)"#);
        for cap in haml_class_regex.captures_iter(content) {
            let class_name = cap[1].trim();
            if !class_name.contains('{') && !class_name.contains('#') {
                classes.push(ExtractedClass::new(class_name, ExtractedClassType::Static));
            }
        }

        // Dynamic classes: .class{condition: 'dynamic-class'}
        let dynamic_regex = regex!(r#"\.([^{]+)\{([^}]+)\}"#);
        for cap in dynamic_regex.captures_iter(content) {
            let base_class = cap[1].trim();
            let condition = &cap[2];
            // Extract potential classes from condition
            for class in self.parse_dynamic_condition(condition) {
                classes.push(ExtractedClass::new(class, ExtractedClassType::Dynamic));
            }
        }

        classes
    }
}
```

### Integration Points

#### 1. Build System Integration

```rust
pub struct MultiLanguageClassExtractor {
    preprocessor: TemplatePreprocessor,
    detector: LanguageDetector,
    base_extractor: ClassExtractor,
}

impl MultiLanguageClassExtractor {
    pub fn extract_from_file(&self, path: &Path) -> Result<Vec<String>, ExtractionError> {
        let language = self.detector.detect_from_path(path);
        let content = fs::read_to_string(path)?;

        let preprocessed = self.preprocessor.preprocess(&content, language);
        let classes = self.base_extractor.extract(&preprocessed)?;

        Ok(classes.into_iter().map(|c| c.name).collect())
    }

    pub fn extract_from_content(&self, content: &str, language: TemplateLanguage) -> Vec<String> {
        let preprocessed = self.preprocessor.preprocess(content, language);
        self.base_extractor.extract(&preprocessed)
            .unwrap_or_default()
            .into_iter()
            .map(|c| c.name)
            .collect()
    }
}
```

#### 2. CLI Integration

```rust
// Usage: tailwind-rs extract --lang=vue --input=src/**/*.vue
#[derive(Parser)]
pub struct ExtractCommand {
    #[arg(long)]
    language: Option<TemplateLanguage>,

    #[arg(long)]
    input: Vec<PathBuf>,

    #[arg(long)]
    output: Option<PathBuf>,
}

impl ExtractCommand {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let extractor = MultiLanguageClassExtractor::new();
        let mut all_classes = Vec::new();

        for input_path in &self.input {
            let classes = extractor.extract_from_file(input_path)?;
            all_classes.extend(classes);
        }

        // Deduplicate and output
        all_classes.sort();
        all_classes.dedup();

        if let Some(output_path) = &self.output {
            fs::write(output_path, all_classes.join("\n"))?;
        } else {
            println!("{}", all_classes.join("\n"));
        }

        Ok(())
    }
}
```

### Implementation Plan

#### Phase 1: Core Infrastructure
- [ ] Create language detection system
- [ ] Implement preprocessing pipeline
- [ ] Add basic template extractor trait

#### Phase 2: Vue.js Support
- [ ] Implement VueExtractor with directive parsing
- [ ] Add Vue-specific preprocessing rules
- [ ] Test with real Vue.js projects

#### Phase 3: Svelte Support
- [ ] Implement SvelteExtractor with class directives
- [ ] Add Svelte preprocessing rules
- [ ] Test with Svelte projects

#### Phase 4: Additional Languages
- [ ] Implement Haml, Pug, Slim extractors
- [ ] Add Clojure, Elixir, Ruby support
- [ ] Extend to other template languages

#### Phase 5: Integration & Testing
- [ ] Integrate with CLI and build tools
- [ ] Add comprehensive test suite
- [ ] Performance optimization and benchmarking

### Performance Characteristics

#### Preprocessing Overhead
- **Caching**: Preprocessed content is cached to avoid repeated work
- **Incremental**: Only preprocess changed files
- **Efficient**: Regex-based preprocessing is fast

#### Memory Usage
- **LRU Cache**: Bounded cache prevents memory leaks
- **Streaming**: Large files processed in chunks
- **Cleanup**: Automatic cache eviction

### Compatibility Considerations

#### Framework Versions
- Support for current and recent versions of each framework
- Clear documentation of supported syntax versions
- Graceful handling of unsupported syntax

#### Encoding Support
- UTF-8 handling for international characters
- Proper handling of escaped characters
- Support for different line endings

### Risk Mitigation

#### False Positives Risk
- **Mitigation**: Conservative extraction rules
- **Validation**: Cross-check extracted classes
- **Filtering**: Remove obviously invalid class names

#### Performance Impact Risk
- **Mitigation**: Preprocessing only when needed
- **Optimization**: Efficient regex patterns
- **Monitoring**: Performance tracking in extraction

#### Maintenance Risk
- **Mitigation**: Modular extractor design
- **Testing**: Comprehensive test coverage
- **Documentation**: Clear guidelines for adding languages

## Success Metrics

- **Coverage**: Support for 10+ template languages
- **Accuracy**: >95% extraction accuracy across languages
- **Performance**: <10% overhead compared to single-language extraction
- **Maintainability**: Easy addition of new language support
