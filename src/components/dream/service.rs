use crate::components::dream::models::{DreamRequest, DreamResponse};
use leptos::logging::{log, error};
use reqwest::Client;

const API_URL: &str = "http://localhost:8080/api/interpret_dream";

pub async fn interpret_dream(dream_text: String, save: bool) -> Result<DreamResponse, String> {
    let request_payload = DreamRequest {
        dream_text,
        save,
    };

    let client = Client::new();

    match client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .json(&request_payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<DreamResponse>().await {
                    Ok(data) => {
                        log!("Received successful interpretation response");
                        Ok(data)
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to parse response: {}", e);
                        error!("{}", error_msg);
                        Err(error_msg)
                    }
                }
            } else {
                let status = response.status();
                match response.text().await {
                    Ok(text) => {
                        let error_msg = format!("Server error ({}): {}", status, text);
                        error!("{}", error_msg);
                        Err(error_msg)
                    }
                    Err(_) => {
                        let error_msg = format!("Server error: Status code {}", status);
                        error!("{}", error_msg);
                        Err(error_msg)
                    }
                }
            }
        }
        Err(e) => {
            let error_msg = format!(
                "Failed to connect to the interpretation service. Please try again later. Error: {}",
                e
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