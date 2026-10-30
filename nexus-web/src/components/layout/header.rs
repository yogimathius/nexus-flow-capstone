use js_sys;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[component]
pub fn Header() -> impl IntoView {
    // Start with default dark mode
    let (is_dark_mode, set_is_dark_mode) = signal(true);

    // Mobile menu state
    let (is_mobile_menu_open, set_is_mobile_menu_open) = signal(false);

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
    let _ = Effect::new(move |_| {
        apply_theme(is_dark_mode.get());
    });

    let toggle_dark_mode = move |_| {
        let new_mode = !is_dark_mode.get();
        set_is_dark_mode.set(new_mode);

        // Save theme to localStorage using JS directly
        let theme = if new_mode { "dark" } else { "light" };
        let _ = js_sys::eval(&format!("localStorage.setItem('theme', '{}')", theme));

        // Apply the theme
        apply_theme(new_mode);
    };

    let toggle_mobile_menu = move |_| {
        set_is_mobile_menu_open.update(|open| *open = !*open);
    };

    view! {
        <header class="header">
            <div class="container">
                <div class="header-content">
                    // Logo (always visible)
                    <a href="/" class="logo" aria-label="Nexus Flow home">
                        <span class="logo-mark">"🌙"</span>
                        <span class="logo-text">"Nexus Flow"</span>
                    </a>

                    // Desktop Navigation
                    <nav class="nav nav-desktop">
                        <a href="/" class="nav-link">
                            "Home"
                        </a>
                        <a href="/interpret" class="nav-link">
                            "Interpret"
                        </a>
                        <a href="/journal" class="nav-link">
                            "Journal"
                        </a>
                    </nav>

                    // Theme toggle and hamburger menu container
                    <div class="header-actions">
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

                        // Mobile menu toggle button
                        <button
                            on:click=toggle_mobile_menu
                            class=move || format!("mobile-menu-toggle {}", if is_mobile_menu_open.get() { "mobile-menu-open" } else { "" })
                            aria-label="Toggle navigation menu"
                            aria-expanded=move || is_mobile_menu_open.get()
                        >
                            <span class="hamburger">
                                <span class="hamburger-line"></span>
                                <span class="hamburger-line"></span>
                                <span class="hamburger-line"></span>
                            </span>
                        </button>
                    </div>

                    // Mobile Navigation Overlay
                    <nav
                        class=move || format!("nav-mobile {}", if is_mobile_menu_open.get() { "nav-mobile-open" } else { "" })
                        aria-hidden=move || !is_mobile_menu_open.get()
                    >
                        <div class="nav-mobile-content">
                            <a
                                href="/"
                                class="nav-link-mobile"
                                on:click=move |_| set_is_mobile_menu_open.set(false)
                            >
                                "Home"
                            </a>
                            <a
                                href="/interpret"
                                class="nav-link-mobile"
                                on:click=move |_| set_is_mobile_menu_open.set(false)
                            >
                                "Interpret"
                            </a>
                            <a
                                href="/journal"
                                class="nav-link-mobile"
                                on:click=move |_| set_is_mobile_menu_open.set(false)
                            >
                                "Journal"
                            </a>
                        </div>
                    </nav>
                </div>
            </div>

            // Mobile menu backdrop
            <div
                class=move || format!("mobile-menu-backdrop {}", if is_mobile_menu_open.get() { "mobile-menu-backdrop-visible" } else { "" })
                on:click=move |_| set_is_mobile_menu_open.set(false)
            ></div>
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
