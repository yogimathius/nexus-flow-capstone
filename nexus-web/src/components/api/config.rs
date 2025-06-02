use leptos::logging::log;

/// Get the base API URL from environment or runtime detection
pub fn get_base_api_url() -> String {
    // First try to get from compile-time environment variable
    if let Some(api_url) = option_env!("API_URL") {
        // Only use the environment variable if it's not empty
        if !api_url.is_empty() {
            log!("Using API_URL from environment: {}", api_url);
            return api_url.to_string();
        }
    }

    // Fallback to runtime detection for development
    let host = if web_sys::window()
        .unwrap()
        .location()
        .hostname()
        .unwrap_or_default()
        == "127.0.0.1"
    {
        "127.0.0.1"
    } else {
        "localhost"
    };

    let fallback_url = format!("http://{}:8080/api", host);
    log!("Using fallback API URL: {}", fallback_url);
    fallback_url
}

/// Get API URL for dream interpretation endpoint
pub fn get_dream_interpret_url() -> String {
    format!("{}/interpret_dream", get_base_api_url())
}

/// Get API URL for dreams/journal endpoint
pub fn get_dreams_url() -> String {
    format!("{}/dreams", get_base_api_url())
}

/// Get API URL for health check endpoint
pub fn get_health_url() -> String {
    format!("{}/health", get_base_api_url())
}
