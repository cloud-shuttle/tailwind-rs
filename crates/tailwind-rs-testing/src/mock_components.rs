//! Mock components for testing tailwind-rs

use std::collections::HashMap;

/// A mock component for testing
#[derive(Debug, Clone)]
pub struct MockComponent {
    pub name: String,
    pub html: String,
    pub classes: std::collections::HashSet<String>,
    pub custom_properties: HashMap<String, String>,
    pub props: HashMap<String, String>,
}

impl MockComponent {
    /// Create a new mock component
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            html: String::new(),
            classes: std::collections::HashSet::new(),
            custom_properties: HashMap::new(),
            props: HashMap::new(),
        }
    }

    /// Set the HTML content
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = html.into();
        self
    }

    /// Add a CSS class
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.insert(class.into());
        self
    }

    /// Add multiple CSS classes
    pub fn with_classes(mut self, classes: impl IntoIterator<Item = String>) -> Self {
        for class in classes {
            self.classes.insert(class);
        }
        self
    }

    /// Add a custom property
    pub fn with_custom_property(
        mut self,
        property: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_properties.insert(property.into(), value.into());
        self
    }

    /// Add a prop
    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }

    /// Generate the final HTML
    pub fn to_html(&self) -> String {
        let mut html = self.html.clone();

        // If the HTML contains placeholder tokens, use the old behavior
        if html.contains("{{classes}}") || html.contains("{{style}}") {
            if !self.classes.is_empty() {
                let mut classes_vec: Vec<String> = self.classes.iter().cloned().collect();
                classes_vec.sort(); // Ensure deterministic ordering
                let classes_str = classes_vec.join(" ");
                html = html.replace("{{classes}}", &format!("class=\"{}\"", classes_str));
            }

            if !self.custom_properties.is_empty() {
                let style_parts: Vec<String> = self
                    .custom_properties
                    .iter()
                    .map(|(k, v)| format!("--{}: {}", k, v))
                    .collect();
                let style_str = format!("style=\"{}\"", style_parts.join("; "));
                html = html.replace("{{style}}", &style_str);
            }

            for (key, value) in &self.props {
                html = html.replace(&format!("{{{{{}}}}}", key), value);
            }
        } else {
            // If no placeholder tokens, wrap the content with classes and styles
            let mut attributes = Vec::new();
            
            if !self.classes.is_empty() {
                let mut classes_vec: Vec<String> = self.classes.iter().cloned().collect();
                classes_vec.sort(); // Ensure deterministic ordering
                let classes_str = classes_vec.join(" ");
                attributes.push(format!("class=\"{}\"", classes_str));
            }

            if !self.custom_properties.is_empty() {
                let style_parts: Vec<String> = self
                    .custom_properties
                    .iter()
                    .map(|(k, v)| format!("--{}: {}", k, v))
                    .collect();
                attributes.push(format!("style=\"{}\"", style_parts.join("; ")));
            }

            // Replace props in the HTML content
            let mut processed_html = html;
            for (key, value) in &self.props {
                processed_html = processed_html.replace(&format!("{{{{{}}}}}", key), value);
            }

            if !attributes.is_empty() {
                // Wrap the content in a div with the attributes
                html = format!("<div {}>{}</div>", attributes.join(" "), processed_html);
            } else {
                html = processed_html;
            }
        }

        html
    }
}

/// Create a mock button component
pub fn create_mock_button() -> MockComponent {
    MockComponent::new("button")
        .with_html("<button {{classes}} {{style}}>{{text}}</button>")
        .with_class("bg-blue-500")
        .with_class("text-white")
        .with_class("px-4")
        .with_class("py-2")
        .with_class("rounded")
        .with_prop("text", "Click me")
}

/// Create a mock card component
pub fn create_mock_card() -> MockComponent {
    MockComponent::new("card")
        .with_html("<div {{classes}} {{style}}>{{content}}</div>")
        .with_class("bg-white")
        .with_class("shadow-lg")
        .with_class("rounded-lg")
        .with_class("p-6")
        .with_prop("content", "Card content")
}

/// Create a mock input component
pub fn create_mock_input() -> MockComponent {
    MockComponent::new("input")
        .with_html("<input {{classes}} {{style}} placeholder=\"{{placeholder}}\" />")
        .with_class("w-full")
        .with_class("px-3")
        .with_class("py-2")
        .with_class("border")
        .with_class("border-gray-300")
        .with_class("rounded-md")
        .with_prop("placeholder", "Enter text")
}

/// Create a mock responsive grid component
pub fn create_mock_responsive_grid() -> MockComponent {
    MockComponent::new("responsive_grid")
        .with_html("<div {{classes}} {{style}}>{{items}}</div>")
        .with_class("grid")
        .with_class("gap-4")
        .with_class("sm:grid-cols-1")
        .with_class("md:grid-cols-2")
        .with_class("lg:grid-cols-3")
        .with_prop("items", "Grid items")
}

/// Create a mock themed component
pub fn create_mock_themed_component() -> MockComponent {
    MockComponent::new("themed_component")
        .with_html("<div {{classes}} {{style}}>{{content}}</div>")
        .with_class("p-4")
        .with_class("rounded-lg")
        .with_class("border")
        .with_custom_property("primary-color", "#3b82f6")
        .with_custom_property("spacing", "1rem")
        .with_prop("content", "Themed content")
}

