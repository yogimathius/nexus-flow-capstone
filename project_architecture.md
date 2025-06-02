# Nexus Flow: Project Architecture and Functionality Map

## System Overview

Nexus Flow is a comprehensive dream interpretation platform that consists of three main components:

1. **Nexus Core (Backend)**: A Rust-based Axum server that handles API requests, database operations, and AI integration
2. **Nexus Web (Frontend)**: A Leptos-based web application that provides the user interface for dream interpretation
3. **Symbol Ontology**: A symbolic reasoning engine that provides a rich database of dream symbols and their interpretations

The system allows users to:

- Submit dreams for AI-powered interpretation
- Save dreams and their interpretations
- Browse past dream interpretations
- View detailed symbolic meanings of elements in their dreams

## Architecture Diagram

```
┌─────────────────┐        ┌─────────────────┐
│    Nexus Web    │◄──────►│    Nexus Core   │
│    (Frontend)   │        │    (Backend)    │
└─────────────────┘        └─────────────────┘
        ▲                          ▲
        │                          │
        ▼                          ▼
┌─────────────────┐        ┌─────────────────┐        ┌─────────────────┐
│  User Interface │        │  Database (PG)  │        │  Symbol Data    │
│  Dream Input    │        │  Dream Storage  │        │  Repository (PG)│
└─────────────────┘        └─────────────────┘        └─────────────────┘
                                    ▲                           ▲
                                    │                           │
                                    ▼                           ▼
                           ┌─────────────────┐        ┌─────────────────┐
                           │  AI Services    │        │   MCP Symbol    │
                           │  OpenRouter/    │◄──────►│   Lookup        │
                           │  Azure          │        │   Service       │
                           └─────────────────┘        └─────────────────┘
```

## 1. Nexus Core (Backend)

### Core Components

#### API Endpoints

| Endpoint               | Method | Description                                           |
| ---------------------- | ------ | ----------------------------------------------------- |
| `/health`              | GET    | Health check endpoint, returns "ok"                   |
| `/api/interpret_dream` | POST   | Interprets a dream using AI services                  |
| `/api/dreams`          | POST   | Saves a dream and its interpretation to the database  |
| `/api/dreams`          | GET    | Lists all saved dreams in reverse chronological order |
| `/api/dreams/:id`      | GET    | Retrieves a specific dream by ID                      |

#### Database Schema

**Dreams Table:**

```sql
CREATE TABLE dreams (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  text TEXT NOT NULL,
  interpretation TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT now()
);
```

#### Services

1. **AI Client**

   - Handles communication with AI models (OpenRouter and Azure)
   - Creates dream interpretation prompts with Jungian psychology framework
   - Extracts key symbols from dream texts
   - Falls back to alternative AI providers if primary service is unavailable

2. **MCP Client**
   - Connects to Symbol Ontology service for symbol lookups
   - Enhances dream interpretations with specific symbolic meanings
   - Provides a bridge between the core backend and the symbol ontology service

### Key Files

- `src/main.rs`: Application entry point, server initialization
- `src/config.rs`: Configuration loading from environment variables
- `src/server.rs`: Server setup, CORS configuration
- `src/api/dream.rs`: Dream interpretation and database API handlers
- `src/services/ai_client.rs`: AI service integration (OpenRouter/Azure)
- `src/services/mcp_client.rs`: MCP integration with symbol ontology
- `src/models/dream.rs`: Database model for dreams

## 2. Nexus Web (Frontend)

### Core Components

#### Pages

- **Home Page**: Entry point with introduction to dream interpretation
- **Dream Interpreter Page**: Main interface for submitting and interpreting dreams
- **Button Demo Page**: Component showcase
- **Badge Demo Page**: Component showcase

#### Components

1. **DreamInterpreter**: Main component for dream interpretation

   - Form for submitting dream text
   - "Common Dream Themes" badges for quick input
   - Interpretation display
   - Error handling for API failures
   - Tips for better dream recall and interpretation

2. **UI Components**:
   - Typography (SectionTitle, SubTitle)
   - Button
   - Badge
   - Layout components

### Key Files

