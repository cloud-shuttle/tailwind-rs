# 🚀 Tailwind-RS Core API Documentation v0.11.0

## 📖 **Overview**

The Tailwind-RS Core API provides a type-safe, programmatic way to generate Tailwind CSS classes. This documentation covers the enhanced ClassBuilder API with all utility methods, pseudo-class support, and best practices.

## 🎯 **Quick Start**

### **Basic Usage**

```rust
use tailwind_rs_core::classes::ClassBuilder;

// Create a new ClassBuilder
let classes = ClassBuilder::new()
    .relative()
    .flex()
    .p_4()
    .bg_blue_500()
    .text_white()
    .build_string();

// Result: "relative flex p-4 bg-blue-500 text-white"
```

### **With Pseudo-classes**

```rust
let classes = ClassBuilder::new()
    .bg_blue_500()
    .hover("bg-blue-600")
    .focus("ring-2")
    .dark("bg-gray-800")
    .build_string();

// Result: "bg-blue-500 hover:bg-blue-600 focus:ring-2 dark:bg-gray-800"
```

## 🏗️ **ClassBuilder API Reference**

### **Core Methods**

#### **Constructor**
```rust
// Create a new ClassBuilder
let builder = ClassBuilder::new();

// Or use Default
let builder = ClassBuilder::default();
```

#### **Building**
```rust
// Build into ClassSet
let class_set = builder.build();

// Build into CSS string
let css_string = builder.build_string();
```

### **Layout Utilities**

#### **Positioning**
```rust
// Positioning classes
.relative()     // position: relative
.absolute()     // position: absolute
.fixed()        // position: fixed
.sticky()       // position: sticky
.static_pos()   // position: static
```

#### **Display**
```rust
// Display classes
.block()        // display: block
.inline()       // display: inline
.inline_block() // display: inline-block
.flex()         // display: flex
.inline_flex()  // display: inline-flex
.grid()         // display: grid
.inline_grid()  // display: inline-grid
.hidden()       // display: none
.visible()      // visibility: visible
```

### **Flexbox Utilities**

#### **Flex Properties**
```rust
// Flex values
.flex_none()      // flex: none
.flex_1()         // flex: 1 1 0%
.flex_auto()      // flex: 1 1 auto
.flex_initial()   // flex: 0 1 auto

// Flex direction
.flex_col()       // flex-direction: column
.flex_row()       // flex-direction: row

// Flex wrap
.flex_wrap()      // flex-wrap: wrap
.flex_nowrap()    // flex-wrap: nowrap
```

### **Transition Utilities**

```rust
// Transition classes
.transition()           // transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms
.transition_all()       // transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms
.transition_colors()    // transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms
.transition_opacity()   // transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms
.transition_shadow()    // transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms
.transition_transform() // transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms
```

### **Pseudo-class Support**

#### **State Pseudo-classes**
```rust
// Hover state
.hover("bg-blue-600")     // hover:bg-blue-600

// Focus state
.focus("ring-2")          // focus:ring-2

// Active state
.active("bg-blue-700")    // active:bg-blue-700

// Dark mode
.dark("bg-gray-800")      // dark:bg-gray-800
```

#### **Group Pseudo-classes**
```rust
// Group hover
.group_hover("bg-blue-600")  // group-hover:bg-blue-600

// Peer hover
.peer_hover("bg-blue-600")   // peer-hover:bg-blue-600
```

### **Spacing Utilities**

```rust
// Padding
.p_4()    // padding: 1rem
.px_4()   // padding-left: 1rem; padding-right: 1rem
.py_4()   // padding-top: 1rem; padding-bottom: 1rem

// Margin
.m_4()    // margin: 1rem
.mx_4()   // margin-left: 1rem; margin-right: 1rem
.my_4()   // margin-top: 1rem; margin-bottom: 1rem
```

### **Color Utilities**

```rust
// Background colors
.bg_blue_500()    // background-color: rgb(59 130 246)

// Text colors
.text_white()     // color: rgb(255 255 255)

// Border colors
.border_gray_300() // border-color: rgb(209 213 219)
```

## 🔧 **Advanced Usage**

### **Responsive Design**

```rust
use tailwind_rs_core::responsive::Breakpoint;

let classes = ClassBuilder::new()
    .flex_col()
    .responsive(Breakpoint::Md, "flex-row")
    .responsive(Breakpoint::Lg, "flex-col")
    .build_string();
```

### **Conditional Classes**

```rust
let classes = ClassBuilder::new()
    .class("base-class")
    .conditional("condition", "conditional-class")
    .build_string();
```

### **Custom Properties**

```rust
let classes = ClassBuilder::new()
    .class("base-class")
    .custom("--custom-property", "value")
    .build_string();
```

### **ARIA and Data Variants**

```rust
let classes = ClassBuilder::new()
    .class("base-class")
    .aria("expanded", "aria-expanded-class")
    .data("state", Some("open"), "data-state-open-class")
    .supports("backdrop-filter", "supports-backdrop-filter-class")
    .build_string();
```

## 🎨 **Real-world Examples**

### **Button Component**

```rust
let button_classes = ClassBuilder::new()
    .relative()
    .inline_flex()
    .items_center()
    .justify_center()
    .px_4()
    .py_2()
    .bg_blue_500()
    .text_white()
    .rounded()
    .hover("bg-blue-600")
    .focus("ring-2")
    .active("bg-blue-700")
    .disabled("opacity-50")
    .transition()
    .transition_colors()
    .build_string();
```

