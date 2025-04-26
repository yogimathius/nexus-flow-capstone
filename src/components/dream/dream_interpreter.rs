use crate::components::{
    badges::{Badge, BadgeVariant},
    buttons::{Button, ButtonVariant},
    typography::{SectionTitle, SubTitle},
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

// Dream interpretation tips
const DREAM_TIPS: &[&str] = &[
    "Record your dreams immediately upon waking for the most accurate recall.",
    "Keep a dream journal by your bed to capture details before they fade.",
    "Pay attention to emotions felt during the dream - they often hold the key to interpretation.",
    "Look for recurring symbols or themes across multiple dreams.",
    "Consider the context of your waking life when interpreting dream symbols.",
    "Dreams often process unresolved issues from your day or deeper subconscious concerns.",
    "Practice reality checks during the day to increase dream awareness and recall.",
    "Some dreams may be metaphorical rather than literal - look for deeper meanings.",
];

// Common dream themes
const COMMON_DREAM_THEMES: &[&str] = &[
    "Flying",
    "Falling",
    "Being chased",
    "Teeth falling out",
    "Being unprepared",
    "Finding a new room",
    "Water",
    "Death or dying",
    "Being trapped",
    "Snakes",
    "Money",
    "Celebrity",
    "School or work",
    "Naked in public",
    "Unable to move",
    "Lost",
    "Reunited",
    "Time travel",
    "Animals",
    "Childhood home",
    "Fire", // Added fire to the list of common themes
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DreamRequest {
    dream_text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DreamResponse {
    interpretation: String,
    status: String,
}

#[component]
pub fn DreamInterpreter() -> impl IntoView {
    // Signal for dream text input
    let (dream_text, set_dream_text) = signal(String::new());

    // Signal for interpretation output
    let (interpretation, set_interpretation) = signal(Option::<String>::None);

    // Signal for loading state - initialized to false
    let (is_loading, set_is_loading) = signal(false);

    // Signal for error messages
    let (error, set_error) = signal(Option::<String>::None);

    // Signal for the button's disabled state
    let (button_disabled, set_button_disabled) = signal(true);

    // API endpoint
    let api_url = "http://localhost:3003/api/interpret_dream";

    // Update the disabled state whenever the dream text changes
    Effect::new(move |_| {
        let is_empty = dream_text.get().trim().is_empty();
        set_button_disabled.set(is_empty || is_loading.get());
    });

    // Handle badge click
    let handle_badge_click = move |theme: &str| {
        let current = dream_text.get();
        if !current.is_empty() && !current.ends_with(' ') {
            set_dream_text.update(|s| s.push(' '));
        }

        let theme_with_space = format!("{} ", theme);
        set_dream_text.update(|s| s.push_str(&theme_with_space));

        // Explicitly enable the button when a dream theme is added
        if dream_text.get().trim().len() > 0 {
            set_button_disabled.set(false);
        }
    };

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        // Reset previous interpretation and error
        set_interpretation.set(None);
        set_error.set(None);

        let dream = dream_text.get();

        // Validate input
        if dream.trim().is_empty() {
            set_error.set(Some("Please enter your dream description".to_string()));
            return;
        }

        // Set loading state to true
        set_is_loading.set(true);

        // Create the request payload
        let request_payload = DreamRequest {
            dream_text: dream.clone(),
        };

        // Use reqwest to make the API call
        spawn_local(async move {
            let client = Client::new();
            
            match client
                .post(api_url)
                .header("Content-Type", "application/json")
                .json(&request_payload)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<DreamResponse>().await {
                            Ok(data) => {
                                set_interpretation.set(Some(data.interpretation));
                            }
                            Err(e) => {
                                set_error.set(Some(format!("Failed to parse response: {}", e)));
                            }
                        }
                    } else {
                        let status = response.status();
                        match response.text().await {
                            Ok(text) => {
                                set_error.set(Some(format!("Server error ({}): {}", status, text)));
                            }
                            Err(_) => {
                                set_error
                                    .set(Some(format!("Server error: Status code {}", status)));
                            }
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!(
                        "Failed to connect to the interpretation service. Please try again later. Error: {}",
                        e
                    )));
                }
            }

            // Set loading state back to false
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="dream-interpreter">
            <SectionTitle text="Dream Interpreter" />
            <p class="intro-text">
                "Enter your dream description below and our AI-powered interpreter will analyze
                the patterns and symbols to provide insights into your subconscious mind. 
                For better results, include key details and emotions you felt during the dream."
            </p>

            <div class="dream-badges">
                <span class="badge-label">"Common Dream Themes:"</span>
                {COMMON_DREAM_THEMES
                    .iter()
                    .map(|&theme| {
                        let theme_clone = theme.to_string();
                        view! {
                            <Badge
                                on_click=Box::new(move || handle_badge_click(&theme_clone))
                                variant=BadgeVariant::Default
                                class="dream-badge".to_string()
                            >
                                {theme}
                            </Badge>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <form on:submit=handle_submit>
                <div class="form-group">
                    <label for="dream-text">"Describe your dream in detail:"</label>
                    <textarea
                        id="dream-text"
                        class="textarea"
                        placeholder="I dreamt that I was flying over mountains and oceans..."
                        prop:value=dream_text
                        on:input=move |ev| {
                            set_dream_text.set(event_target_value(&ev));
                        }
                        rows=6
                    ></textarea>
                </div>

                {move || {
                    error
                        .get()
                        .map(|err_msg| {
                            view! { <div class="error-message">{err_msg}</div> }
                        })
                }}

                <Button
                    class="submit-btn".to_string()
                    // disabled=button_disabled.get()
                    variant=ButtonVariant::Primary
                >
                    {move || {
                        if is_loading.get() { "Interpreting..." } else { "Interpret My Dream" }
                    }}
                </Button>
            </form>

            {move || {
                interpretation
                    .get()
                    .map(|interp| {
                        view! {
                            <div class="interpretation">
                                <SubTitle text="Your Dream Interpretation" />
                                <p class="interpretation-text">{interp}</p>
                            </div>
                        }
                    })
            }}
        </div>

        <div class="tips-section">
            <h3 class="tips-title">"Tips for Better Dream Recall & Interpretation"</h3>
            <ul class="tips-list">
                {DREAM_TIPS
                    .iter()
                    .map(|&tip| {
                        view! { <li class="tip-item">{tip}</li> }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
