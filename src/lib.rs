use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
pub mod components;
mod pages;

// Top-Level pages
use crate::pages::badge_demo::BadgeDemo;
use crate::pages::button_demo::ButtonDemo;
use crate::pages::dream_interpreter::DreamInterpreterPage;
use crate::pages::home::Home;

// Layout components
use crate::components::layout::Layout;

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
            <Layout>
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
                    <Route
                        path=path!("/symbols")
                        view=|| view! { <h1>"Symbol Library - Coming Soon"</h1> }
                    />
                    <Route path=path!("/interpret") view=DreamInterpreterPage />

                    {}
                    <Route path=path!("/buttons") view=ButtonDemo />
                    <Route path=path!("/badges") view=BadgeDemo />
                </Routes>
            </Layout>
        </Router>
    }
}
