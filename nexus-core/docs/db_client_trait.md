# Database Layer Dependency Injection

This document proposes a design for implementing dependency injection in the database layer using a trait-based approach similar to what we've done with the AI client.

## Design Goals

1. Abstract database operations behind trait interfaces
2. Enable easy mocking for tests without requiring a real database
3. Improve testability and maintainability
4. Preserve type safety and compile-time checking

## Proposed `DbClientTrait`

```rust
use crate::models::dream::Dream;
use crate::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

/// Trait defining the interface for database operations
#[async_trait]
pub trait DbClientTrait: Send + Sync {
    /// Clone this trait object (following the clone_box pattern)
    fn clone_box(&self) -> Box<dyn DbClientTrait>;

    /// Create a new dream in the database
    async fn create_dream(&self, text: &str, interpretation: &str) -> AppResult<Uuid>;

    /// Get a dream by its ID
    async fn get_dream(&self, id: Uuid) -> AppResult<Option<Dream>>;

    /// List all dreams in the database
    async fn list_dreams(&self) -> AppResult<Vec<Dream>>;

    /// Update a dream's interpretation
    async fn update_dream_interpretation(&self, id: Uuid, interpretation: &str) -> AppResult<bool>;

    /// Delete a dream
    async fn delete_dream(&self, id: Uuid) -> AppResult<bool>;
}

/// Implement Clone for Box<dyn DbClientTrait>
impl Clone for Box<dyn DbClientTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
```

## Real Implementation: `SqlDbClient`

```rust
use sqlx::PgPool;

/// Real database client implementation using SQLx and PostgreSQL
pub struct SqlDbClient {
    pool: PgPool,
}

impl SqlDbClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DbClientTrait for SqlDbClient {
    fn clone_box(&self) -> Box<dyn DbClientTrait> {
        Box::new(Self {
            pool: self.pool.clone(),
        })
    }

    async fn create_dream(&self, text: &str, interpretation: &str) -> AppResult<Uuid> {
        // Implementation that uses self.pool to execute SQL
        // This would move code from Dream::create to here
    }

    // Implement other methods...
}
```

## Mock Implementation: `MockDbClient`

```rust
/// Mock database client for testing
pub struct MockDbClient {
    dreams: Arc<Mutex<HashMap<Uuid, Dream>>>,
    should_fail: bool,
}

impl MockDbClient {
    pub fn new(should_fail: bool) -> Self {
        Self {
            dreams: Arc::new(Mutex::new(HashMap::new())),
            should_fail,
        }
    }

    pub fn with_dreams(mut self, dreams: Vec<Dream>) -> Self {
        let mut map = HashMap::new();
        for dream in dreams {
            map.insert(dream.id, dream);
        }
        self.dreams = Arc::new(Mutex::new(map));
        self
    }
}

#[async_trait]
impl DbClientTrait for MockDbClient {
    fn clone_box(&self) -> Box<dyn DbClientTrait> {
        Box::new(Self {
            dreams: self.dreams.clone(),
            should_fail: self.should_fail,
        })
    }

    async fn create_dream(&self, text: &str, interpretation: &str) -> AppResult<Uuid> {
        if self.should_fail {
            return Err(AppError::DatabaseError(sqlx::Error::PoolClosed));
        }

        let id = Uuid::new_v4();
        let dream = Dream {
            id,
            text: text.to_string(),
            interpretation: interpretation.to_string(),
            created_at: chrono::Utc::now(),
        };

        self.dreams.lock().unwrap().insert(id, dream);
        Ok(id)
    }

    // Implement other methods...
}
```

## Integration in AppState

```rust
pub struct AppState {
    pub db_client: Arc<Box<dyn DbClientTrait>>,
    pub ai_client: Arc<Box<dyn AiClientTrait>>,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        api_key: String,
        github_token: Option<String>,
    ) -> Self {
        let db_client = Arc::new(Box::new(SqlDbClient::new(db_pool)) as Box<dyn DbClientTrait>);
        let ai_client = Arc::new(Box::new(AiClient::new(api_key, github_token)) as Box<dyn AiClientTrait>);

        AppState { db_client, ai_client }
    }
}
```

## Testing with Mocks

```rust
#[tokio::test]
async fn test_dream_creation() {
    // Create mock DB client
    let db_client = MockDbClient::new(false);

    // Create app state with mocks
    let app_state = AppState {
        db_client: Arc::new(Box::new(db_client) as Box<dyn DbClientTrait>),
        ai_client: Arc::new(Box::new(MockAiClient::new(true, "Test")) as Box<dyn AiClientTrait>),
    };

    // Test API handler
    let request = DreamRequest {
        dream_text: "Test dream".to_string(),
        save: true,
    };

    let response = interpret_dream_handler(
        State(app_state),
        Json(request),
    ).await;

    // Assert response is success
    // ...
}
```

## Benefits

1. **Testability**: Tests can run without a real database
2. **Isolation**: Components can be tested independently
3. **Performance**: Tests run faster without database setup
4. **Flexibility**: Different database implementations can be swapped easily
