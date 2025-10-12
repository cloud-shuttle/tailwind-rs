use tailwind_rs_core::css_generator::parsers::animations::AnimationParser;

fn main() {
    let parser = AnimationParser::default();
    let result = parser.parse_class("animate-pulse");
    println!("animate-pulse result: {:?}", result);
    
    let patterns = parser.get_supported_patterns();
    println!("supported patterns: {:?}", patterns);
    
    let priority = parser.get_priority();
    println!("priority: {}", priority);
}
