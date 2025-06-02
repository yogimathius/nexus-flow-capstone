use crate::components::DreamInterpreter;
use leptos::prelude::*;

#[component]
pub fn DreamInterpreterPage() -> impl IntoView {
    view! {
        <div class="fade-in">
            <section class="section-sm bg-gradient">
                <div class="container">
                    <div class="stack items-center text-center">
                        <h1 class="slide-up">"Dream Interpretation Portal"</h1>
                        <p class="slide-up animation-delay-1">
                    "Our AI-powered dream interpreter helps you uncover the hidden meanings in your dreams.
                            Share your dream and receive insights into your subconscious mind."
                </p>
            </div>
                </div>
            </section>

            <section class="section">
                <div class="container">
                <DreamInterpreter />
            </div>
            </section>
        </div>
    }
}
