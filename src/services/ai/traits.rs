use crate::error::AppResult;
use std::any::Any;

/// Common interface for all AI providers
///
/// This trait represents any service that can generate text completions
/// from a prompt. Implementations include GitHub Copilot, OpenRouter, etc.
pub trait AiProvider: Send + Sync {
    /// Generate a completion from the AI provider
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt to send to the AI provider
    ///
    /// # Returns
    ///
    /// A Future containing either:
    /// * The completion text as a String
    /// * An AppError if the request fails
    fn generate_completion<'a>(
        &'a self,
        prompt: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<String>> + Send + 'a>>;

    /// Clone this provider into a boxed trait object
    ///
    /// This method enables the clone_box pattern for trait objects
    /// since the standard Clone trait is not object-safe.
    fn clone_box(&self) -> Box<dyn AiProvider>;

    /// Get a reference to Self as Any for downcasting
    ///
    /// This allows consumers to downcast to the concrete provider type
    /// when needed for provider-specific operations.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Main interface for AI client services in the application
///
/// This trait defines the high-level API for interacting with AI services.
/// It abstracts over the implementation details of specific AI providers
/// and offers application-specific operations like dream interpretation.
pub trait AiClientTrait: Send + Sync {
    /// Clone this client into a boxed trait object
    ///
    /// This method enables the clone_box pattern for trait objects
    /// since the standard Clone trait is not object-safe.
    fn clone_box(&self) -> Box<dyn AiClientTrait>;

    /// Request a dream interpretation from an AI service
    ///
    /// # Arguments
    ///
    /// * `dream_text` - The text of the dream to be interpreted
    ///
    /// # Returns
    ///
    /// A Future containing either:
    /// * The dream interpretation as a String
    /// * An AppError if the interpretation fails
    fn request_dream_interpretation<'a>(
        &'a self,
        dream_text: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<String>> + Send + 'a>>;

    /// Check if this client has an MCP client connected
    fn has_mcp_client(&self) -> bool;

    /// Get mutable access to this object as Any
    ///
    /// This allows for downcasting to concrete types when needed
    /// for operations that require access to the concrete type.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Enable cloning of AiProvider trait objects
impl Clone for Box<dyn AiProvider> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Enable cloning of AiClientTrait trait objects
impl Clone for Box<dyn AiClientTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
