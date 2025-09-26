//! Comprehensive Leptos Integration Example
//! 
//! This example demonstrates the full capabilities of Tailwind-RS with Leptos,
//! including reactive components, state management, and advanced styling.

use leptos::prelude::*;
use tailwind_rs_leptos::*;

/// Main App Component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (theme, set_theme) = create_signal("light");
    
    // Reactive class building
    let container_classes = move || {
        ClassBuilder::new()
            .class("min-h-screen")
            .class("bg-gradient-to-br")
            .class("from-blue-50")
            .class("to-indigo-100")
            .class("dark:from-gray-900")
            .class("dark:to-gray-800")
            .class("transition-colors")
            .class("duration-300")
            .build()
    };
    
    let card_classes = move || {
        ClassBuilder::new()
            .class("bg-white")
            .class("dark:bg-gray-800")
            .class("rounded-xl")
            .class("shadow-lg")
            .class("p-8")
            .class("backdrop-blur-sm")
            .class("border")
            .class("border-white/20")
            .class("transition-all")
            .class("duration-300")
            .class("hover:shadow-xl")
            .class("hover:scale-105")
            .build()
    };
    
    let button_classes = move || {
        ClassBuilder::new()
            .class("px-6")
            .class("py-3")
            .class("bg-blue-600")
            .class("hover:bg-blue-700")
            .class("active:bg-blue-800")
            .class("text-white")
            .class("font-semibold")
            .class("rounded-lg")
            .class("transition-colors")
            .class("duration-200")
            .class("transform")
            .class("hover:scale-105")
            .class("active:scale-95")
            .class("shadow-md")
            .class("hover:shadow-lg")
            .build()
    };
    
    let counter_classes = move || {
        ClassBuilder::new()
            .class("text-6xl")
            .class("font-bold")
            .class("text-gray-800")
            .class("dark:text-white")
            .class("transition-colors")
            .class("duration-300")
            .build()
    };
    
    let theme_button_classes = move || {
        ClassBuilder::new()
            .class("px-4")
            .class("py-2")
            .class("bg-gray-200")
            .class("dark:bg-gray-700")
            .class("hover:bg-gray-300")
            .class("dark:hover:bg-gray-600")
            .class("text-gray-800")
            .class("dark:text-white")
            .class("rounded-md")
            .class("transition-colors")
            .class("duration-200")
            .class("text-sm")
            .class("font-medium")
            .build()
    };

    view! {
        <div class=container_classes()>
            <div class="container mx-auto px-4 py-8">
                <div class="max-w-2xl mx-auto">
                    <div class=card_classes()>
                        <div class="text-center space-y-6">
                            <h1 class="text-4xl font-bold text-gray-800 dark:text-white mb-2">
                                "Tailwind-RS + Leptos"
                            </h1>
                            <p class="text-lg text-gray-600 dark:text-gray-300">
                                "A comprehensive example showcasing reactive styling"
                            </p>
                            
                            <div class="space-y-4">
                                <div class="flex items-center justify-center space-x-4">
                                    <button 
                                        class=button_classes()
                                        on:click=move |_| set_count.update(|c| *c += 1)
                                    >
                                        "Increment"
                                    </button>
                                    <button 
                                        class=button_classes()
                                        on:click=move |_| set_count.update(|c| *c -= 1)
                                    >
                                        "Decrement"
                                    </button>
                                </div>
                                
                                <div class="p-6 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                    <div class=counter_classes()>
                                        {count}
                                    </div>
                                </div>
                                
                                <div class="flex items-center justify-center space-x-4">
                                    <button 
                                        class=theme_button_classes()
                                        on:click=move |_| set_theme.set("light")
                                    >
                                        "Light Theme"
                                    </button>
                                    <button 
                                        class=theme_button_classes()
                                        on:click=move |_| set_theme.set("dark")
                                    >
                                        "Dark Theme"
                                    </button>
                                </div>
                            </div>
                            
                            <div class="mt-8 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                                <h3 class="text-lg font-semibold text-blue-800 dark:text-blue-200 mb-2">
                                    "Features Demonstrated"
                                </h3>
                                <ul class="text-sm text-blue-700 dark:text-blue-300 space-y-1">
                                    <li>"• Reactive class building with signals"</li>
                                    <li>"• Dark mode support"</li>
                                    <li>"• Hover and active states"</li>
                                    <li>"• Transform animations"</li>
                                    <li>"• Backdrop blur effects"</li>
                                    <li>"• Gradient backgrounds"</li>
                                    <li>"• Responsive design"</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Interactive Button Component
#[component]
fn InteractiveButton(
    children: Children,
    #[prop(optional)] variant: Option<String>,
    #[prop(optional)] size: Option<String>,
) -> impl IntoView {
    let base_classes = ClassBuilder::new()
        .class("font-medium")
        .class("rounded-lg")
        .class("transition-all")
        .class("duration-200")
        .class("transform")
        .class("hover:scale-105")
        .class("active:scale-95")
        .class("focus:outline-none")
        .class("focus:ring-2")
        .class("focus:ring-offset-2")
        .build();
    
    let variant_classes = match variant.as_deref() {
        Some("primary") => ClassBuilder::new()
            .class("bg-blue-600")
            .class("hover:bg-blue-700")
            .class("text-white")
            .class("focus:ring-blue-500")
            .build(),
        Some("secondary") => ClassBuilder::new()
            .class("bg-gray-600")
            .class("hover:bg-gray-700")
            .class("text-white")
            .class("focus:ring-gray-500")
            .build(),
        Some("danger") => ClassBuilder::new()
            .class("bg-red-600")
            .class("hover:bg-red-700")
            .class("text-white")
            .class("focus:ring-red-500")
            .build(),
        _ => ClassBuilder::new()
            .class("bg-gray-200")
            .class("hover:bg-gray-300")
            .class("text-gray-800")
            .class("focus:ring-gray-500")
            .build(),
    };
    
    let size_classes = match size.as_deref() {
        Some("sm") => ClassBuilder::new()
            .class("px-3")
            .class("py-1.5")
            .class("text-sm")
            .build(),
        Some("lg") => ClassBuilder::new()
            .class("px-8")
            .class("py-4")
            .class("text-lg")
            .build(),
        _ => ClassBuilder::new()
            .class("px-6")
            .class("py-3")
            .class("text-base")
            .build(),
    };
    
    let final_classes = ClassBuilder::new()
        .class(base_classes.to_string())
        .class(variant_classes.to_string())
        .class(size_classes.to_string())
        .build();

    view! {
        <button class=final_classes>
            {children()}
        </button>
    }
}

/// Card Component with Advanced Styling
#[component]
fn StyledCard(
    title: String,
    description: String,
    children: Children,
) -> impl IntoView {
    let card_classes = ClassBuilder::new()
        .class("bg-white")
        .class("dark:bg-gray-800")
        .class("rounded-xl")
        .class("shadow-lg")
        .class("p-6")
        .class("border")
        .class("border-gray-200")
        .class("dark:border-gray-700")
        .class("transition-all")
        .class("duration-300")
        .class("hover:shadow-xl")
        .class("hover:scale-105")
        .build();
    
    let title_classes = ClassBuilder::new()
        .class("text-xl")
        .class("font-bold")
        .class("text-gray-800")
        .class("dark:text-white")
        .class("mb-2")
        .build();
    
    let description_classes = ClassBuilder::new()
        .class("text-gray-600")
        .class("dark:text-gray-300")
        .class("mb-4")
        .build();

    view! {
        <div class=card_classes>
            <h3 class=title_classes>{title}</h3>
            <p class=description_classes>{description}</p>
            <div class="space-y-2">
                {children()}
            </div>
        </div>
    }
}

/// Main function to run the Leptos app
fn main() {
    leptos::mount_to_body(App)
}
