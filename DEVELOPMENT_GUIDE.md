# Nexus Web Development Guide

This guide provides comprehensive information for developing the Nexus Web Frontend application, including project roadmap, common errors, and testing guidelines.

## Table of Contents

1. [Project Status & Roadmap](#project-status--roadmap)
2. [Common Errors & Solutions](#common-errors--solutions)
3. [Testing Guidelines](#testing-guidelines)
4. [Best Practices](#best-practices)

## Project Status & Roadmap

### Current Status

- **Status: Basic Functionality Complete** ✅
- Dream submission form and interpretation display are implemented
- UI components (buttons, badges, typography) are in place
- Basic responsive design is implemented
- Error handling for API failures exists

### Development Phases

#### Phase 1: Code Cleanup & Error Handling ✅

- [x] Improve form validation and error messages
- [x] Implement consistent loading states across components
- [x] Ensure proper error handling for all API calls
- [x] Verify ARIA compliance for accessibility

#### Phase 2: Feature Extensions ✅

- [x] Add search capabilities to dream journal
- [x] Implement offline support for dream saving
- [x] Add dream categorization and tagging UI
- [x] Implement sorting and filtering in the dream journal

#### Phase 3: Authentication UI

- [ ] Create login and registration forms
- [ ] Implement token storage and refresh mechanism
- [ ] Add protected routes for authenticated users
- [ ] Create user profile management UI

#### Phase 4: Testing & Quality Assurance

- [ ] Implement component tests for UI elements
- [ ] Add integration tests for form submissions
- [ ] Test error handling and loading states
- [ ] Verify responsive design across device sizes

#### Phase 5: Documentation

- [ ] Create user guide with feature explanations
- [ ] Add troubleshooting section for common issues
- [ ] Document dream interpretation methodology
- [ ] Include privacy and data handling information

#### Phase 6: Advanced Features

- [ ] Create visualization of dream patterns over time
- [ ] Implement UI for sharing dream interpretations
- [ ] Create discussion functionality for shared dreams
- [ ] Implement mobile-specific UX improvements

## Common Errors & Solutions

This section documents common errors encountered during Leptos development and their solutions.

### Component Callback Errors

#### Error 1: Unboxed Callback Type Mismatch

**Error Message:**

```
mismatched types
expected struct `Box<(dyn Fn() + 'static)>`
found closure `{closure@src/components/dream/dream_themes.rs:34:38: 34:46}`
```

**Root Cause:**  
The Badge component expects a `Box<dyn Fn()>` for its `on_click` prop, but we passed an unboxed closure.

**Solution:**

```rust
// Before
on_click=move |_| on_click_callback(theme_for_callback.clone())

// After
on_click=Box::new(move || on_click_callback(theme_for_callback.clone()))
```

**Developer Friction Points:**

- Error points to the line but doesn't indicate boxing is required
- Leptos component props with function types need explicit box wrapping
- Rust's trait object system requires explicit boxing for closures passed as callbacks
- No helpful suggestion in the error message about boxing the closure

#### Error 2: Function Parameter Type Mismatch

**Error Message:**

```
DreamThemesPropsBuilder<{closure@dream_interpreter.rs:47:30}, (...,)>: leptos::prelude::Render
type mismatch in closure arguments
expected closure signature `fn(std::string::String) -> _`
found closure signature `fn(&str) -> _`
```

**Root Cause:**  
The `DreamThemes` component expects a callback that takes a `String`, but our function was defined to take a `&str`.

**Solution:**

```rust
// Before
let handle_badge_click = move |theme: &str| {
    let sentence = get_theme_sentence(theme);
    // ...
}

// After
let handle_badge_click = move |theme: String| {
    let sentence = get_theme_sentence(&theme);
    // ...
}
```

**Developer Friction Points:**

- Error message is overwhelmingly verbose with trait bound information
- Hard to extract the actual problem from complicated trait bound errors
- Multiple potential solutions listed (different traits to implement)
- Error doesn't clearly point to the parameter type as the problem
- Leptos component prop types must match exactly (no automatic coercion from `&str` to `String`)

### Tips for Debugging Leptos Components

1. Carefully check component prop signatures in the definitions
2. Function callbacks often need to be boxed: `Box::new(move || ...)`
3. Parameter types must match exactly (`String` vs `&str`)
4. Use explicit type annotations when defining callbacks
5. When in doubt, refer to component definition to see expected types

## Testing Guidelines

This section outlines the best practices and methods for testing Leptos v0.7 components.

### Testing Approaches

Leptos offers several approaches to testing your components:

1. **Unit Testing Business Logic**: Extract and test business logic separately from UI components
2. **Component Testing**: Testing individual components in isolation
3. **End-to-End Testing**: Testing the full application flow using tools like:
   - wasm-bindgen-test
   - Playwright
   - Gherkin/Cucumber Tests

### Setting Up the Testing Environment

#### 1. Install Required Dependencies

Add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
web-sys = { version = "0.3", features = ["HtmlElement", "Element", "HtmlButtonElement", "MouseEvent", "DomRect"] }
```

#### 2. Configure wasm-bindgen-test

In your test files, configure the test environment:

```rust
use wasm_bindgen_test::*;

// Configure tests to run in browser
wasm_bindgen_test_configure!(run_in_browser);
```

#### 3. Create Test Utilities

Create a setup module with helper functions for testing:

```rust
// tests/setup.rs
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, MouseEvent};

// Mount a component to a container
pub fn mount_to_container<IV: IntoView + 'static>(container: &web_sys::Element, component: IV) {
    let container_html = container.clone().dyn_into::<HtmlElement>().unwrap();
    let _ = leptos::mount_to(container_html, move || component);
}

// Execute a test with a temporary container
pub fn with_test_container<F>(test_fn: F)
where
    F: FnOnce(&Element),
{
    let window = web_sys::window().expect("Window not found");
    let document = window.document().expect("Document not found");
    let body = document.body().expect("Body not found");

    // Create a temporary container for the test
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    // Run the test
    test_fn(&container);

    // Clean up
    body.remove_child(&container).unwrap();
}

// Create a click event
pub fn create_click_event() -> web_sys::Event {
    let document = web_sys::window().unwrap().document().unwrap();
    let event = document.create_event("MouseEvent").unwrap();
    let mouse_event: &MouseEvent = event.unchecked_ref();
    mouse_event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg(
        "click", true, true,
    );
    event
}
```

### Unit Testing Components

#### Testing Business Logic

For components with complex logic, extract that logic into separate functions or structs that can be tested independently:

```rust
// Component with extracted logic
pub struct TodoList(Vec<Todo>);

impl TodoList {
    pub fn new(todos: Vec<Todo>) -> Self {
        Self(todos)
    }

    pub fn remaining_count(&self) -> usize {
        self.0.iter().filter(|todo| !todo.completed).count()
    }
}

#[component]
pub fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(TodoList::new(vec![/* initial todos */]));
    let remaining = move || todos.get().remaining_count();

    view! {
        <div>
            <span>"Remaining: " {remaining}</span>
            // ...rest of component
        </div>
    }
}

// Test the logic separately
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remaining_count() {
        let todos = vec![
            Todo { title: "Task 1".into(), completed: false },
            Todo { title: "Task 2".into(), completed: true },
        ];
        let todo_list = TodoList::new(todos);
        assert_eq!(todo_list.remaining_count(), 1);
    }
}
```

#### Testing Component Output

Test that components render the expected output:

```rust
#[wasm_bindgen_test]
fn test_button_renders_correctly() {
    with_test_container(|container| {
        // Render the component
        let button_view = view! { <Button>"Test Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();
        mount_to_container(&div, button_view);

        // Test the output
        let button = query_selector(container, "button").expect("Button should be rendered");
        assert_eq!(button.text_content().unwrap(), "Test Button");
    });
}
```

### Testing Reactivity

To test reactive behavior:

```rust
#[wasm_bindgen_test]
fn test_toggle_button() {
    with_test_container(|container| {
        // Create a signal
        let (count, set_count) = create_signal(0);

        // Render component with the signal
        mount_to_container(container, view! {
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Count: " {count}
            </button>
        });

        // Get the button
        let button = query_selector(container, "button").unwrap();

        // Test initial state
        assert_eq!(button.text_content().unwrap(), "Count: 0");

        // Simulate click
        button.dispatch_event(&create_click_event()).unwrap();

        // Test updated state
        assert_eq!(button.text_content().unwrap(), "Count: 1");
    });
}
```

## Best Practices

### Component Design

1. **Extract Business Logic**: Separate pure business logic from UI components for testability
2. **Use Typed Props**: Define component props with explicit types for better error messages
3. **Consistent Error Handling**: Implement proper error states in all components
4. **Accessibility First**: Always include ARIA attributes and ensure keyboard navigation

### Styling

1. **CSS Organization**: Keep related styles together and use consistent naming
2. **Mobile-First**: Design for mobile screens first, then enhance for larger screens
3. **Color Variables**: Use CSS variables for theme colors to enable dark mode and easy changes
4. **Animation Performance**: Use transform and opacity for animations when possible

### State Management

1. **Signals for Local State**: Use signals for component-specific state
2. **Resources for API Data**: Use resources for data fetching with proper loading/error states
3. **Prefer Small Components**: Break UI into small, focused components that are easier to test
4. **Memo for Expensive Calculations**: Use memo for derived values that are expensive to compute
