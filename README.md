# Nexus Core

Nexus Core is a powerful backend service for dream interpretation and analysis, built with Rust and the Axum web framework. It leverages AI capabilities through OpenRouter API integration and supports the Model Context Protocol (MCP) for enhanced functionality.

## Features

- **Dream Interpretation API**: Submit dream text and receive AI-powered interpretations
- **Dream Storage**: Save, retrieve, and list interpreted dreams in a PostgreSQL database
- **Dream Tagging**: Add and remove tags to categorize dreams
- **Dream Search**: Search dreams by text content or filter by tags
- **Pagination and Sorting**: Flexible listing with pagination and custom sort orders
- **Model Context Protocol (MCP)**: Optional integration with MCP for enhanced symbol analysis
- **Multiple AI Providers**: Support for OpenRouter and GitHub/Azure AI APIs
- **RESTful API Design**: Clean API endpoints with proper error handling
- **Docker Support**: Easy containerization for deployment
- **CORS Configuration**: Pre-configured for local development

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable or nightly)
- PostgreSQL database
- OpenRouter API key (obtain from [OpenRouter.ai](https://openrouter.ai))
- GitHub token (optional, for Azure AI API access)

### Optional Components

- **MCP Server**: For enhanced symbol analysis, you can run a separate MCP server (like a symbolic ontology service)
  - Set the MCP_URL environment variable to your given mcp server to connect
  - If no MCP server is found, the application will run without MCP functionality

### Environment Setup

Create a `.env` file in the project root with the following variables:

```
DATABASE_URL=postgresql://username:password@localhost:5432/nexus_core
OPENROUTER_API_KEY=your_openrouter_api_key
GITHUB_TOKEN=your_github_token (optional, for Azure AI API)
PORT=3003 (optional, defaults to 3003)
ENABLE_MCP=true (optional, defaults to true)
MCP_SERVER_URL=http://localhost:3002/sse (optional)
```

### Running the Server

```bash
# Install dependencies and run the server
cargo run -- --port 3003

# Or use the provided script
./run.sh 3003
```

### Using Docker

```bash
# Build the Docker image
docker build -t nexus-core .

# Run the container
docker run -p 8080:8080 \
  -e DATABASE_URL=your_database_url \
  -e OPENROUTER_API_KEY=your_api_key \
  -e GITHUB_TOKEN=your_github_token \
  nexus-core
```

## API Endpoints

### Dream Interpretation

- **POST /api/interpret_dream**
  - Interprets a dream using AI
  - Request: `{ "dream_text": "I was flying over mountains", "save": true }`
  - Response: `{ "interpretation": "...", "status": "success", "id": "uuid" }`
  - The `save` parameter is optional and defaults to false

### Dream Management

- **POST /api/dreams**

  - Creates a new dream record
  - Request: `{ "text": "Dream text", "interpretation": "Dream interpretation" }`
  - Response: `{ "id": "uuid", "status": "success" }`

- **GET /api/dreams**

  - Lists all saved dreams with pagination and sorting
  - Query parameters:
    - `page`: Page number (default: 1)
    - `limit`: Items per page (default: 10, max: 100)
    - `sort_field`: Field to sort by (`created_at`, `text`, etc.)
    - `sort_order`: `asc` or `desc` (default: `desc`)
    - `tag`: Filter by specific tag
  - Response: Array of dream objects with pagination metadata

- **GET /api/dreams/search**

  - Search for dreams by text content
  - Query parameters:
    - `search`: Text to search for
    - (All pagination parameters from /api/dreams endpoint)
  - Response: Array of matching dream objects with pagination metadata

- **GET /api/dreams/{id}**
  - Retrieves a specific dream by ID
  - Response: Dream object or 404 error

### Dream Tags

- **POST /api/dreams/{id}/tags**

  - Adds a tag to a dream
  - Request: `{ "tag": "lucid" }`
  - Response: `{ "status": "success" }`

- **DELETE /api/dreams/{id}/tags/{tag}**
  - Removes a tag from a dream
  - Response: `{ "status": "success" }`

### Health Check

- **GET /health**
  - Simple health check endpoint
  - Response: `"OK"`

## API Response Format

All API responses follow a consistent format:

### Success Response

```json
{
  "status": "success",
  ... (response data)
}
```

### Error Response

```json
{
  "status": "error",
  "message": "Error description",
  "code": "ERROR_CODE",
  "details": {} (optional)
}
```

## Architecture

The application follows a modular architecture:

- **API Layer**: Handles HTTP requests and responses
- **Service Layer**: Contains business logic and external service integrations
- **Data Layer**: Manages database operations and migrations
- **Configuration**: Environment-based configuration management

## Development

### Project Structure

```
src/
├── api/            # API endpoints and handlers
├── db/             # Database connection and migrations
├── models/         # Data models and database operations
├── services/       # Business logic and external services
├── config.rs       # Configuration management
├── lib.rs          # Core module exports
├── logging.rs      # Logging setup
├── main.rs         # Application entry point
└── server.rs       # Server implementation
```

### Design Patterns

#### Dependency Injection

Nexus Core uses trait-based dependency injection for better testability and modularity:

- **Trait Interfaces**: Services expose trait interfaces (e.g., `AiClientTrait`, `McpClientTrait`)
- **Trait Objects**: Dynamic dispatch via `Box<dyn Trait>` and `Arc<Box<dyn Trait>>`
- **Mock Implementations**: Easy creation of mock services for testing

For more details, see [docs/dependency_injection.md](docs/dependency_injection.md).

### Running Tests

```bash
cargo test
```

### API Testing Script

A test script is provided for quick API testing:

```bash
./test_api.sh
```

For more comprehensive API tests:

```bash
./curl_tests.sh
```

## Deployment

### Fly.io

A `fly.toml` configuration is included for deployment to Fly.io.

```bash
fly deploy
```

## Where to Get API Keys

- **OpenRouter API Key**: Register at [OpenRouter.ai](https://openrouter.ai) to get an API key
- **GitHub Token**: Create a personal access token in your GitHub account settings

## License

This project is proprietary software.
