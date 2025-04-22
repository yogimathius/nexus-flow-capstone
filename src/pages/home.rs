use leptos::prelude::*;
use leptos_router::components::A;

/// Home Page with Features and Testimonials
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <section class="hero">
                <div class="heroGradient"></div>
                <div class="container">
                    <h1 class="heroTitle">"Dream Interpreter AI"</h1>
                    <p class="heroTagline">
                        "Unlock the secrets of your subconscious mind with AI-powered dream analysis. Discover meaning behind your dreams and gain insights into your inner world."
                    </p>
                    <A href="/interpret" attr:class="ctaButton">
                        "Interpret Your Dream"
                    </A>
                </div>
            </section>

            <section class="container">
                <div class="features">
                    <div class="feature">
                        <div class="featureIcon">"🔮"</div>
                        <h3 class="featureTitle">"AI-Powered Analysis"</h3>
                        <p class="featureDescription">
                            "Our advanced AI model interprets dream symbols and patterns,
                            providing personalized insights based on psychological research
                            and cultural symbolism."
                        </p>
                    </div>

                    <div class="feature">
                        <div class="featureIcon">"📚"</div>
                        <h3 class="featureTitle">"Symbol Library"</h3>
                        <p class="featureDescription">
                            "Access our extensive library of dream symbols and their meanings,
                            categorized and searchable to help you understand recurring themes
                            in your dreams."
                        </p>
                    </div>

                    <div class="feature">
                        <div class="featureIcon">"🔒"</div>
                        <h3 class="featureTitle">"Private & Secure"</h3>
                        <p class="featureDescription">
                            "Your dream data stays private with client-side processing.
                            No dreams are stored or shared, ensuring your innermost thoughts
                            remain confidential."
                        </p>
                    </div>
                </div>
            </section>

            <section class="container symbolsSection">
                <h2 class="sectionTitle">"Popular Dream Symbols"</h2>
                <div class="symbols">{render_symbol_cards()}</div>
            </section>

            <section class="container testimonials">
                <h2 class="sectionTitle">"What Dreamers Say"</h2>
                <div class="testimonialGrid">
                    <div class="testimonial">
                        <p class="testimonialText">
                            "This app helped me understand recurring dream patterns that were causing anxiety.
                            The interpretations were surprisingly accurate and insightful!"
                        </p>
                        <div class="testimonialAuthor">
                            <div class="testimonialAvatar">"JD"</div>
                            <div class="authorInfo">
                                <span class="authorName">"Jamie D."</span>
                                <span class="authorTitle">"Dream Enthusiast"</span>
                            </div>
                        </div>
                    </div>

                    <div class="testimonial">
                        <p class="testimonialText">
                            "I've been journaling my dreams for years, but this AI interpretation adds a
                            new dimension to my understanding. Love the symbol library feature!"
                        </p>
                        <div class="testimonialAuthor">
                            <div class="testimonialAvatar">"SL"</div>
                            <div class="authorInfo">
                                <span class="authorName">"Sarah L."</span>
                                <span class="authorTitle">"Sleep Researcher"</span>
                            </div>
                        </div>
                    </div>

                    <div class="testimonial">
                        <p class="testimonialText">
                            "The connections the AI makes between different symbols in my dreams
                            have been eye-opening. It's like having a therapist in my pocket!"
                        </p>
                        <div class="testimonialAuthor">
                            <div class="testimonialAvatar">"MT"</div>
                            <div class="authorInfo">
                                <span class="authorName">"Mark T."</span>
                                <span class="authorTitle">"Psychology Student"</span>
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
        .map(|(name, category, description)| {
            view! {
                <div class="symbolCard">
                    <h3 class="symbolName">{name}</h3>
                    <span class=format!("symbolCategory {}", category)>{category}</span>
                    <p class="symbolDescription">{description}</p>
                </div>
            }
        })
        .collect_view()
}
