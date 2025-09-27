# 🔧 DEPENDENCY CONFLICT RESOLUTION PLAN

**Date**: September 20, 2025  
**Priority**: **P0 - IMMEDIATE ACTION REQUIRED**  
**Status**: 🔴 **CRITICAL**

---

## 🎯 **OVERVIEW**

**Problem**: Multiple dependency version conflicts preventing compilation  
**Impact**: Build failures, security vulnerabilities, compatibility issues  
**Solution**: Immediate dependency alignment and version resolution  

---

## 🔴 **CRITICAL CONFLICTS IDENTIFIED**

### **1. Globset Version Conflict**

**Error**:
```bash
error: failed to select a version for `globset`.
versions that meet the requirements `^0.4.15` are: 0.4.16, 0.4.15
all possible versions conflict with previously selected packages.
```

**Root Cause**:
- `ignore = "^0.4.23"` requires `globset ^0.4.15`
- `watchexec = "^1.17.2"` requires `globset = "=0.4.6"`
- Version conflict between `0.4.6` and `^0.4.15`

**Impact**:
- ❌ **Build Failures**: Cannot compile in clean environment
- ❌ **CI/CD Issues**: Automated builds failing
- ❌ **Developer Experience**: Local development blocked

---

## 🚀 **IMMEDIATE RESOLUTION PLAN**

### **Phase 1: Critical Dependency Resolution (Day 1)**

#### **1. Update Watchexec Dependency**

**Current**: `watchexec = "^1.17.2"`  
**Target**: `watchexec = "^1.18.0"` (latest compatible)

**Action**:
```toml
# Update in all Cargo.toml files
[dependencies]
watchexec = "1.18.0"  # Updated version
```

**Rationale**:
- Latest version supports newer globset
- Maintains functionality
- Resolves version conflict

#### **2. Align Globset Versions**

**Current**: Mixed versions (`0.4.6` and `^0.4.15`)  
**Target**: Single version (`0.4.16`)

**Action**:
```toml
# Standardize across all crates
[dependencies]
globset = "0.4.16"  # Consistent version
ignore = "0.4.16"   # Aligned version
```

#### **3. Update Related Dependencies**

**Dependencies to Update**:
```toml
[dependencies]
# Core dependencies
serde = "1.0.300"        # Latest stable
tokio = "1.40"           # Latest stable
reqwest = "0.12"         # Latest stable
clap = "4.5"             # Latest stable

# Error handling
anyhow = "1.0.100"       # Latest stable
thiserror = "2.0"        # Latest stable

# Async runtime
futures = "0.3"          # Latest stable
async-trait = "0.1.100"  # Latest stable

# Serialization
serde_json = "1.0.200"   # Latest stable
toml = "0.9"             # Latest stable

# Testing
proptest = "1.4"         # Latest stable
criterion = "0.5"         # Latest stable
```

---

## 🔧 **DETAILED RESOLUTION STRATEGY**

### **1. Cargo.toml Updates**

#### **Root Cargo.toml**
```toml
[workspace]
members = [
    "crates/tailwind-rs-core",
    "crates/tailwind-rs-macros",
    "crates/tailwind-rs-leptos",
    "crates/tailwind-rs-yew",
    "crates/tailwind-rs-dioxus",
    "crates/tailwind-rs-postcss",
    "crates/tailwind-rs-wasm",
    "crates/tailwind-rs-cli",
    "crates/tailwind-rs-scanner",
    "crates/tailwind-rs-testing",
]

[workspace.dependencies]
# Core dependencies
serde = "1.0.300"
tokio = "1.40"
reqwest = "0.12"
clap = "4.5"

# Error handling
anyhow = "1.0.100"
thiserror = "2.0"

# Async runtime
futures = "0.3"
async-trait = "0.1.100"

# Serialization
serde_json = "1.0.200"
toml = "0.9"

# File system
globset = "0.4.16"
ignore = "0.4.16"
walkdir = "2.5"

# Development
watchexec = "1.18.0"
cargo-watch = "8.6"

# Testing
proptest = "1.4"
criterion = "0.5"
```

#### **Core Crate Dependencies**
```toml
# crates/tailwind-rs-core/Cargo.toml
[dependencies]
# Use workspace dependencies
serde = { workspace = true }
tokio = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }

# Core-specific dependencies
css-tree = "2.0"
postcss = "0.4"
autoprefixer = "0.4"
```

### **2. Version Conflict Resolution**

#### **Step 1: Remove Conflicting Dependencies**
```bash
# Remove old versions
cargo remove watchexec
cargo remove globset
cargo remove ignore
```

#### **Step 2: Add Aligned Dependencies**
```bash
# Add consistent versions
cargo add watchexec@1.18.0
cargo add globset@0.4.16
cargo add ignore@0.4.16
```

