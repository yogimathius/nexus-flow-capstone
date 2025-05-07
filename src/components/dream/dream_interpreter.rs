use crate::components::{
    badges::{Badge, BadgeVariant},
    buttons::{Button, ButtonVariant},
    typography::{SectionTitle, SubTitle},
};
use leptos::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

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
    save: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DreamResponse {
    interpretation: String,
    status: String,
    id: Option<String>,
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

    // Signal for the auto-save checkbox
    let (auto_save, set_auto_save) = signal(true);

    // Signal for the saved dream ID
    let (saved_dream_id, set_saved_dream_id) = signal(Option::<String>::None);

    // API endpoint
    let api_url = "http://localhost:3003/api/interpret_dream";

    // Update the disabled state whenever the dream text changes
    Effect::new(move |_| {
        let is_empty = dream_text.get().trim().is_empty();
        set_button_disabled.set(is_empty || is_loading.get());
    });

    // Handle badge click
    let handle_badge_click = move |theme: &str| {
        // Create sentence templates for each theme
        let sentence = match theme {
            "Flying" => "I was flying over a landscape and felt a sense of freedom.",
            "Falling" => "I experienced falling from a great height and felt scared.",
            "Being chased" => "Something or someone was chasing me and I couldn't escape.",
            "Teeth falling out" => "My teeth started to crumble and fall out unexpectedly.",
            "Being unprepared" => {
                "I found myself in a situation where I was completely unprepared."
            }
            "Finding a new room" => {
                "I discovered a new room in a familiar building that I never knew existed."
            }
            "Water" => "There was a large body of water that played an important role in my dream.",
            "Death or dying" => {
                "The dream involved death or dying in some way that felt significant."
            }
            "Being trapped" => "I felt trapped in a confined space with no way to escape.",
            "Snakes" => "There were snakes in my dream that seemed to have some special meaning.",
            "Money" => {
                "Money played an important role in my dream, affecting the events that unfolded."
            }
            "Celebrity" => "A celebrity appeared in my dream and interacted with me directly.",
            "School or work" => {
                "I was back at school or work, but things were different than in real life."
            }
            "Naked in public" => {
                "I realized I was naked or underdressed in a public place and felt embarrassed."
            }
            "Unable to move" => {
                "I tried to move but my body wouldn't respond, leaving me paralyzed."
            }
            "Lost" => "I was lost in an unfamiliar place and couldn't find my way back.",
            "Reunited" => "I was reunited with someone I haven't seen in a long time.",
            "Time travel" => "I traveled through time and experienced events from a different era.",
            "Animals" => "Animals played a significant role in my dream with unusual behaviors.",
            "Childhood home" => {
                "I was back in my childhood home, noticing details I had forgotten."
            }
            "Fire" => "There was a fire that both frightened and fascinated me in the dream.",
            _ => match theme.to_lowercase().as_str() {
                "dogs" | "dog" => {
                    "I saw a dog in my dream and it seemed to represent something important."
                }
                "cats" | "cat" => "A cat appeared in my dream and its behavior seemed symbolic.",
                "ocean" | "sea" => {
                    "I was near the ocean, and the waves seemed to reflect my emotions."
                }
                "mountains" | "mountain" => {
                    "I was in the mountains, feeling both small and powerful."
                }
                "family" | "parents" => "My family appeared in my dream in an unusual context.",
                "work" | "job" => {
                    "I was at work in my dream, but things were different than reality."
                }
                "baby" | "child" => {
                    "There was a baby or child in my dream that seemed significant."
                }
                _ => "My dream featured this theme in a way that felt meaningful.",
            },
        };

        // Get current text and prepare to add new content
        let current = dream_text.get();
        let new_text = if current.is_empty() {
            sentence.to_string()
        } else if current.trim_end().ends_with('.')
            || current.trim_end().ends_with('!')
            || current.trim_end().ends_with('?')
        {
            format!("{} {}", current, sentence)
        } else {
            format!("{}. {}", current, sentence)
        };

        // Set the new text directly
        set_dream_text.set(new_text);

        // Enable the button
        set_button_disabled.set(false);
    };

    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        // Reset previous interpretation, error and saved ID
        set_interpretation.set(None);
        set_error.set(None);
        set_saved_dream_id.set(None);

        let dream = dream_text.get();

        // Validate input
        if dream.trim().is_empty() {
            set_error.set(Some("Please enter your dream description".to_string()));
            return;
        }

        // Set loading state to true
        set_is_loading.set(true);

        // Create the request payload with save parameter
        let request_payload = DreamRequest {
            dream_text: dream.clone(),
            save: auto_save.get(),
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
                                // Process the interpretation to improve formatting
                                let formatted_interpretation =
                                    format_interpretation(&data.interpretation);
                                set_interpretation.set(Some(formatted_interpretation));

                                // Set the saved dream ID if available
                                if let Some(id) = data.id {
                                    set_saved_dream_id.set(Some(id));
                                }
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

    // Format the interpretation text to display better in HTML
    fn format_interpretation(text: &str) -> String {
        // Remove <think> blocks
        let text = if let Some(start_idx) = text.find("<think>") {
            if let Some(end_idx) = text.find("</think>") {
                let before = &text[0..start_idx];
                let after = &text[(end_idx + 8)..];
                format!("{}{}", before, after)
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };

        // Replace markdown headings with styled HTML
        let text = text
            .replace("###", "<h3 class=\"interp-heading\">")
            .replace("####", "<h4 class=\"interp-subheading\">")
            .replace("\n\n", "<br/><br/>")
            .replace("\n---\n", "<hr class=\"interp-divider\"/>");

        // Replace markdown bold/italic with HTML
        let text = text.replace("**", "<strong>").replace("*", "<em>");

        // Replace any remaining newlines with breaks
        text.replace("\n", "<br/>")
    }

    view! {
        <div class="stack gap-6 fade-in">
            <div class="card">
                <form on:submit=handle_submit class="stack gap-6">
                    <div class="stack gap-2">
                        <label for="dream-text" class="label">"Describe your dream"</label>
                        <textarea
                            id="dream-text"
                            class="textarea"
                            placeholder="Enter your dream description..."
                            prop:value=dream_text
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_dream_text.set(value);
                            }
                        />
                    </div>

                    <div class="flex items-center gap-2">
                        <input
                            type="checkbox"
                            id="auto-save"
                            checked=auto_save
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                set_auto_save.set(checked);
                            }
                        />
                        <label for="auto-save" class="label m-0">"Save to dream journal"</label>
                    </div>

                    <Button
                        variant=ButtonVariant::Primary
                        disabled=button_disabled.get()
                        class="full-width".to_string()
                    >
                        {move || if is_loading.get() { "Interpreting..." } else { "Interpret Dream" }}
                    </Button>

                    {move || error.get().map(|err| view! {
                        <div class="text-error">{err}</div>
                    })}
                </form>
            </div>

            <div class="card">
                <h3 class="card-header">"Common Dream Themes"</h3>
                <p class="text-secondary mb-4">
                    "Click on any theme to add a template to your dream description."
                </p>
                <div class="cluster">
                    {COMMON_DREAM_THEMES.iter().map(|&theme| {
                        let theme_str = theme.to_string();
                        let variant = match theme {
                            "Water" | "Fire" => BadgeVariant::Nature,
                            "Flying" | "Falling" | "Being chased" | "Being trapped" =>
                                BadgeVariant::Emotion,
                            "Snakes" | "Animals" => BadgeVariant::Animal,
                            "Teeth falling out" | "Money" | "Naked in public" | "Finding a new room" =>
                                BadgeVariant::Object,
                            "School or work" | "Celebrity" | "Childhood home" =>
                                BadgeVariant::Person,
                            _ => BadgeVariant::Dream
                        };
                        view! {
                            <Badge
                                variant=variant
                                on_click=Box::new(move || handle_badge_click(&theme_str))
                            >
                                {theme}
                            </Badge>
                        }
                    }).collect_view()}
                </div>
            </div>

            {move || interpretation.get().map(|text| view! {
                <div class="card fade-in">
                    <h3 class="card-header">"Dream Interpretation"</h3>
                    <div class="prose" inner_html=text></div>
                    {move || saved_dream_id.get().map(|id| view! {
                        <div class="card-footer">
                            <div class="flex items-center gap-2">
                                <Badge variant=BadgeVariant::Dream>"Saved"</Badge>
                                <span class="text-secondary">{"Dream saved to journal with ID: "}{id}</span>
                            </div>
                        </div>
                    })}
                </div>
            })}

            <div class="card">
                <h3 class="card-header">"Dream Interpretation Tips"</h3>
                <ul class="stack gap-2">
                    {DREAM_TIPS.iter().map(|tip| view! {
                        <li class="flex gap-2">
                            <span class="text-primary">{"•"}</span>
                            {tip.to_string()}
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}
