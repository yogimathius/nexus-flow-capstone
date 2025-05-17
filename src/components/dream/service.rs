use crate::components::dream::models::{DreamRequest, DreamResponse};
use leptos::logging::{log, error};
use reqwest::Client;
use std::time::Duration;

const API_URL: &str = "http://localhost:8080/api/interpret_dream";
const REQUEST_TIMEOUT: u64 = 30; // seconds

pub async fn interpret_dream(dream_text: String, save: bool) -> Result<DreamResponse, String> {
    let request_payload = DreamRequest {
        dream_text,
        save,
    };

    let client = Client::new();

    match client
        .post(API_URL)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .header("Content-Type", "application/json")
        .json(&request_payload)
        .send()
        .await
    {
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
        Err(e) => {
            // More specific network-related errors
            let error_detail = if e.is_timeout() {
                "The request timed out. The server might be busy."
            } else if e.is_connect() {
                "Could not connect to the service."
            } else if e.is_request() {
                "There was a problem with your request."
            } else {
                "Please check your internet connection and try again."
            };
            
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