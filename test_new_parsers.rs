// Test the new BasicTransformsParser and ScaleParser
use tailwind_rs_core::css_generator::parsers::{BasicTransformsParser, ScaleParser};

fn main() {
    let basic_transforms = BasicTransformsParser::new();
    let scale = ScaleParser::new();
    
    // Test basic translate classes
    let translate_x_4 = basic_transforms.parse_class("translate-x-4");
    println!("translate-x-4: {:?}", translate_x_4);
    
    let translate_y_2 = basic_transforms.parse_class("translate-y-2");
    println!("translate-y-2: {:?}", translate_y_2);
    
    // Test scale classes
    let scale_x_50 = scale.parse_class("scale-x-50");
    println!("scale-x-50: {:?}", scale_x_50);
    
    let scale_y_75 = scale.parse_class("scale-y-75");
    println!("scale-y-75: {:?}", scale_y_75);
    
    println!("All parsers working correctly!");
}
