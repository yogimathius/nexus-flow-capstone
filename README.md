<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Dream Interpreter AI

A modern WebAssembly application built with Rust and Leptos that uses AI to decode the hidden meanings in your dreams.

## 🌠 Vision

Dream Interpreter AI bridges the gap between your subconscious and conscious mind. By leveraging artificial intelligence to analyze dream patterns and symbols, we provide meaningful insights into your dream experiences. Our vision is to create an intuitive, beautiful interface where users can:

- Document their dreams in a private, secure journal
- Receive personalized interpretations powered by advanced AI
- Track patterns and recurring symbols in their dream life
- Understand themselves better through dream analysis

## ✨ Key Features

### Core Functionality

- **AI-Powered Interpretations**: Submit dreams and receive detailed symbolic analysis
- **Personal Dream Journal**: Chronological history of dreams with search capabilities
- **Rich Formatting**: Markdown support for dream content and interpretations

### User Experience

- **Responsive Design**: Seamless experience across mobile and desktop devices
- **Dark Mode**: Eye-friendly interface optimized for bedtime dream recording
- **Animations**: Subtle, elegant transitions and loading states
- **Elegant Typography**: Carefully crafted text hierarchy for readability

### Technical Features

- **Offline Support**: Save dreams locally when offline (coming soon)
- **Fast Performance**: WebAssembly-powered rendering for near-native speed
- **Accessibility**: ARIA-compliant components for inclusive user experience

## 🏗️ Application Architecture

### Frontend Structure

```
nexus-web/
├── src/
│   ├── components/         # Reusable UI components
│   │   ├── layout/         # Layout components (header, footer)
│   │   ├── markdown/       # Markdown rendering components
│   │   └── typography/     # Typography components
│   ├── pages/              # Page components mapped to routes
│   │   ├── home.rs         # Landing page
│   │   ├── dream_interpreter.rs  # Dream submission & interpretation
│   │   ├── dream_journal.rs      # Journal listing page
│   │   ├── journal_entry.rs      # Individual dream detail
│   │   └── *_test.rs       # Component test files
│   └── lib.rs              # Main app entry point and routing
├── public/                 # Static assets
└── .cursor/rules/          # Project documentation
```

### Component Hierarchy

- **App** (`lib.rs`): Main router component
  - **Layout**: Persistent app shell
    - **Pages**: Content area with route-based views
      - Home, Interpreter, Journal, Entry Detail
    - **UI Components**: Buttons, cards, forms, etc.

### Data Flow

1. **Dream Submission Flow**:

   - User enters dream in the form
   - Dream is sent to API for interpretation
   - Result is displayed and optionally saved to journal

2. **Journal Entry Flow**:
   - Journal page lists all saved dreams with previews
   - User clicks on a dream entry
   - App navigates to `/journal/{entry-id}`
   - JournalEntryPage fetches and displays full dream and interpretation

### State Management

- **Local Component State**: Leptos signals for component-specific state
- **Reactive Data Fetching**: Effects trigger API requests on parameter changes
- **UI State Machine**: Loading → Error/Success states with appropriate feedback

## 🧪 Testing Strategy

Tests are structured using WebAssembly-based testing through `wasm-bindgen-test`:

- **Component Tests**: Test individual components in isolation
- **State Tests**: Verify components handle loading/error/success states correctly
- **Formatting Tests**: Ensure date formatting and Markdown processing work as expected

Each test file accompanies its respective component file with matching test patterns.

## 🖌️ Styling Architecture

The project follows a modular CSS approach using CSS Modules with Stylance:

```rust
// Component with scoped styles
use stylance::import_style;

import_style!(style, "components/dream_card.module.css");

#[component]
fn DreamCard(dream: Dream) -> impl IntoView {
    view! {
        <div class=style::card>
            <h3 class=style::title>{dream.title}</h3>
        </div>
    }
}
```

- **Global Variables**: Consistent theme colors, spacing, and typography
- **Component Scoping**: Styles isolated to their components
- **Animation System**: Consistent slide/fade animations with delay classes
- **Responsive Utilities**: Mobile-first design with adaptive layouts

## 🚀 Getting Started

### Prerequisites

- Rust 1.85+
- Trunk (WebAssembly bundler)
- Node.js (optional, for development conveniences)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/dream-interpreter.git
cd dream-interpreter

# Install Rust dependencies
cargo install trunk
cargo install stylance-cli

# Install Node.js dependencies (optional)
npm install
```

### Development

```bash
# Start the development server
npm run dev
# OR directly with Trunk
trunk serve
```

The application will be available at `http://localhost:8080`.

### Building for Production

```bash
npm run build
# OR
trunk build --release
```

Production assets will be generated in the `dist` directory.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
