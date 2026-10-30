use leptos::logging::log;

#[cfg(test)]
mod tests {
    use super::{health_url_from_api_root, normalize_api_root};

    #[test]
    fn api_origin_and_api_root_resolve_to_same_routes() {
        assert_eq!(normalize_api_root("http://localhost:3003"), "http://localhost:3003/api");
        assert_eq!(normalize_api_root("http://localhost:3003/api/"), "http://localhost:3003/api");
        assert_eq!(health_url_from_api_root("http://localhost:3003/api"), "http://localhost:3003/health");
    }
}

fn normalize_api_root(url: &str) -> String {
    let url = url.trim_end_matches('/');
    if url.ends_with("/api") {
        url.to_string()
    } else {
        format!("{url}/api")
    }
}

fn health_url_from_api_root(api_root: &str) -> String {
    format!("{}/health", api_root.trim_end_matches("/api"))
}

/// Get the API root from the build environment, defaulting to the local Rust API.
pub fn get_base_api_url() -> String {
    // First try to get from compile-time environment variable
    if let Some(api_url) = option_env!("API_URL") {
        // Only use the environment variable if it's not empty
        if !api_url.is_empty() {
            log!("Using API_URL from environment: {}", api_url);
            return normalize_api_root(api_url);
        }
    }

    let local_url = "http://localhost:3003/api".to_string();
    log!("Using local API: {}", local_url);
    local_url
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
    health_url_from_api_root(&get_base_api_url())
}
