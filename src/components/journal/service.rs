use crate::components::journal::models::{Dream, Pagination};
use gloo_net::http::Request;
use leptos::logging::log;
use wasm_bindgen_futures::spawn_local;

// Get API URL based on current host
pub fn get_api_url() -> String {
    // Try checking if we're running on localhost or IP
    let host = if web_sys::window()
        .unwrap()
        .location()
        .hostname()
        .unwrap_or_default() == "127.0.0.1"
    {
        "127.0.0.1"
    } else {
        "localhost"
    };
    format!("http://{}:8080/api/dreams", host)
}

// Check if server is running
pub fn check_server_status(
    callback: impl Fn(bool) + 'static,
) {
    let health_url = format!(
        "http://{}:8080/health",
        if web_sys::window()
            .unwrap()
            .location()
            .hostname()
            .unwrap_or_default() == "127.0.0.1"
        {
            "127.0.0.1"
        } else {
            "localhost"
        }
    );

    spawn_local(async move {
        match reqwest::Client::builder().build() {
            Ok(client) => {
                match client.get(&health_url).send().await {
                    Ok(_) => {
                        callback(true);
                    }
                    Err(_) => {
                        callback(false);
                    }
                }
            }
            Err(e) => {
                log!("Failed to create HTTP client: {}", e);
                callback(false);
            }
        }
    });
}

// Fetch dreams with filters
pub fn fetch_dreams(
    pagination: &Pagination,
    search_query: &str,
    date_filter: &str,
    on_success: impl Fn(Vec<Dream>) + 'static,
    on_error: impl Fn(String) + 'static,
) {
    // Build query parameters
    let mut query_params = vec![
        format!("page={}", pagination.current_page),
        format!("limit={}", pagination.items_per_page),
    ];

    // Add search query if present
    if !search_query.is_empty() {
        query_params.push(format!("search={}", search_query));
    }

    // Add date filter if present
    if !date_filter.is_empty() {
        query_params.push(format!("date_filter={}", date_filter));
    }

    let page_url = format!("{}?{}", get_api_url(), query_params.join("&"));

    log!("Fetching dreams from: {}", page_url);

    spawn_local(async move {
        match Request::get(&page_url).send().await {
            Ok(response) => {
                match response.json::<Vec<Dream>>().await {
                    Ok(data) => {
                        on_success(data);
                    }
                    Err(e) => {
                        on_error(format!("Failed to parse response: {}", e));
                    }
                }
            }
            Err(e) => {
                on_error(format!("Failed to fetch dreams: {}", e));
            }
        }
    });
} 