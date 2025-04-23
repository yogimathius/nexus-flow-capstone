use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use log::{error, info};

// Import our component libraries
use crate::components::typography::{SubTitle, SectionTitle};
use crate::components::button_fixed::SubmitButton;

#[derive(Deserialize, Clone, Debug)]
struct DreamResponse {
    interpretation: String,
    status: String,
}

#[derive(Serialize, Debug)]
struct DreamRequest {
    dream_text: String,
}

#[component]
pub fn DreamInterpreter() -> impl IntoView {
    let (dream_text, set_dream_text) = signal(String::new());
    let (interpretation, set_interpretation) = signal(None::<String>);
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (use_api, set_use_api) = signal(true); // Flag to control whether to use API or mock
    
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
    
    // Fallback function for mock interpretation
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

    let handle_api_toggle = move |_| {
        set_use_api.update(|val| *val = !*val);
    };

    async fn fetch_interpretation(dream_text: String) -> Result<DreamResponse, String> {
        let client = reqwest::Client::new();
        let request_body = DreamRequest { dream_text };
        
        // Try different server URLs based on what might be available
        let api_urls = [
            "http://0.0.0.0:3003/api/interpret_dream",
            "http://localhost:3003/api/interpret_dream",
            "http://127.0.0.1:3003/api/interpret_dream"
        ];
        
        let mut last_error = String::from("Failed to connect to any API endpoints");
        
        // Try each URL in sequence
        for url in api_urls {
            info!("Trying API endpoint: {}", url);
            
            match client
                .post(url)
                .json(&request_body)
                .header("Origin", "https://localhost:3000") // Spoof the origin to match what the server expects
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<DreamResponse>().await {
                            Ok(dream_data) => return Ok(dream_data),
                            Err(e) => {
                                let error_msg = format!("Failed to parse response: {}", e);
                                info!("{}", error_msg);
                                last_error = error_msg;
                            }
                        }
                    } else {
                        last_error = format!("API returned error status: {}", response.status());
                        info!("{}", last_error);
                    }
                },
                Err(e) => {
                    last_error = format!("Failed to connect to API at {}: {}", url, e);
                    info!("{}", last_error);
                }
            }
        }
        
        // If we got here, all attempts failed
        Err(last_error)
    }

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
        let should_use_api = use_api.get();

        // Clones for use inside the async block
        let set_interpretation_clone = set_interpretation;
        let set_is_loading_clone = set_is_loading;
        let set_error_clone = set_error;
        
        spawn_local(async move {
            // Add a slight delay for better UX
            TimeoutFuture::new(800).await;
            
            if should_use_api {
                // Use the API
                info!("Using API for interpretation");
                match fetch_interpretation(current_dream_text.clone()).await {
                    Ok(response) => {
                        if response.status == "success" {
                            set_interpretation_clone(Some(response.interpretation));
                        } else {
                            set_error_clone(Some(format!("API error: {}", response.status)));
                            
                            // Fall back to mock mode after a brief delay
                            TimeoutFuture::new(1500).await;
                            info!("Falling back to mock data after API error");
                            let fallback = generate_mock_interpretation(&current_dream_text);
                            set_error_clone(None);
                            set_interpretation_clone(Some(format!("(Fallback interpretation) {}", fallback)));
                        }
                    },
                    Err(err) => {
                        error!("API error: {}", err);
                        set_error_clone(Some(format!("API connection error: {}", err)));
                        
                        // Fall back to mock after showing the error message for a moment
                        TimeoutFuture::new(1500).await;
                        info!("Falling back to mock data after API error");
                        let fallback = generate_mock_interpretation(&current_dream_text);
                        set_error_clone(None);
                        set_interpretation_clone(Some(format!("(Fallback interpretation) {}", fallback)));
                    }
                }
            } else {
                // Use the mock generator
                info!("Using mock interpretation");
                let mock_interpretation = generate_mock_interpretation(&current_dream_text);
                set_interpretation_clone(Some(mock_interpretation));
            }
            
            set_is_loading_clone(false);
        });
    };

    view! {
        <div class="dream-interpreter">
            <SubTitle text="Dream Interpreter" />

            <p>
                "Uncover the hidden meanings in your dreams with our AI-powered interpreter. Describe your dream in detail for the most insightful analysis."
            </p>

            <div class="api-toggle">
                <label class="toggle-label">
                    <input type="checkbox" checked=use_api on:change=handle_api_toggle />
                    <span>"Use API for interpretation"</span>
                </label>
                <span class="api-status">
                    {move || {
                        if use_api() { "Using external AI API" } else { "Using local mock data" }
                    }}
                </span>
            </div>

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
                    .map(|err_text| {
                        view! {
                            <div class="error-message">
                                <p>{err_text}</p>
                                <small>
                                    {if err_text.contains("API") {
                                        "Falling back to local interpretation in a moment..."
                                    } else {
                                        "Please try again later."
                                    }}
                                </small>
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
