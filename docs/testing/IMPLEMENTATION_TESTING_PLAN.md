# 🧪 Implementation Testing Plan for Tailwind-RS v4.1 Features

## 📋 Overview

This document provides a practical, step-by-step testing plan for implementing Tailwind CSS v4.1 features in Tailwind-RS while preventing regressions and maintaining quality.

## 🎯 Testing Infrastructure Setup

### **1. Pre-commit Testing Configuration**

#### **`.pre-commit-config.yaml`**
```yaml
repos:
  - repo: local
    hooks:
      - id: rust-tests
        name: Run Rust tests
        entry: cargo test --workspace
        language: system
        pass_filenames: false
        always_run: true
        stages: [pre-commit]
      
      - id: rust-clippy
        name: Run Clippy
        entry: cargo clippy --workspace -- -D warnings
        language: system
        pass_filenames: false
        always_run: true
        stages: [pre-commit]
      
      - id: rust-fmt
        name: Run rustfmt
        entry: cargo fmt --workspace -- --check
        language: system
        pass_filenames: false
        always_run: true
        stages: [pre-commit]
      
      - id: wasm-tests
        name: Run WASM tests
        entry: cd crates/tailwind-rs-wasm && wasm-pack test --node
        language: system
        pass_filenames: false
        always_run: true
        stages: [pre-commit]
```

### **2. GitHub Actions Workflow**

#### **`.github/workflows/regression-prevention.yml`**
```yaml
name: Regression Prevention Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta]
        feature: [text-shadow, mask-utilities, backdrop-filters, container-queries]
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
        override: true
    
    - name: Run unit tests
      run: cargo test --workspace --verbose
    
    - name: Run feature-specific tests
      run: cargo test --features ${{ matrix.feature }} --verbose
    
    - name: Run clippy
      run: cargo clippy --workspace -- -D warnings
    
    - name: Check formatting
      run: cargo fmt --workspace -- --check
    
    - name: Run property-based tests
      run: cargo test --features proptest --verbose
    
    - name: Run integration tests
      run: cargo test --features integration-tests --verbose

  performance:
    runs-on: ubuntu-latest
    needs: test
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run performance tests
      run: cargo test --features performance-tests --verbose
    
    - name: Check bundle size
      run: |
        cd crates/tailwind-rs-wasm
        wasm-pack build --target web --out-dir pkg --release
        ls -la pkg/tailwind_rs_wasm_bg.wasm
        # Fail if bundle size exceeds 25KB
        if [ $(stat -c%s pkg/tailwind_rs_wasm_bg.wasm) -gt 25000 ]; then
          echo "Bundle size exceeds 25KB limit"
          exit 1
        fi

  browser-tests:
    runs-on: ubuntu-latest
    needs: test
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
    
    - name: Install dependencies
      run: |
        cd crates/tailwind-rs-wasm
        npm install
    
    - name: Build WASM
      run: |
        cd crates/tailwind-rs-wasm
        wasm-pack build --target web --out-dir pkg --release
    
    - name: Install Playwright
      run: npm install -D @playwright/test
    
    - name: Install Playwright browsers
      run: npx playwright install
    
    - name: Run Playwright tests
      run: npx playwright test --project=chromium --project=firefox --project=webkit
```

### **3. Test Structure Organization**

#### **Test Directory Structure**
```
crates/tailwind-rs-core/
├── src/
│   ├── text_shadow.rs
│   ├── mask_utilities.rs
│   └── ...
├── tests/
│   ├── unit/
│   │   ├── text_shadow_tests.rs
│   │   ├── mask_utilities_tests.rs
│   │   └── ...
│   ├── integration/
│   │   ├── framework_tests.rs
│   │   └── cross_feature_tests.rs
│   ├── performance/
│   │   ├── bundle_size_tests.rs
│   │   └── runtime_performance_tests.rs
│   └── regression/
│       ├── baseline_tests.rs
│       └── api_stability_tests.rs
```

## 🧪 Feature-Specific Testing

### **1. Text Shadow Utilities Testing**

