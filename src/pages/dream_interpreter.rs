use crate::components::DreamInterpreter;
use leptos::prelude::*;

// Import CSS classes directly to reference them in our HTML
const _: &str = include_str!("./dream_interpreter.module.css");

#[component]
pub fn DreamInterpreterPage() -> impl IntoView {
    view! {
        <div>
            <div class="pageHeader">
                <h1 class="pageTitle">"Dream Interpretation Portal"</h1>
                <p class="pageDescription">
                    "Our AI-powered dream interpreter helps you uncover the hidden meanings in your dreams.
                    Dreams reflect our subconscious thoughts and emotions."
                </p>
            </div>

            <div class="mainContent">
                <DreamInterpreter />
            </div>
        </div>
    }
}
