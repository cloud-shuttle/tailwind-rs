# 🔄 Migration Guide: Core to PostCSS

**Document**: Migration from tailwind-rs-core to tailwind-rs-postcss  
**Version**: 1.0  
**Date**: January 2025  
**Status**: 📋 **READY FOR USE**

---

## 🎯 **Why Migrate?**

### **Current Limitations with Core**
- ❌ Limited class support (only 5 hardcoded classes)
- ❌ "Unknown class" errors for 95% of Tailwind CSS
- ❌ No interactive states (hover, focus, active)
- ❌ No transitions, transforms, or advanced features
- ❌ Poor developer experience

### **Benefits of PostCSS**
- ✅ Full Tailwind CSS compatibility
- ✅ No "Unknown class" errors
- ✅ Complete feature set
- ✅ Better performance
- ✅ Future-proof

---

## 🚀 **Quick Migration**

### **Step 1: Update Dependencies**

**Before (Core Approach)**
```toml
[dependencies]
tailwind-rs-core = "0.16.0"
tailwind-rs-leptos = "0.16.0"
```

**After (PostCSS Approach)**
```toml
[dependencies]
tailwind-rs-postcss = "0.16.0"
tailwind-rs-leptos = "0.16.0"
```

### **Step 2: Update Imports**

**Before**
```rust
use tailwind_rs_core::{CssGenerator, ClassBuilder};
```

**After**
```rust
use tailwind_rs_postcss::{PostCSSEngine, PostCSSConfig};
```

### **Step 3: Initialize PostCSS Engine**

**Before**
```rust
let mut generator = CssGenerator::new();
generator.add_class("px-4")?;
let css = generator.generate_css();
```

**After**
```rust
#[tokio::main]
async fn main() {
    let config = PostCSSConfig::default();
    let engine = PostCSSEngine::new(config).await?;
    
    // Your app code here
}
```

---

## 📝 **Detailed Migration Examples**

### **Example 1: Basic Component Migration**

**Before (Core - Limited)**
```rust
use leptos::prelude::*;
use tailwind_rs_core::{CssGenerator, ClassBuilder};

#[component]
fn Button() -> impl IntoView {
    // This will cause "Unknown class" errors for many classes
    let classes = "px-4 py-2 rounded-md font-medium bg-blue-600 text-white hover:bg-blue-700 transition-colors";
    
    view! { 
        <button class=classes>
            "Click me"
        </button>
    }
}
```

**After (PostCSS - Full Support)**
```rust
use leptos::prelude::*;
use tailwind_rs_postcss::{PostCSSEngine, PostCSSConfig};

#[component]
fn Button() -> impl IntoView {
    // All classes work without errors!
    let classes = "px-4 py-2 rounded-md font-medium bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 active:scale-95 transition-all duration-200";
    
    view! { 
        <button class=classes>
            "Click me"
        </button>
    }
}

#[tokio::main]
async fn main() {
    let config = PostCSSConfig::default();
    let _engine = PostCSSEngine::new(config).await.expect("Failed to create PostCSS engine");
    
    leptos::mount_to_body(Button)
}
```

### **Example 2: Advanced Component Migration**

**Before (Core - Broken)**
```rust
use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;

#[component]
fn Card() -> impl IntoView {
    // Many of these classes will cause "Unknown class" errors
    let classes = "bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition-shadow duration-300 transform hover:scale-105";
    
    view! {
        <div class=classes>
            <h3 class="text-xl font-bold text-gray-900 mb-2">
                "Card Title"
            </h3>
            <p class="text-gray-600 leading-relaxed">
                "Card content with proper typography and spacing."
            </p>
        </div>
    }
}
```

**After (PostCSS - Full Support)**
```rust
use leptos::prelude::*;
use tailwind_rs_postcss::{PostCSSEngine, PostCSSConfig};

#[component]
fn Card() -> impl IntoView {
    // All classes work perfectly!
    let classes = "bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition-shadow duration-300 transform hover:scale-105 focus:ring-2 focus:ring-blue-500";
    
    view! {
        <div class=classes>
            <h3 class="text-xl font-bold text-gray-900 mb-2">
                "Card Title"
            </h3>
            <p class="text-gray-600 leading-relaxed">
                "Card content with proper typography and spacing."
            </p>
        </div>
    }
}

#[tokio::main]
async fn main() {
    let config = PostCSSConfig::default();
    let _engine = PostCSSEngine::new(config).await.expect("Failed to create PostCSS engine");
    
    leptos::mount_to_body(Card)
}
```

### **Example 3: Responsive Design Migration**

**Before (Core - Limited)**
```rust
use leptos::prelude::*;

#[component]
fn ResponsiveGrid() -> impl IntoView {
    // Responsive classes may not work properly
    let classes = "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4";
    
    view! {
        <div class=classes>
            // Grid items
        </div>
    }
}
```

