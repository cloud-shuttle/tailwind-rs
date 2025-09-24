# Testing Strategy for `tailwind-rs`

This document outlines our comprehensive testing strategy for `tailwind-rs`, following our **Test-Driven Development (TDD) first approach** and **full testing pyramid** as defined in our ADRs.

## 🎯 Testing Philosophy

### TDD First Approach (ADR-001)
We follow a strict **Test-Driven Development** methodology:

1. **Red**: Write a failing test first
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve code while keeping tests green
4. **Repeat**: Continue the cycle for each feature

### Testing Pyramid Strategy (ADR-002)
Our testing follows a comprehensive pyramid structure:

- **Unit Tests (70%)**: Fast, focused tests for individual functions
- **Integration Tests (20%)**: Test component interactions and data flow
- **End-to-End Tests (10%)**: Complete user workflows with Playwright

## 🧪 Unit Testing

### Rust Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::*;

    #[test]
    fn test_class_generation() {
        // Given
        let classes = classes! {
            base: "px-4 py-2",
            variant: "bg-blue-600 text-white",
        };
        
        // When
        let result = classes.to_string();
        
        // Then
        assert!(result.contains("px-4"));
        assert!(result.contains("py-2"));
        assert!(result.contains("bg-blue-600"));
        assert!(result.contains("text-white"));
    }

    #[test]
    fn test_dynamic_class_generation() {
        // Given
        let color = Color::Blue;
        let intensity = 600;
        
        // When
        let classes = classes! {
            background: color.background(intensity),
            text: color.text(),
        };
        
        // Then
        assert_eq!(classes.background(), "bg-blue-600");
        assert_eq!(classes.text(), "text-white");
    }

    #[test]
    fn test_invalid_class_validation() {
        // Given
        let invalid_class = "invalid-class-name";
        
        // When & Then
        assert!(matches!(
            validate_class(invalid_class),
            Err(ValidationError::InvalidClass(_))
        ));
    }

    #[test]
    fn test_responsive_class_generation() {
        // Given
        let responsive = Responsive {
            sm: "text-sm",
            md: "text-base",
            lg: "text-lg",
        };
        
        // When
        let classes = classes! {
            responsive: responsive,
        };
        
        // Then
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
        assert!(classes.contains("lg:text-lg"));
    }
}
```

### Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_class_validation_properties(class_name in "[a-zA-Z0-9-]+") {
        let result = validate_class(&class_name);
        
        // Properties that should always hold
        match result {
            Ok(_) => {
                // Valid classes should be accepted
                prop_assert!(true);
            }
            Err(ValidationError::InvalidClass(_)) => {
                // Invalid classes should be rejected
                prop_assert!(true);
            }
            _ => prop_assert!(false, "Unexpected error type"),
        }
    }
}
```

## 🔗 Integration Testing

### Component Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use leptos::*;
    use tailwind_rs::*;

    #[test]
    fn test_button_component_integration() {
        // Given
        let button = Button::new()
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Medium);
        
        // When
        let classes = button.generate_classes();
        let html = button.render();
        
        // Then
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(html.contains("button"));
    }

    #[test]
    fn test_theme_system_integration() {
        // Given
        let theme = Theme::new()
            .primary_color(Color::Blue)
            .secondary_color(Color::Gray);
        
        let button = Button::new().theme(theme);
        
        // When
        let classes = button.generate_classes();
        
        // Then
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("hover:bg-blue-700"));
    }

    #[tokio::test]
    async fn test_async_class_generation() {
        // Given
        let async_generator = AsyncClassGenerator::new();
        
        // When
        let classes = async_generator.generate_classes().await;
        
        // Then
        assert!(!classes.is_empty());
        assert!(classes.contains("async-generated"));
    }
}
```

### API Integration Tests
```rust
#[cfg(test)]
mod api_integration_tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_css_generation_api() {
        // Given
        let client = Client::new();
        let request = CssGenerationRequest {
            classes: vec!["bg-blue-600".to_string(), "text-white".to_string()],
            optimize: true,
        };
        
        // When
        let response = client
            .post("http://localhost:3000/api/css/generate")
            .json(&request)
            .send()
            .await
            .unwrap();
        
