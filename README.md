# Nexus Flow: AI-Powered Dream Interpretation Platform

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://mozilla.org/MPL/2.0/)

Nexus Flow is a comprehensive dream interpretation platform that combines AI-powered analysis with symbolic reasoning to provide meaningful insights into your dreams. Built with Rust across the entire stack, it offers a modern, fast, and reliable experience for dream analysis and journaling.

## 🌟 Overview

Nexus Flow consists of three integrated components that work together to provide a complete dream interpretation experience:

- **Nexus Core**: Rust-based backend API server with AI integration
- **Nexus Web**: Modern WebAssembly frontend built with Leptos
- **Symbol Ontology**: MCP-compliant symbolic reasoning engine

## ✨ Key Features

### 🧠 AI-Powered Dream Interpretation

- **Multiple AI Providers**: Supports OpenRouter and GitHub/Azure AI APIs with automatic fallback
- **Jungian Psychology Framework**: Interpretations based on established psychological principles
- **Symbol Enhancement**: Enriched interpretations using symbolic ontology database
- **Contextual Analysis**: Extracts and analyzes key symbols from dream narratives

### 📖 Personal Dream Journal

- **Dream Storage**: Save and organize your dreams with interpretations
- **Full-Text Search**: Search through your dream history by content
- **Tag Management**: Add and remove tags to categorize dreams
- **Pagination & Sorting**: Flexible browsing with customizable sort orders
- **Offline Support**: Dreams saved to localStorage for offline access

### 🔍 Symbol Analysis

- **Comprehensive Symbol Database**: Extensive collection of dream symbols and meanings
- **Category-Based Organization**: Symbols organized by themes and categories
- **Symbol Sets**: Curated collections of related symbolic meanings
- **MCP Integration**: Model Context Protocol compliance for enhanced AI interactions

### 🎨 Modern User Interface

- **Responsive Design**: Optimized for both mobile and desktop
- **Dark Mode Support**: Eye-friendly interface for bedtime dream recording
- **Loading States**: Visual feedback during API interactions
- **Error Handling**: Robust error handling with user-friendly messages
- **Accessibility**: ARIA-compliant components for screen readers

### 🔧 Developer Features

- **RESTful API**: Clean, well-documented API endpoints
- **Docker Support**: Easy containerization for all components
- **Database Migrations**: Automated PostgreSQL schema management
- **Comprehensive Testing**: Unit and integration test suites
- **CORS Configuration**: Pre-configured for local development

## 🏗️ Architecture

```
┌─────────────────┐        ┌─────────────────┐        ┌─────────────────┐
│    Nexus Web    │◄──────►│    Nexus Core   │◄──────►│ Symbol Ontology │
│   (Frontend)    │        │   (Backend)     │        │  (MCP Server)   │
│                 │        │                 │        │                 │
│ • Leptos/WASM   │        │ • Axum Server   │        │ • Symbol DB     │
│ • Dream UI      │        │ • AI Integration│        │ • MCP Protocol  │
│ • Journal       │        │ • PostgreSQL    │        │ • Rust Engine   │
└─────────────────┘        └─────────────────┘        └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites

Before setting up Nexus Flow, ensure you have the following installed:

- **Rust** (latest stable): [Install Rust](https://rustup.rs/)
- **PostgreSQL** (12+): [Install PostgreSQL](https://www.postgresql.org/download/)
- **Trunk** (for frontend): `cargo install trunk`
- **Node.js & pnpm** (optional, for convenience scripts): [Install pnpm](https://pnpm.io/installation)

### API Keys Required

- **OpenRouter API Key**: Register at [OpenRouter.ai](https://openrouter.ai)
- **GitHub Token** (optional): For Azure AI API fallback

### 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/your-username/nexus-flow-full.git
cd nexus-flow-full

# Set up environment variables (see Environment Configuration below)
```

### 2. Database Setup

```bash
# Create PostgreSQL databases
createdb nexus_core
createdb symbol_ontology

# Set up environment variables
export DATABASE_URL="postgresql://username:password@localhost:5432/nexus_core"
export SYMBOL_DB_URL="postgresql://username:password@localhost:5432/symbol_ontology"
```

### 3. Start All Services

