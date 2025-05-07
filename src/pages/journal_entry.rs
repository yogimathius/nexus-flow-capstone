use crate::components::markdown::{DreamText, InterpretationText};
use crate::components::typography::{SectionTitle, SubTitle, Title};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

// Dream model (same as in other pages)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String,
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[component]
pub fn JournalEntryPage() -> impl IntoView {
    // Get dream ID from route params
    let params = use_params_map();
    let dream_id = move || {
        params.with(|params| {
            params
                .get("entry")
                .map(|s| s.to_string())
                .unwrap_or_default()
        })
    };

    // State
    let (dream, set_dream) = signal(Option::<Dream>::None);
    let (is_loading, set_is_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Format date from ISO string to readable format
    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y at %H:%M").to_string()
        } else {
            date_str.to_string()
        }
    };

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

    // Load dream when the component mounts
    Effect::new(move |_| {
        let id = dream_id();
        if !id.is_empty() {
            load_dream(id);
        }
    });

    view! {
        <div class="fade-in">
            <section class="section-sm bg-gradient">
                <div class="container">
                    <div class="stack items-center text-center">
                        <h1 class="slide-up">"Dream Journal Entry"</h1>
                        <p class="slide-up animation-delay-1">
                            "Explore the interpretation of your dream in detail."
                        </p>
                    </div>
                </div>
            </section>

            <section class="section">
                <div class="container">
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
                                                        <div class="card text-center slide-up">
                                                            <p class="text-error mb-4">"Something went wrong. Please try again."</p>
                                                            <a href="/journal" class="button">
                                                                "Back to Journal"
                                                            </a>
                                                        </div>
                                                    }
                                                }
                                            >
                                                {move || {
                                                    let dream_data = dream.get().unwrap();
                                                    let date = format_date(&dream_data.created_at);

                                                    view! {
                                                        <div class="stack gap-6">
                                                            <div class="flex items-center justify-between slide-up">
                                                                <a href="/journal" class="button-outline">
                                                                    <svg
                                                                        xmlns="http://www.w3.org/2000/svg"
                                                                        width="24"
                                                                        height="24"
                                                                        viewBox="0 0 24 24"
                                                                        fill="none"
                                                                        stroke="currentColor"
                                                                        stroke-width="2"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    >
                                                                        <path d="M19 12H5"></path>
                                                                        <path d="M12 19l-7-7 7-7"></path>
                                                                    </svg>
                                                                    "Back to Journal"
                                                                </a>
                                                                <span class="text-secondary">{date}</span>
                                                            </div>

                                                            <div class="card slide-up animation-delay-1">
                                                                <h2 class="mb-4">"Dream"</h2>
                                                                <DreamText content={dream_data.text.clone()} />
                                                            </div>

                                                            <div class="card slide-up animation-delay-2">
                                                                <h2 class="mb-4">"Interpretation"</h2>
                                                                <InterpretationText content={dream_data.interpretation.clone()} />
                                                            </div>

                                                            <div class="flex gap-4 justify-center slide-up animation-delay-3">
                                                                <a href="/interpret" class="button">
                                                                    "Interpret Another Dream"
                                                                </a>
                                                                <a href="/journal" class="button-outline">
                                                                    "Return to Journal"
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
                                            <div class="card text-center slide-up">
                                                <h2 class="text-error mb-4">"Error"</h2>
                                                <p class="mb-4">{err}</p>
                                                <a href="/journal" class="button">
                                                    "Back to Journal"
                                                </a>
                                            </div>
                                        }
                                    }}
                                </Show>
                            }
                        }
                    >
                        <div class="loading-container">
                            <div class="loading"></div>
                            <p class="loading-text pulse">"Loading dream details..."</p>
                        </div>
                    </Show>
                </div>
            </section>
        </div>
    }
}
