# Capstone Implementation Journey

This document chronicles my journey through the development of my capstone project, including the technical decisions, pivots, challenges, and solutions that shaped the final implementation.

## 1. Initial Vision: Praxis Forge

### Original Technical Stack

- **Frontend**: Rust/Yew with WebAssembly
- **Desktop Packaging**: Tauri
- **Backend**: Elixir/Phoenix
- **Features**: Task management, habit tracking, goal setting

### Strategic Intent

The initial vision was to create a comprehensive personal development system with multiple integrated components:

- **Forge Practices**: Habit tracking and formation
- **Praxis Paths**: Goal setting and tracking
- **Forge Rituals**: Daily task management
- **Praxis Matrix**: Progress analytics and visualization
- **Forge Mastery**: Skill development tracking

This ambitious system would leverage the performance benefits of Rust for the frontend while using Elixir's concurrency model for the backend.

## 2. First Pivot: Leptos Framework Adoption

### Technical Decision

After starting implementation with Yew, I discovered Leptos, a newer Rust-based reactive framework that offered:

- Significantly reduced boilerplate code
- More modern reactive programming model
- Better performance characteristics

### Implementation Progress

- Successfully migrated from Yew to Leptos
- Maintained Tauri for desktop packaging
- Continued integration with Elixir backend

### Initial Benefits

The switch to Leptos provided immediate benefits:

- Cleaner component structure
- More intuitive reactive programming model
- Reduced code verbosity

## 3. Challenge: Integration Complexity

### GraphQL/Cynic Integration

Attempting to integrate GraphQL via the Cynic crate to connect to the Elixir API revealed unexpected challenges:

- **Build time impact**: Dramatic increase in compilation times
- **Complex type mapping**: Difficulty aligning Rust types with GraphQL schema
- **Test integration issues**: Conflicts between WebAssembly test frameworks and GraphQL clients

### CSS Framework Integration

Multiple attempts to integrate CSS frameworks failed:

- Tailwind CSS integration caused build conflicts
- Component libraries like Leptonic had version compatibility issues
- CSS modules exhibited inconsistent behavior between components

### Result: Development Friction

These integration challenges significantly impacted development velocity:

- The WASM development cycle became prohibitively slow
- Error messages were cryptic and difficult to debug
- IDE feedback was limited due to macro-heavy code
- Testing implementation became extremely complex

## 4. Second Pivot: Praxis Shop v2

### Focused Implementation

Created a more targeted implementation focusing specifically on core functionality:

- Simplified scope to focus on goals and tasks components only
- Maintained Leptos and Tauri as the primary frontend technologies
- Continued with GraphQL for data communication

### Continued Challenges

Despite the more focused approach, fundamental issues persisted:

- Slow build times continued to hamper development
- CSS integration remained problematic
- Type system friction created a frustrating development experience
- WebAssembly-specific challenges limited browser API interaction

## 5. Major Pivot: The Nexus Vision

### Strategic Reevaluation

Based on the accumulated painpoints and emerging technology trends, I made a significant pivot:

- Shifted from personal development tools to AI-integrated services
- Decided to create a platform leveraging symbolic reasoning and AI
- Maintained commitment to Rust as the primary language

### New Project Focus: Dream Interpretation

The new direction focused on creating a dream interpretation platform with:

- Symbolic reasoning backend
- AI-powered interpretation
- Modern, responsive UI
- MCP protocol integration to align with emerging standards

## 6. Distributed Architecture Design

### Initial SSR Leptos Attempt

First attempted to use Server-Side Rendering with Leptos:

- Encountered severe IDE feedback issues
- Half the codebase showed as unused due to macro-based server/client code separation
- Application would fail with limited developer feedback
- Development experience was extremely challenging

### Pivot to Distributed Architecture

Made the critical decision to separate concerns into distinct applications:

1. **Symbol Ontology MCP**: Rust-based MCP server for symbolic reasoning
2. **MCP Insight (Nexus Core)**: Axum-based API server integrating AI and MCP
3. **Dream Interpreter AI**: Leptos CSR frontend for user interaction

