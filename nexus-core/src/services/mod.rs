pub mod ai;
pub mod mcp;

// Re-export the MCP client for backward compatibility
pub use mcp::McpClient;
pub use mcp::McpClientTrait;
pub use mcp::SymbolMeaning;
pub use mcp::TextOperation;
pub use mcp::client as mcp_client;
