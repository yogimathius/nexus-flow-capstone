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

    // Simple production check - if hostname contains fly.dev, we're in production
    let is_production = web_sys::window()
        .and_then(|w| w.location().hostname().ok())
        .map(|hostname| hostname.contains(".fly.dev"))
        .unwrap_or(false);

    if is_production {
        let production_url = "https://nexus-core.fly.dev/api".to_string();
        log!("Production environment detected, using: {}", production_url);
        production_url
    } else {
        let local_url = "http://localhost:8080/api".to_string();
        log!("Local environment detected, using: {}", local_url);
        local_url
    }
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
