# Todo App Example

This guide demonstrates a complete todo application built with `tailwind-rs`, showcasing real-world usage patterns, state management, and comprehensive testing.

## Project Structure

```
todo-app/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── todo_item.rs
│   │   ├── todo_list.rs
│   │   ├── todo_form.rs
│   │   └── todo_filters.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── todo.rs
│   └── utils/
│       ├── mod.rs
│       └── storage.rs
├── tests/
│   ├── integration_tests.rs
│   └── e2e_tests.spec.ts
├── build.rs
├── tailwind.toml
└── Cargo.toml
```

## Dependencies

**Cargo.toml**
```toml
[package]
name = "todo-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.6"
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4"] }

[build-dependencies]
tailwind-rs-cli = "0.1.0"

[dev-dependencies]
tailwind-rs-testing = "0.1.0"
leptos-testing = "0.6"
wasm-bindgen-test = "0.3"
```

## Data Models

**src/models/todo.rs**
```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            completed: false,
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn matches(&self, todo: &Todo) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        }
    }
}
```

## Todo Item Component

**src/components/todo_item.rs**
```rust
use leptos::*;
use tailwind_rs::classes;
use crate::models::todo::Todo;

#[component]
pub fn TodoItem(
    todo: Todo,
    on_toggle: impl Fn() + 'static,
    on_delete: impl Fn() + 'static,
    on_edit: impl Fn(String) + 'static,
) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_text, set_edit_text) = create_signal(todo.title.clone());
    
    let item_classes = classes!(
        "flex",
        "items-center",
        "space-x-3",
        "p-4",
        "bg-white",
        "border-b",
        "border-gray-200",
        "hover:bg-gray-50",
        "transition-colors",
        "duration-200"
    );
    
    let checkbox_classes = classes!(
        "w-5",
        "h-5",
        "text-blue-600",
        "bg-gray-100",
        "border-gray-300",
        "rounded",
        "focus:ring-blue-500",
        "focus:ring-2"
    );
    
    let title_classes = classes!(
        "flex-1",
        "text-gray-900",
        "text-sm",
        "font-medium"
    );
    
    let completed_title_classes = classes!(
        "flex-1",
        "text-gray-500",
        "text-sm",
        "font-medium",
        "line-through"
    );
    
    let button_classes = classes!(
        "text-gray-400",
        "hover:text-gray-600",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "rounded",
        "p-1"
    );
    
    let edit_input_classes = classes!(
        "flex-1",
        "px-2",
        "py-1",
        "text-sm",
        "border",
        "border-gray-300",
        "rounded",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:border-transparent"
    );
    
    let handle_edit = move || {
        if !edit_text().is_empty() {
            on_edit(edit_text());
            set_is_editing(false);
        }
    };
    
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            handle_edit();
        } else if ev.key() == "Escape" {
            set_edit_text(todo.title.clone());
            set_is_editing(false);
        }
    };
    
    view! {
        <div class=item_classes>
            <input
                type="checkbox"
                class=checkbox_classes
                checked=todo.completed
                on:change=move |_| on_toggle()
            />
            
            {if is_editing() {
                view! {
                    <input
                        type="text"
                        class=edit_input_classes
                        value=edit_text
                        on:input=move |ev| set_edit_text(event_target_value(&ev))
                        on:keydown=handle_keydown
                        on:blur=move |_| handle_edit()
                        autofocus=true
                    />
                }
            } else {
                view! {
                    <span 
                        class=if todo.completed { completed_title_classes } else { title_classes }
                        on:dblclick=move |_| set_is_editing(true)
                    >
                        {todo.title.clone()}
                    </span>
                }
            }}
            
            <div class="flex space-x-2">
                <button
                    class=button_classes
                    on:click=move |_| set_is_editing(true)
                >
                    "✏️"
                </button>
                <button
                    class=button_classes
                    on:click=move |_| on_delete()
                >
                    "🗑️"
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;
    use crate::models::todo::Todo;

    #[test]
    fn test_todo_item_renders_with_correct_classes() {
        let todo = Todo::new("Test todo".to_string());
        let item = TodoItem(
            todo,
            || {},
            || {},
            |_| {}
        );
        
        let classes = extract_classes(item);
        assert!(classes.contains("flex"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("space-x-3"));
        assert!(classes.contains("p-4"));
        assert!(classes.contains("bg-white"));
    }
    
    #[test]
    fn test_todo_item_shows_title() {
        let todo = Todo::new("Test todo".to_string());
        let item = TodoItem(
            todo,
            || {},
            || {},
            |_| {}
        );
        
        let html = render_to_string(item);
        assert!(html.contains("Test todo"));
    }
}
```