        // Then
        assert_eq!(response.status(), 200);
        let css: CssGenerationResponse = response.json().await.unwrap();
        assert!(css.css.contains(".bg-blue-600"));
        assert!(css.css.contains(".text-white"));
    }
}
```

## 🎭 End-to-End Testing with Playwright

### Demo Testing (ADR-003)
```typescript
import { test, expect } from '@playwright/test';

test.describe('Tailwind-rs Demo Testing', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo');
    await page.waitForLoadState('networkidle');
  });

  test('should demonstrate class generation functionality', async ({ page }) => {
    // Test basic class generation
    await page.click('[data-testid="generate-classes-button"]');
    
    await expect(page.locator('[data-testid="generated-classes"]'))
      .toBeVisible();
    
    await expect(page.locator('[data-testid="generated-classes"]'))
      .toContainText('bg-blue-600 text-white');
  });

  test('should demonstrate dynamic styling', async ({ page }) => {
    // Test dynamic class generation
    await page.selectOption('[data-testid="color-selector"]', 'green');
    await page.selectOption('[data-testid="intensity-selector"]', '700');
    
    await page.click('[data-testid="apply-dynamic-styles"]');
    
    await expect(page.locator('[data-testid="dynamic-element"]'))
      .toHaveClass(/bg-green-700/);
  });

  test('should demonstrate responsive design', async ({ page }) => {
    // Test responsive classes
    await page.setViewportSize({ width: 375, height: 667 }); // Mobile
    await expect(page.locator('[data-testid="responsive-element"]'))
      .toHaveClass(/sm:text-sm/);
    
    await page.setViewportSize({ width: 768, height: 1024 }); // Tablet
    await expect(page.locator('[data-testid="responsive-element"]'))
      .toHaveClass(/md:text-base/);
    
    await page.setViewportSize({ width: 1920, height: 1080 }); // Desktop
    await expect(page.locator('[data-testid="responsive-element"]'))
      .toHaveClass(/lg:text-lg/);
  });

  test('should demonstrate theme system', async ({ page }) => {
    // Test theme switching
    await page.click('[data-testid="theme-toggle"]');
    
    await expect(page.locator('[data-testid="themed-element"]'))
      .toHaveClass(/dark:bg-gray-800/);
    
    await expect(page.locator('[data-testid="themed-element"]'))
      .toHaveClass(/dark:text-white/);
  });

  test('should handle error scenarios gracefully', async ({ page }) => {
    // Test invalid class handling
    await page.fill('[data-testid="custom-class-input"]', 'invalid-class-name');
    await page.click('[data-testid="apply-custom-class"]');
    
    await expect(page.locator('[data-testid="error-message"]'))
      .toContainText('Invalid class name');
  });
});
```

### Performance Testing
```typescript
test.describe('Performance Testing', () => {
  test('should generate classes within performance budget', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/demo');
    await page.waitForLoadState('networkidle');
    const loadTime = Date.now() - startTime;

    // Assert load time is under 2 seconds
    expect(loadTime).toBeLessThan(2000);

    // Test class generation performance
    const generationStartTime = Date.now();
    await page.click('[data-testid="generate-classes-button"]');
    await expect(page.locator('[data-testid="generated-classes"]'))
      .toBeVisible();
    const generationTime = Date.now() - generationStartTime;

    // Assert class generation is under 100ms
    expect(generationTime).toBeLessThan(100);
  });

  test('should handle large class sets efficiently', async ({ page }) => {
    await page.goto('/demo');
    
    // Generate large set of classes
    await page.click('[data-testid="generate-large-class-set"]');
    
    const startTime = Date.now();
    await expect(page.locator('[data-testid="large-class-set"]'))
      .toBeVisible({ timeout: 5000 });
    const processingTime = Date.now() - startTime;

    // Assert processing time is reasonable
    expect(processingTime).toBeLessThan(1000);
  });
});
```

### Cross-Browser Testing
```typescript
const browsers = ['chromium', 'firefox', 'webkit'];

