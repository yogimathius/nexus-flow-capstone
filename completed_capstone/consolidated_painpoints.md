# Rust WASM Frontend Development Painpoints

This document consolidates the challenges and painpoints encountered during the development of Rust/WASM frontend applications using frameworks like Leptos, based on experiences from multiple projects.

## 1. Development Experience & Build Performance

### Compilation and Build Optimization Issues

- **Slow compile times**: Development iteration cycle much slower than with JavaScript frameworks
- **Build optimization needs**: Workspace optimization in Cargo.toml and dynamic linking for development builds required
- **Large components**: Need to split large components into smaller modules to improve build performance
- **Large build artifacts**: Clean builds removing multi-gigabyte data

## 2. Component Development Challenges

### UI Component Implementation

- **Modal system complexity**: Challenges implementing a modal system in pure Rust without JavaScript dependencies
- **Form handling difficulty**: Creating comprehensive form validation, state management, and submission handling in Rust
- **Animation system limitations**: Implementing animations in Rust/WASM instead of relying on CSS or JavaScript
- **Callback typing issues**: Component callbacks requiring explicit boxing (`Box::new(move || ...)`)
- **Unboxed callback type mismatches**: Errors when passing unboxed closures to components expecting boxed function types

### Type System Friction

- **String type mismatches**: Type errors between `String` and `&str` in component props
- **Verbose error messages**: Trait bound errors with overwhelming complexity making the core problem hard to identify
- **Parameter type exactness**: Component prop types must match exactly with no automatic coercion
- **Cryptic type errors**: Error messages when passing Actions to components expecting Callbacks are difficult to debug
- **Send trait requirements**: `Action::new()` requiring `Send` trait causing issues with reqwest in WASM

### Styling Infrastructure

- **SCSS integration**: Need for custom SCSS build pipeline with Trunk.toml configuration
- **CSS architecture setup**: Creating foundational SCSS architecture with variables, mixins, and component styles
- **CSS module inconsistencies**: Different patterns working in some components but not others
- **Migration complexity**: Migrating between styling approaches introducing unnecessary complexity

## 3. WebAssembly-Specific Challenges

### Integration with Browser APIs

- **WebSocket complexity**: Implementing WebSocket subscriptions in Leptos for real-time updates
- **Native browser APIs**: Accessing and utilizing native browser APIs through WebAssembly boundary
- **JavaScript API safety**: Rust's type system cannot verify JavaScript API signatures in extern blocks
- **Generic JsValue limitations**: Using generic JsValue type loses specific type information

### Performance Optimization

- **Native operation development**: Need to implement CPU-intensive operations as native Rust functions
- **File operations performance**: Creating efficient file handling using Rust
- **Text processing native implementation**: Implementing text search and processing algorithms in Rust
- **Limited tooling**: Lack of tools for monitoring and analyzing WASM application performance

## 4. Data & State Management

### Model Development and Testing

- **Data model complexity**: Creating comprehensive data models for application entities
- **Task validation**: Implementing robust validation for form inputs
- **Test setup complexity**: Feature-gated test dependencies required to avoid build failures
- **Proc macro conflicts**: Conflicts between framework and test macros

### Reactive State Management

- **Excessive cloning**: Need for excessive cloning of shared state due to Rust's ownership rules
- **Verbose boilerplate**: Common patterns requiring verbose boilerplate code
- **Signal conversion errors**: Obscure type conversion errors between closures and signals
- **Testing reactive behavior**: Challenges in setting up tests for reactive components

## 5. AI Integration Challenges

### Context Management and Performance

- **Context management system**: Building a system to manage user context for AI interactions
- **Dynamic prompt generation**: Creating context-aware prompts for AI interactions
- **Response time optimization**: Implementing caching, batch processing, and streaming for LLM responses
- **Smart context caching**: Developing cache management and invalidation for AI context

### Security and Privacy

- **Security framework**: Need for prompt injection protection and response sanitization
- **Privacy controls**: Implementing user data anonymization and privacy protection
- **Memory tracking challenges**: Difficulty implementing memory usage tracking

## 6. Developer Experience Improvements Needed

### Diagnostics and Documentation

- **Improved error messages needed**: Better error messages for component prop type mismatches
- **Missing patterns**: Lack of documentation for common patterns like CSS module usage
- **Code editor integration**: Missing code folding support for HTML in framework macros
- **Routing issues**: Location pathname not updating on route changes

## Best Practices & Mitigations

### Component Design

1. **Extract Business Logic**: Separate pure business logic from UI components for testability
2. **Use Typed Props**: Define component props with explicit types for better error messages
3. **Consistent Error Handling**: Implement proper error states in all components
4. **Small Focused Components**: Break UI into small, focused components that are easier to test

### Type System

1. **Explicit Type Annotations**: Always use explicit type annotations when defining callbacks
2. **Check Component Signatures**: Carefully check component prop signatures in the definitions
3. **Box Callbacks Explicitly**: Box function callbacks with `Box::new(move || ...)`
4. **Match Parameter Types Exactly**: Ensure parameter types match exactly (`String` vs `&str`)

### Testing Strategy

1. **Unit Test Business Logic**: Extract and test business logic separately from UI components
2. **Component Testing**: Test individual components in isolation with controlled inputs
3. **End-to-End Testing**: Use wasm-bindgen-test for integration testing
4. **Testing Utilities**: Create helper functions to reduce test setup boilerplate

## Conclusion

Building frontend applications with Rust, WebAssembly, and frameworks like Leptos presents significant challenges compared to traditional web development frameworks. While the potential benefits include improved performance, type safety, and memory efficiency, the developer experience is still maturing.

These painpoints reflect the current state of Rust frontend development and the trade-offs involved in choosing a more type-safe, performance-oriented approach. As the ecosystem evolves, many of these challenges will likely be addressed through improved tooling, documentation, and framework design.
