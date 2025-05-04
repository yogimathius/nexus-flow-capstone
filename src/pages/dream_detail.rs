use crate::components::typography::{SectionTitle, SubTitle};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

// Import CSS module
const _: &str = include_str!("./dream_detail.module.css");

// Dream model from API (same as in dream_journal.rs)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String, // Keep as String for frontend to handle UUID
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String, // ISO 8601 date string
}

#[component]
pub fn DreamDetailPage() -> impl IntoView {
    // Get dream ID from route params
    let params = use_params_map();
    let dream_id =
        move || params.with(|params| params.get("id").map(|s| s.to_string()).unwrap_or_default());

    // State
    let (dream, set_dream) = signal(Option::<Dream>::None);
    let (is_loading, set_is_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Load dream by ID
    let load_dream = move |id: String| {
        if id.is_empty() {
            set_error.set(Some("Invalid dream ID".to_string()));
            set_is_loading.set(false);
            return;
        }

        set_is_loading.set(true);
        set_error.set(None);

        let api_url = format!("http://localhost:3003/api/dreams/{}", id);

        spawn_local(async move {
            let client = Client::new();
            match client.get(&api_url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<Dream>().await {
                            Ok(data) => {
                                set_dream.set(Some(data));
                            }
                            Err(e) => {
                                set_error.set(Some(format!("Failed to parse response: {}", e)));
                            }
                        }
                    } else if response.status().as_u16() == 404 {
                        set_error.set(Some("Dream not found".to_string()));
                    } else {
                        set_error.set(Some(format!(
                            "Server error: {}",
                            response.status().as_str()
                        )));
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to connect to the server: {}", e)));
                }
            }
            set_is_loading.set(false);
        });
    };

    // Format date from ISO string to readable format
    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y at %H:%M").to_string()
        } else {
            date_str.to_string()
        }
    };

    // Format interpretation for HTML display
    let format_interpretation = |text: &str| -> String {
        // Similar to the formatting in dream_interpreter.rs
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

        let text = text
            .replace("###", "<h3 class=\"interp-heading\">")
            .replace("####", "<h4 class=\"interp-subheading\">")
            .replace("\n\n", "<br/><br/>")
            .replace("\n---\n", "<hr class=\"interp-divider\"/>");

        let text = text.replace("**", "<strong>").replace("*", "<em>");
        text.replace("\n", "<br/>")
    };

    // Load dream on component mount using the ID from the route
    let _ = Effect::new(move |_| {
        let id = dream_id();
        if !id.is_empty() {
            load_dream(id);
        }
    });

    view! {
        <div class="dream-detail">
            <div class="dream-content">
                <Show
                    when=move || is_loading.get()
                    fallback=move || {
                        view! {
                            <Show
                                when=move || error.get().is_some()
                                fallback=move || {
                                    view! {
                                        <Show
                                            when=move || dream.get().is_some()
                                            fallback=move || {
                                                view! {
                                                    <div class="error-wrapper">
                                                        <div class="error-container">
                                                            <p class="error-message">"Something went wrong. Please try again."</p>
                                                            <a href="/journal" class="back-link">"Back to Dream Journal"</a>
                                                        </div>
                                                    </div>
                                                }
                                            }
                                        >
                                            {move || {
                                                let dream_data = dream.get().unwrap();
                                                let date = format_date(&dream_data.created_at);
                                                let interpretation_html = format_interpretation(&dream_data.interpretation);

                                                view! {
                                                    <div class="dream-details">
                                                        <div class="dream-header">
                                                            <a href="/journal" class="back-link">"← Back to Journal"</a>
                                                            <div class="dream-date">{date}</div>
                                                        </div>

                                                        <SectionTitle text="Dream" />
                                                        <div class="dream-text">
                                                            {dream_data.text}
                                                        </div>

                                                        <SubTitle text="Interpretation" />
                                                        <div class="interpretation-text" inner_html=interpretation_html></div>

                                                        <div class="actions">
                                                            <a href="/interpret" class="action-button">
                                                                "Interpret Another Dream"
                                                            </a>
                                                        </div>
                                                    </div>
                                                }
                                            }}
                                        </Show>
                                    }
                                }
                            >
                                {move || {
                                    let err = error.get().unwrap();
                                    view! {
                                        <div class="error-wrapper">
                                            <div class="error-container">
                                                <h2 class="error-title">"Error"</h2>
                                                <p class="error-message">{err}</p>
                                                <a href="/journal" class="back-link">"Back to Dream Journal"</a>
                                            </div>
                                        </div>
                                    }
                                }}
                            </Show>
                        }
                    }
                >
                    <div class="loading-container">
                        <div class="loading">
                            <div></div>
                            <div></div>
                            <div></div>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
