# Examples

This section provides comprehensive examples demonstrating `tailwind-rs` usage across different scenarios and frameworks.

## Available Examples

### 🚀 Getting Started Examples
- **[Basic Usage](./basic-usage.md)** - Simple examples to get you started
- **[Hello World](./hello-world.md)** - Minimal setup examples for each framework

### 🎨 UI Component Examples
- **[Button Components](./button-components.md)** - Various button styles and states
- **[Card Components](./card-components.md)** - Card layouts and variations
- **[Form Components](./form-components.md)** - Form inputs and validation
- **[Navigation Components](./navigation-components.md)** - Navigation bars and menus

### 🎯 Framework-Specific Examples
- **[Leptos Examples](./leptos-examples.md)** - Leptos-specific implementations
- **[Yew Examples](./yew-examples.md)** - Yew-specific implementations
- **[Dioxus Examples](./dioxus-examples.md)** - Dioxus-specific implementations

### 🧪 Testing Examples
- **[Unit Testing](./unit-testing.md)** - Component unit tests
- **[Integration Testing](./integration-testing.md)** - Integration test examples
- **[Playwright E2E Testing](./playwright-testing.md)** - End-to-end testing examples

### 🎨 Advanced Examples
- **[Dynamic Styling](./dynamic-styling.md)** - Runtime class generation
- **[Theme System](./theme-system.md)** - Custom themes and design tokens
- **[Responsive Design](./responsive-design.md)** - Responsive layouts and breakpoints
- **[Animation Examples](./animations.md)** - CSS animations and transitions

### 🏗️ Real-World Examples
- **[Todo App](./todo-app.md)** - Complete todo application
- **[Dashboard](./dashboard.md)** - Admin dashboard example
- **[E-commerce](./ecommerce.md)** - Product catalog and shopping cart
- **[Blog](./blog.md)** - Blog layout and components

## Example Structure

Each example includes:

- **📝 Code**: Complete, runnable code examples
- **🧪 Tests**: Unit, integration, and E2E tests
- **📖 Explanation**: Detailed explanations of concepts
- **🎯 Best Practices**: Recommended patterns and approaches
- **🔧 Setup**: Required dependencies and configuration

## Running Examples

### Prerequisites
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js and PNPM (for Playwright tests)
curl -fsSL https://get.pnpm.io/install.sh | sh
pnpm install
```

### Quick Start
```bash
# Clone the repository
git clone https://github.com/your-org/tailwind-rs.git
cd tailwind-rs

# Run a specific example
cargo run --example hello-world

# Run tests for an example
cargo test --example hello-world

# Run Playwright tests
pnpm test:e2e
```

## Example Categories

### 🎯 Beginner Examples
Perfect for newcomers to `tailwind-rs`:
- Basic component creation
- Simple styling patterns
- Framework integration

### 🚀 Intermediate Examples
For developers familiar with the basics:
- Dynamic styling
- Component composition
- State management integration

### 🏆 Advanced Examples
For experienced developers:
- Complex layouts
- Performance optimization
- Custom theme systems

## Contributing Examples

We welcome contributions! Please see our [Contributing Guidelines](../contributing.md) for details on:

- Example structure and format
- Testing requirements
- Documentation standards
- Code review process

## Getting Help

- 📖 Check the [Getting Started Guide](../getting-started.md)
- 🏗️ Review [Architecture Documentation](../architecture.md)
- 🧪 Follow [Testing Guidelines](../testing.md)
- 💬 Join our community discussions

## Next Steps

1. 🚀 Start with [Basic Usage](./basic-usage.md)
2. 🎨 Explore [UI Component Examples](./button-components.md)
3. 🧪 Learn [Testing Patterns](./unit-testing.md)
4. 🏗️ Build [Real-World Applications](./todo-app.md)

