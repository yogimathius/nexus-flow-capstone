# Praxis Forge Repository Documentation

Praxis Forge is a task and habit management system in early development, built with Rust, Elixir, and modern web technologies. The repository contains the following main components:

## Repository Structure

1. **atlas-waypoints/** - Project planning and issue tracking files

   - Contains JSON files defining different issue categories (goals, habits, tasks, infrastructure, etc.)
   - Includes a script (`create_issues.sh`) to automate creation of GitHub issues

2. **forge-core/** - Elixir/Phoenix backend

   - Standard Phoenix application structure with config, lib, and test directories
   - Will handle data processing, persistence, and real-time communication
   - Planning to use Rust NIFs for performance-critical operations

3. **praxis-workshop/** - Rust/Yew frontend (version 1)

   - Built with Rust, Yew framework, and WebAssembly
   - Uses Tauri for desktop application packaging
   - Contains standard Rust project structure with Cargo.toml and src directory
   - Includes HTML, CSS, and Rust source files

4. **praxis-workshop-two/** - Alternative Rust frontend implementation

   - Similar structure to praxis-workshop but appears to be an experimental or alternative implementation
   - Uses Rust, Leptos, and Tauri

5. **praxis-atlas/** - Project documentation and planning
   - Contains comprehensive project documentation including:
     - System design documents
     - Database schema (with ERD diagram)
     - Requirements documentation
     - Project planning materials
     - Testing strategy
     - Deployment information
     - Architecture decisions records

## Core Components (Planned)

According to the project documentation, Praxis Forge will include these main features:

1. **Forge Practices** - Habit tracking and formation system
2. **Praxis Paths** - Goal setting and tracking
3. **Forge Rituals** - Daily task management
4. **Praxis Matrix** - Progress analytics and visualization
5. **Forge Mastery** - Skill development tracking

## Technical Stack

- **Backend**: Elixir/Phoenix
- **Frontend**: Rust/Yew with WebAssembly
- **Desktop Packaging**: Tauri
- **Performance Optimizations**: Rust NIFs for critical operations
- **Features**: Real-time sync, offline support, end-to-end encryption

## Development Status

The project is in early development stage. The repository contains mostly planning documents, initial project setup, and scaffolding for both the frontend and backend components. It appears to be an ambitious project with a well-thought-out architecture and philosophy, but implementation is still in the early phases.
