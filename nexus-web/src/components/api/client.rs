use super::config::get_health_url;
use leptos::logging::log;
use wasm_bindgen_futures::spawn_local;

/// Check if the API server is running
pub fn check_server_status(callback: impl Fn(bool) + 'static) {
    let health_url = get_health_url();

    spawn_local(async move {
        match reqwest::Client::builder().build() {
            Ok(client) => match client.get(&health_url).send().await {
                Ok(response) => {
                    let is_healthy = response.status().is_success();
                    log!(
                        "Server health check: {}",
                        if is_healthy { "healthy" } else { "unhealthy" }
                    );
                    callback(is_healthy);
                }
                Err(e) => {
                    log!("Server health check failed: {}", e);
                    callback(false);
                }
            },
            Err(e) => {
                log!("Failed to create HTTP client: {}", e);
                callback(false);
            }
        }
    });
}

/// Common HTTP error handling for API responses
pub fn handle_http_error(status: u16) -> String {
    match status {
        400 => "The request was invalid. Please check your input and try again.".to_string(),
        401 | 403 => "You're not authorized to use this feature. Please log in or check your account permissions.".to_string(),
        404 => "The requested resource could not be found. Please try again later.".to_string(),
        429 => "You've made too many requests. Please wait a moment before trying again.".to_string(),
        500 | 502 | 503 | 504 => "The service is currently unavailable. Please try again later.".to_string(),
        _ => "An unexpected error occurred. Please try again later.".to_string(),
    }
}
