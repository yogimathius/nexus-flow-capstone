# Dependency Injection in Nexus Core

This document explains how dependency injection is implemented in the Nexus Core application, focusing on the trait-based approach for service interfaces.

## Overview

Dependency injection allows us to:

1. Decouple components and their dependencies
2. Easily swap implementations for testing
3. Write more testable and maintainable code

## Key Interfaces

### AiClientTrait

The `AiClientTrait` defines the interface for AI services used in our application:

```rust
pub trait AiClientTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn AiClientTrait>;

    fn request_dream_interpretation<'a>(
        &'a self,
        dream_text: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<String>> + Send + 'a>>;

    fn has_mcp_client(&self) -> bool;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

### McpClientTrait

The `McpClientTrait` defines the interface for MCP (Model Context Protocol) services:

```rust
pub trait McpClientTrait: Send + Sync {
    fn clone_box(&self) -> Box<dyn McpClientTrait>;

    // Other methods...
}
```

## Making Trait Objects Cloneable

Since Rust doesn't allow `Clone` to be object safe (can't use with `dyn Trait`), we use the `clone_box` pattern:

```rust
// Enable cloning of trait objects
impl Clone for Box<dyn AiClientTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
```

## Concrete Implementations

- `AiClient`: Real implementation using OpenRouter API
- `MockAiClient`: Test implementation with configurable behavior

## Usage in Application State

The `AppState` struct uses trait objects instead of concrete implementations:

```rust
pub struct AppState {
    pub db_pool: PgPool,
    pub ai_client: Arc<Box<dyn AiClientTrait>>,
}
```

## Using Mock Implementations in Tests

Example of creating a mock AI client for testing:

```rust
// Create a mock AI client
let mock_ai_client = MockAiClient::new(true, "This is a mock interpretation");

// Create app state with the mock
let app_state = AppState {
    db_pool: db_pool,
    ai_client: Arc::new(Box::new(mock_ai_client) as Box<dyn AiClientTrait>),
};
```

## Benefits

1. **Testability**: Services can be easily mocked for testing
2. **Modularity**: Components are decoupled and can be developed independently
3. **Flexibility**: Implementations can be swapped at runtime
4. **Maintainability**: Interfaces are stable while implementations can evolve

## Future Improvements

Consider extending this pattern to other services:

- Database layer
- Configuration providers
- External API clients
