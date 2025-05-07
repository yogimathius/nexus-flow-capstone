# Nexus Web Development Friction Points

This document tracks the friction points we've encountered while developing the Nexus Web application using Leptos, Rust, and WebAssembly.

## WebAssembly and JavaScript Interop

1. **Module Resolution Errors**:

   - Error: `Failed to resolve module specifier "env". Relative references must start with either "/", "./", or "../"`
   - Solution: Changed wasm-bindgen target from "web" to "bundler" in Trunk.toml.
   - Root cause: JavaScript module imports in WebAssembly output need proper module resolution.

2. **MIME Type Issues with ES Modules**:
   - Error: `Failed to load module script: Expected a JavaScript module script but the server responded with a MIME type of "text/html"`
   - This happens when the browser tries to load ES modules but the server doesn't serve them with the correct MIME type.

## State Management in Leptos

1. **Signal Effects Not Firing**:

   - When using `Effect::watch` to track signal changes, the effects sometimes don't fire correctly.
   - This causes issues like buttons staying disabled even when they should be enabled.
   - Potential causes: dependency tracking issues or improper signal updates.
   - Solution: Use `Effect::new()` instead of `create_effect()` and ensure proper signal access inside.

2. **Button Disabled State Not Updating**:

   - Issue: Button remains disabled even after text is entered.
   - Root cause: Signal value not being properly propagated to the disabled attribute.
   - Solution:
     - Directly update the button's disabled state in the textarea input handler.
     - Use `.get()` to extract the boolean value from the signal when passing it to components.
     - Set the disabled signal value directly when badges are clicked.

3. **Component Re-rendering Issues**:

   - Components sometimes don't re-render when state changes.
   - Potential fix: Make sure to call `.update()` on signals or use `create_effect`.

4. **Type Mismatches with Signal Values**:

   - Error: `expected bool, found ReadSignal<bool>`
   - Signal values need to be extracted with `.get()` when passed to components expecting the raw value.
   - Components may silently accept signal values in some cases but fail in others, creating confusing behavior.

5. **Borrow Checker Issues with Event Handlers**:
   - Borrowing issues when setting signals in event handlers.
   - Rust's borrow checker makes signal updates in callbacks challenging.
   - Solution: Structure code to avoid using values after they're moved into signals.

## Build System and Tooling

1. **Slow Compilation Times**:

   - Rust WASM compilation can be very slow, especially on first build.
   - This impacts development velocity significantly.

2. **Trunk Version Inconsistencies**:

   - Warning messages about using an older version of Trunk.
   - Potential improvements by upgrading to newer versions.

3. **Port Conflicts with Development Server**:
   - Error: `Address already in use (os error 48)`
   - Need to specify different ports when running multiple servers.

## Browser Compatibility

1. **WebAssembly Support Variations**:
   - Different browsers support different WebAssembly features.
   - Need to test across browsers to ensure compatibility.

## Debugging Challenges

1. **WASM Debugging Complexity**:
   - Difficult to debug WASM code compared to regular JavaScript.
   - Stack traces often unclear or unhelpful.
   - Solution: Add extensive logging throughout the application to track state changes.
   - Example: Added logging in the dream interpreter component to track text input and button state.

---

_This document will be updated as we encounter and resolve additional friction points._
