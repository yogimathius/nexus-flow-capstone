# Symbolic Ontology MCP Repository Documentation

The Symbol Ontology MCP repository contains a symbolic reasoning engine built in Rust that serves as an MCP-compliant server for symbolic ontology. This system enables applications, particularly AI tools like Claude, to query and reason about symbols, their meanings, and their relationships.

## Repository Structure

1. **ontology-core/** - Core domain models and database logic

   - Contains the fundamental data structures and database interactions
   - Provides repository interfaces and implementations for symbols
   - Handles core business logic and domain models

2. **ontology-api-server/** - API server implementation

   - REST API endpoints for symbol management
   - JSON response format for external applications
   - Database-backed persistence

3. **symbol-mcp-client/** - Public MCP client binary

   - Standalone Model Context Protocol (MCP) client
   - Implements SSE transport for integration with AI tools
   - Direct database connectivity without requiring the API server
   - Complete executable binary distribution

4. **data/** - Symbol datasets and database migrations

   - Contains symbol definitions and relationships
   - Migration scripts for database setup

5. **docs/** - Documentation for the project

   - Usage guides and API references
   - Implementation details and architecture

6. **tests/** - Integration tests

   - End-to-end tests for the entire system
   - Validation of MCP compliance

7. **Root configuration**
   - Docker configuration for containerized deployment
   - Cargo workspace setup for the entire Rust project
   - Licensing information (dual-licensed under MPL 2.0 and Commercial)

## MCP Capabilities

The symbolic-ontology-mcp project implements the following MCP methods:

1. **get_symbols** - List all symbols with optional limit parameter
2. **search_symbols** - Search symbols by text query
3. **filter_by_category** - Get symbols filtered by category
4. **get_categories** - Get all available symbol categories
5. **get_symbol_sets** - List all symbol sets with optional limit
6. **search_symbol_sets** - Search symbol sets by name or description

## Technical Stack

- **Language**: Rust (98.5% of the codebase)
- **Database Support**:
  - PostgreSQL (for production)
  - SQLite (for development/testing)
- **API Protocol**: REST and Model Context Protocol (MCP)
- **Transport**: HTTP/JSON for REST, SSE for MCP
- **Deployment**: Docker containerization and Fly.io
- **Dependencies**:
  - `rmcp` - Rust implementation of the MCP protocol
  - `tokio` - Asynchronous runtime
  - Various database drivers and web frameworks

## Integration Capabilities

The symbolic-ontology-mcp project is designed to integrate with:

1. **AI Tools**: Direct integration with Claude through Cursor AI
2. **Custom Applications**: Via the REST API endpoints
3. **Development Environments**: Through local MCP server instances

## Development Status

The project is in active development, with a structured approach to code organization and multiple components that work together to provide symbolic reasoning capabilities. The codebase follows Rust best practices with separation of concerns across multiple crates.

The MCP client component is already functional and can be installed directly from GitHub or built from source. The system is designed for both development use with SQLite and production deployment with PostgreSQL.

## Deployment Options

1. **Local Development**:

   - Direct cargo build and run
   - SQLite database for simplicity

2. **Production Deployment**:
   - Docker containerization
   - PostgreSQL database backend
   - Fly.io deployment configuration included

## Licensing

The project is dual-licensed under:

- Mozilla Public License 2.0 for non-commercial use
- Commercial License for business/for-profit use

This licensing model allows for open-source contributions while maintaining commercial viability for business applications.
