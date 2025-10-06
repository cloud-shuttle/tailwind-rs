//! Init Command Implementation
//!
//! The init command creates the initial project structure and configuration
//! files for a new Tailwind-RS project.

use crate::CliError;
use std::path::PathBuf;
use tokio::fs;
use anyhow::Result;

/// Arguments for the init command
#[derive(Debug)]
pub struct InitArgs {
    /// Configuration file path to create
    pub config: Option<PathBuf>,
    /// Whether to create TypeScript config
    pub typescript: bool,
    /// Whether to force overwrite existing files
    pub force: bool,
}

/// Execute the init command
pub async fn execute(args: InitArgs) -> Result<(), CliError> {
    println!("🚀 Initializing Tailwind-RS project...");

    // Determine config file path
    let config_path = args.config.unwrap_or_else(|| {
        if args.typescript {
            PathBuf::from("tailwind.config.ts")
        } else {
            PathBuf::from("tailwind.config.js")
        }
    });

    // Check if config file already exists
    if config_path.exists() && !args.force {
        return Err(CliError::Init(InitError::FileExists(config_path)));
    }

    // Create config file
    create_config_file(&config_path, args.typescript).await?;

    // Create input CSS file
    create_input_css_file().await?;

    // Create basic project structure
    create_project_structure().await?;

    println!("✅ Tailwind-RS project initialized!");
    println!("📄 Config: {}", config_path.display());
    println!("📄 CSS: tailwind.css");
    println!("📁 Dist: dist/");

    println!("\n🚀 Next steps:");
    println!("  1. Add your content files to the 'content' array in {}", config_path.display());
    println!("  2. Import tailwind.css in your HTML/JavaScript");
    println!("  3. Run 'tailwind-rs build' to generate CSS");

    Ok(())
}

/// Create the configuration file
async fn create_config_file(config_path: &PathBuf, typescript: bool) -> Result<(), CliError> {
    let config_content = if typescript {
        r#"/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
    "./public/index.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
"#.to_string()
    } else {
        r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
    "./public/index.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
"#.to_string()
    };

    fs::write(config_path, config_content).await
        .map_err(|e| CliError::Io(e))?;

    Ok(())
}

/// Create the input CSS file with Tailwind directives
async fn create_input_css_file() -> Result<(), CliError> {
    let css_content = r#"/* Tailwind CSS directives */
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom styles can be added here */
"#;

    fs::write("tailwind.css", css_content).await
        .map_err(|e| CliError::Io(e))?;

    Ok(())
}

/// Create basic project structure
async fn create_project_structure() -> Result<(), CliError> {
    // Create dist directory
    fs::create_dir_all("dist").await
        .map_err(|e| CliError::Io(e))?;

    // Create src directory with example files
    fs::create_dir_all("src").await
        .map_err(|e| CliError::Io(e))?;

    // Create example HTML file
    let html_content = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Tailwind-RS Example</title>
  <link href="/dist/output.css" rel="stylesheet">
</head>
<body class="bg-gray-100 text-gray-900">
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold text-center mb-8 text-blue-600">
      Welcome to Tailwind-RS!
    </h1>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div class="bg-white p-6 rounded-lg shadow-md hover:shadow-lg transition-shadow">
        <h2 class="text-xl font-semibold mb-2 text-gray-800">Fast</h2>
        <p class="text-gray-600">
          Built with Rust for maximum performance and reliability.
        </p>
      </div>

      <div class="bg-white p-6 rounded-lg shadow-md hover:shadow-lg transition-shadow">
        <h2 class="text-xl font-semibold mb-2 text-gray-800">Modern</h2>
        <p class="text-gray-600">
          Supports all modern Tailwind CSS features and plugins.
        </p>
      </div>

      <div class="bg-white p-6 rounded-lg shadow-md hover:shadow-lg transition-shadow">
        <h2 class="text-xl font-semibold mb-2 text-gray-800">Developer Experience</h2>
        <p class="text-gray-600">
          Excellent tooling with watch mode and hot reload.
        </p>
      </div>
    </div>

    <div class="text-center mt-8">
      <button class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition-colors">
        Get Started
      </button>
    </div>
  </div>
</body>
</html>
"#;

    fs::write("src/index.html", html_content).await
        .map_err(|e| CliError::Io(e))?;

    Ok(())
}

/// Init-specific error types
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("File already exists: {}", .0.display())]
    FileExists(PathBuf),

    #[error("Failed to create project structure: {0}")]
    ProjectCreation(String),

    #[error("Template error: {0}")]
    Template(String),
}
