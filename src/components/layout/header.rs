use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let (is_dark_mode, set_is_dark_mode) = create_signal(true); // Default to dark mode

    let toggle_dark_mode = move |_| {
        let new_mode = !is_dark_mode.get();
        set_is_dark_mode.set(new_mode);

        // Update the data-theme attribute on the HTML element
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let theme = if new_mode { "dark" } else { "light" };
                html.set_attribute("data-theme", theme).ok();
            }
        }
    };

    view! {
        <header class="header">
            <div class="container">
                <div class="header-content">
                    <a href="/" class="logo">
                        "🧠 Dream Interpreter AI"
                    </a>
                    <nav>
                        <a href="/" class="nav-link">
                            "Home"
                        </a>
                        <a href="/symbols" class="nav-link">
                            "Symbols"
                        </a>
                        <a href="/interpret" class="nav-link">
                            "Interpret"
                        </a>
                    </nav>
                    <button
                        on:click=toggle_dark_mode
                        class="theme-toggle"
                        aria-label="Toggle dark mode"
                    >
                        {move || if is_dark_mode.get() { "🌞" } else { "🌙" }}
                    </button>
                </div>
            </div>
        </header>
    }
}
