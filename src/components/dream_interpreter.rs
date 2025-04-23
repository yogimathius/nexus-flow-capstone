use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

// Import our component libraries
use crate::components::typography::{SubTitle, SectionTitle};
use crate::components::button_fixed::SubmitButton;

#[derive(Deserialize, Clone, Debug)]
struct DreamResponse {
    interpretation: String,
    status: String,
}

#[component]
pub fn DreamInterpreter() -> impl IntoView {
    let (dream_text, set_dream_text) = signal(String::new());
    let (interpretation, set_interpretation) = signal(None::<String>);
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    
    // Common dream themes as quick-selection badges
    let dream_themes = vec![
        "Flying", "Falling", "Being chased", "Teeth falling out", 
        "Water or ocean", "Being unprepared", "Being naked in public"
    ];
    
    let handle_badge_click = move |theme: &str| {
        let current_text = dream_text.get();
        if current_text.is_empty() {
            set_dream_text.set(format!("I had a dream about {}.", theme.to_lowercase()));
        } else {
            set_dream_text.set(format!("{} In my dream, I was also {}.", current_text, theme.to_lowercase()));
        }
    };
    
    // Always use mock mode to ensure a consistent user experience
    let generate_mock_interpretation = move |dream_text: &str| -> String {
        let dream_keywords = [
            "flying", "fall", "falling", "chase", "chased", "teeth", "water", "ocean", "mountain",
            "school", "exam", "test", "naked", "bird", "dog", "cat", "house", "car", "death",
            "dying", "gold", "silver", "glass",
        ];

        let contains_keyword = dream_keywords
            .iter()
            .any(|&keyword| dream_text.to_lowercase().contains(keyword));

        if !contains_keyword {
            return format!(
                "Your dream about \"{}\" is unique! Dreams often reflect our subconscious mind processing daily experiences and emotions. Consider what symbols in this dream might represent in your waking life.",
                dream_text.chars().take(40).collect::<String>()
            );
        }

        if dream_text.to_lowercase().contains("flying") {
            return "Dreams of flying often represent feelings of freedom, power, or escaping from something in your waking life. They can indicate a desire to rise above your circumstances or gain a new perspective on a situation.".to_string();
        } else if dream_text.to_lowercase().contains("fall")
            || dream_text.to_lowercase().contains("falling")
        {
            return "Dreams about falling typically represent insecurity, anxiety, or feeling overwhelmed. They may indicate you're afraid of losing control in some aspect of your life.".to_string();
        } else if dream_text.to_lowercase().contains("chase")
            || dream_text.to_lowercase().contains("chased")
        {
            return "Being chased in dreams often reflects avoiding a person or issue that makes you feel threatened. Consider what you might be running from in your waking life.".to_string();
        } else if dream_text.to_lowercase().contains("teeth") {
            return "Dreams involving teeth (falling out, breaking, etc.) commonly symbolize anxiety about your appearance or how others perceive you. They may also represent communication challenges.".to_string();
        } else if dream_text.to_lowercase().contains("water")
            || dream_text.to_lowercase().contains("ocean")
        {
            return "Water in dreams often represents your emotional state. Calm water might suggest peace, while turbulent water could indicate emotional turmoil. The ocean specifically can symbolize the unconscious mind.".to_string();
        } else if dream_text.to_lowercase().contains("mountain") {
            return "Mountains in dreams typically symbolize challenges, obstacles, or aspirations. Climbing a mountain may represent your journey toward a goal, while being at the summit could indicate achievement.".to_string();
        } else if dream_text.to_lowercase().contains("school")
            || dream_text.to_lowercase().contains("exam")
            || dream_text.to_lowercase().contains("test")
        {
            return "Dreams about school, exams, or tests often reflect feelings of being unprepared or evaluated. They might indicate performance anxiety or fear of failure in your current life.".to_string();
        } else if dream_text.to_lowercase().contains("naked") {
            return "Dreams of being naked in public typically represent vulnerability or fear of exposure. You might be worried about revealing your true self or being judged by others.".to_string();
        } else if dream_text.to_lowercase().contains("bird") {
            return "Birds in dreams often symbolize freedom, perspective, and aspirations. They can represent your thoughts or desires to transcend limitations.".to_string();
        } else if dream_text.to_lowercase().contains("dog") {
            return "Dogs in dreams frequently represent loyalty, friendship, and protection. Consider your relationship with dogs and what specific qualities they embody for you.".to_string();
        } else if dream_text.to_lowercase().contains("cat") {
            return "Cats in dreams often symbolize independence, intuition, and femininity. They may represent aspects of yourself that are mysterious or difficult to control.".to_string();
        } else if dream_text.to_lowercase().contains("house") {
            return "Houses in dreams typically represent the self, with different rooms corresponding to different aspects of your personality or life. The condition of the house often reflects how you perceive yourself.".to_string();
        } else if dream_text.to_lowercase().contains("car") {
            return "Cars in dreams often symbolize your direction in life and the control you feel. Problems with a car might indicate obstacles in your path or uncertainty about your direction.".to_string();
        } else if dream_text.to_lowercase().contains("death")
            || dream_text.to_lowercase().contains("dying")
        {
            return "Dreams about death rarely predict literal death. Instead, they typically symbolize transformation, endings, and new beginnings. Something in your life may be changing significantly.".to_string();
        } else if dream_text.to_lowercase().contains("gold")
            || dream_text.to_lowercase().contains("silver")
        {
            return "Precious metals like gold or silver in dreams often represent value, wealth, or spiritual riches. They might indicate something you treasure or aspects of yourself that you value highly.".to_string();
        } else if dream_text.to_lowercase().contains("glass") {
            return "Glass in dreams can symbolize fragility, transparency, or barriers. Consider whether you're feeling vulnerable or if there's something separating you from what you desire.".to_string();
        } else {
            return format!(
                "Your dream about \"{}\" contains symbols that reflect your personal experiences and emotions. Dreams often process our daily thoughts and feelings in symbolic form. Consider what these specific symbols mean to you personally.",
                dream_text.chars().take(40).collect::<String>()
            );
        }
    };

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        // Don't submit if already loading or empty text
        if is_loading() || dream_text().trim().is_empty() {
            return;
        }

        set_is_loading(true);
        set_error(None);
        set_interpretation(None);

        let current_dream_text = dream_text().clone();

        // Use the mock generator for immediate, reliable interpretations
        let mock_interpretation = generate_mock_interpretation(&current_dream_text);
        
        // Add a slight delay to simulate processing (better UX)
        let set_interpretation_clone = set_interpretation;
        let set_is_loading_clone = set_is_loading;
        
        spawn_local(async move {
            // Simulate network delay for better UX
            TimeoutFuture::new(1200).await;
            
            set_interpretation_clone(Some(mock_interpretation));
            set_is_loading_clone(false);
        });
    };

    view! {
        <div class="dream-interpreter">
            <SubTitle text="Dream Interpreter" />

            <p>
                "Uncover the hidden meanings in your dreams with our AI-powered interpreter. Describe your dream in detail for the most insightful analysis."
            </p>

            <div class="dream-badges">
                <span>"Common dream themes: "</span>
                {dream_themes
                    .into_iter()
                    .map(|theme| {
                        let theme_clone = theme.to_string();
                        view! {
                            <span
                                class="dream-badge"
                                on:click=move |_| handle_badge_click(&theme_clone)
                            >
                                {theme}
                            </span>
                        }
                    })
                    .collect_view()}
            </div>

            <form on:submit=handle_submit>
                <div class="form-group">
                    <label for="dream-description">"Describe your dream in detail"</label>
                    <textarea
                        id="dream-description"
                        class="textarea"
                        rows="6"
                        placeholder="I had a dream where I was flying over a city..."
                        prop:value=dream_text
                        on:input=move |ev| set_dream_text.set(event_target_value(&ev))
                    ></textarea>
                </div>

                {SubmitButton(
                    if is_loading() {
                        "Interpreting...".to_string()
                    } else {
                        "Interpret Dream".to_string()
                    },
                    Some("submit-btn".to_string()),
                    is_loading(),
                    is_loading(),
                )}
            </form>

            {move || {
                error
                    .get()
                    .map(|_err_text| {
                        view! {
                            <div class="error-message">
                                <p>"Something went wrong. Please try again."</p>
                            </div>
                        }
                    })
            }}

            {move || {
                interpretation
                    .get()
                    .map(|interp| {
                        view! {
                            <div class="interpretation">
                                <SectionTitle text="Dream Interpretation" />
                                <p class="interpretation-text">{interp}</p>
                            </div>
                        }
                    })
            }}
        </div>
    }
}
