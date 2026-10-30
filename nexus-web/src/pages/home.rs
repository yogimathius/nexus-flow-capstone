use leptos::prelude::*;
use leptos_router::components::A;

/// Home page describing the current local experience.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="fade-in">
            <section class="section bg-gradient">
                <div class="container">
                    <div class="stack items-center text-center">
                        <h1 class="slide-up">"Nexus Flow"</h1>
                        <p class="text-secondary mb-4 slide-up animation-delay-1">
                        "A quiet space for dream reflection. Write what you remember, consider a possible interpretation, and keep a journal only when you choose to."
                    </p>
                        <div class="slide-up animation-delay-2">
                            <div class="button">
                                <A href="/interpret">
                        "Reflect on a Dream"
                    </A>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="section-sm">
                <div class="container">
                    <div class="grid grid-3">
                        <div class="card slide-up">
                            <div class="text-4xl mb-4 float animated-emoji">"✍️"</div>
                            <h3>"Begin with your words"</h3>
                            <p>
                            "Describe a dream in your own words. You decide whether to request an interpretation and whether to save the result."
                        </p>
                    </div>

                        <div class="card slide-up animation-delay-1">
                            <div class="text-4xl mb-4 float animated-emoji">"🧭"</div>
                            <h3>"Consider a reading"</h3>
                            <p>
                            "The Rust API can bring symbol context from the separate Symbols Awakening MCP into an AI prompt. Meanings remain open to your interpretation."
                        </p>
                    </div>

                        <div class="card slide-up animation-delay-2">
                            <div class="text-4xl mb-4 float animated-emoji">"🔒"</div>
                            <h3>"Know where your words go"</h3>
                            <p>
                            "When an AI provider is configured, requesting an interpretation sends your text to it. If you choose to save, the journal stores it in PostgreSQL and caches fetched entries in this browser."
                        </p>
                        </div>
                    </div>
                </div>
            </section>

        </div>
    }
}