browsers.forEach(browser => {
  test.describe(`${browser} compatibility`, () => {
    test.use({ browserName: browser });

    test('should work consistently across browsers', async ({ page }) => {
      await page.goto('/demo');
      
      // Test core functionality
      await page.click('[data-testid="generate-classes-button"]');
      await expect(page.locator('[data-testid="generated-classes"]'))
        .toBeVisible();
      
      // Test WASM functionality
      const wasmResult = await page.evaluate(() => {
        return window.tailwindRsModule ? 'loaded' : 'not loaded';
      });
      expect(wasmResult).toBe('loaded');
    });
  });
});
```

## 📊 Testing Metrics and Coverage

### Coverage Requirements
- **Minimum coverage**: 95% line coverage
- **Branch coverage**: 90% branch coverage
- **Function coverage**: 100% public function coverage
- **Integration coverage**: 100% API endpoint coverage

### Quality Gates
```yaml
# .github/workflows/testing.yml
name: Testing Pipeline
on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Run unit tests
        run: cargo test --verbose
      
      - name: Generate coverage report
        run: cargo tarpaulin --out Html --output-dir coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
      
      - name: Run integration tests
        run: cargo test --test integration

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
      
      - name: Setup PNPM
        uses: pnpm/action-setup@v2
        with:
          version: 8
      
      - name: Install dependencies
        run: pnpm install --frozen-lockfile
      
      - name: Install Playwright
        run: pnpm run playwright:install
      
      - name: Run E2E tests
        run: pnpm run test:e2e
```

## 🛠️ Testing Tools and Technologies

### Rust Testing
- **cargo test**: Built-in testing framework
- **criterion**: Benchmarking and performance testing
- **proptest**: Property-based testing
- **mockall**: Mocking framework for unit tests
- **tarpaulin**: Code coverage analysis

### Web Testing
- **Playwright**: End-to-end testing
- **wasm-bindgen-test**: WebAssembly testing
- **js-sys**: JavaScript interop testing
- **web-sys**: Web API testing

### CI/CD Integration
- **GitHub Actions**: Automated testing pipeline
- **Coverage reporting**: Integration with coverage services
- **Test result reporting**: Detailed test result analysis
- **Performance monitoring**: Continuous performance testing

## 📈 Testing Metrics and Monitoring

### Test Metrics
- **Test execution time**: Track test suite performance
- **Test coverage**: Monitor coverage percentages
- **Test reliability**: Track flaky test identification
- **Test maintenance**: Monitor test maintenance overhead

### Quality Metrics
- **Bug escape rate**: Measure bugs found in production
- **Mean time to detection**: Track time to find issues
- **Mean time to resolution**: Track time to fix issues
- **Customer satisfaction**: Monitor client feedback on quality

## 🔄 Continuous Testing

### Pre-commit Hooks
```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run unit tests
cargo test

# Check code coverage
cargo tarpaulin --out Html --output-dir coverage

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run rustfmt
cargo fmt --all -- --check
```

### Development Workflow
1. **Write failing test** (Red phase)
2. **Implement minimal code** (Green phase)
3. **Refactor and improve** (Refactor phase)
4. **Run full test suite**
5. **Commit changes**

## 📚 Testing Best Practices

### Test Organization
```
tests/
├── unit/                    # Unit tests
│   ├── class_generation.rs
│   ├── validation.rs
│   └── theme_system.rs
├── integration/             # Integration tests
│   ├── component_tests.rs
│   ├── api_tests.rs
│   └── framework_tests.rs
└── e2e/                     # End-to-end tests
    ├── demo_workflows.spec.ts
    ├── performance.spec.ts
    └── cross_browser.spec.ts
```

### Test Naming Conventions
- **Unit tests**: `test_[function_name]_[scenario]`
- **Integration tests**: `test_[component]_integration_[scenario]`
- **E2E tests**: `should [expected behavior] when [condition]`

### Test Data Management
- Use factories for test data generation
- Keep test data minimal and focused
- Use property-based testing for edge cases
- Mock external dependencies

## 🎯 Success Criteria

### Technical Metrics
- **Test Coverage**: ≥95% line coverage
- **Test Execution Time**: <30 seconds for full suite
- **Test Reliability**: <1% flaky test rate
- **Performance**: <100ms for class generation

### Quality Metrics
- **Bug Escape Rate**: <1% styling-related runtime errors
- **Mean Time to Detection**: <1 hour for test failures
- **Mean Time to Resolution**: <4 hours for test fixes
- **Developer Satisfaction**: >90% positive feedback

---

This testing strategy ensures that `tailwind-rs` meets the highest quality standards while providing a reliable, performant, and well-tested solution for Rust web frameworks.