## Todo List Component

**src/components/todo_list.rs**
```rust
use leptos::*;
use tailwind_rs::classes;
use crate::models::todo::{Todo, Filter};
use crate::components::todo_item::TodoItem;

#[component]
pub fn TodoList(
    todos: ReadSignal<Vec<Todo>>,
    filter: ReadSignal<Filter>,
    on_toggle: impl Fn(Uuid) + 'static,
    on_delete: impl Fn(Uuid) + 'static,
    on_edit: impl Fn(Uuid, String) + 'static,
) -> impl IntoView {
    let filtered_todos = move || {
        todos()
            .into_iter()
            .filter(|todo| filter().matches(todo))
            .collect::<Vec<_>>()
    };
    
    let list_classes = classes!(
        "bg-white",
        "rounded-lg",
        "shadow-sm",
        "overflow-hidden"
    );
    
    let empty_classes = classes!(
        "p-8",
        "text-center",
        "text-gray-500",
        "text-sm"
    );
    
    view! {
        <div class=list_classes>
            {if filtered_todos().is_empty() {
                view! {
                    <div class=empty_classes>
                        {match filter() {
                            Filter::All => "No todos yet. Add one above!",
                            Filter::Active => "No active todos. Great job!",
                            Filter::Completed => "No completed todos yet.",
                        }}
                    </div>
                }
            } else {
                view! {
                    <div class="divide-y divide-gray-200">
                        {for filtered_todos().into_iter().map(|todo| {
                            let todo_id = todo.id;
                            view! {
                                <TodoItem
                                    todo=todo
                                    on_toggle=move || on_toggle(todo_id)
                                    on_delete=move || on_delete(todo_id)
                                    on_edit=move |title| on_edit(todo_id, title)
                                />
                            }
                        })}
                    </div>
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;
    use crate::models::todo::{Todo, Filter};
    use uuid::Uuid;

    #[test]
    fn test_todo_list_renders_with_correct_classes() {
        let todos = create_signal(vec![Todo::new("Test todo".to_string())]);
        let filter = create_signal(Filter::All);
        
        let list = TodoList(
            todos.0,
            filter.0,
            |_| {},
            |_| {},
            |_, _| {}
        );
        
        let classes = extract_classes(list);
        assert!(classes.contains("bg-white"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("shadow-sm"));
        assert!(classes.contains("overflow-hidden"));
    }
    
    #[test]
    fn test_todo_list_shows_empty_message() {
        let todos = create_signal(vec![]);
        let filter = create_signal(Filter::All);
        
        let list = TodoList(
            todos.0,
            filter.0,
            |_| {},
            |_| {},
            |_, _| {}
        );
        
        let html = render_to_string(list);
        assert!(html.contains("No todos yet. Add one above!"));
    }
}
```

## Todo Form Component

**src/components/todo_form.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn TodoForm(on_submit: impl Fn(String) + 'static) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(String::new());
    
    let form_classes = classes!(
        "flex",
        "space-x-3",
        "mb-6"
    );
    
    let input_classes = classes!(
        "flex-1",
        "px-4",
        "py-2",
        "border",
        "border-gray-300",
        "rounded-lg",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:border-transparent",
        "text-sm",
        "placeholder-gray-500"
    );
    
    let button_classes = classes!(
        "px-6",
        "py-2",
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-semibold",
        "rounded-lg",
        "transition-colors",
        "duration-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:ring-offset-2",
        "disabled:bg-gray-400",
        "disabled:cursor-not-allowed"
    );
    
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let value = input_value().trim().to_string();
        if !value.is_empty() {
            on_submit(value);
            set_input_value(String::new());
        }
    };
    
    view! {
        <form class=form_classes on:submit=handle_submit>
            <input
                type="text"
                class=input_classes
                placeholder="What needs to be done?"
                value=input_value
                on:input=move |ev| set_input_value(event_target_value(&ev))
            />
            <button
                type="submit"
                class=button_classes
                disabled=move || input_value().trim().is_empty()
            >
                "Add Todo"
            </button>
        </form>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_todo_form_renders_with_correct_classes() {
        let form = TodoForm(|_| {});
        let classes = extract_classes(form);
        
        assert!(classes.contains("flex"));
        assert!(classes.contains("space-x-3"));
        assert!(classes.contains("mb-6"));
    }
    
    #[test]
    fn test_todo_form_has_correct_input_placeholder() {
        let form = TodoForm(|_| {});
        let html = render_to_string(form);
        
        assert!(html.contains("placeholder=\"What needs to be done?\""));
    }
}
```

## Todo Filters Component

**src/components/todo_filters.rs**
```rust
use leptos::*;
use tailwind_rs::classes;
use crate::models::todo::{Filter, Todo};