### Architecture Benefits

This distributed approach provided immediate benefits:

- Clear separation of concerns
- More manageable codebases
- Better IDE feedback for each component
- Easier testing and deployment strategies
- Ability to leverage the strengths of each framework

## 7. Component Implementation

### Symbol Ontology MCP

Created a complete MCP-compliant server:

- Implemented the MCP protocol for symbolic reasoning
- Built efficient database models for symbol storage
- Created comprehensive API endpoints for symbol querying
- Designed for both development and production (PostgreSQL)
- Packaged as a deployable server and standalone binary

### MCP Insight Backend

Developed the AI integration and business logic layer:

- Built with Axum for efficient HTTP handling
- Created client implementation for Symbol Ontology MCP
- Integrated with OpenRouter API for AI capabilities
- Implemented PostgreSQL persistence for dream storage
- Designed a trait-based architecture for testability

### Dream Interpreter Frontend

Built a modern, responsive client application:

- Used Leptos CSR for improved development experience
- Implemented comprehensive error handling
- Created intuitive UI for dream submission and browsing
- Added offline capabilities for persistent dream storage
- Focused on user experience and accessibility

## 8. Technical Challenges & Solutions

### Challenge: Slow Build Times

**Solution:**

- Distributed architecture reduced the scope of each project
- Moved away from complex GraphQL/WebAssembly integrations
- Implemented specific build optimizations for each component
- Created detailed documentation of build issues for easier troubleshooting

### Challenge: Cryptic Error Messages

**Solution:**

- Created comprehensive painpoints documentation to track errors and solutions
- Extracted business logic from UI components for better testability
- Used explicit type annotations to reduce type inference errors
- Developed patterns for common Leptos operations to avoid repeated issues

### Challenge: Integration Testing

**Solution:**

- Designed trait-based abstractions for testable components
- Created mock implementations for external services
- Implemented component testing strategies for frontend
- Separated business logic from framework code where possible

### Challenge: CSS and Styling

**Solution:**

- Moved to custom CSS with consistent naming conventions
- Created reusable component styles for consistency
- Prioritized responsive design patterns

## 9. Deployment Strategy

### Current Status

- Symbol Ontology MCP deployed as a server instance
- Investigating options for distribution as a scriptable binary
- MCP Insight backend prepared for containerized deployment
- Dream Interpreter frontend ready for static hosting

### Deployment Considerations

- **Security**: Implemented prompt injection protection and response sanitization
- **Privacy**: Added user data anonymization and privacy controls
- **Performance**: Optimized response times with caching and batch processing
- **Maintenance**: Built with clean architecture for future maintainability

## 10. Lessons Learned & Future Direction

### Key Learnings

- **Framework Selection**: Early framework decisions have cascading impacts on development velocity
- **Architecture Matters**: Monolithic approaches with new technologies can create development friction
- **Documentation**: Tracking painpoints and solutions saves significant time
- **Distributed Systems**: Smaller, focused components can be easier to develop and maintain
- **Technical Pivots**: Being willing to pivot based on implementation feedback leads to better outcomes

### Future Plans

- Complete deployment of all system components
- Enhance the AI integration with more advanced models
- Expand the symbolic reasoning capabilities
- Consider implementing a comprehensive authentication system
- Explore options for improving dream analysis with personal context

## Conclusion

The journey from Praxis Forge to the Nexus dream interpretation platform represents a practical evolution based on both technical realities and strategic alignment with emerging technologies. While the final implementation differs significantly from the initial vision, it successfully addresses the core challenges that emerged during development while providing valuable experience with Rust-based web development, AI integration, and distributed systems architecture.

The pivot to a distributed architecture focusing on AI and symbolic reasoning allowed for a more manageable implementation while aligning with cutting-edge technology trends. The documented painpoints and solutions provide valuable insights for future Rust-based web development projects, especially those involving WebAssembly and reactive frameworks like Leptos.

This capstone journey demonstrates the importance of technical flexibility, strategic pivoting, and detailed documentation in modern software development.
