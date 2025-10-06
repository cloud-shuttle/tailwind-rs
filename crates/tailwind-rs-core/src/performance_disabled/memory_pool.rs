//! Memory Pool Optimizations for Tailwind-RS
//!
//! This module provides memory pool management for frequently allocated objects:
//! - Object pooling for ElementContext, CssRule, and other structs
//! - Memory arena allocation for temporary objects
//! - Garbage collection optimizations
//! - Memory usage monitoring

use crate::css_generator::types::CssProperty;
use crate::performance::parallel::MemoryPool;
use std::alloc::{alloc, dealloc, Layout};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// Memory arena for temporary allocations
#[derive(Debug)]
pub struct MemoryArena {
    /// Pointer to allocated memory block
    buffer: *mut u8,
    /// Total size of the arena
    capacity: usize,
    /// Current allocation offset
    offset: AtomicUsize,
    /// Layout for deallocation
    layout: Layout,
}

impl MemoryArena {
    /// Create a new memory arena with specified capacity
    pub fn new(capacity: usize) -> Result<Self, std::alloc::AllocError> {
        let layout = Layout::from_size_align(capacity, 8)
            .map_err(|_| std::alloc::AllocError)?;

        let buffer = unsafe { alloc(layout) };
        if buffer.is_null() {
            return Err(std::alloc::AllocError);
        }

        Ok(Self {
            buffer,
            capacity,
            offset: AtomicUsize::new(0),
            layout,
        })
    }

    /// Allocate memory from the arena
    pub fn allocate(&self, size: usize, align: usize) -> Option<*mut u8> {
        let align_mask = align - 1;
        let current_offset = self.offset.load(Ordering::Relaxed);

        // Calculate aligned offset
        let aligned_offset = (current_offset + align_mask) & !align_mask;
        let new_offset = aligned_offset + size;

        if new_offset > self.capacity {
            return None; // Out of memory
        }

        // Try to update offset atomically
        match self.offset.compare_exchange(
            current_offset,
            new_offset,
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                // Allocation successful
                Some(unsafe { self.buffer.add(aligned_offset) })
            }
            Err(_) => {
                // Concurrent allocation, try again or fail
                None
            }
        }
    }

    /// Reset the arena (not thread-safe, use with caution)
    pub fn reset(&self) {
        self.offset.store(0, Ordering::Release);
    }

    /// Get current memory usage
    pub fn used_memory(&self) -> usize {
        self.offset.load(Ordering::Relaxed)
    }

    /// Get total capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get allocation efficiency (0.0 to 1.0)
    pub fn efficiency(&self) -> f64 {
        let used = self.used_memory();
        if used == 0 {
            1.0
        } else {
            // Calculate internal fragmentation (simplified)
            // In a real implementation, this would track actual allocation sizes
            used as f64 / self.capacity as f64
        }
    }
}

impl Drop for MemoryArena {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.buffer, self.layout);
        }
    }
}

unsafe impl Send for MemoryArena {}
unsafe impl Sync for MemoryArena {}

/// Object pool manager for common Tailwind-RS objects
#[derive(Debug)]
pub struct ObjectPoolManager {
    /// CssProperty pool
    css_property_pool: MemoryPool<CssProperty>,
    /// String pool for common class names
    string_pool: StringInterner,
    /// Memory arenas for temporary allocations
    arenas: Mutex<Vec<MemoryArena>>,
    /// Statistics
    stats: Mutex<PoolStats>,
}

#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub css_properties_reused: usize,
    pub strings_interned: usize,
    pub total_memory_allocated: usize,
}

impl ObjectPoolManager {
    /// Create a new object pool manager
    pub fn new() -> Self {
        Self {
            css_property_pool: MemoryPool::new(|| CssProperty::new("temp".to_string(), "temp".to_string()), 10000),
            string_pool: StringInterner::new(),
            arenas: Mutex::new(Vec::new()),
            stats: Mutex::new(PoolStats::default()),
        }
    }

    /// Get a CssProperty from the pool
    pub fn get_css_property(&self) -> CssProperty {
        let mut stats = self.stats.lock().unwrap();
        stats.css_properties_reused += 1;
        self.css_property_pool.get()
    }

    /// Return a CssProperty to the pool
    pub fn return_css_property(&self, property: CssProperty) {
        self.css_property_pool.put(property);
    }

