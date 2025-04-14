use crate::components::dream_interpreter::DreamInterpreter;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="container">
                    <h1>"Something Went Wrong"</h1>
                    <div class="error-message">
                        <p>"The application encountered an error:"</p>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}
                        </ul>
                    </div>
                </div>
            }
        }>
            <header>
                <div class="container">
                    <h1>"Dream Interpreter AI"</h1>
                    <p class="tagline">
                        "Unlock the secrets of your subconscious mind with AI-powered dream analysis"
                    </p>
                </div>
            </header>

            <main>
                <div class="container">
                    <DreamInterpreter />
                </div>
            </main>

            <footer>
                <div class="container">
                    <p>"© 2025 Dream Interpreter AI - Powered by Rust and Leptos"</p>
                </div>
            </footer>
        </ErrorBoundary>
    }
}
