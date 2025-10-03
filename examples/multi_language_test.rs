use tailwind_rs_core::boundary::TemplateLanguage;
use tailwind_rs_core::multi_language::MultiLanguageRegistry;

fn main() {
    println!("🌍 Testing Multi-Language Template Support");

    let registry = MultiLanguageRegistry::new();

    // Test HTML extraction
    println!("\n📄 HTML Template:");
    let html = r#"<div class="flex items-center px-4 bg-blue-500 text-white"></div>"#;
    let html_classes = registry.extract_classes(html, TemplateLanguage::HTML);
    println!("  Template: {}", html);
    println!("  Classes: {:?}", html_classes);

    // Test Vue.js extraction
    println!("\n🖖 Vue.js Template:");
    let vue = r#"<div class="flex" :class="{'active': isActive, 'hidden': !visible}" @click="handleClick"></div>"#;
    let vue_classes = registry.extract_classes(vue, TemplateLanguage::Vue);
    println!("  Template: {}", vue);
    println!("  Classes: {:?}", vue_classes);

    // Test React/JSX extraction
    println!("\n⚛️  React/JSX Template:");
    let react = r#"<div className={clsx('flex items-center', condition && 'bg-blue-500', 'px-4')}></div>"#;
    let react_classes = registry.extract_classes(react, TemplateLanguage::React);
    println!("  Template: {}", react);
    println!("  Classes: {:?}", react_classes);

    // Test Svelte extraction
    println!("\n🧡 Svelte Template:");
    let svelte = r#"<div class="flex" class:active={isActive} class:hidden={!visible}></div>"#;
    let svelte_classes = registry.extract_classes(svelte, TemplateLanguage::Svelte);
    println!("  Template: {}", svelte);
    println!("  Classes: {:?}", svelte_classes);

    // Test Angular extraction
    println!("\n🅰️  Angular Template:");
    let angular = r#"<div class="flex" [class.active]="isActive" [ngClass]="{'px-4': true, 'bg-blue-500': condition}"></div>"#;
    let angular_classes = registry.extract_classes(angular, TemplateLanguage::Angular);
    println!("  Template: {}", angular);
    println!("  Classes: {:?}", angular_classes);

    // Test Haml extraction
    println!("\n💎 Haml Template:");
    let haml = "%div.flex.items-center.px-4.bg-blue-500";
    let haml_classes = registry.extract_classes(haml, TemplateLanguage::Haml);
    println!("  Template: {}", haml);
    println!("  Classes: {:?}", haml_classes);

    // Test Pug extraction
    println!("\n🐶 Pug Template:");
    let pug = ".flex.items-center.px-4.bg-blue-500";
    let pug_classes = registry.extract_classes(pug, TemplateLanguage::Pug);
    println!("  Template: {}", pug);
    println!("  Classes: {:?}", pug_classes);

    // Test auto-detection
    println!("\n🔍 Auto-Detection Test:");
    let templates = vec![
        (r#"<div class="flex"></div>"#, "HTML"),
        (r#"<div :class="{}"></div>"#, "Vue"),
        (r#"<div className=""></div>"#, "React"),
        (r#"<div class:active=""></div>"#, "Svelte"),
        (r#"<div [class]=""></div>"#, "Angular"),
        (r#"%div.flex"#, "Haml"),
        (r#".flex"#, "Pug"),
    ];

    for (template, expected) in templates {
        let detected = registry.detect_language(template);
        let classes = registry.extract_classes_auto(template);
        println!("  '{}' -> {} ({} classes)", template, expected, classes.len());
    }

    // Test complex multi-language scenario
    println!("\n🚀 Complex Multi-Language Scenario:");
    let complex_templates = vec![
        r#"<div class="flex items-center" :class="{active: isActive}" className={clsx('px-4')}></div>"#,
        r#"%div.flex.items-center{:class => "{active: isActive}", :className => "px-4"}"#,
    ];

    for template in complex_templates {
        let detected = registry.detect_language(template);
        let classes = registry.extract_classes_auto(template);
        println!("  Template: {}", template);
        println!("  Detected: {:?}, Classes: {:?}", detected, classes);
    }

    println!("\n✅ Multi-Language Template Support test completed!");
    println!("🌟 Tailwind-RS now supports parsing classes from:");
    println!("   • HTML (class=\"...\")");
    println!("   • Vue.js (:class, v-bind:class)");
    println!("   • React/JSX (className, clsx, cn)");
    println!("   • Svelte (class:, {{variables}})");
    println!("   • Angular ([class], [ngClass])");
    println!("   • Haml (%tag.class#id)");
    println!("   • Pug (.class#id)");
    println!("   • Auto-detection for mixed templates");
}
