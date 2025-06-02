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

### Currently Implemented Features

- **AI-Powered Interpretations**: Submit dreams and receive detailed symbolic analysis
- **Personal Dream Journal**: Store and browse your dream history
- **Dream Search & Filtering**: Search by text, category, tags, date, and favorites
- **Offline Support**: Dreams are saved to localStorage for offline access
- **Rich UI Components**: Custom buttons, badges, and typography components
- **Loading States**: Visual feedback during API interactions
- **Error Handling**: Robust error handling for API failures
- **Responsive Design**: Optimized for both mobile and desktop experiences
- **Dark Mode**: Eye-friendly interface for bedtime dream recording
- **Animations**: Subtle transitions and loading states

### Upcoming Features

- **User Authentication**: Login, registration, and profile management
- **Symbol Library**: Comprehensive database of dream symbols and meanings
- **Dream Analytics**: Visualize patterns and trends in your dream history
- **Social Features**: Optional sharing of dreams and interpretations
- **Advanced Filtering**: More sophisticated options for organizing dreams
- **Mobile App**: Dedicated mobile experience with offline capabilities

## 🏗️ Application Architecture

### Frontend Structure

```
nexus-web/
├── src/
│   ├── components/         # Reusable UI components
│   │   ├── dream/          # Dream interpretation components
│   │   ├── journal/        # Journal-related components
│   │   ├── layout/         # Layout components (header, footer)
│   │   ├── markdown/       # Markdown rendering components
│   │   ├── buttons/        # Button component variations
│   │   ├── badges/         # Badge component variations
│   │   ├── feedback/       # Loading and error components
│   │   └── typography/     # Typography components
│   ├── pages/              # Page components mapped to routes
│   │   ├── home.rs         # Landing page
│   │   ├── dream_interpreter.rs  # Dream submission & interpretation
│   │   ├── dream_journal.rs      # Journal listing page
│   │   ├── journal_entry.rs      # Individual dream detail
│   │   └── *_test.rs       # Component test files
│   └── lib.rs              # Main app entry point and routing
├── styles/                 # CSS styling files
├── public/                 # Static assets
└── tests/                  # Integration tests
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
- **Offline Data**: LocalStorage for saving dreams when offline

## 🧪 Testing Strategy

Tests are structured using WebAssembly-based testing through `wasm-bindgen-test`:

- **Component Tests**: Test individual components in isolation (in progress)
- **State Tests**: Verify components handle loading/error/success states correctly (in progress)
- **Formatting Tests**: Ensure date formatting and Markdown processing work as expected (in progress)

Each test file accompanies its respective component file with matching test patterns.

## 🖌️ Styling Architecture

The project uses a clean CSS approach with a focus on maintainability:

- **Global Variables**: Consistent theme colors, spacing, and typography defined in variables.css
- **Component Styles**: Organized by component type in components.css
- **Layout System**: Flexbox and grid-based responsive layout in layout.css
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
git clone https://git.us.qwasar.io/capstone-submission_182993_kyqxpf/capstone-submission.git nexus-flow
cd nexus-flow/dream-interpreter

# Install Rust dependencies
cargo install trunk

# Install Node.js dependencies (optional)
pnpm install
```

### Development

```bash
# Start the development server
pnpm run dev
# OR directly with Trunk
trunk serve
```

The application will be available at `http://localhost:8080`.

### Building for Production

```bash
pnpm run build
# OR
trunk build --release
```

Production assets will be generated in the `dist` directory.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