    /// Intern a string (return a reference to a cached version)
    pub fn intern_string(&self, s: &str) -> &'static str {
        let mut stats = self.stats.lock().unwrap();
        stats.strings_interned += 1;
        self.string_pool.intern(s)
    }

    /// Create a new memory arena
    pub fn create_arena(&self, capacity: usize) -> Result<(), Box<dyn std::error::Error>> {
        let arena = MemoryArena::new(capacity)?;
        self.arenas.lock().unwrap().push(arena);
        Ok(())
    }

    /// Allocate from available arenas
    pub fn allocate_from_arena(&self, size: usize, align: usize) -> Option<*mut u8> {
        let arenas = self.arenas.lock().unwrap();
        for arena in arenas.iter() {
            if let Some(ptr) = arena.allocate(size, align) {
                return Some(ptr);
            }
        }
        None
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.lock().unwrap().clone()
    }

    /// Reset all arenas
    pub fn reset_arenas(&self) {
        let arenas = self.arenas.lock().unwrap();
        for arena in arenas.iter() {
            arena.reset();
        }
    }

    /// Calculate memory efficiency across all pools
    pub fn memory_efficiency(&self) -> f64 {
        let arenas = self.arenas.lock().unwrap();
        if arenas.is_empty() {
            1.0
        } else {
            let total_efficiency: f64 = arenas.iter().map(|a| a.efficiency()).sum();
            total_efficiency / arenas.len() as f64
        }
    }

    /// Perform garbage collection (cleanup unused objects)
    pub fn garbage_collect(&self) {
        // Reset arenas that are mostly unused
        let mut arenas = self.arenas.lock().unwrap();
        arenas.retain(|arena| {
            let usage_ratio = arena.used_memory() as f64 / arena.capacity() as f64;
            usage_ratio > 0.1 // Keep arenas with >10% usage
        });
    }
}

/// String interner for efficient string storage
#[derive(Debug)]
pub struct StringInterner {
    /// Map from string content to static reference
    strings: Mutex<HashMap<String, &'static str>>,
    /// Backing storage for interned strings
    storage: Mutex<Vec<String>>,
}

impl StringInterner {
    /// Create a new string interner
    pub fn new() -> Self {
        Self {
            strings: Mutex::new(HashMap::new()),
            storage: Mutex::new(Vec::new()),
        }
    }

    /// Intern a string and return a static reference
    pub fn intern(&self, s: &str) -> &'static str {
        let mut strings = self.strings.lock().unwrap();

        if let Some(&interned) = strings.get(s) {
            return interned;
        }

        // Need to intern the string
        let mut storage = self.storage.lock().unwrap();
        storage.push(s.to_string());

        // Get a static reference (unsafe but controlled)
        let interned: &'static str = unsafe {
            let s_ptr = storage.last().unwrap().as_str() as *const str;
            &*s_ptr
        };

        strings.insert(s.to_string(), interned);
        interned
    }

    /// Get interner statistics
    pub fn stats(&self) -> InternerStats {
        let strings = self.strings.lock().unwrap();
        let storage = self.storage.lock().unwrap();

        let total_memory: usize = storage.iter().map(|s| s.len()).sum();

        InternerStats {
            unique_strings: strings.len(),
            total_memory,
            average_string_length: if strings.is_empty() {
                0.0
            } else {
                total_memory as f64 / strings.len() as f64
            },
        }
    }
}

/// String interner statistics
#[derive(Debug, Clone)]
pub struct InternerStats {
    pub unique_strings: usize,
    pub total_memory: usize,
    pub average_string_length: f64,
}

/// Garbage collector for memory management
#[derive(Debug)]
pub struct GarbageCollector {
    /// Threshold for triggering collection
    collection_threshold: usize,
    /// Current allocation count
    allocation_count: AtomicUsize,
    /// Object pools to clean up
    pools: Vec<std::sync::Weak<ObjectPoolManager>>,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new(threshold: usize) -> Self {
        Self {
            collection_threshold: threshold,
            allocation_count: AtomicUsize::new(0),
            pools: Vec::new(),
        }
    }

    /// Register an object pool for cleanup
    pub fn register_pool(&mut self, pool: std::sync::Weak<ObjectPoolManager>) {
        self.pools.push(pool);
    }

