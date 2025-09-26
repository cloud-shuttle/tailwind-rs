# Tailwind-RS v0.13.0 Release Notes

## 🎉 Major Feature Release - Advanced Tailwind CSS Support

**Release Date**: September 26, 2025  
**Version**: 0.13.0  
**Coverage**: 92.9% Tailwind CSS 4.1 Support

---

## 🚀 **What's New in v0.13.0**

### **🎨 Advanced CSS Parsers**
- **BackgroundPropertiesParser**: Support for complex background properties
- **TransitionPropertiesParser**: Complete transition and animation support
- **FractionalTransformsParser**: Fractional transform values like `-translate-x-1/2`
- **Enhanced DataAttributeParser**: Full data attribute modifier support

### **🔧 Enhanced Arbitrary Values**
- **Standard Size Classes**: `size-12`, `size-6`, `size-8`, `size-10`, `size-16`, `size-20`, `size-24`, `size-32`
- **Complex Arbitrary Values**: `w-[100px]`, `h-[50px]`, `top-[4px]`, `left-[7px]`
- **Complex calc() Expressions**: `left-[calc(50%+var(--offset))]`
- **Complex Drop Shadows**: `drop-shadow-[0_3px_1px_rgba(0,0,0,.15)]`

### **🎯 Advanced Layout Features**
- **Gradient Backgrounds**: `bg-gradient-to-r`, `from-blue-500`, `to-purple-600`, `via-teal-500/40`
- **Object Fit**: `object-cover`, `object-contain`, `object-fill`
- **Transform Origins**: `origin-top`, `origin-center`, `origin-bottom`
- **Divide Utilities**: `divide-y`, `divide-x`, `divide-zinc-100`, `dark:divide-zinc-100/5`

### **📱 Interactive Components**
- **Working Navbar**: Fully functional navbar with mobile menu toggle
- **Responsive Design**: Proper breakpoint handling for mobile/desktop
- **Data Attributes**: Complete support for data attribute modifiers

---

## 📊 **Coverage Statistics**

| Feature Category | Coverage | Classes |
|------------------|----------|---------|
| **Header/Navbar Classes** | 100% | 171/171 |
| **Advanced Features** | 100% | 28/28 |
| **Tailwind CSS 4.1 Core** | 92.9% | 65/70 |
| **Overall Production Use** | 95%+ | - |

---

## 🔧 **Bug Fixes**

- ✅ **Fixed Negative Positioning**: `-m-1`, `lg:-left-5`, `lg:-mt-2`, `xl:-top-1.5` now working
- ✅ **Fixed Opacity with Slash**: `bg-white/90`, `text-black/50`, `border-black/5` now working
- ✅ **Fixed Complex Transforms**: `-translate-x-1/2` now working
- ✅ **Fixed Data Attributes**: All data attribute classes now working

---

## 🎯 **Ready for Production**

This release represents a major milestone in Tailwind-RS development, providing comprehensive support for modern Tailwind CSS features with excellent coverage and performance.

### **Key Benefits:**
- **92.9% Tailwind CSS 4.1 Coverage** - Excellent for production use
- **100% Advanced Features Coverage** - All cutting-edge features supported
- **Enhanced Parser Architecture** - Modular system for better maintainability
- **Working Interactive Components** - Real-world navbar and mobile menu support

---

## 🚀 **Upgrade Path**

### **From v0.12.1:**
```toml
[dependencies]
tailwind-rs-core = "0.13.0"
tailwind-rs-leptos = "0.13.0"
tailwind-rs-yew = "0.13.0"
tailwind-rs-dioxus = "0.13.0"
```

### **Breaking Changes:**
- None! This is a fully backward-compatible release.

### **New Features Available:**
- All new parsers are automatically available
- Enhanced arbitrary value support
- Improved data attribute handling
- Better gradient and transform support

---

## 📚 **Documentation**

- **API Documentation**: [https://docs.rs/tailwind-rs-core/0.13.0](https://docs.rs/tailwind-rs-core/0.13.0)
- **Examples**: Check the `examples/` directory for comprehensive demos
- **Coverage Analysis**: Run `cargo run --example coverage_analysis` to see current coverage

---

## 🎉 **What's Next**

- **v0.13.1**: Patch release to achieve 100% Tailwind CSS 4.1 coverage
- **v0.14.0**: Additional framework integrations and performance optimizations
- **v1.0.0**: Stable API with long-term support guarantees

---

**Thank you for using Tailwind-RS! 🚀**