### **Card Component**

```rust
let card_classes = ClassBuilder::new()
    .relative()
    .bg_white()
    .rounded_lg()
    .shadow_md()
    .p_6()
    .hover("shadow_lg")
    .dark("bg-gray-800")
    .transition()
    .transition_shadow()
    .build_string();
```

### **Form Input**

```rust
let input_classes = ClassBuilder::new()
    .block()
    .w_full()
    .px_3()
    .py_2()
    .border()
    .border_gray_300()
    .rounded_md()
    .focus("ring-2")
    .focus("ring-blue-500")
    .focus("border-blue-500")
    .disabled("opacity-50")
    .disabled("cursor-not-allowed")
    .build_string();
```

## ⚠️ **Error Handling**

### **Pseudo-class Validation**

The API includes built-in validation for pseudo-classes to prevent common mistakes:

```rust
// ❌ This will panic with a helpful error message
.hover("")  // Error: "hover: class cannot be empty. Use hover(\"bg-blue-500\") instead of hover(\"\")"

// ❌ This will panic with a helpful error message
.hover("hover:bg-blue-500")  // Error: "hover: class should not contain ':' prefix. Use hover(\"bg-blue-500\") instead of hover(\"hover:bg-blue-500\")"

// ✅ This is correct
.hover("bg-blue-500")
```

### **Common Mistakes to Avoid**

1. **Don't include ':' prefix in pseudo-class methods**
   ```rust
   // ❌ Wrong
   .hover("hover:bg-blue-500")
   
   // ✅ Correct
   .hover("bg-blue-500")
   ```

2. **Don't pass empty strings to pseudo-class methods**
   ```rust
   // ❌ Wrong
   .hover("")
   
   // ✅ Correct
   .hover("bg-blue-500")
   ```

3. **Use the correct method names**
   ```rust
   // ❌ Wrong (these methods don't exist)
   .relative_position()
   .flex_none()
   .transition_all()
   
   // ✅ Correct
   .relative()
   .flex_none()
   .transition_all()
   ```

## 🚀 **Migration Guide**

### **From String-based Classes**

#### **Before (String-based)**
```rust
let classes = "relative flex p-4 bg-blue-500 text-white hover:bg-blue-600 focus:ring-2";
```

#### **After (Programmatic)**
```rust
let classes = ClassBuilder::new()
    .relative()
    .flex()
    .p_4()
    .bg_blue_500()
    .text_white()
    .hover("bg-blue-600")
    .focus("ring-2")
    .build_string();
```

### **Benefits of Migration**

- ✅ **Type Safety**: Compile-time validation of class names
- ✅ **IDE Support**: Autocomplete and IntelliSense
- ✅ **Refactoring**: Easy to rename and update classes
- ✅ **Documentation**: Self-documenting code
- ✅ **Error Prevention**: Catch mistakes at compile time

## 📚 **Best Practices**

### **1. Use Fluent API Design**
```rust
// ✅ Good: Fluent, readable
let classes = ClassBuilder::new()
    .relative()
    .flex()
    .flex_col()
    .p_4()
    .bg_blue_500()
    .text_white()
    .build_string();

// ❌ Avoid: Mixing string-based and programmatic
let classes = ClassBuilder::new()
    .class("relative flex flex-col p-4")
    .bg_blue_500()
    .text_white()
    .build_string();
```

### **2. Group Related Classes**
```rust
// ✅ Good: Logical grouping
let classes = ClassBuilder::new()
    // Layout
    .relative()
    .flex()
    .flex_col()
    // Spacing
    .p_4()
    .m_2()
    // Colors
    .bg_blue_500()
    .text_white()
    // States
    .hover("bg-blue-600")
    .focus("ring-2")
    .build_string();
```

### **3. Use Descriptive Variable Names**
```rust
// ✅ Good: Descriptive names
let button_classes = ClassBuilder::new()
    .relative()
    .inline_flex()
    .px_4()
    .py_2()
    .bg_blue_500()
    .text_white()
    .build_string();

let card_classes = ClassBuilder::new()
    .relative()
    .bg_white()
    .rounded_lg()
    .shadow_md()
    .p_6()
    .build_string();
```

### **4. Handle Responsive Design**
```rust
// ✅ Good: Responsive design
let classes = ClassBuilder::new()
    .flex_col()
    .responsive(Breakpoint::Md, "flex-row")
    .responsive(Breakpoint::Lg, "flex-col")
    .build_string();
```

## 🔍 **Troubleshooting**

### **Common Issues**

1. **Method not found**
   - Check if you're using the correct method name
   - Ensure you're importing the ClassBuilder correctly

2. **Pseudo-class errors**
   - Don't include ':' prefix in pseudo-class methods
   - Don't pass empty strings to pseudo-class methods

3. **Build errors**
   - Ensure all methods are chained correctly
   - Check that you're calling `.build_string()` at the end

### **Getting Help**

- 📖 Check this documentation
- 🐛 Report issues on GitHub
- 💬 Join the community discussions
- 📧 Contact the maintainers

## 🎯 **Next Steps**

1. **Explore the API**: Try the examples above
2. **Read the source**: Check the implementation details
3. **Contribute**: Help improve the API
4. **Share**: Let others know about Tailwind-RS

---

**Happy coding with Tailwind-RS! 🚀**
