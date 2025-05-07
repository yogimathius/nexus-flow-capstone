use leptos::prelude::*;
use leptos_router::components::A;

/// Home Page with Features and Testimonials
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="fade-in">
            <section class="section bg-gradient">
                <div class="container">
                    <div class="stack items-center text-center">
                        <h1 class="slide-up">"Dream Interpreter AI"</h1>
                        <p class="text-secondary mb-4 slide-up animation-delay-1">
                            "Unlock the secrets of your subconscious mind with AI-powered dream analysis. Discover meaning behind your dreams and gain insights into your inner world."
                        </p>
                        <div class="slide-up animation-delay-2">
                            <div class="button">
                                <A href="/interpret">
                                    "Interpret Your Dream"
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
                            <div class="text-4xl mb-4 float animated-emoji">"🔮"</div>
                            <h3>"AI-Powered Analysis"</h3>
                            <p>
                                "Our advanced AI model interprets dream symbols and patterns,
                                providing personalized insights based on psychological research
                                and cultural symbolism."
                            </p>
                        </div>

                        <div class="card slide-up animation-delay-1">
                            <div class="text-4xl mb-4 float animated-emoji">"📚"</div>
                            <h3>"Symbol Library"</h3>
                            <p>
                                "Access our extensive library of dream symbols and their meanings,
                                categorized and searchable to help you understand recurring themes
                                in your dreams."
                            </p>
                        </div>

                        <div class="card slide-up animation-delay-2">
                            <div class="text-4xl mb-4 float animated-emoji">"🔒"</div>
                            <h3>"Private & Secure"</h3>
                            <p>
                                "Your dream data stays private with client-side processing.
                                No dreams are stored or shared, ensuring your innermost thoughts
                                remain confidential."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="section-sm bg-card">
                <div class="container">
                    <h2 class="text-center mb-4 slide-up">"Popular Dream Symbols"</h2>
                    <div class="grid grid-3">{render_symbol_cards()}</div>
                </div>
            </section>

            <section class="section">
                <div class="container">
                    <h2 class="text-center mb-4 slide-up">"What Dreamers Say"</h2>
                    <div class="grid grid-3">
                        <div class="card slide-up">
                            <p class="mb-4">
                                "This app helped me understand recurring dream patterns that were causing anxiety.
                                The interpretations were surprisingly accurate and insightful!"
                            </p>
                            <div class="flex items-center gap-4">
                                <div class="badge person">"JD"</div>
                                <div class="stack gap-2">
                                    <span class="font-bold">"Jamie D."</span>
                                    <span class="text-secondary">"Dream Enthusiast"</span>
                                </div>
                            </div>
                        </div>

                        <div class="card slide-up animation-delay-1">
                            <p class="mb-4">
                                "I've been journaling my dreams for years, but this AI interpretation adds a
                                new dimension to my understanding. Love the symbol library feature!"
                            </p>
                            <div class="flex items-center gap-4">
                                <div class="badge dream">"SL"</div>
                                <div class="stack gap-2">
                                    <span class="font-bold">"Sarah L."</span>
                                    <span class="text-secondary">"Sleep Researcher"</span>
                                </div>
                            </div>
                        </div>

                        <div class="card slide-up animation-delay-2">
                            <p class="mb-4">
                                "The connections the AI makes between different symbols in my dreams
                                have been eye-opening. It's like having a therapist in my pocket!"
                            </p>
                            <div class="flex items-center gap-4">
                                <div class="badge emotion">"MT"</div>
                                <div class="stack gap-2">
                                    <span class="font-bold">"Mark T."</span>
                                    <span class="text-secondary">"Psychology Student"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

/// Render mock symbol cards
fn render_symbol_cards() -> impl IntoView {
    let symbols = vec![
        (
            "Water",
            "nature",
            "Represents emotions, the unconscious mind, and transitions in life.",
        ),
        (
            "Flying",
            "emotion",
            "Often symbolizes freedom, perspective, or escape from limitations.",
        ),
        (
            "Snake",
            "animal",
            "Can symbolize transformation, hidden fears, or healing depending on context.",
        ),
        (
            "Door",
            "object",
            "Represents new opportunities, transitions, or barriers in your life path.",
        ),
        (
            "Child",
            "person",
            "May represent your inner child, innocence, or new beginnings.",
        ),
        (
            "Falling",
            "emotion",
            "Often related to insecurities, loss of control, or letting go of something.",
        ),
    ];

    symbols
        .into_iter()
        .enumerate()
        .map(|(index, (name, category, description))| {
            let badge_class = format!("badge {}", category);
            let delay_class = match index % 3 {
                0 => "animation-delay-1",
                1 => "animation-delay-2",
                _ => "animation-delay-3",
            };
            let card_class = format!("card slide-up {}", delay_class);

            view! {
                <div class={card_class}>
                    <h3 class="mb-2">{name}</h3>
                    <span class={badge_class}>{category}</span>
                    <p class="mt-4">{description}</p>
                </div>
            }
        })
        .collect_view()
}
