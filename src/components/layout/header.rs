use js_sys;
use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[component]
pub fn Header() -> impl IntoView {
    // Start with default dark mode
    let (is_dark_mode, set_is_dark_mode) = signal(true);

    // Store emojis as a static constant to avoid ownership issues
    const EMOJIS: &[&str] = &["🧠", "💭", "🌙", "✨", "🔮"];
    let (emoji_index, set_emoji_index) = signal(0);

    // Initialize theme when component loads
    let init_theme = move || {
        // Try to get theme from localStorage using JS directly
        if let Ok(value) = js_sys::eval("localStorage.getItem('theme')") {
            if !value.is_null() {
                let theme_str = value.as_string().unwrap_or_default();
                return theme_str == "dark";
            }
        }

        // Check system preference
        let prefers_dark = detect_system_dark_mode();
        prefers_dark
    };

    // Set initial theme
    set_is_dark_mode.set(init_theme());

    // Apply initial theme
    let _ = create_effect(move |_| {
        apply_theme(is_dark_mode.get());
    });

    // Set up emoji rotation
    set_interval(
        move || {
            let next_index = (emoji_index.get() + 1) % EMOJIS.len();
            set_emoji_index.set(next_index);
        },
        Duration::from_millis(3000),
    );

    let toggle_dark_mode = move |_| {
        let new_mode = !is_dark_mode.get();
        set_is_dark_mode.set(new_mode);

        // Save theme to localStorage using JS directly
        let theme = if new_mode { "dark" } else { "light" };
        let _ = js_sys::eval(&format!("localStorage.setItem('theme', '{}')", theme));

        // Apply the theme
        apply_theme(new_mode);
    };

    // Function to access the current emoji
    let current_emoji = move || EMOJIS[emoji_index.get()];

    view! {
        <header class="header">
            <div class="container">
                <div class="header-content">
                    <div class="nav">
                        <a href="/" class="nav-link">
                            "Home"
                        </a>
                        <a href="/interpret" class="nav-link">
                            "Interpret"
                        </a>
                        <a href="/journal" class="nav-link">
                            "Journal"
                        </a>
                    </div>

                    <a href="/" class="logo logo-centered">
                        <span class="animated-emoji">{current_emoji}</span>
                        " Dream Interpreter AI"
                    </a>

                    <div class="flex items-center">
                        <button
                            on:click=toggle_dark_mode
                            class="theme-toggle"
                            aria-label=move || {
                                if is_dark_mode.get() {
                                    "Switch to light mode"
                                } else {
                                    "Switch to dark mode"
                                }
                            }
                        >
                            {move || if is_dark_mode.get() { "🌞" } else { "🌙" }}
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}

// Helper function to detect system dark mode preference using JS
fn detect_system_dark_mode() -> bool {
    let result = js_sys::eval(
        "window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches",
    );

    if let Ok(result) = result {
        result.as_bool().unwrap_or(false)
    } else {
        false
    }
}

// Apply theme to HTML element
fn apply_theme(is_dark: bool) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                // Add transition class
                let _ = html.class_list().add_1("theme-transition");

                // Apply theme after a brief delay
                let theme = if is_dark { "dark" } else { "light" };
                let _ = html.set_attribute("data-theme", theme);

                // Set a timeout to remove the transition class
                let html_clone = html.clone();
                let timeout_closure = Closure::wrap(Box::new(move || {
                    let _ = html_clone.class_list().remove_1("theme-transition");
                }) as Box<dyn FnMut()>);

                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_closure.as_ref().unchecked_ref(),
                    1000, // 1 second
                );

                // Forget the closure to prevent memory leaks
                timeout_closure.forget();
            }
        }
    }
}
