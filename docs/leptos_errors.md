# Leptos Errors Documentation

This document tracks common Leptos errors encountered in our project and how to fix them.

## Type Compatibility Errors

### 1. `if` and `else` have incompatible types

**Error:**

```
`if` and `else` have incompatible types
expected struct `View<HtmlElement<Div, (Class<Static<"cors-settings">>,), (...,)>>`
found unit type `()`
```

**Solution:**
This occurs because Leptos expects the same return type from both branches of an `if/else` expression. The statically-typed view tree requires consistent types. There are three ways to fix this:

1. **Use the `<Show>` component (recommended)**:
   This is the most idiomatic way to handle conditional rendering in Leptos:

   ```rust
   <Show
     when=move || condition()
     fallback=|| view! { <></> } // or any alternative content
   >
     <div class="some-class">...</div>
   </Show>
   ```

2. **Return the same type from both branches**:

   ```rust
   if condition {
       view! { <div class="some-class">...</div> }
   } else {
       // Return empty div with same class to match type
       view! { <div class="some-class" style="display:none;"></div> }
   }
   ```

3. **Use `.into_any()` to convert to a common type**:
   ```rust
   if condition {
       view! { <div>...</div> }.into_any()
   } else {
       view! { <span>...</span> }.into_any()
   }
   ```

The `<Show>` component is designed specifically for conditional rendering and avoids many typing issues. It also optimizes re-rendering by only rendering each branch once when the condition changes.

Reference: [Leptos Book - Control Flow](https://book.leptos.dev/view/06_control_flow.html)

### 2. Incompatible opaque types with `impl Trait`

**Error:**

```
`if` and `else` have incompatible types
expected opaque type `impl leptos::IntoView`
found opaque type `impl leptos::IntoView`
distinct uses of `impl Trait` result in different opaque types
```

**Solution:**
When functions return `impl Trait`, each instance is a unique opaque type. Convert them to a common type with `.into_any()` or `.into_view()`:

```rust
if condition {
    first_function().into_any()
} else {
    second_function().into_any()
}
```

## Stylance CSS Module Errors

### 1. Failed to resolve import_style_classes_rel

**Error:**

```
error[E0433]: failed to resolve: could not find `import_style_classes_rel` in `internal`
  --> src/components/dream_interpreter.rs:11:1
   |
11 | import_style!(style, "components/dream_interpreter.module.css");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `import_style_classes_rel` in `internal`
```

**Solution:**
This error occurs when Stylance can't find or generate the CSS classes. The most common causes and solutions are:

1. **Immediate fix - Comment out Stylance temporarily**:

   ```rust
   // Temporarily disable Stylance
   //use stylance::import_style;
   //import_style!(style, "components/dream_interpreter.module.css");

   // Use regular CSS classes instead
   <div class="dream-interpreter"> instead of <div class=style::dream_interpreter>
   ```

2. **Complete Setup Requirements**:

   a. **Install Stylance CLI**:

   ```bash
   cargo install stylance-cli
   ```

   b. **Generate CSS files first**:

   ```bash
   mkdir -p dist && stylance . --output-file dist/styles.css
   ```

   c. **Update `Trunk.toml` with pre-build hook**:

   ```toml
   [[hooks]]
   stage = "pre_build"
   command = "stylance"
   command_arguments = [".", "--output-file", "dist/styles.css"]
   ```

   d. **Verify CSS path in import statement**:
   The path should be relative to your workspace root, not the file importing it:

   ```rust
   // Correct - full path from project root
   import_style!(style, "src/components/dream_interpreter.module.css");

   // Incorrect - relative path that might not resolve
   import_style!(style, "components/dream_interpreter.module.css");
   import_style!(style, "./dream_interpreter.module.css");
   ```

   e. **Check CSS module filename**:
   Ensure the CSS file has `.module.css` extension and exists at the specified path.

3. **Dependencies version mismatch**:
   Make sure your Cargo.toml has the correct dependencies:

   ```toml
   [dependencies]
   stylance = "0.4" # Or the version you're using

   [build-dependencies]
   stylance-core = "0.4" # Same version as stylance
   ```

4. **Project structure issues**:
   In some cases, the workspace or project structure might not match what Stylance expects:

   - Try placing your CSS module in the same directory as the Rust file
   - Make sure your CSS module is included in the build process

5. **Reset and rebuild**:
   Sometimes a clean build can resolve the issue:
   ```bash
   cargo clean
   rm -rf dist/styles.css
   stylance . --output-file dist/styles.css
   trunk serve
   ```

If all else fails, the safest approach is to use regular CSS classes until you can properly set up Stylance integration.
