# Dream Interpreter AI Frontend Documentation

Dream Interpreter AI is a modern WebAssembly frontend application built with Rust and Leptos that provides an intuitive interface for users to record and interpret their dreams. This client application represents the user-facing component of a comprehensive dream interpretation ecosystem that connects to the MCP Insight backend and leverages the Symbol Ontology MCP service.

## Repository Structure

1. **src/components/** - Reusable UI components

   - **components/dream/** - Dream interpretation components
     - Form submission for dream text
     - Result display for interpretations
     - Service layer for API communication
   - **components/journal/** - Dream journal components
     - Listing and filtering saved dreams
     - Individual dream entry display
   - **components/layout/** - Application structure components
   - **components/markdown/** - Rendering interpreted content
   - **components/feedback/** - Loading and error states

2. **src/pages/** - Page components mapped to routes

   - Home page
   - Dream interpreter page
   - Journal listing page
   - Individual dream entry page

3. **styles/** - CSS styling

   - Component-specific styles
   - Global design system
   - Responsive layout utilities

4. **Configuration Files**
   - Rust/WebAssembly build configuration
   - Package management

## Key Features

1. **AI-Powered Dream Interpretation**

   - Submit dream text for detailed symbolic analysis
   - Receive AI-generated interpretations enriched with symbolic context
   - Save interpretations to personal dream journal

2. **Personal Dream Journal**

   - Browse history of interpreted dreams
   - Search and filter dreams by various criteria
   - View full dream and interpretation details

3. **Rich User Experience**

   - Responsive design for all device sizes
   - Dark mode support
   - Loading states and error handling
   - Interactive UI components
   - Markdown rendering for formatted interpretations

4. **Offline Capabilities**
   - Local storage for offline access to saved dreams
   - Graceful handling of connectivity issues

## Integration with Backend Services

The Dream Interpreter AI frontend connects to the dream interpretation ecosystem through several integration points:

1. **MCP Insight API Integration**

   - HTTP requests to the Nexus Core backend API
   - Submits dream text for interpretation
   - Receives detailed interpretations enhanced with symbolic analysis
   - Persists dreams to the backend database

2. **Symbol-Enhanced Interpretation**

   - Dreams are analyzed by the MCP Insight service
   - Symbolic elements in dreams are identified
   - Symbolism is enriched via the Symbol Ontology MCP
   - Rich, context-aware interpretations are returned to the user

3. **Authentication Flow**
   - (Planned) User authentication for secure dream storage
   - (Planned) Profile management

## Technical Architecture

1. **Leptos Framework**

   - Reactive Rust-based web framework
   - Component-based architecture
   - Signal-based state management
   - Fine-grained reactivity

2. **WebAssembly Compilation**

   - Native-speed performance in browser
   - Built with Trunk bundler
   - Optimized for size and speed

3. **State Management**

   - Local component state using Leptos signals
   - Reactive data fetching with resource components
   - Comprehensive loading/error states

4. **API Communication**
   - Typed HTTP requests to MCP Insight backend
   - JSON serialization/deserialization
   - Error handling and retry mechanisms

## User Journey

1. **Dream Submission**

   - User enters dream text in the interpreter form
   - Client sends request to MCP Insight backend
   - Backend processes dream with AI and symbolic analysis
   - Client receives and displays interpretation
   - User can save dream to journal

2. **Journal Management**
   - User browses saved dreams in journal view
   - Filters and searches for specific dreams
   - Views detailed interpretations of past dreams

## Development Workflow

1. **Local Development**

   - Rust/WebAssembly development with hot reload
   - Trunk bundler for asset compilation
   - Component-level testing

2. **Production Build**
   - Optimized WebAssembly compilation
   - Minified CSS and assets
   - Deployment-ready static files

## Connection to Full System

The Dream Interpreter AI frontend forms the user-facing component of a three-tier architecture:

1. **Frontend (This Repository)**

   - User interface for dream submission and browsing
   - WebAssembly application for cross-platform compatibility

2. **MCP Insight Backend (Sprint Three-Point-Two)**

   - REST API for dream processing
   - AI-powered interpretation
   - Database persistence
   - Integration with Symbol Ontology MCP

3. **Symbol Ontology MCP (Sprint Three)**
   - MCP-compliant server for symbolic reasoning
   - Provides symbolic analysis and meaning
   - Enhances interpretations with archetypal context

This architecture demonstrates a complete, full-stack Rust application with distributed components that work together to provide a seamless user experience for dream interpretation and analysis.
