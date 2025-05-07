use crate::components::markdown::{DreamText, InterpretationText};
use crate::components::typography::{SectionTitle, SubTitle};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

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

    // Load dream on component mount using the ID from the route
    let _ = Effect::new(move |_| {
        let id = dream_id();
        if !id.is_empty() {
            load_dream(id);
        }
    });

    view! {
        <div class="container container-md py-8">
            <div class="stack gap-8">
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
                                                    <div class="card bg-card p-6">
                                                        <p class="text-error mb-4">"Something went wrong. Please try again."</p>
                                                        <a href="/journal" class="button button-outline">"Back to Dream Journal"</a>
                                                    </div>
                                                }
                                            }
                                        >
                                            {move || {
                                                let dream_data = dream.get().unwrap();
                                                let date = format_date(&dream_data.created_at);

                                                view! {
                                                    <div class="stack gap-6">
                                                        <div class="flex items-center justify-between">
                                                            <a href="/journal" class="flex items-center gap-2 text-secondary hover:text-primary">
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
                                                            <div class="text-secondary">{date}</div>
                                                        </div>

                                                        <div class="card bg-card p-6">
                                                            <SectionTitle text="Dream" />
                                                            <DreamText content={dream_data.text.clone()} />
                                                        </div>

                                                        <div class="card bg-card p-6">
                                                            <SubTitle text="Interpretation" />
                                                            <InterpretationText content={dream_data.interpretation.clone()} />
                                                        </div>

                                                        <div class="flex justify-center">
                                                            <a href="/interpret" class="button">
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
                                        <div class="card bg-card p-6">
                                            <h2 class="text-4xl mb-4 text-error">"Error"</h2>
                                            <p class="text-error mb-4">{err}</p>
                                            <a href="/journal" class="button button-outline">"Back to Dream Journal"</a>
                                        </div>
                                    }
                                }}
                            </Show>
                        }
                    }
                >
                    <div class="flex justify-center items-center min-h-[200px]">
                        <div class="loading"></div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
