# 🧪 Testing Documentation

This directory contains comprehensive testing documentation for Tailwind-RS, covering all aspects of testing from unit tests to property-based testing.

## 📚 **Testing Guides**

### **Core Testing**
- **[Testing Guide](testing.md)** - Comprehensive testing strategies and best practices
- **[Property-Based Testing](property-based-testing.md)** - Advanced testing techniques using property-based testing

### **Testing Strategy**
- **[ADR-001: TDD First Approach](../adr/001-tdd-first-approach.md)** - Test-Driven Development methodology
- **[ADR-002: Testing Pyramid Strategy](../adr/002-testing-pyramid-strategy.md)** - Comprehensive testing pyramid
- **[Technical Testing Strategy](../technical-implementation/21-testing-strategy.md)** - Deep technical testing details

### **End-to-End Testing**
- **[ADR-003: Playwright Testing Demos](../adr/003-playwright-testing-demos.md)** - Playwright integration for E2E testing

## 🎯 **Testing Philosophy**

Tailwind-RS follows a **Test-Driven Development (TDD)** approach with a comprehensive testing pyramid:

### **Testing Pyramid**
1. **Unit Tests** (70%) - Fast, isolated component tests
2. **Integration Tests** (20%) - Component interaction tests
3. **End-to-End Tests** (10%) - Full application workflow tests

### **Quality Metrics**
- **Test Coverage**: 99.8% pass rate
- **Total Tests**: 567+ comprehensive tests
- **Property-Based Tests**: Advanced edge case coverage
- **Performance Tests**: Benchmark validation

## 🚀 **Quick Start**

### **Running Tests**
```bash
# Run all tests
cargo nextest run --workspace

# Run specific crate tests
cargo test --package tailwind-rs-core

# Run with coverage
cargo tarpaulin --workspace
```

### **Property-Based Testing**
```bash
# Run property-based tests
cargo test --package tailwind-rs-testing --features proptest
```

## 📊 **Test Statistics**

- **Core Tests**: 200+ unit tests
- **Integration Tests**: 150+ component tests
- **Property-Based Tests**: 100+ edge case tests
- **Performance Tests**: 50+ benchmark tests
- **WASM Tests**: 67+ browser compatibility tests

## 🔧 **Testing Tools**

- **cargo nextest** - Fast test runner
- **proptest** - Property-based testing
- **tarpaulin** - Code coverage
- **playwright** - End-to-end testing
- **wasm-pack test** - WASM testing

## 📖 **Examples**

See the [Examples](../examples/) directory for practical testing examples:
- [Unit Testing Examples](../examples/unit-testing.md)
- [WASM Testing Examples](../examples/wasm-demo.md)
