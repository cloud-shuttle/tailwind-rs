use tailwind_rs_core::cursor::Cursor;

fn main() {
    println!("🧪 Testing SIMD cursor functionality");

    // Test basic cursor operations
    let input = "   \t\n\r   hello world   \t\n";
    let mut cursor = Cursor::new(input.as_bytes());

    println!("📝 Input: {:?}", input);
    println!("📍 Initial position: {}", cursor.pos);

    // Test SIMD whitespace skipping
    let skipped = cursor.skip_whitespace_simd();
    println!("⚡ SIMD skip result: {}", skipped);
    println!("📍 Position after SIMD skip: {}", cursor.pos);

    // Test fallback whitespace skipping
    cursor.skip_whitespace_fallback();
    println!("📍 Position after fallback skip: {}", cursor.pos);

    println!("🔤 Current char: {:?}", cursor.current_char());
    println!("📄 Remaining: {:?}", String::from_utf8_lossy(cursor.remaining()));

    // Test fast skip directly
    let test_input = b"   \t\n\r   test";
    let test_cursor = Cursor::new(test_input);
    let skip_result = tailwind_rs_core::cursor::fast_skip::fast_skip_whitespace(&test_cursor);
    println!("🎯 Direct fast skip result: {:?}", skip_result);

    println!("✅ Cursor SIMD test completed successfully!");
}