<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Dream Interpreter AI - Leptos CSR Application

A client-side rendered web application built with Rust and Leptos that interprets dreams using AI.

## CSS Architecture

This project follows a modular CSS approach using CSS Modules with Stylance. This approach offers several benefits:

- **Component-specific styling**: Each component has its own CSS file, scoped to that component
- **No style collisions**: Class names are automatically hashed to prevent conflicts
- **Better organization**: Styles are co-located with their components
- **Improved maintainability**: Changes to one component's styles don't affect others

### Structure

- `public/styles.css`: Global styles, CSS variables, and base styling
- `src/components/*.module.css`: Component-specific styles
- `dist/styles.css`: Generated CSS from Stylance (don't edit directly)

### How to Use CSS Modules

1. Create a `.module.css` file next to your component
2. Import it in your component using `import_style!(style, "path/to/file.module.css")`
3. Use the classes with `class=style::class_name`

Example:

```rust
// In your_component.rs
use stylance::import_style;

import_style!(style, "components/your_component.module.css");

#[component]
fn YourComponent() -> impl IntoView {
    view! {
        <div class=style::container>
            <h1 class=style::title>"Hello, world!"</h1>
        </div>
    }
}
```

```css
/* In components/your_component.module.css */
.container {
  padding: 1rem;
}

.title {
  color: var(--primary);
}
```

## Technologies

- Rust 1.85+
- Leptos 0.8
- Stylance 0.4 (CSS Modules)
- Trunk (WebAssembly builder)

## Setup

1. Install dependencies:

```bash
# Install Rust dependencies
cargo install trunk
cargo install stylance-cli

# Install Node.js dependencies
npm install
```

2. Run the development server:

```bash
npm run dev
# OR
trunk serve
```

3. Build for production:

```bash
npm run build
# OR
trunk build --release
```

## License

MIT