/// Create a mock component with custom configuration
pub fn create_mock_component(name: impl Into<String>) -> MockComponent {
    MockComponent::new(name)
}

/// Create a mock component for testing specific scenarios
pub fn create_mock_component_for_test(test_name: &str) -> MockComponent {
    match test_name {
        "button" => create_mock_button(),
        "card" => create_mock_card(),
        "input" => create_mock_input(),
        "responsive_grid" => create_mock_responsive_grid(),
        "themed_component" => create_mock_themed_component(),
        _ => create_mock_component(test_name),
    }
}

/// Create multiple mock components for testing
pub fn create_mock_components() -> Vec<MockComponent> {
    vec![
        create_mock_button(),
        create_mock_card(),
        create_mock_input(),
        create_mock_responsive_grid(),
        create_mock_themed_component(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_component_creation() {
        let component = MockComponent::new("test");
        assert_eq!(component.name, "test");
        assert!(component.html.is_empty());
        assert!(component.classes.is_empty());
        assert!(component.custom_properties.is_empty());
        assert!(component.props.is_empty());
    }

    #[test]
    fn test_mock_component_with_html() {
        let component = MockComponent::new("test").with_html("<div>Test</div>");
        assert_eq!(component.html, "<div>Test</div>");
    }

    #[test]
    fn test_mock_component_with_classes() {
        let component = MockComponent::new("test")
            .with_class("bg-blue-500")
            .with_class("text-white");

        assert!(component.classes.contains("bg-blue-500"));
        assert!(component.classes.contains("text-white"));
    }

    #[test]
    fn test_mock_component_with_custom_properties() {
        let component = MockComponent::new("test")
            .with_custom_property("primary-color", "#3b82f6")
            .with_custom_property("spacing", "1rem");

        assert_eq!(
            component.custom_properties.get("primary-color"),
            Some(&"#3b82f6".to_string())
        );
        assert_eq!(
            component.custom_properties.get("spacing"),
            Some(&"1rem".to_string())
        );
    }

    #[test]
    fn test_mock_component_with_props() {
        let component = MockComponent::new("test")
            .with_prop("text", "Hello World")
            .with_prop("count", "42");

        assert_eq!(
            component.props.get("text"),
            Some(&"Hello World".to_string())
        );
        assert_eq!(component.props.get("count"), Some(&"42".to_string()));
    }

    #[test]
    fn test_mock_component_to_html() {
        let component = MockComponent::new("test")
            .with_html("<button {{classes}} {{style}}>{{text}}</button>")
            .with_class("bg-blue-500")
            .with_class("text-white")
            .with_custom_property("primary-color", "#3b82f6")
            .with_prop("text", "Click me");

        let html = component.to_html();
        assert!(html.contains("class=\"bg-blue-500 text-white\""));
        assert!(html.contains("style=\"--primary-color: #3b82f6\""));
        assert!(html.contains("Click me"));
    }

    #[test]
    fn test_mock_button_component() {
        let button = create_mock_button();
        let html = button.to_html();

        assert!(html.contains("<button"));
        assert!(html.contains("bg-blue-500"));
        assert!(html.contains("text-white"));
        assert!(html.contains("Click me"));
    }

    #[test]
    fn test_mock_card_component() {
        let card = create_mock_card();
        let html = card.to_html();

        assert!(html.contains("<div"));
        assert!(html.contains("bg-white"));
        assert!(html.contains("shadow-lg"));
        assert!(html.contains("Card content"));
    }

    #[test]
    fn test_mock_input_component() {
        let input = create_mock_input();
        let html = input.to_html();

        assert!(html.contains("<input"));
        assert!(html.contains("w-full"));
        assert!(html.contains("border"));
        assert!(html.contains("Enter text"));
    }

    #[test]
    fn test_mock_responsive_grid_component() {
        let grid = create_mock_responsive_grid();
        let html = grid.to_html();

        assert!(html.contains("<div"));
        assert!(html.contains("grid"));
        assert!(html.contains("sm:grid-cols-1"));
        assert!(html.contains("md:grid-cols-2"));
        assert!(html.contains("lg:grid-cols-3"));
    }

    #[test]
    fn test_mock_themed_component() {
        let themed = create_mock_themed_component();
        let html = themed.to_html();

        assert!(html.contains("<div"));
        assert!(html.contains("p-4"));
        assert!(html.contains("rounded-lg"));
        assert!(html.contains("--primary-color: #3b82f6"));
        assert!(html.contains("--spacing: 1rem"));
    }

    #[test]
    fn test_create_mock_components() {
        let components = create_mock_components();
        assert_eq!(components.len(), 5);

        let names: Vec<String> = components.iter().map(|c| c.name.clone()).collect();
        assert!(names.contains(&"button".to_string()));
        assert!(names.contains(&"card".to_string()));
        assert!(names.contains(&"input".to_string()));
        assert!(names.contains(&"responsive_grid".to_string()));
        assert!(names.contains(&"themed_component".to_string()));
    }
}