#[component]
pub fn TodoFilters(
    filter: ReadSignal<Filter>,
    todos: ReadSignal<Vec<Todo>>,
    on_filter_change: impl Fn(Filter) + 'static,
    on_clear_completed: impl Fn() + 'static,
) -> impl IntoView {
    let active_count = move || {
        todos()
            .into_iter()
            .filter(|todo| !todo.completed)
            .count()
    };
    
    let completed_count = move || {
        todos()
            .into_iter()
            .filter(|todo| todo.completed)
            .count()
    };
    
    let container_classes = classes!(
        "flex",
        "items-center",
        "justify-between",
        "p-4",
        "bg-white",
        "border-t",
        "border-gray-200",
        "text-sm",
        "text-gray-500"
    );
    
    let count_classes = classes!(
        "text-sm",
        "text-gray-500"
    );
    
    let filter_classes = classes!(
        "flex",
        "space-x-1"
    );
    
    let filter_button_classes = |is_active: bool| {
        classes!(
            "px-3",
            "py-1",
            "text-sm",
            "font-medium",
            "rounded",
            "transition-colors",
            "duration-200",
            "focus:outline-none",
            "focus:ring-2",
            "focus:ring-blue-500",
            if is_active {
                "bg-blue-500 text-white"
            } else {
                "text-gray-500 hover:text-gray-700 hover:bg-gray-100"
            }
        )
    };
    
    let clear_button_classes = classes!(
        "text-sm",
        "text-gray-500",
        "hover:text-gray-700",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "rounded",
        "px-2",
        "py-1"
    );
    
    view! {
        <div class=container_classes>
            <span class=count_classes>
                {active_count()} " active"
            </span>
            
            <div class=filter_classes>
                <button
                    class=filter_button_classes(filter() == Filter::All)
                    on:click=move |_| on_filter_change(Filter::All)
                >
                    "All"
                </button>
                <button
                    class=filter_button_classes(filter() == Filter::Active)
                    on:click=move |_| on_filter_change(Filter::Active)
                >
                    "Active"
                </button>
                <button
                    class=filter_button_classes(filter() == Filter::Completed)
                    on:click=move |_| on_filter_change(Filter::Completed)
                >
                    "Completed"
                </button>
            </div>
            
            {if completed_count() > 0 {
                view! {
                    <button
                        class=clear_button_classes
                        on:click=move |_| on_clear_completed()
                    >
                        "Clear completed"
                    </button>
                }
            } else {
                view! {
                    <span></span>
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;
    use crate::models::todo::{Filter, Todo};

    #[test]
    fn test_todo_filters_renders_with_correct_classes() {
        let filter = create_signal(Filter::All);
        let todos = create_signal(vec![Todo::new("Test todo".to_string())]);
        
        let filters = TodoFilters(
            filter.0,
            todos.0,
            |_| {},
            || {}
        );
        
        let classes = extract_classes(filters);
        assert!(classes.contains("flex"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("justify-between"));
        assert!(classes.contains("p-4"));
        assert!(classes.contains("bg-white"));
    }
    
    #[test]
    fn test_todo_filters_shows_active_count() {
        let filter = create_signal(Filter::All);
        let todos = create_signal(vec![
            Todo::new("Active todo".to_string()),
            {
                let mut completed = Todo::new("Completed todo".to_string());
                completed.completed = true;
                completed
            }
        ]);
        
        let filters = TodoFilters(
            filter.0,
            todos.0,
            |_| {},
            || {}
        );
        
        let html = render_to_string(filters);
        assert!(html.contains("1 active"));
    }
}
```

## Main Application

**src/main.rs**
```rust
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeFile;
use uuid::Uuid;

mod components;
mod models;
mod utils;

use components::{TodoForm, TodoList, TodoFilters};
use models::todo::{Todo, Filter};

#[component]
pub fn App() -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<Todo>>(vec![]);
    let (filter, set_filter) = create_signal(Filter::All);
    
    let add_todo = move |title: String| {
        set_todos.update(|todos| {
            todos.push(Todo::new(title));
        });
    };
    
    let toggle_todo = move |id: Uuid| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.toggle();
            }
        });
    };
    
    let delete_todo = move |id: Uuid| {
        set_todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
    };
    
    let edit_todo = move |id: Uuid, title: String| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.title = title;
            }
        });
    };
    
    let clear_completed = move || {
        set_todos.update(|todos| {
            todos.retain(|t| !t.completed);
        });
    };
    
    view! {
        <div class="min-h-screen bg-gray-100">
            <div class="container mx-auto px-4 py-8 max-w-2xl">
                <div class="bg-white rounded-lg shadow-lg overflow-hidden">
                    <div class="p-6 border-b border-gray-200">
                        <h1 class="text-3xl font-bold text-center text-gray-800">
                            "Todo App"
                        </h1>
                        <p class="text-center text-gray-600 mt-2">
                            "Built with tailwind-rs"
                        </p>
                    </div>
                    
                    <div class="p-6">
                        <TodoForm on_submit=add_todo />
                        <TodoList
                            todos=todos
                            filter=filter
                            on_toggle=toggle_todo
                            on_delete=delete_todo
                            on_edit=edit_todo
                        />
                    </div>
                    
                    <TodoFilters
                        filter=filter
                        todos=todos
                        on_filter_change=move |f| set_filter(f)
                        on_clear_completed=clear_completed
                    />
                </div>
            </div>
        </div>
    }
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(ServeFile::new("dist/styles.css"))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```

## Integration Tests

**tests/integration_tests.rs**
```rust
use todo_app::*;
use tailwind_rs::testing::*;
use crate::models::todo::{Todo, Filter};

