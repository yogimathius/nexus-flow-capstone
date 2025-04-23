use crate::components::dream_interpreter::DreamInterpreter;
use leptos::prelude::*;

// Import CSS classes directly to reference them in our HTML
use std::path::Path;
const _: &str = include_str!("dream_interpreter.module.css");

#[component]
pub fn DreamInterpreterPage() -> impl IntoView {
    view! {
        <div>
            <div class="pageHeader">
                <h1 class="pageTitle">"Dream Interpretation Portal"</h1>
                <p class="pageDescription">
                    "Our AI-powered dream interpreter helps you uncover the hidden meanings in your dreams.
                    Dreams often reflect our subconscious thoughts, emotions, and experiences. 
                    By understanding your dreams, you can gain valuable insights into your inner world."
                </p>
            </div>

            <div class="mainContent">
                <DreamInterpreter />

                <div class="tipsList">
                    <h3 class="tipTitle">"Tips for Better Dream Interpretation"</h3>
                    <ul class="tips">
                        <li>"Record your dream as soon as you wake up, before the details fade"</li>
                        <li>
                            "Include emotions, colors, people, and objects - everything you can remember"
                        </li>
                        <li>"Note recurring patterns or symbols that appear in multiple dreams"</li>
                        <li>"Consider what was happening in your life before the dream"</li>
                        <li>
                            "Try to describe the dream in the present tense for more vivid recall"
                        </li>
                    </ul>
                </div>

                <div class="footer">
                    <p>
                        "Dream interpretation is both an art and a science. Results may vary based on personal experiences."
                    </p>
                    <p>
                        "For persistent disturbing dreams, consider consulting with a mental health professional."
                    </p>
                </div>
            </div>
        </div>
    }
}