- `src/main.rs`: Frontend application entry point
- `src/lib.rs`: Router setup and component registration
- `src/pages/dream_interpreter.rs`: Dream interpretation page
- `src/components/dream_interpreter.rs`: Main dream interpretation component
- `src/pages/home.rs`: Homepage with introduction and symbol samples

## 3. Symbol Ontology

### Core Components

#### Domain Models

1. **Symbol**:

   - Represents a symbolic entity with interpretations
   - Contains: id, name, category, description, interpretations, related symbols
   - Primary model for dream symbolism

2. **SymbolSet**:
   - Collection of related symbols organized into an ontology
   - Contains: id, name, description, category, symbols

#### Repositories

- **SymbolRepository**: Interface for accessing and storing symbols
- **MemorySymbolRepository**: In-memory implementation for development/testing
- **PostgresSymbolRepository**: Database implementation for production

#### MCP Integration

- **SymbolService**: Exposes symbol lookup functionality through MCP
- Symbol search by query term
- Symbol filtering by category
- Detailed symbol information retrieval

### Key Files

- `src/domain/symbols.rs`: Symbol domain model
- `src/domain/ontology.rs`: SymbolSet domain model
- `src/infrastructure/memory_repository.rs`: In-memory symbol storage
- `src/infrastructure/postgres_repository.rs`: Database symbol storage
- `src/mcp/service.rs`: MCP service exposing symbol API
- `data/`: CSV and JSON files with symbol definitions

## Data Flow

### Dream Interpretation Flow

1. User enters a dream text in the frontend (Nexus Web)
2. Frontend sends dream text to the backend (Nexus Core) via `/api/interpret_dream` endpoint
3. Backend extracts key symbols from the dream text
4. If Symbol Ontology is connected via MCP:
   - Backend looks up extracted symbols in Symbol Ontology
   - Symbol meanings are incorporated into the AI prompt
5. Backend makes API request to AI service (OpenRouter or Azure)
   - Includes dream text, symbol meanings, and prompt for Jungian-style interpretation
   - Falls back to alternative provider if primary is unavailable
6. Backend returns interpretation to frontend
7. Frontend displays the interpretation to the user

### Dream Storage Flow

1. User can save a dream and its interpretation
2. Backend stores the dream in PostgreSQL database
3. Dreams can be retrieved from database individually or as a list

## Symbol Ontology Integration

The Symbol Ontology service provides rich symbolic meanings for dream elements through:

1. **Memory Repository**: Pre-loaded symbols for development/testing
2. **Postgres Repository**: Production storage of symbols
3. **MCP Service**: Integration point for Nexus Core
4. **Symbol Definitions**:
   - Dream symbols (water, fire, flying, etc.)
   - Mythological symbols
   - Jungian archetypes
   - Each with meanings, interpretations, and relationships

## Technologies Used

- **Backend**:

  - Rust (programming language)
  - Axum (web framework)
  - SQLx (database toolkit)
  - PostgreSQL (database)
  - OpenRouter/Azure (AI providers)

- **Frontend**:

  - Rust + Leptos (web framework)
  - CSS Modules (styling)
  - WebAssembly (WASM)

- **Symbol Ontology**:
  - Rust (programming language)
  - MCP (Model-Consumer Protocol)
  - PostgreSQL (database)

## Development and Deployment

### Running Locally

1. **Backend**:

   ```
   cd nexus-core
   cargo run
   ```

2. **Frontend**:

   ```
   cd nexus-web
   trunk serve
   ```

3. **Symbol Ontology**:
   ```
   cd symbol-ontology
   cargo run
   ```

### Environment Configuration

Key environment variables:

- `DATABASE_URL`: PostgreSQL connection string
- `OPENROUTER_API_KEY`: API key for OpenRouter
- `GITHUB_TOKEN`: Optional token for Azure API access
- `MCP_SERVER_URL`: URL to Symbol Ontology MCP server
- `ENABLE_MCP`: Flag to enable/disable MCP integration

## Future Enhancements

1. **User Authentication**: Account creation and login
2. **Personalized Dream Journal**: User-specific dream storage
3. **Advanced Symbol Analysis**: More detailed symbolic breakdowns
4. **Pattern Recognition**: Identifying recurring themes across multiple dreams
5. **Social Sharing**: Ability to share dream interpretations
6. **Mobile Application**: Native mobile experience
