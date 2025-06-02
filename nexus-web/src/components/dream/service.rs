use crate::components::dream::models::{DreamRequest, DreamResponse};
use leptos::logging::{log, error};
use reqwest::Client;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

// Get API URL from environment or fallback to default
fn get_api_url() -> String {
    // First try to get from compile-time environment variable
    if let Some(api_url) = option_env!("API_URL") {
        return format!("{}/interpret_dream", api_url);
    }
    
    // Fallback to production URL
    "https://nexus-core.fly.dev/api/interpret_dream".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
const REQUEST_TIMEOUT: u64 = 30; // seconds

pub async fn interpret_dream(dream_text: String, save: bool) -> Result<DreamResponse, String> {
    let request_payload = DreamRequest {
        dream_text,
        save,
    };

    let client = Client::new();
    
    // Build the request with conditional timeout for non-WASM targets
    let request_builder = client.post(&get_api_url());
    
    // Apply timeout only for non-WASM targets
    #[cfg(not(target_arch = "wasm32"))]
    let request_builder = request_builder.timeout(Duration::from_secs(REQUEST_TIMEOUT));
    
    // Continue with the rest of the request configuration
    let request_builder = request_builder
        .header("Content-Type", "application/json")
        .json(&request_payload);

    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            
            if response.status().is_success() {
                match response.json::<DreamResponse>().await {
                    Ok(data) => {
                        log!("Received successful interpretation response");
                        Ok(data)
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to parse server response: {}", e);
                        error!("{}", error_msg);
                        Err("Unable to process the interpretation response. Please try again.".to_string())
                    }
                }
            } else {
                match status.as_u16() {
                    400 => {
                        match response.text().await {
                            Ok(text) => {
                                let error_msg = format!("Bad Request: {}", text);
                                error!("{}", error_msg);
                                Err("The dream description you provided was invalid. Please check and try again.".to_string())
                            },
                            Err(_) => Err("Your request was invalid. Please check your dream description.".to_string())
                        }
                    },
                    401 | 403 => {
                        error!("Authentication error: {}", status);
                        Err("You're not authorized to use this feature. Please log in or check your account permissions.".to_string())
                    },
                    404 => {
                        error!("API endpoint not found: {}", status);
                        Err("The dream interpretation service could not be found. Please try again later.".to_string())
                    },
                    429 => {
                        error!("Rate limit exceeded: {}", status);
                        Err("You've made too many requests. Please wait a moment before trying again.".to_string())
                    },
                    500 | 502 | 503 | 504 => {
                        error!("Server error: {}", status);
                        Err("The dream interpretation service is currently unavailable. Please try again later.".to_string())
                    },
                    _ => {
                        match response.text().await {
                            Ok(text) => {
                                let error_msg = format!("Server error ({}): {}", status, text);
                                error!("{}", error_msg);
                                Err("An unexpected error occurred while interpreting your dream. Please try again later.".to_string())
                            },
                            Err(_) => {
                                let error_msg = format!("Server error: Status code {}", status);
                                error!("{}", error_msg);
                                Err("An unexpected error occurred. Please try again later.".to_string())
                            }
                        }
                    }
                }
            }
        }
        Err(_e) => {
            // Different error handling for different platforms
            #[cfg(not(target_arch = "wasm32"))]
            let error_detail = if _e.is_timeout() {
                "The request timed out. The server might be busy."
            } else if _e.is_connect() {
                "Could not connect to the service."
            } else if _e.is_request() {
                "There was a problem with your request."
            } else {
                "Please check your internet connection and try again."
            };
            
            // Simplified error handling for WASM target
            #[cfg(target_arch = "wasm32")]
            let error_detail = "Please check your internet connection and try again.";
            
            let error_msg = format!(
                "Failed to connect to the interpretation service. {}",
                error_detail
            );
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

pub fn format_interpretation(text: &str) -> String {
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
} 