#### **Unit Tests**
```rust
// tests/unit/text_shadow_tests.rs
#[cfg(test)]
mod text_shadow_tests {
    use tailwind_rs_core::text_shadow::*;

    #[test]
    fn test_text_shadow_none() {
        let shadow = TextShadow::None;
        assert_eq!(shadow.to_css(), "text-shadow: none;");
    }

    #[test]
    fn test_text_shadow_sm() {
        let shadow = TextShadow::Sm;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)"));
    }

    #[test]
    fn test_text_shadow_lg() {
        let shadow = TextShadow::Lg;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1)"));
    }

    #[test]
    fn test_text_shadow_custom() {
        let shadow = TextShadow::Custom("0 2px 4px rgba(0,0,0,0.1)".to_string());
        assert_eq!(shadow.to_css(), "text-shadow: 0 2px 4px rgba(0,0,0,0.1);");
    }

    #[test]
    fn test_text_shadow_arbitrary() {
        let shadow = TextShadow::Arbitrary("0 0 10px #ff0000".to_string());
        assert_eq!(shadow.to_css(), "text-shadow: 0 0 10px #ff0000;");
    }
}

#[cfg(test)]
mod text_shadow_integration_tests {
    use tailwind_rs_core::ClassBuilder;

    #[test]
    fn test_text_shadow_with_other_utilities() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Sm)
            .color(Color::new(ColorPalette::Blue, ColorShade::Shade500));
        
        let classes = builder.build();
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_text_shadow_responsive() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Sm)
            .responsive(ResponsiveBreakpoint::Md, |b| b.text_shadow(TextShadow::Lg));
        
        let classes = builder.build();
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("md:text-shadow-lg"));
    }
}
```

#### **Property-Based Tests**
```rust
// tests/property/text_shadow_proptest.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_text_shadow_arbitrary_values(value in "[a-zA-Z0-9\\s\\-\\+\\*\\(\\)\\,\\.\\#]+") {
        let shadow = TextShadow::Custom(value.clone());
        let css = shadow.to_css();
        assert!(css.starts_with("text-shadow: "));
        assert!(css.ends_with(";"));
        assert!(css.contains(&value));
    }

    #[test]
    fn test_text_shadow_combinations(
        shadow1 in any::<TextShadow>(),
        shadow2 in any::<TextShadow>()
    ) {
        let builder = ClassBuilder::new()
            .text_shadow(shadow1.clone())
            .text_shadow(shadow2.clone());
        
        let classes = builder.build();
        // Should contain the last shadow applied
        assert!(classes.contains(&format!("text-shadow-{}", shadow2.variant())));
    }
}
```

### **2. Mask Utilities Testing**

#### **Unit Tests**
```rust
// tests/unit/mask_utilities_tests.rs
#[cfg(test)]
mod mask_utilities_tests {
    use tailwind_rs_core::mask_utilities::*;

    #[test]
    fn test_mask_clip_none() {
        let clip = MaskClip::None;
        assert_eq!(clip.to_css(), "mask-clip: none;");
    }

    #[test]
    fn test_mask_clip_text() {
        let clip = MaskClip::Text;
        assert_eq!(clip.to_css(), "mask-clip: text;");
    }

    #[test]
    fn test_mask_image_none() {
        let image = MaskImage::None;
        assert_eq!(image.to_css(), "mask-image: none;");
    }

    #[test]
    fn test_mask_image_linear_gradient() {
        let image = MaskImage::LinearGradient("to right, blue, red".to_string());
        assert_eq!(image.to_css(), "mask-image: linear-gradient(to right, blue, red);");
    }

    #[test]
    fn test_mask_image_radial_gradient() {
        let image = MaskImage::RadialGradient("circle, blue, red".to_string());
        assert_eq!(image.to_css(), "mask-image: radial-gradient(circle, blue, red);");
    }
}
```

### **3. Performance Testing**

#### **Bundle Size Tests**
```rust
// tests/performance/bundle_size_tests.rs
#[cfg(test)]
mod bundle_size_tests {
    use std::fs;

    #[test]
    fn test_wasm_bundle_size() {
        let wasm_path = "crates/tailwind-rs-wasm/pkg/tailwind_rs_wasm_bg.wasm";
        let metadata = fs::metadata(wasm_path).expect("WASM file not found");
        let size = metadata.len();
        
        // Bundle size should be under 25KB
        assert!(size < 25_000, "Bundle size {} bytes exceeds 25KB limit", size);
    }

    #[test]
    fn test_js_bundle_size() {
        let js_path = "crates/tailwind-rs-wasm/pkg/tailwind_rs_wasm.js";
        let metadata = fs::metadata(js_path).expect("JS file not found");
        let size = metadata.len();
        
        // JS bundle should be under 10KB
        assert!(size < 10_000, "JS bundle size {} bytes exceeds 10KB limit", size);
    }
}
```

#### **Runtime Performance Tests**
```rust
// tests/performance/runtime_performance_tests.rs
#[cfg(test)]
mod runtime_performance_tests {
    use tailwind_rs_core::ClassBuilder;
    use std::time::Instant;

    #[test]
    fn test_class_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .text_shadow(TextShadow::Sm)
                .mask_clip(MaskClip::Text)
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Class generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many class builders
        let builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new()
                .text_shadow(TextShadow::Sm)
                .mask_clip(MaskClip::Text))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 1_000_000, "Memory usage too high: {} bytes", memory_increase);
    }
}
```

### **4. Visual Regression Testing**

