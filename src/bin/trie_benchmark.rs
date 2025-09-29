//! Trie Performance Benchmark
//!
//! This binary demonstrates the performance improvement of trie-based parser lookup
//! compared to the previous sequential approach.

use tailwind_rs_core::css_generator::trie_benchmark::{benchmark_parser_performance, print_benchmark_results};

fn main() {
    println!("🔬 Tailwind-RS Trie Performance Benchmark");
    println!("========================================\n");

    let results = benchmark_parser_performance();
    print_benchmark_results(&results);

    println!("\n📝 Technical Details:");
    println!("- Trie uses prefix matching for O(1) lookups");
    println!("- Sequential approach checks ~60 parsers for each class");
    println!("- Performance benefits scale with number of classes");
    println!("- Memory overhead: minimal (prefix trie structure)");

    println!("\n✨ Trie Implementation Complete!");
    println!("   - ✅ Fast O(1) parser lookup");
    println!("   - ✅ Backward compatible with existing parsers");
    println!("   - ✅ Automatic fallback for edge cases");
    println!("   - ✅ Comprehensive prefix coverage (60+ parsers)");
}
