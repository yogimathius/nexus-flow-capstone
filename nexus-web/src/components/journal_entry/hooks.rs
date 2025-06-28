use crate::components::journal::models::Dream;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct DreamState {
    pub dream: Signal<Option<Dream>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub format_date: fn(&str) -> String,
}

// Get dreams from localStorage
fn get_dreams_from_storage() -> Result<Vec<Dream>, String> {
    if let Some(window) = web_sys::window() {
        if let Ok(storage) = window.local_storage() {
            if let Some(storage) = storage {
                match storage.get_item("nexus_flow_dreams") {
                    Ok(Some(json)) => {
                        return match serde_json::from_str::<Vec<Dream>>(&json) {
                            Ok(dreams) => Ok(dreams),
                            Err(e) => Err(format!("Failed to parse dreams: {}", e)),
                        };
                    }
                    Ok(None) => return Ok(Vec::new()),
                    Err(_) => return Err("Failed to read from localStorage".to_string()),
                }
            }
        }
    }
    Err("Failed to access localStorage".to_string())
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
        // First try to find in localStorage
        spawn_local(async move {
            match get_dreams_from_storage() {
                Ok(dreams) => {
                    if let Some(dream_data) = dreams.iter().find(|d| d.id == dream_id) {
                        set_dream.set(Some(dream_data.clone()));
                        set_is_loading.set(false);
                        return;
                    }
                }
                Err(_) => {}
            }
            
            // If not found in localStorage, show error since API endpoint doesn't exist
            set_error.set(Some("Dream not found. The dream may have been deleted or the ID is invalid.".to_string()));
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