**After (PostCSS - Full Support)**
```rust
use leptos::prelude::*;
use tailwind_rs_postcss::{PostCSSEngine, PostCSSConfig};

#[component]
fn ResponsiveGrid() -> impl IntoView {
    // All responsive classes work perfectly!
    let classes = "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-4";
    
    view! {
        <div class=classes>
            <div class="bg-blue-100 p-4 rounded">"Item 1"</div>
            <div class="bg-green-100 p-4 rounded">"Item 2"</div>
            <div class="bg-red-100 p-4 rounded">"Item 3"</div>
            <div class="bg-yellow-100 p-4 rounded">"Item 4"</div>
        </div>
    }
}

#[tokio::main]
async fn main() {
    let config = PostCSSConfig::default();
    let _engine = PostCSSEngine::new(config).await.expect("Failed to create PostCSS engine");
    
    leptos::mount_to_body(ResponsiveGrid)
}
```

---

## 🔧 **Configuration Migration**

### **Tailwind Configuration**

**Before (Core - Limited Config)**
```rust
// Core has limited configuration options
let generator = CssGenerator::new();
```

**After (PostCSS - Full Config)**
```rust
use tailwind_rs_postcss::{PostCSSConfig, PostCSSEngine};

let config = PostCSSConfig {
    plugins: vec![
        // Add any PostCSS plugins you need
    ],
    source_map: true,
    source_map_options: SourceMapOptions {
        inline: false,
        file: Some("styles.css.map".to_string()),
        source_root: Some("src/".to_string()),
        sources_content: true,
    },
    parser_options: ParseOptions {
        // Customize parsing options
    },
    transform_options: TransformOptions {
        // Customize transformation options
    },
    performance: PerformanceOptions {
        enable_cache: true,
        cache_size_limit: 1000,
        parallel_processing: true,
        memory_optimization: true,
    },
};

let engine = PostCSSEngine::new(config).await?;
```

---

## 🐛 **Common Migration Issues**

### **Issue 1: Async Runtime Required**

**Problem**: PostCSS requires async runtime
```rust
// This won't work
fn main() {
    let engine = PostCSSEngine::new(config)?; // Error: requires async
}
```

**Solution**: Use async main
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PostCSSConfig::default();
    let engine = PostCSSEngine::new(config).await?;
    
    // Your app code here
    Ok(())
}
```

### **Issue 2: Import Changes**

**Problem**: Different import paths
```rust
// Old imports don't work
use tailwind_rs_core::{CssGenerator, ClassBuilder};
```

**Solution**: Update imports
```rust
use tailwind_rs_postcss::{PostCSSEngine, PostCSSConfig};
```

### **Issue 3: Error Handling**

**Problem**: Different error types
```rust
// Old error handling
match generator.add_class("px-4") {
    Ok(_) => println!("Success"),
    Err(e) => println!("Error: {}", e),
}
```

**Solution**: Use PostCSS error handling
```rust
match engine.process_css(input_css).await {
    Ok(result) => println!("Generated CSS: {}", result.css),
    Err(e) => println!("PostCSS Error: {}", e),
}
```

---

## 📊 **Migration Checklist**

### **Pre-Migration**
- [ ] Identify all components using `tailwind-rs-core`
- [ ] List all Tailwind classes currently in use
- [ ] Check for any custom configurations
- [ ] Backup current implementation

### **During Migration**
- [ ] Update `Cargo.toml` dependencies
- [ ] Update import statements
- [ ] Add async runtime (`#[tokio::main]`)
- [ ] Initialize PostCSS engine
- [ ] Test all components
- [ ] Verify all classes work

### **Post-Migration**
- [ ] Run comprehensive tests
- [ ] Check for any remaining "Unknown class" errors
- [ ] Verify responsive design works
- [ ] Test interactive states (hover, focus, active)
- [ ] Performance testing
- [ ] Update documentation

---

## 🎯 **Migration Benefits**

### **Immediate Benefits**
- ✅ No more "Unknown class" errors
- ✅ Full Tailwind CSS support
- ✅ Better developer experience
- ✅ Future-proof implementation

### **Long-term Benefits**
- ✅ Access to latest Tailwind CSS features
- ✅ Better performance
- ✅ Plugin ecosystem support
- ✅ Source map generation
- ✅ Advanced CSS processing

---

## 🚀 **Next Steps**

1. **Start Migration**: Begin with simple components
2. **Test Thoroughly**: Verify all classes work
3. **Update Documentation**: Reflect new approach
4. **Share Experience**: Help other developers migrate

---

**Need Help?** Check out our [PostCSS Examples](../examples/) or [API Documentation](../api/) for more detailed information.
