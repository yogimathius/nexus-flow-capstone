use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />

        // sets the document title
        <Title text="Dream Interpreter AI | Decode Your Dreams" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta
            name="description"
            content="Unlock the secrets of your subconscious with our AI-powered Dream Interpreter"
        />

        <Router>
            <Routes fallback=|| {
                view! {
                    <div class="container">
                        <h1>"Page Not Found"</h1>
                        <p>"The page you are looking for does not exist."</p>
                        <a href="/">"Return to Home"</a>
                    </div>
                }
            }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