#### Option A: Manual Setup (Recommended for Development)

```bash
# Terminal 1: Start Symbol Ontology MCP Server
cd symbol-ontology
export DATABASE_URL="postgresql://username:password@localhost:5432/symbol_ontology"
export MCP_PORT=3002
cargo run --package symbol-mcp-client

# Terminal 2: Start Nexus Core Backend
cd nexus-core
export DATABASE_URL="postgresql://username:password@localhost:5432/nexus_core"
export OPENROUTER_API_KEY="your_openrouter_api_key"
export MCP_SERVER_URL="http://localhost:3002/sse"
cargo run -- --port 3003

# Terminal 3: Start Nexus Web Frontend
cd nexus-web
export API_URL="http://localhost:3003"
trunk serve --port 3004
```

#### Option B: Docker Compose (Coming Soon)

```bash
# Set up environment variables in .env file
cp .env.example .env
# Edit .env with your configuration

# Start all services
docker-compose up -d
```

### 4. Access the Application

- **Frontend**: http://localhost:3004
- **Backend API**: http://localhost:3003
- **Symbol Ontology**: http://localhost:3002
- **API Documentation**: http://localhost:3003/health

## ⚙️ Environment Configuration

### Nexus Core (.env)

```bash
# Database
DATABASE_URL=postgresql://username:password@localhost:5432/nexus_core

# AI Services
OPENROUTER_API_KEY=your_openrouter_api_key
GITHUB_TOKEN=your_github_token  # Optional, for Azure AI fallback

# Server Configuration
PORT=3003
RUST_LOG=info

# MCP Integration
ENABLE_MCP=true
MCP_SERVER_URL=http://localhost:3002/sse
```

### Symbol Ontology (.env)

```bash
# Database
DATABASE_URL=postgresql://username:password@localhost:5432/symbol_ontology

# MCP Server
MCP_PORT=3002
RUST_LOG=info
```

### Nexus Web (Environment Variables)

```bash
# API Configuration
API_URL=http://localhost:3003
```

## 📚 Component Documentation

### Nexus Core (Backend)

**Location**: `nexus-core/`

The backend API server built with Axum that handles dream interpretation, storage, and AI integration.

#### Key Features:

- RESTful API with comprehensive endpoints
- AI integration with OpenRouter and Azure APIs
- PostgreSQL database with full-text search
- MCP client for symbol ontology integration
- Comprehensive error handling and logging

#### API Endpoints:

| Endpoint                      | Method | Description                 |
| ----------------------------- | ------ | --------------------------- |
| `/health`                     | GET    | Health check                |
| `/api/interpret_dream`        | POST   | AI dream interpretation     |
| `/api/dreams`                 | POST   | Save dream to database      |
| `/api/dreams`                 | GET    | List dreams with pagination |
| `/api/dreams/{id}`            | GET    | Get specific dream          |
| `/api/dreams/search`          | GET    | Search dreams by text       |
| `/api/dreams/{id}/tags`       | POST   | Add tag to dream            |
| `/api/dreams/{id}/tags/{tag}` | DELETE | Remove tag from dream       |

#### Database Schema:

- **dreams**: Core dream storage with full-text search
- **dream_tags**: Tag management with indexing

### Nexus Web (Frontend)

**Location**: `nexus-web/`

Modern WebAssembly frontend built with Leptos for dream interpretation and journaling.

#### Key Features:

- Responsive design with mobile-first approach
- Real-time dream interpretation with loading states
- Offline dream storage using localStorage
- Advanced search and filtering capabilities
- Accessible UI components with ARIA support

#### Pages:

- **Home**: Introduction and feature overview
- **Dream Interpreter**: Main interpretation interface
- **Dream Journal**: Browse and search saved dreams
- **Journal Entry**: Detailed dream view

#### Components:

- **Dream Forms**: Submission and editing interfaces
- **UI Library**: Buttons, badges, typography, layout
- **Feedback**: Loading states and error handling
- **Markdown**: Rich text rendering for interpretations

### Symbol Ontology (MCP Server)

**Location**: `symbol-ontology/`

Symbolic reasoning engine that provides rich symbol meanings through the Model Context Protocol.

#### Key Features:

