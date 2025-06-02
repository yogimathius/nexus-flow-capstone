use crate::components::journal_entry::Dream;
use leptos::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

fn get_api_url() -> String {
    if let Some(api_url) = option_env!("API_URL") {
        return api_url.to_string();
    }
    
    "https://nexus-core.fly.dev/api".to_string()
}

#[derive(Clone)]
pub struct DreamState {
    pub dream: Signal<Option<Dream>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub format_date: fn(&str) -> String,
}

pub fn use_dream_data(dream_id: String) -> DreamState {
    let (dream, set_dream) = signal(Option::<Dream>::None);
    let (is_loading, set_is_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y at %H:%M").to_string()
        } else {
            date_str.to_string()
        }
    };

    if dream_id.is_empty() {
        set_error.set(Some("Invalid dream ID".to_string()));
        set_is_loading.set(false);
    } else {
        let api_url = format!("{}/dreams/{}", get_api_url(), dream_id);

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
    }

    DreamState {
        dream: dream.into(),
        is_loading: is_loading.into(),
        error: error.into(),
        format_date,
    }
} 