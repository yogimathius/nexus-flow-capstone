use crate::components::DreamInterpreter;
use leptos::prelude::*;

#[component]
pub fn DreamInterpreterPage() -> impl IntoView {
    view! {
        <div class="fade-in">
            <section class="section-sm bg-gradient">
                <div class="container">
                    <div class="stack items-center text-center">
                        <h1 class="slide-up">"Reflect on a dream"</h1>
                        <p class="slide-up animation-delay-1">
                    "Describe a dream and explore one possible reading. Interpretations are prompts for reflection, not diagnoses or fixed meanings. Your text is sent to the configured AI provider when you submit it."
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
