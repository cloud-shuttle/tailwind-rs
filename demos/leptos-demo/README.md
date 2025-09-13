# Tailwind-RS Leptos Demo

A comprehensive demonstration of the Tailwind-RS library integrated with Leptos, showcasing Rust-native Tailwind CSS capabilities in a WebAssembly environment.

## 🚀 Features

- **Full Leptos Integration**: Complete Leptos 0.8.5 application with reactive components
- **Dynamic Class Generation**: Real-time Tailwind class generation and preview
- **Theme Support**: Light/dark mode toggle with seamless transitions
- **Responsive Design**: Mobile-first design that works across all devices
- **Component Showcase**: Comprehensive examples of buttons, cards, and form elements
- **Performance Optimized**: Efficient WASM runtime with minimal bundle size
- **Comprehensive Testing**: Full Playwright test suite with E2E, visual regression, and accessibility tests

## 🏗️ Architecture

### Core Components

- **App**: Main application component with routing and theme management
- **Header**: Navigation bar with theme toggle and responsive menu
- **HomePage**: Landing page with feature showcase
- **ComponentsPage**: Interactive component library
- **DynamicPage**: Real-time class generation playground
- **Footer**: Application footer with branding

### Key Features

- **Reactive State Management**: Using Leptos signals for state management
- **Dynamic Styling**: Runtime class generation with `ClassBuilder`
- **Theme Context**: Global theme state with automatic propagation
- **Responsive Navigation**: Mobile-friendly navigation with proper accessibility

## 🧪 Testing Strategy

### Test Categories

1. **Navigation Tests** (`tests/navigation.spec.ts`)
   - Header functionality
   - Page navigation
   - Active state management
   - Theme toggle

2. **Page-Specific Tests**
   - **Home Page** (`tests/home-page.spec.ts`): Hero section, feature cards, CTAs
   - **Components Page** (`tests/components-page.spec.ts`): Button variants, card components
   - **Dynamic Page** (`tests/dynamic-page.spec.ts`): Class generation, preset styles

3. **Accessibility Tests** (`tests/accessibility.spec.ts`)
   - Heading hierarchy
   - Keyboard navigation
   - Screen reader support
   - Color contrast
   - ARIA attributes

4. **Performance Tests** (`tests/performance.spec.ts`)
   - Load times
   - Navigation speed
   - Dynamic class generation efficiency
   - Memory usage
   - Concurrent interactions

5. **Visual Regression Tests** (`tests/visual-regression.spec.ts`)
   - Screenshot comparisons
   - Cross-browser consistency
   - Responsive design validation
   - Dark mode visual consistency

6. **End-to-End Workflows** (`tests/e2e-workflow.spec.ts`)
   - Complete user journeys
   - State persistence
   - Error handling
   - Responsive behavior

### Test Configuration

- **Multi-browser Testing**: Chrome, Firefox, Safari
- **Mobile Testing**: iOS Safari, Android Chrome
- **Visual Regression**: Screenshot comparison across viewports
- **Performance Monitoring**: Load time and interaction metrics
- **Accessibility Validation**: WCAG compliance checking

## 🛠️ Development

### Prerequisites

- Rust 1.70+
- Node.js 18+
- Python 3.8+ (for local server)

### Setup

1. **Install Rust dependencies**:
   ```bash
   cargo build
   ```

2. **Install Node.js dependencies**:
   ```bash
   npm install
   ```

3. **Install Playwright browsers**:
   ```bash
   npm run install:playwright
   ```

### Building

```bash
# Development build
npm run build

# Production build
npm run build:release
```

### Running Tests

```bash
# Run all tests
npm test

# Run with UI
npm run test:ui

# Run specific test suites
npm run test:visual
npm run test:e2e
npm run test:accessibility
npm run test:performance

# Run on specific browsers
npm run test:chromium
npm run test:firefox
npm run test:webkit

# Run mobile tests
npm run test:mobile
```

### Local Development

```bash
# Start local server
npm run serve

# Run tests in development mode
npm run dev
```

## 📱 Responsive Design

The demo is built with a mobile-first approach:

- **Mobile (375px)**: Single column layout, stacked navigation
- **Tablet (768px)**: Two-column grids, expanded navigation
- **Desktop (1920px)**: Multi-column layouts, full navigation

## 🎨 Theme System

### Light Mode
- Clean, minimal design with subtle shadows
- High contrast text for readability
- Blue accent colors for interactive elements

### Dark Mode
- Dark backgrounds with light text
- Maintained contrast ratios
- Consistent color scheme across components

## 🔧 Technical Details

### WASM Integration
- **Bundle Size**: ~2MB including runtime
- **Load Time**: <5 seconds on average connection
- **Memory Usage**: Optimized with efficient string interning

### Performance Optimizations
- **Tree Shaking**: Dead code elimination
- **Lazy Loading**: Components loaded on demand
- **Efficient Re-rendering**: Minimal DOM updates
- **Caching**: Class generation results cached

### Browser Support
- **Modern Browsers**: Chrome 90+, Firefox 88+, Safari 14+
- **Mobile Browsers**: iOS Safari 14+, Android Chrome 90+
- **WASM Support**: All modern browsers with WASM support

## 🚀 Deployment

### Static Hosting
The demo can be deployed to any static hosting service:

```bash
# Build for production
npm run build:release

# Deploy the entire directory
# (includes pkg/, index.html, styles.css)
```

### CDN Integration
- Tailwind CSS loaded from CDN for demo purposes
- WASM modules can be served from CDN
- Optimized for global distribution

## 📊 Performance Metrics

### Load Times
- **Initial Load**: <3 seconds
- **Navigation**: <100ms
- **Theme Switch**: <50ms
- **Class Generation**: <10ms per class

### Bundle Analysis
- **WASM Module**: ~1.5MB
- **JavaScript Runtime**: ~500KB
- **Total Bundle**: ~2MB

## 🔍 Debugging

### Console Logging
The demo includes comprehensive logging:
- WASM module loading
- Leptos application startup
- Component rendering
- Error handling

### Development Tools
- **Browser DevTools**: Full debugging support
- **Playwright Inspector**: Test debugging
- **Performance Profiler**: Built-in performance monitoring

## 🤝 Contributing

### Adding New Components
1. Create component in `src/lib.rs`
2. Add to appropriate page
3. Write tests in corresponding test file
4. Update visual regression tests

### Adding New Tests
1. Create test file in `tests/`
2. Follow existing patterns
3. Include accessibility checks
4. Add visual regression screenshots

### Performance Guidelines
- Keep bundle size minimal
- Optimize for mobile performance
- Maintain <100ms interaction times
- Test across all target browsers

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- **Leptos Team**: For the excellent Rust web framework
- **Tailwind CSS Team**: For the inspiration and design system
- **Playwright Team**: For the comprehensive testing framework
- **Rust Community**: For the amazing ecosystem and tools