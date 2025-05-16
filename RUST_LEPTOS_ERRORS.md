# Rust Leptos Error Friction Log

This document tracks cryptic Rust/Leptos errors and their solutions to help future development.

## Component Callback Errors

### Error 1: Unboxed Callback Type Mismatch

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

### Error 2: Function Parameter Type Mismatch

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

## Tips for Debugging Leptos Components

1. Carefully check component prop signatures in the definitions
2. Function callbacks often need to be boxed: `Box::new(move || ...)`
3. Parameter types must match exactly (`String` vs `&str`)
4. Use explicit type annotations when defining callbacks
5. When in doubt, refer to component definition to see expected types