- MCP-compliant server for AI integration
- Comprehensive symbol database
- Category-based symbol organization
- Symbol sets for thematic groupings
- Direct database connectivity

#### MCP Methods:

- `get_symbols`: List all symbols with filtering
- `search_symbols`: Text-based symbol search
- `filter_by_category`: Category-based filtering
- `get_categories`: List all symbol categories
- `get_symbol_sets`: List symbol collections
- `search_symbol_sets`: Search symbol collections

## 🧪 Testing

### Running Tests

```bash
# Test all components
cargo test --workspace

# Test specific component
cd nexus-core && cargo test
cd nexus-web && wasm-pack test --headless --firefox
cd symbol-ontology && cargo test
```

### API Testing

```bash
# Test Nexus Core API
cd nexus-core
./test_api.sh

# Comprehensive API tests
./curl_tests.sh
```

## 🐳 Docker Deployment

### Individual Components

```bash
# Nexus Core
cd nexus-core
docker build -t nexus-core .
docker run -p 3003:8080 -e DATABASE_URL=... nexus-core

# Nexus Web
cd nexus-web
docker build -t nexus-web .
docker run -p 3004:8080 nexus-web

# Symbol Ontology
cd symbol-ontology
docker build -t symbol-ontology .
docker run -p 3002:3002 -e DATABASE_URL=... symbol-ontology
```

### Production Deployment

Each component includes Fly.io configuration for easy deployment:

```bash
# Deploy to Fly.io
fly deploy
```

## 🔧 Development

### Project Structure

```
nexus-flow-full/
├── nexus-core/              # Backend API server
│   ├── src/
│   │   ├── api/             # API endpoints
│   │   ├── services/        # Business logic
│   │   ├── models/          # Database models
│   │   └── db/              # Database operations
│   ├── migrations/          # Database migrations
│   └── tests/               # Integration tests
├── nexus-web/               # Frontend application
│   ├── src/
│   │   ├── components/      # UI components
│   │   ├── pages/           # Page components
│   │   └── lib.rs           # App router
│   ├── styles/              # CSS styling
│   └── tests/               # Component tests
├── symbol-ontology/         # MCP server
│   ├── ontology-core/       # Core domain models
│   ├── ontology-api-server/ # REST API server
│   ├── symbol-mcp-client/   # MCP client binary
│   └── tests/               # Integration tests
└── docs/                    # Documentation
```

### Development Workflow

1. **Backend Development**: Start with `nexus-core` for API changes
2. **Frontend Development**: Use `nexus-web` with hot reloading via Trunk
3. **Symbol Integration**: Extend `symbol-ontology` for new symbolic meanings
4. **Testing**: Run component and integration tests regularly
5. **Documentation**: Update README and component docs

### Common Development Tasks

```bash
# Start development servers
pnpm run dev:all          # Start all services (if package.json exists)

# Database operations
cd nexus-core
cargo run --bin setup_db   # Initialize database
sqlx migrate run           # Run migrations

# Build for production
cd nexus-web
trunk build --release

# Run specific tests
cargo test --package nexus-core
wasm-pack test --headless --firefox  # Frontend tests
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust best practices and idioms
- Write tests for new functionality
- Update documentation for API changes
- Ensure all components work together
- Test across different browsers for frontend changes

## 📄 License

This project is dual-licensed:

- **Mozilla Public License 2.0** for non-commercial use
- **Commercial License** for business/for-profit use

See [LICENSE.md](LICENSE.md) for details.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/your-username/nexus-flow-full/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/nexus-flow-full/discussions)
- **Email**: info@yogimathius.dev

## 🗺️ Roadmap

### Current Status: MVP Complete ✅

- ✅ AI-powered dream interpretation
- ✅ Dream storage and retrieval
- ✅ Symbol-enhanced analysis
- ✅ Modern web interface
- ✅ MCP integration

### Upcoming Features

- **User Authentication**: Account creation and login
- **Personalized Journals**: User-specific dream storage
- **Advanced Analytics**: Pattern recognition across dreams
- **Social Features**: Dream sharing and community
- **Mobile App**: Native mobile experience
- **API Integrations**: Third-party service connections

---

**Built with ❤️ using Rust, Leptos, and the power of symbolic reasoning.**