#### **Playwright Visual Tests**
```javascript
// tests/visual/visual-regression.spec.js
const { test, expect } = require('@playwright/test');

test.describe('Visual Regression Tests', () => {
  test('text shadow utilities render correctly', async ({ page }) => {
    await page.goto('/comprehensive-demo.html');
    
    // Test text shadow utilities
    await page.click('.nav-tab[data-tab="utilities"]');
    await expect(page.locator('.text-shadow-sm')).toBeVisible();
    await expect(page.locator('.text-shadow-lg')).toBeVisible();
    
    // Take screenshot for visual comparison
    await expect(page).toHaveScreenshot('text-shadow-utilities.png');
  });

  test('mask utilities render correctly', async ({ page }) => {
    await page.goto('/comprehensive-demo.html');
    
    // Test mask utilities
    await page.click('.nav-tab[data-tab="utilities"]');
    await expect(page.locator('.mask-clip-text')).toBeVisible();
    await expect(page.locator('.mask-image-gradient')).toBeVisible();
    
    // Take screenshot for visual comparison
    await expect(page).toHaveScreenshot('mask-utilities.png');
  });

  test('backdrop filters render correctly', async ({ page }) => {
    await page.goto('/comprehensive-demo.html');
    
    // Test backdrop filters
    await page.click('.nav-tab[data-tab="utilities"]');
    await expect(page.locator('.backdrop-blur-sm')).toBeVisible();
    await expect(page.locator('.backdrop-blur-lg')).toBeVisible();
    
    // Take screenshot for visual comparison
    await expect(page).toHaveScreenshot('backdrop-filters.png');
  });
});
```

## 🚀 Implementation Workflow

### **Phase 1: Foundation Setup (Week 1)**

#### **Day 1-2: Testing Infrastructure**
1. **Set up pre-commit hooks**
2. **Configure GitHub Actions**
3. **Create test directory structure**
4. **Set up baseline tests**

#### **Day 3-4: Performance Baselines**
1. **Measure current bundle size**
2. **Establish performance baselines**
3. **Set up performance monitoring**
4. **Create performance test suite**

#### **Day 5: Documentation Testing**
1. **Set up documentation testing**
2. **Create example validation**
3. **Set up code example testing**

### **Phase 2: Feature Implementation (Weeks 2-8)**

#### **Week 2: Text Shadow Utilities**
1. **Implement TextShadow enum**
2. **Add unit tests**
3. **Add integration tests**
4. **Add performance tests**
5. **Add visual regression tests**
6. **Update documentation**

#### **Week 3: Mask Utilities**
1. **Implement MaskClip and MaskImage enums**
2. **Add comprehensive test suite**
3. **Add visual regression tests**
4. **Update documentation**

#### **Week 4: Enhanced Backdrop Filters**
1. **Extend existing backdrop filter support**
2. **Add new backdrop filter utilities**
3. **Add comprehensive test suite**
4. **Update documentation**

#### **Week 5: Container Queries**
1. **Implement container query support**
2. **Add responsive container queries**
3. **Add comprehensive test suite**
4. **Update documentation**

#### **Week 6: CSS Grid Subgrid**
1. **Implement subgrid support**
2. **Add grid subgrid utilities**
3. **Add comprehensive test suite**
4. **Update documentation**

#### **Week 7: Logical Properties**
1. **Implement logical property support**
2. **Add logical property utilities**
3. **Add comprehensive test suite**
4. **Update documentation**

#### **Week 8: Advanced Plugin System**
1. **Implement plugin system**
2. **Add plugin testing framework**
3. **Add comprehensive test suite**
4. **Update documentation**

### **Phase 3: Validation and Polish (Week 9)**

#### **Day 1-2: Full Regression Testing**
1. **Run complete test suite**
2. **Validate all baselines**
3. **Check performance metrics**
4. **Verify cross-browser compatibility**

#### **Day 3-4: Documentation and Examples**
1. **Update all documentation**
2. **Create comprehensive examples**
3. **Validate all code examples**
4. **Create migration guides**

#### **Day 5: Final Validation**
1. **Run final test suite**
2. **Validate bundle size**
3. **Check performance metrics**
4. **Prepare for release**

## 📊 Success Metrics

### **Test Coverage**
- **Unit Tests**: 100% coverage for new features
- **Integration Tests**: All framework combinations
- **Performance Tests**: Bundle size and runtime performance
- **Visual Tests**: Cross-browser compatibility

### **Quality Gates**
- **All tests pass**: 100% success rate
- **Performance**: No regression in bundle size or runtime
- **Compatibility**: All supported frameworks work
- **Documentation**: All examples compile and run

### **Regression Prevention**
- **Automated testing**: Every commit and PR
- **Performance monitoring**: Continuous performance tracking
- **Compatibility testing**: Cross-browser and cross-framework
- **API stability**: Backward compatibility maintained

This comprehensive testing plan ensures that implementing Tailwind v4.1 features won't introduce regressions while maintaining the high quality and performance standards of Tailwind-RS.
