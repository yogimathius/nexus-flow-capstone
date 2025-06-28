# MCP Insight (Nexus Core) Repository Documentation

MCP Insight (also called Nexus Core) is a backend service for dream interpretation and analysis that connects to the Symbol Ontology MCP server. Built with Rust and the Axum web framework, it demonstrates practical integration with the MCP protocol and leverages AI capabilities through OpenRouter API.

## Repository Structure

1. **src/api/** - API endpoints and handlers

   - RESTful API implementation for dream interpretation and management
   - HTTP route configuration and request/response handling

2. **src/services/** - Business logic and external service integrations

   - **services/ai/** - AI service integrations for dream interpretation
   - **services/mcp/** - MCP client for symbol analysis
     - Client implementation for connecting to the Symbol Ontology MCP server
     - Symbol operations and type definitions
     - Trait-based abstractions for testing and flexibility

3. **src/models/** - Data models and database operations

   - Dream and interpretation data structures
   - Database operations for persistence

4. **src/db/** - Database connection and migrations

   - PostgreSQL connection handling
   - Migration management

5. **Configuration and Setup**
   - Environment-based configuration
   - Server implementation and startup
   - Error handling and logging

## MCP Integration

The MCP Insight repository establishes a direct connection to the Symbol Ontology MCP server (from sprint_three) with these key components:

1. **McpClient Implementation**:

   - Connects to the Symbol Ontology MCP server via Server-Sent Events (SSE)
   - Sends symbol search and filtering requests to the MCP server
   - Processes symbol data received from the server

2. **Symbol Analysis Features**:

   - Extracts symbolic elements from dream content
   - Queries the Symbol Ontology MCP server for symbol meanings
   - Enhances dream interpretations with symbolic context

3. **Fallback Mechanisms**:
   - Gracefully handles situations when the MCP server is unavailable
   - Continues functioning with basic interpretation capabilities

## Core Functionality

1. **Dream Interpretation**:

   - Accepts dream text submissions via API
   - Processes dreams using AI interpretation
   - Enhances interpretations with symbolic analysis via the MCP connection
   - Returns detailed interpretations with symbolic insights

2. **Dream Storage and Retrieval**:

   - Persists dreams and their interpretations to PostgreSQL
   - Provides API endpoints for listing and retrieving past dreams
   - Maintains associations between dreams and their symbolic elements

3. **Symbol-Enhanced Analysis**:
   - Identifies potential symbols in dream content
   - Retrieves symbol meanings and associations from the Symbol Ontology MCP
   - Incorporates symbolic context into dream interpretations

## Technical Stack

- **Web Framework**: Axum (Rust-based async web framework)
- **Database**: PostgreSQL with SQLx for type-safe queries
- **AI Integration**: OpenRouter API for dream interpretation
- **MCP Client**: Custom implementation for Symbol Ontology connectivity
- **Deployment**: Docker containerization and Fly.io support
- **Design Pattern**: Trait-based dependency injection for testability

## Integration with Symbol Ontology MCP

The MCP Insight service demonstrates a practical application of the Symbol Ontology MCP server by:

1. **Connecting as a Client**:

   - Establishes an SSE connection to the Symbol Ontology MCP server
   - Authenticates if required by the MCP server configuration

2. **Utilizing Symbol Lookup**:

   - Searches for symbols related to dream content
   - Filters symbols by category relevance
   - Retrieves detailed symbolic meanings

3. **Enhancing AI Interpretations**:
   - Combines AI-generated interpretations with symbolic insights
   - Provides deeper contextual understanding of dream elements
   - Creates a richer analysis by connecting dreams to archetypal patterns

## Configuration

The system is configured via environment variables:

- **DATABASE_URL**: PostgreSQL connection string
- **OPENROUTER_API_KEY**: API key for AI service
- **ENABLE_MCP**: Toggle for MCP functionality (defaults to true)
- **MCP_SERVER_URL**: URL to the Symbol Ontology MCP server
- **PORT**: Server port (defaults to 3003)

## Deployment Options

1. **Local Development**:

   - Standard Cargo-based development workflow
   - Environment variables via .env file
   - Optional local MCP server connection

2. **Production Deployment**:
   - Docker containerization
   - Fly.io deployment configuration
   - External PostgreSQL database
   - Connection to production MCP server

## Development Status

The project demonstrates a functional integration between an application server and the Symbol Ontology MCP, showcasing how symbolic reasoning can enhance AI-powered interpretation services. The code follows modern Rust practices with trait-based abstractions, proper error handling, and comprehensive testing.