#[test]
fn test_app_renders_correctly() {
    let app = create_test_app(|| view! { <App /> });
    let html = render_to_string(app);
    
    assert!(html.contains("Todo App"));
    assert!(html.contains("Built with tailwind-rs"));
    assert!(html.contains("What needs to be done?"));
}

#[test]
fn test_todo_form_integration() {
    let form = TodoForm(|_| {});
    let html = render_to_string(form);
    
    assert!(html.contains("placeholder=\"What needs to be done?\""));
    assert!(html.contains("Add Todo"));
}

#[test]
fn test_todo_list_integration() {
    let todos = create_signal(vec![
        Todo::new("Test todo 1".to_string()),
        Todo::new("Test todo 2".to_string()),
    ]);
    let filter = create_signal(Filter::All);
    
    let list = TodoList(
        todos.0,
        filter.0,
        |_| {},
        |_| {},
        |_, _| {}
    );
    
    let html = render_to_string(list);
    assert!(html.contains("Test todo 1"));
    assert!(html.contains("Test todo 2"));
}
```

## Playwright E2E Tests

**tests/e2e_tests.spec.ts**
```typescript
import { test, expect } from '@playwright/test';

test.describe('Todo App', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should render the app correctly', async ({ page }) => {
    await expect(page.locator('h1')).toHaveText('Todo App');
    await expect(page.locator('p')).toHaveText('Built with tailwind-rs');
    await expect(page.locator('input[placeholder="What needs to be done?"]')).toBeVisible();
    await expect(page.locator('button:has-text("Add Todo")')).toBeVisible();
  });

  test('should add a new todo', async ({ page }) => {
    const input = page.locator('input[placeholder="What needs to be done?"]');
    const addButton = page.locator('button:has-text("Add Todo")');
    
    await input.fill('Test todo');
    await addButton.click();
    
    await expect(page.locator('text=Test todo')).toBeVisible();
    await expect(input).toHaveValue('');
  });

  test('should toggle todo completion', async ({ page }) => {
    // Add a todo
    await page.locator('input[placeholder="What needs to be done?"]').fill('Test todo');
    await page.locator('button:has-text("Add Todo")').click();
    
    // Toggle completion
    const checkbox = page.locator('input[type="checkbox"]');
    await checkbox.click();
    
    // Check if todo is marked as completed
    const todoItem = page.locator('text=Test todo');
    await expect(todoItem).toHaveClass(/line-through/);
  });

  test('should delete a todo', async ({ page }) => {
    // Add a todo
    await page.locator('input[placeholder="What needs to be done?"]').fill('Test todo');
    await page.locator('button:has-text("Add Todo")').click();
    
    // Delete the todo
    const deleteButton = page.locator('button:has-text("🗑️")');
    await deleteButton.click();
    
    // Check if todo is removed
    await expect(page.locator('text=Test todo')).not.toBeVisible();
  });

  test('should filter todos', async ({ page }) => {
    // Add multiple todos
    await page.locator('input[placeholder="What needs to be done?"]').fill('Active todo');
    await page.locator('button:has-text("Add Todo")').click();
    
    await page.locator('input[placeholder="What needs to be done?"]').fill('Completed todo');
    await page.locator('button:has-text("Add Todo")').click();
    
    // Complete one todo
    const checkboxes = page.locator('input[type="checkbox"]');
    await checkboxes.nth(1).click();
    
    // Filter by active
    await page.locator('button:has-text("Active")').click();
    await expect(page.locator('text=Active todo')).toBeVisible();
    await expect(page.locator('text=Completed todo')).not.toBeVisible();
    
    // Filter by completed
    await page.locator('button:has-text("Completed")').click();
    await expect(page.locator('text=Active todo')).not.toBeVisible();
    await expect(page.locator('text=Completed todo')).toBeVisible();
  });

  test('should clear completed todos', async ({ page }) => {
    // Add and complete todos
    await page.locator('input[placeholder="What needs to be done?"]').fill('Todo 1');
    await page.locator('button:has-text("Add Todo")').click();
    
    await page.locator('input[placeholder="What needs to be done?"]').fill('Todo 2');
    await page.locator('button:has-text("Add Todo")').click();
    
    // Complete first todo
    const checkboxes = page.locator('input[type="checkbox"]');
    await checkboxes.first().click();
    
    // Clear completed
    await page.locator('button:has-text("Clear completed")').click();
    
    // Check if only active todo remains
    await expect(page.locator('text=Todo 1')).not.toBeVisible();
    await expect(page.locator('text=Todo 2')).toBeVisible();
  });

  test('should edit todo inline', async ({ page }) => {
    // Add a todo
    await page.locator('input[placeholder="What needs to be done?"]').fill('Original todo');
    await page.locator('button:has-text("Add Todo")').click();
    
    // Double-click to edit
    const todoText = page.locator('text=Original todo');
    await todoText.dblclick();
    
    // Edit the text
    const editInput = page.locator('input[type="text"]').last();
    await editInput.fill('Edited todo');
    await editInput.press('Enter');
    
    // Check if todo is updated
    await expect(page.locator('text=Edited todo')).toBeVisible();
    await expect(page.locator('text=Original todo')).not.toBeVisible();
  });
});
```

## Running the Application

### Development Mode
```bash
# Start the development server
cargo run

