use crate::components::api::{get_dream_interpret_url, handle_http_error};
use crate::components::dream::models::{DreamRequest, DreamResponse};
use leptos::logging::{error, log};
use reqwest::Client;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
const REQUEST_TIMEOUT: u64 = 30; // seconds

pub async fn interpret_dream(dream_text: String, save: bool) -> Result<DreamResponse, String> {
    let request_payload = DreamRequest { dream_text, save };

    let client = Client::new();

    // Build the request with conditional timeout for non-WASM targets
    let request_builder = client.post(&get_dream_interpret_url());

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
                        Err(
                            "Unable to process the interpretation response. Please try again."
                                .to_string(),
                        )
                    }
                }
            } else {
                let status_code = status.as_u16();
                match response.text().await {
                    Ok(text) if !text.is_empty() => {
                        let error_msg = format!("Server error ({}): {}", status_code, text);
                        error!("{}", error_msg);
                        // Use shared error handling but provide more specific context for certain errors
                        match status_code {
                            400 => Err("The dream description you provided was invalid. Please check and try again.".to_string()),
                            404 => Err("The dream interpretation service could not be found. Please try again later.".to_string()),
                            _ => Err(handle_http_error(status_code))
                        }
                    }
                    _ => {
                        let error_msg = format!("Server error: Status code {}", status_code);
                        error!("{}", error_msg);
                        Err(handle_http_error(status_code))
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