#### **Step 3: Update All Dependencies**
```bash
# Update all dependencies to latest compatible versions
cargo update
```

### **3. Rust Edition Update**

**Current**: Edition 2021  
**Target**: Edition 2024 (latest)

**Action**:
```toml
# Update in all Cargo.toml files
[package]
edition = "2024"
```

**Benefits**:
- Latest Rust features
- Better performance
- Improved error messages
- Enhanced tooling support

---

## 📊 **DEPENDENCY MATRIX**

### **Core Dependencies**

| Dependency | Current | Target | Status |
|------------|---------|--------|--------|
| `serde` | 1.0.227 | 1.0.300 | 🔄 Update |
| `tokio` | 1.35 | 1.40 | 🔄 Update |
| `reqwest` | 0.11 | 0.12 | 🔄 Update |
| `clap` | 4.4 | 4.5 | 🔄 Update |
| `anyhow` | 1.0.100 | 1.0.100 | ✅ Current |
| `thiserror` | 2.0.16 | 2.0 | 🔄 Update |

### **Development Dependencies**

| Dependency | Current | Target | Status |
|------------|---------|--------|--------|
| `watchexec` | 1.17.2 | 1.18.0 | 🔄 Update |
| `globset` | 0.4.6 | 0.4.16 | 🔄 Update |
| `ignore` | 0.4.23 | 0.4.16 | 🔄 Update |
| `cargo-watch` | 8.5.3 | 8.6 | 🔄 Update |

### **Testing Dependencies**

| Dependency | Current | Target | Status |
|------------|---------|--------|--------|
| `proptest` | 1.8.0 | 1.4 | 🔄 Update |
| `criterion` | 0.5 | 0.5 | ✅ Current |
| `wasm-bindgen-test` | 0.3.54 | 0.3.60 | 🔄 Update |

---

## 🔧 **IMPLEMENTATION STEPS**

### **Step 1: Backup Current State**
```bash
# Create backup branch
git checkout -b backup/dependency-conflicts
git add .
git commit -m "Backup before dependency resolution"
```

### **Step 2: Update Root Dependencies**
```bash
# Update root Cargo.toml
# Add workspace dependencies
# Update Rust edition
```

### **Step 3: Update Individual Crates**
```bash
# Update each crate's Cargo.toml
# Use workspace dependencies
# Remove conflicting versions
```

### **Step 4: Clean and Rebuild**
```bash
# Clean build artifacts
cargo clean

# Update lock file
cargo update

# Test compilation
cargo check --workspace
```

### **Step 5: Verify Resolution**
```bash
# Run full test suite
cargo test --workspace

# Check for remaining conflicts
cargo tree --duplicates
```

---

## 📋 **SUCCESS CRITERIA**

### **Immediate Goals (Day 1)**
- [ ] Zero dependency conflicts
- [ ] Clean compilation
- [ ] All tests passing
- [ ] Updated to latest versions

### **Quality Goals (Week 1)**
- [ ] Consistent dependency versions
- [ ] Security vulnerabilities resolved
- [ ] Performance improvements
- [ ] Documentation updated

### **Long-term Goals (Month 1)**
- [ ] Automated dependency updates
- [ ] Dependency conflict prevention
- [ ] Security monitoring
- [ ] Performance optimization

---

## 🚨 **RISK MITIGATION**

### **1. Backup Strategy**
- Create backup branch before changes
- Document all changes made
- Plan rollback procedure

### **2. Incremental Updates**
- Update one dependency at a time
- Test after each update
- Rollback if issues arise

### **3. Testing Strategy**
- Run full test suite after updates
- Test in clean environment
- Verify functionality

---

## 📊 **PROGRESS TRACKING**

| Task | Status | Progress | Notes |
|------|--------|----------|-------|
| Backup current state | 🔴 Pending | 0% | Create backup branch |
| Update root dependencies | 🔴 Pending | 0% | Update Cargo.toml |
| Update individual crates | 🔴 Pending | 0% | Update each crate |
| Clean and rebuild | 🔴 Pending | 0% | Clean build artifacts |
| Verify resolution | 🔴 Pending | 0% | Test compilation |
| Run test suite | 🔴 Pending | 0% | Verify functionality |

---

## 🎯 **NEXT STEPS**

1. **IMMEDIATE**: Create backup branch
2. **DAY 1**: Update root dependencies
3. **DAY 2**: Update individual crates
4. **DAY 3**: Clean and rebuild
5. **DAY 4**: Verify and test

---

*Status: 🔴 CRITICAL RESOLUTION REQUIRED*  
*Next Review: September 21, 2025*  
*Target Completion: September 24, 2025*