# In another terminal, start the CSS watcher
cargo run --bin tailwind-watch
```

### Production Build
```bash
# Build the application
cargo build --release

# Build CSS
cargo run --bin tailwind-build
```

### Running Tests
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run Playwright tests
pnpm test:e2e
```

## Key Features Demonstrated

### 1. Component Composition
- Modular component design
- Props and callbacks for communication
- Reusable UI components

### 2. State Management
- Signal-based state management
- Local component state
- Global application state

### 3. Type Safety
- Type-safe class generation
- Compile-time validation
- Strong typing throughout

### 4. Testing Strategy
- Unit tests for individual components
- Integration tests for component interactions
- E2E tests for user workflows

### 5. Responsive Design
- Mobile-first approach
- Flexible layouts
- Touch-friendly interactions

## Next Steps

1. 🎨 Explore [Advanced Examples](./dynamic-styling.md)
2. 🧪 Learn [Testing Patterns](./unit-testing.md)
3. 🎯 Try [Framework-Specific Examples](./leptos-examples.md)
4. 🏗️ Build [More Complex Applications](./dashboard.md)

## Additional Resources

- [Getting Started Guide](../getting-started.md)
- [API Reference](../api/README.md)
- [Testing Guidelines](../testing.md)
- [Architecture Documentation](../architecture.md)

