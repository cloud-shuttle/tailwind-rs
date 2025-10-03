# Boundary Classification System Design

## Overview

Implement a sophisticated boundary classification system using procedural macros, inspired by the official Tailwind Oxide implementation's `boundary.rs` and `ClassifyBytes` macro.

## Problem Statement

Current parsing lacks context-aware boundary checking. Classes can be incorrectly parsed when they appear in invalid contexts (inside strings, comments, etc.).

## Solution Architecture

### Core Components

#### 1. Boundary Classification Macro

```rust
use classification_macros::ClassifyBytes;

#[derive(Debug, Clone, Copy, ClassifyBytes)]
enum BoundaryClass {
    /// Valid boundaries (whitespace, quotes, etc.)
    #[bytes(b'\t', b'\n', b'\x0C', b'\r', b' ')]
    #[bytes(b'"', b'\'', b'`')]
    #[bytes(b'\0')]
    ValidBoundary,

    /// Valid characters before a class
    #[bytes(b'.')]
    #[bytes(b'}')]
    #[bytes(b'>')]
    BeforeValid,

    /// Valid characters after a class
    #[bytes(b']')]
    #[bytes(b'{')]
    #[bytes(b'=')]
    #[bytes(b'<')]
    AfterValid,

    /// Invalid boundary characters
    #[fallback]
    Invalid,
}
```

#### 2. Context-Aware Span Validation

```rust
pub struct BoundaryValidator {
    input: &[u8],
}

impl BoundaryValidator {
    pub fn has_valid_boundaries(&self, span: &Span) -> bool {
        let before = self.get_char_before(span.start);
        let after = self.get_char_after(span.end);

        self.is_valid_before_boundary(before) && self.is_valid_after_boundary(after)
    }

    fn get_char_before(&self, pos: usize) -> u8 {
        if pos == 0 {
            b'\0' // Start of input
        } else {
            self.input[pos - 1]
        }
    }

    fn get_char_after(&self, pos: usize) -> u8 {
        if pos >= self.input.len() {
            b'\0' // End of input
        } else {
            self.input[pos]
        }
    }
}
```

#### 3. Multi-Language Boundary Rules

```rust
pub struct LanguageBoundaryRules {
    language: TemplateLanguage,
    custom_boundaries: HashMap<u8, BoundaryType>,
}

impl LanguageBoundaryRules {
    pub fn for_language(lang: TemplateLanguage) -> Self {
        match lang {
            TemplateLanguage::HTML => Self::html_rules(),
            TemplateLanguage::Vue => Self::vue_rules(),
            TemplateLanguage::React => Self::react_rules(),
            TemplateLanguage::Svelte => Self::svelte_rules(),
            // ... other languages
        }
    }

    fn html_rules() -> Self {
        let mut rules = HashMap::new();
        rules.insert(b'<', BoundaryType::AfterValid);  // End tags
        rules.insert(b'>', BoundaryType::BeforeValid); // Start tags
        // ... more rules
        Self {
            language: TemplateLanguage::HTML,
            custom_boundaries: rules,
        }
    }
}
```

### Integration Points

#### 1. Parser Integration

Update all parsers to use boundary validation:

```rust
impl ColorParser {
    pub fn parse_class(&self, class: &str, context: &ParseContext) -> Option<Vec<CssProperty>> {
        // Check boundaries before parsing
        if !context.boundary_validator.has_valid_boundaries(class) {
            return None;
        }

        // Continue with normal parsing...
    }
}
```

#### 2. Extractor Enhancement

```rust
pub struct ClassExtractor {
    boundary_validator: BoundaryValidator,
    language_rules: LanguageBoundaryRules,
}

impl ClassExtractor {
    pub fn extract_classes(&self, content: &str, language: TemplateLanguage) -> Vec<String> {
        let rules = LanguageBoundaryRules::for_language(language);

        // Extract potential classes with boundary validation
        content
            .split_whitespace()
            .filter(|candidate| self.is_valid_class(candidate, &rules))
            .map(|s| s.to_string())
            .collect()
    }
}
```

### Language-Specific Rules

#### HTML/XHTML
```rust
// Valid before: whitespace, =, ", ', `, >, }
// Valid after: whitespace, =, ", ', `, <, {, \0
```

#### JavaScript/JSX
```rust
// Valid before: whitespace, =, (, [, {, ", ', `
// Valid after: whitespace, ), ], }, ,, ;, :, ", ', `
```

#### Vue.js
```rust
// Additional boundaries for Vue directives
// :class, v-bind:class, etc.
```

#### Svelte
```rust
// Additional boundaries for Svelte syntax
// class:, style:, etc.
```

### Implementation Plan

#### Phase 1: Core Boundary Infrastructure
- [ ] Create `boundary.rs` with classification logic
- [ ] Implement `ClassifyBytes` procedural macro
- [ ] Add basic boundary validation functions

#### Phase 2: Language-Specific Rules
- [ ] Define boundary rules for HTML, JS, Vue, Svelte
- [ ] Create extensible rule system
- [ ] Add template language detection

#### Phase 3: Parser Integration
- [ ] Update all parsers to accept boundary context
- [ ] Modify parser interfaces to include validation
- [ ] Add boundary checking to extraction logic

#### Phase 4: Testing & Validation
- [ ] Unit tests for boundary classification
- [ ] Integration tests with real templates
- [ ] Performance impact assessment

### Performance Characteristics

#### Expected Improvements

- **Accuracy**: Eliminate false positive class extractions
- **Performance**: Early boundary rejection prevents unnecessary parsing
- **Language Support**: Proper handling of template syntax

#### Memory Overhead

- **Minimal**: Classification lookup tables are small
- **Cached**: Language rules can be pre-computed
- **Efficient**: Byte-level classification is fast

### Compatibility Considerations

#### Template Language Support
- Extensible system for new template languages
- Backward compatibility with existing parsers
- Opt-in boundary validation for performance-critical paths

#### Error Handling
- Graceful degradation when boundary rules are unavailable
- Clear error messages for invalid configurations
- Fallback to basic boundary checking

### Risk Mitigation

#### False Negatives Risk
- **Mitigation**: Conservative boundary rules
- **Fallback**: Option to disable strict boundary checking
- **Testing**: Comprehensive test coverage for edge cases

#### Performance Impact Risk
- **Mitigation**: Boundary checking only when needed
- **Optimization**: Fast byte-level classification
- **Benchmarking**: Performance regression monitoring

## Success Metrics

- **Accuracy**: 99%+ reduction in false positive extractions
- **Performance**: <5% overhead for boundary validation
- **Extensibility**: Easy addition of new template languages
- **Maintainability**: Clear separation of boundary logic