    /// Record an allocation
    pub fn record_allocation(&self) {
        let count = self.allocation_count.fetch_add(1, Ordering::Relaxed);

        if count >= self.collection_threshold {
            self.collect();
            self.allocation_count.store(0, Ordering::Release);
        }
    }

    /// Perform garbage collection
    fn collect(&self) {
        // Clean up weak references and collect live pools
        let live_pools: Vec<_> = self.pools.iter()
            .filter_map(|weak| weak.upgrade())
            .collect();

        // Perform collection on each pool
        for pool in live_pools {
            pool.garbage_collect();
            pool.reset_arenas();
        }
    }

    /// Get collection statistics
    pub fn stats(&self) -> GcStats {
        GcStats {
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            collection_threshold: self.collection_threshold,
            registered_pools: self.pools.len(),
        }
    }
}

/// Garbage collector statistics
#[derive(Debug, Clone)]
pub struct GcStats {
    pub allocation_count: usize,
    pub collection_threshold: usize,
    pub registered_pools: usize,
}

/// Memory-mapped file cache for large datasets
#[cfg(feature = "mmap")]
pub mod mmap_cache {
    use std::fs::OpenOptions;
    use std::path::Path;

    /// Memory-mapped cache for persistent storage
    pub struct MemoryMappedCache {
        // Implementation would use memmap2 crate
        // This is a placeholder for the concept
    }

    impl MemoryMappedCache {
        pub fn new<P: AsRef<Path>>(path: P, size: usize) -> std::io::Result<Self> {
            // Create or open memory-mapped file
            let _file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)?;

            // Memory map the file
            // Implementation would use memmap2::MmapMut

            unimplemented!("Memory-mapped cache requires memmap2 crate")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_arena_basic() {
        let arena = MemoryArena::new(1024).unwrap();

        // Test allocation
        let ptr1 = arena.allocate(64, 8);
        assert!(ptr1.is_some());

        let ptr2 = arena.allocate(128, 16);
        assert!(ptr2.is_some());

        // Check usage
        assert!(arena.used_memory() >= 64 + 128);
        assert!(arena.capacity() == 1024);
        assert!(arena.efficiency() > 0.0);

        // Test reset
        arena.reset();
        assert_eq!(arena.used_memory(), 0);
    }

    #[test]
    fn test_object_pool_manager() {
        let manager = ObjectPoolManager::new();

        // Test ElementContext pooling
        let context = manager.get_element_context();
        assert!(context.has_gradients().is_some()); // Default context has some state

        manager.return_element_context(context);

        // Test string interning
        let s1 = manager.intern_string("test-class");
        let s2 = manager.intern_string("test-class");
        assert_eq!(s1, s2); // Same reference

        let stats = manager.stats();
        assert!(stats.element_contexts_reused >= 1);
        assert!(stats.strings_interned >= 2);
    }

    #[test]
    fn test_string_interner() {
        let interner = StringInterner::new();

        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");
        let s3 = interner.intern("world");

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);

        let stats = interner.stats();
        assert_eq!(stats.unique_strings, 2);
        assert!(stats.total_memory >= 10); // "hello" + "world"
    }

    #[test]
    fn test_garbage_collector() {
        let mut gc = GarbageCollector::new(10);

        // Record allocations
        for _ in 0..15 {
            gc.record_allocation();
        }

        let stats = gc.stats();
        assert_eq!(stats.collection_threshold, 10);
        assert!(stats.allocation_count >= 15);
    }

    #[test]
    fn test_memory_efficiency() {
        let manager = ObjectPoolManager::new();

        // Create an arena
        manager.create_arena(4096).unwrap();

        // Test efficiency calculation
        let efficiency = manager.memory_efficiency();
        assert!(efficiency >= 0.0 && efficiency <= 1.0);
    }

    #[test]
    fn test_css_object_pooling() {
        let manager = ObjectPoolManager::new();

        // Test CSS property pooling
        let property = manager.get_css_property();
        assert_eq!(property.name, ""); // Default is empty

        manager.return_css_property(property);

        // Test CSS rule pooling
        let rule = manager.get_css_rule();
        assert_eq!(rule.selector, ""); // Default is empty

        manager.return_css_rule(rule);

        let stats = manager.stats();
        assert!(stats.css_properties_reused >= 1);
        assert!(stats.css_rules_reused >= 1);
    }
}
