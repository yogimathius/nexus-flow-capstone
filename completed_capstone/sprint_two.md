# Praxis Shop v2 Repository Documentation

Praxis Shop v2 is a goal and task management application built with Rust, using modern web technologies and a desktop wrapper. It represents a continuation of the Praxis Forge project with a more focused implementation using different framework choices.

## Repository Structure

1. **src/** - Rust frontend code using Leptos framework

   - **components/** - UI components for the application
   - **graphql/** - GraphQL query and mutation definitions
   - **pages/** - Page components including goals, tasks, and home views
   - **services/** - Services for handling API communication
   - **state/** - State management code
   - **lib.rs** - Main application setup with routing definitions

2. **src-tauri/** - Tauri desktop application wrapper

   - Contains configuration for packaging the web app as a desktop application
   - Includes icons and capabilities definitions
   - Manages platform-specific functionality

3. **Root files**
   - **Cargo.toml** - Rust dependencies including Leptos, cynic for GraphQL, and other utilities
   - **schema.graphql** - GraphQL schema defining the data model for goals and tasks
   - **index.html** - HTML entry point for the application
   - **styles.css** - Global application styling

## Core Components

According to the GraphQL schema and application code, Praxis Shop v2 focuses on two main entities:

1. **Goals** - Larger objectives with tracking for completion progress

   - Title and description
   - Tasks required for completion
   - Progress tracking via completed tasks count

2. **Tasks** - Individual items that can be completed
   - Title and description
   - Completion status
   - Association with parent goals

## Technical Stack

- **Frontend Framework**: Leptos (Rust-based reactive framework)
- **GraphQL Client**: Cynic (Rust GraphQL client)
- **Desktop Packaging**: Tauri
- **Styling**: CSS
- **Data Storage**: External GraphQL API (referenced but not included in the repository)

## Application Features

- **Goal Management**: Create, view, update, and delete goals
- **Task Management**: Create, view, update, and delete tasks
- **Progress Tracking**: Monitor completion status of tasks and goals
- **Responsive UI**: Clean interface that works on both web and desktop
- **Routing**: Multiple views for different aspects of the application

## Development Status

The project appears to be a more targeted implementation focusing specifically on the goals and tasks components from the original Praxis Forge vision. It represents a shift from the Yew framework used in the original project to Leptos, a newer Rust-based reactive framework.

The application is structured as a desktop-first experience using Tauri, but maintains web compatibility. The GraphQL schema suggests integration with a backend service, though the backend implementation is not included in this repository.

This implementation focuses on the core functionality of task and goal management rather than the more comprehensive personal development system described in the Praxis Forge project. It likely serves as a practical, focused implementation of the task tracking components that were conceptualized in the larger project vision.
