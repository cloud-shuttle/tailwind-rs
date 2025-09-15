# Contributing to `tailwind-rs`

Thank you for your interest in contributing to `tailwind-rs`! This document outlines our contribution guidelines, development standards, and processes that align with our Architecture Decision Records (ADRs).

## 🎯 Our Development Philosophy

### Test-Driven Development (TDD) First (ADR-001)
We follow a strict **Test-Driven Development** approach:
1. **Red**: Write a failing test first
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve code while keeping tests green
4. **Repeat**: Continue the cycle for each feature

### Testing Pyramid Strategy (ADR-002)
- **Unit Tests (70%)**: Fast, focused tests for individual functions
- **Integration Tests (20%)**: Test component interactions and data flow
- **End-to-End Tests (10%)**: Complete user workflows with Playwright

### Rust Coding Standards (ADR-007)
- **Latest Rust**: Always use the latest stable Rust (currently 1.89.0)
- **Edition 2021**: Use Rust 2021 edition for all new projects
- **Code Quality**: 100% test coverage, comprehensive documentation
- **Performance**: Benchmark critical code paths

## 🚀 Getting Started

### Prerequisites
- Rust 1.89.0+ (latest stable)
- Node.js 18+ and PNPM (for Playwright tests)
- Git
- VS Code with rust-analyzer (recommended)

### Development Setup
```bash
# Clone the repository
git clone https://github.com/your-org/tailwind-rs.git
cd tailwind-rs

# Install Rust dependencies
cargo build

# Install Node.js dependencies for testing
pnpm install

# Install Playwright browsers
pnpm run playwright:install

# Run the test suite
cargo test
pnpm run test:e2e
```

## 🧪 Development Workflow

### 1. TDD Development Process

#### Step 1: Write Failing Test (Red)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Given
        let input = "test input";
        
        // When
        let result = new_feature(input);
        
        // Then
        assert_eq!(result, "expected output");
    }
}
```

#### Step 2: Implement Minimal Code (Green)
```rust
pub fn new_feature(input: &str) -> String {
    "expected output".to_string()
}
```

#### Step 3: Refactor and Improve
```rust
pub fn new_feature(input: &str) -> String {
    // Improved implementation
    input.to_uppercase()
}
```

### 2. Testing Requirements

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_class_generation() {
        // Test specific functionality
        let classes = classes! {
            base: "px-4 py-2",
            variant: "bg-blue-600",
        };
        
        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }

    // Property-based testing
    proptest! {
        #[test]
        fn test_class_validation_properties(class_name in "[a-zA-Z0-9-]+") {
            let result = validate_class(&class_name);
            // Properties that should always hold
            match result {
                Ok(_) | Err(ValidationError::InvalidClass(_)) => {
                    prop_assert!(true);
                }
                _ => prop_assert!(false, "Unexpected error type"),
            }
        }
    }
}
```

#### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_component_integration() {
        // Test component interactions
        let component = Component::new();
        let result = component.render();
        
        assert!(result.contains("expected-html"));
    }

    #[tokio::test]
    async fn test_async_integration() {
        // Test async functionality
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

#### End-to-End Tests (Playwright)
```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature E2E Testing', () => {
  test('should demonstrate new feature', async ({ page }) => {
    await page.goto('/demo');
    
    // Test user workflow
    await page.click('[data-testid="new-feature-button"]');
    await expect(page.locator('[data-testid="result"]'))
      .toBeVisible();
    
    // Test performance
    const startTime = Date.now();
    await page.click('[data-testid="performance-test"]');
    const endTime = Date.now();
    
    expect(endTime - startTime).toBeLessThan(1000);
  });
});
```

## 📝 Code Standards

### Rust Code Standards (ADR-007)

#### Cargo.toml Configuration
```toml
[package]
name = "tailwind-rs"
version = "0.1.0"
edition = "2021"
rust-version = "1.89.0"

[dependencies]
# Latest stable versions
leptos = { version = "0.8.8", features = ["csr"] }
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.4"
mockall = "0.12"
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

#### Code Formatting
```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

#### Clippy Configuration
```toml
# .clippy.toml
deny = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::nursery",
    "clippy::cargo",
]
warn = [
    "clippy::missing_docs_in_private_items",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
]
```

### Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TailwindError {
    #[error("Invalid class name: {0}")]
    InvalidClassName(String),
    
    #[error("CSS generation failed: {0}")]
    CssGenerationFailed(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// Usage
pub fn process_classes(input: &str) -> Result<ProcessedClasses, TailwindError> {
    let parsed = parse_input(input)?;
    let processed = transform_classes(parsed)?;
    Ok(processed)
}
```

### Documentation Standards
```rust
/// Generates Tailwind CSS classes with type safety and validation.
///
/// # Examples
///
/// ```rust
/// use tailwind_rs::*;
///
/// let classes = classes! {
///     base: "px-4 py-2",
///     variant: "bg-blue-600 text-white",
/// };
/// 
/// assert!(classes.contains("px-4"));
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - Invalid class names are provided
/// - CSS generation fails
///
/// # Performance
///
/// This function is optimized for performance and should complete
/// in under 1ms for typical use cases.
pub fn generate_classes(config: ClassConfig) -> Result<String, TailwindError> {
    // Implementation
}
```

## 🔧 Development Tools

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality and consistency. The hooks are automatically set up when you clone the repository.

#### Setup

The pre-commit hooks are already configured in the repository. They will automatically run when you make commits.

#### What the Hooks Do

The pre-commit hooks perform the following checks:

1. **Run Tests**: Executes tests using `cargo nextest` (excluding broken framework crates)
2. **Code Formatting**: Checks that code is properly formatted with `cargo fmt`
3. **Linting**: Runs `cargo clippy` with strict warnings enabled
4. **Documentation**: Ensures documentation builds successfully
5. **Conventional Commits**: Enforces conventional commit message format
6. **Security**: Scans for potential secrets in committed files
7. **Large Files**: Warns about large files that should use Git LFS

#### Manual Hook Script

If you need to run the hooks manually:

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🔍 Running pre-commit checks..."

# Run tests (excluding broken framework crates for now)
if command -v cargo-nextest &> /dev/null; then
    cargo nextest run -p tailwind-rs-core -p tailwind-rs-macros -p tailwind-rs-testing || {
        echo "❌ Tests failed"
        exit 1
    }
else
    cargo test -p tailwind-rs-core -p tailwind-rs-macros -p tailwind-rs-testing || {
        echo "❌ Tests failed"
        exit 1
    }
fi
echo "✅ All tests passed"

# Check formatting
cargo fmt --all -- --check || {
    echo "❌ Code formatting check failed"
    echo "💡 Run 'cargo fmt --all' to fix formatting issues"
    exit 1
}
echo "✅ Code formatting is correct"

# Run clippy
cargo clippy -p tailwind-rs-core -p tailwind-rs-macros -p tailwind-rs-testing --all-targets --all-features -- -D warnings || {
    echo "❌ Clippy found issues"
    exit 1
}
echo "✅ Clippy checks passed"

# Check documentation
cargo doc --no-deps || {
    echo "❌ Documentation check failed"
    exit 1
}
echo "✅ Documentation builds successfully"

echo "🎉 All pre-commit checks passed!"
echo "📝 Ready to commit"
```

#### Modern Pre-commit Setup

The project also includes a `.pre-commit-config.yaml` file for use with the [pre-commit](https://pre-commit.com/) tool:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the git hook scripts
pre-commit install

# Run against all files
pre-commit run --all-files
```

#### Bypassing Hooks

If you need to bypass the pre-commit hooks (not recommended):

```bash
git commit --no-verify -m "your commit message"
```

#### Troubleshooting

- **Formatting Issues**: Run `cargo fmt --all` to fix formatting
- **Clippy Warnings**: Fix the warnings or add `#[allow(clippy::warning_name)]` if appropriate
- **Test Failures**: Ensure all tests pass before committing
- **Commit Message Format**: Use conventional commit format: `type(scope): description`

### VS Code Configuration
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": true
  }
}
```

## 📋 Pull Request Process

### 1. Create Feature Branch
```bash
git checkout -b feature/new-feature
```

### 2. Follow TDD Process
- Write failing test
- Implement minimal code
- Refactor and improve
- Ensure all tests pass

### 3. Update Documentation
- Update API documentation
- Add examples and usage guides
- Update README if needed

### 4. Run Full Test Suite
```bash
# Rust tests
cargo test
cargo test --test integration

# Playwright tests
pnpm run test:e2e

# Performance tests
cargo bench
```

### 5. Create Pull Request
- Use descriptive title and description
- Reference related issues
- Include test results and coverage
- Request review from maintainers

### Pull Request Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] E2E tests added/updated
- [ ] All tests pass

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Performance impact assessed
```

## 🎭 Demo Requirements (ADR-003)

### Demo Creation Process
1. **Create comprehensive demo** showcasing new features
2. **Write Playwright tests** for all demo functionality
3. **Test across browsers** (Chrome, Firefox, Safari)
4. **Performance testing** to ensure demo meets targets
5. **Accessibility testing** for WCAG compliance

### Demo Testing Example
```typescript
test.describe('New Feature Demo', () => {
  test('should demonstrate feature functionality', async ({ page }) => {
    await page.goto('/demo/new-feature');
    
    // Test core functionality
    await page.click('[data-testid="feature-button"]');
    await expect(page.locator('[data-testid="result"]'))
      .toBeVisible();
    
    // Test performance
    const startTime = Date.now();
    await page.click('[data-testid="performance-test"]');
    const endTime = Date.now();
    
    expect(endTime - startTime).toBeLessThan(1000);
  });
});
```

## 📊 Quality Gates

### Coverage Requirements
- **Unit Tests**: 100% coverage for new code
- **Integration Tests**: 100% coverage for new APIs
- **E2E Tests**: 100% coverage for new user workflows
- **Documentation**: 100% coverage for public APIs

### Performance Requirements
- **Class Generation**: <1ms for typical use cases
- **CSS Generation**: <100ms for large class sets
- **Bundle Size**: <50KB for typical applications
- **Demo Performance**: <2s load time

### Code Quality Requirements
- **Clippy**: No warnings allowed
- **Rustfmt**: All code must be formatted
- **Documentation**: All public APIs documented
- **Error Handling**: Proper error types and messages

## 🚀 Release Process

### Version Management
- Follow semantic versioning (SemVer)
- Update CHANGELOG.md for all releases
- Tag releases in Git
- Publish to crates.io

### Release Checklist
- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Release notes prepared
- [ ] Demo updated and tested

## 🤝 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Provide constructive feedback
- Help others learn and grow
- Follow the golden rule

### Communication
- Use GitHub Issues for bug reports
- Use GitHub Discussions for questions
- Use Discord for real-time chat
- Use Stack Overflow for technical questions

### Recognition
- Contributors are recognized in CONTRIBUTORS.md
- Significant contributions get co-author credits
- Regular contributors may be invited as maintainers

## 📚 Resources

### Learning Materials
- [Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Documentation](https://leptos.dev/)
- [Playwright Testing](https://playwright.dev/)
- [TDD Best Practices](https://testdriven.io/)

### Development Resources
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

## 🆘 Getting Help

### Support Channels
- **GitHub Issues**: Bug reports and feature requests
- **Discord**: Real-time community support
- **Stack Overflow**: Technical questions with `tailwind-rs` tag
- **Email**: contact@dataengineeringpro.com

### Mentorship
- New contributors can request mentorship
- Pair programming sessions available
- Code review feedback and guidance
- Learning resources and tutorials

---

Thank you for contributing to `tailwind-rs`! Together, we're building the future of Rust web development with reliable, performant, and well-tested Tailwind CSS integration.

