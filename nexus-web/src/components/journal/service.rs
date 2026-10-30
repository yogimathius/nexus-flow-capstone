use crate::components::api::get_dreams_url;
use crate::components::journal::models::{Dream, DreamSearchFilters, Pagination};
use gloo_net::http::Request;
use leptos::logging::{error, log};
use serde_json::{from_str, to_string};
use wasm_bindgen_futures::spawn_local;

const STORAGE_KEY: &str = "nexus_flow_dreams";

// Save a dream to localStorage for offline support
pub fn save_dream_offline(dream: &Dream) -> Result<(), String> {
    let mut dreams = get_dreams_from_storage().unwrap_or_default();

    // Check if dream already exists (update) or is new (add)
    let existing_index = dreams.iter().position(|d| d.id == dream.id);

    if let Some(index) = existing_index {
        dreams[index] = dream.clone(); // Update existing dream
    } else {
        dreams.push(dream.clone()); // Add new dream
    }

    save_dreams_to_storage(&dreams)
}

// Save all dreams to localStorage
fn save_dreams_to_storage(dreams: &[Dream]) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        if let Ok(storage) = window.local_storage() {
            if let Some(storage) = storage {
                if let Ok(json) = to_string(dreams) {
                    return match storage.set_item(STORAGE_KEY, &json) {
                        Ok(_) => {
                            log!(
                                "Successfully saved {} dreams to local storage",
                                dreams.len()
                            );
                            Ok(())
                        }
                        Err(_) => Err("Failed to write to localStorage".to_string()),
                    };
                }
            }
        }
    }

    Err("Failed to access localStorage".to_string())
}

// Load dreams from localStorage
fn get_dreams_from_storage() -> Result<Vec<Dream>, String> {
    if let Some(window) = web_sys::window() {
        if let Ok(storage) = window.local_storage() {
            if let Some(storage) = storage {
                match storage.get_item(STORAGE_KEY) {
                    Ok(Some(json)) => {
                        return match from_str::<Vec<Dream>>(&json) {
                            Ok(dreams) => {
                                log!("Loaded {} dreams from local storage", dreams.len());
                                Ok(dreams)
                            }
                            Err(e) => Err(format!("Failed to parse dreams: {}", e)),
                        };
                    }
                    Ok(None) => {
                        log!("No dreams found in local storage");
                        return Ok(Vec::new());
                    }
                    Err(_) => return Err("Failed to read from localStorage".to_string()),
                }
            }
        }
    }

    Err("Failed to access localStorage".to_string())
}

// Remove a dream from localStorage
pub fn remove_dream_offline(dream_id: &str) -> Result<(), String> {
    let mut dreams = get_dreams_from_storage().unwrap_or_default();
    dreams.retain(|d| d.id != dream_id);
    save_dreams_to_storage(&dreams)
}

// Filter dreams based on search criteria
fn filter_dreams(dreams: &[Dream], filters: &DreamSearchFilters) -> Vec<Dream> {
    dreams
        .iter()
        .filter(|dream| {
            // Text search
            let text_match = filters.search_query.is_empty()
                || dream
                    .text
                    .to_lowercase()
                    .contains(&filters.search_query.to_lowercase())
                || dream
                    .interpretation
                    .to_lowercase()
                    .contains(&filters.search_query.to_lowercase());

            // Category filter
            let category_match = filters.category.is_empty()
                || filters.category == "All Categories"
                || dream
                    .category
                    .as_ref()
                    .map_or(false, |cat| cat == &filters.category);

            // Tags filter
            let tags_match =
                filters.tags.is_empty() || filters.tags.iter().all(|tag| dream.tags.contains(tag));

            // Favorites filter
            let favorites_match = !filters.favorites_only || dream.is_favorite;

            // Date filter
            let date_match = match filters.date_filter.as_str() {
                "day" => {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&dream.created_at) {
                        let now = chrono::Utc::now();
                        let duration = now.signed_duration_since(date.with_timezone(&chrono::Utc));
                        duration.num_hours() < 24
                    } else {
                        true
                    }
                }
                "week" => {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&dream.created_at) {
                        let now = chrono::Utc::now();
                        let duration = now.signed_duration_since(date.with_timezone(&chrono::Utc));
                        duration.num_days() < 7
                    } else {
                        true
                    }
                }
                "month" => {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&dream.created_at) {
                        let now = chrono::Utc::now();
                        let duration = now.signed_duration_since(date.with_timezone(&chrono::Utc));
                        duration.num_days() < 30
                    } else {
                        true
                    }
                }
                "year" => {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&dream.created_at) {
                        let now = chrono::Utc::now();
                        let duration = now.signed_duration_since(date.with_timezone(&chrono::Utc));
                        duration.num_days() < 365
                    } else {
                        true
                    }
                }
                _ => true,
            };

            text_match && category_match && tags_match && favorites_match && date_match
        })
        .cloned()
        .collect()
}

// Get paginated set of dreams
fn paginate_dreams(dreams: Vec<Dream>, pagination: &Pagination) -> Vec<Dream> {
    let start = (pagination.current_page - 1) * pagination.items_per_page;
    let _end = start + pagination.items_per_page;

    dreams
        .into_iter()
        .skip(start)
        .take(pagination.items_per_page)
        .collect()
}

// Fetch dreams with filters, with fallback to offline mode
pub fn fetch_dreams(
    pagination: &Pagination,
    search_query: &str,
    on_success: impl Fn(Vec<Dream>, Pagination) + 'static,
    on_error: impl Fn(String) + 'static,
) {
    // Create filters
    let filters = DreamSearchFilters {
        search_query: search_query.to_string(),
        ..Default::default()
    };

    // Build query parameters
    let mut query_params = form_urlencoded::Serializer::new(String::new());
    query_params.append_pair("page", &pagination.current_page.to_string());
    query_params.append_pair("limit", &pagination.items_per_page.to_string());
    if !search_query.is_empty() {
        query_params.append_pair("search", search_query);
    }

    let page_url = format!("{}?{}", get_dreams_url(), query_params.finish());

    log!("Fetching dreams from: {}", page_url);

    // Clone pagination and filters for use in the async block
    let pagination_clone = pagination.clone();
    let filters_clone = filters.clone();

    // Define response structure to match API format
    #[derive(serde::Deserialize)]
    struct PaginatedResponse {
        dreams: Vec<Dream>,
        #[allow(dead_code)]
        total: usize,
        page: usize,
        limit: usize,
        total_pages: usize,
    }

    spawn_local(async move {
        match Request::get(&page_url).send().await {
            Ok(response) => {
                match response.json::<PaginatedResponse>().await {
                    Ok(data) => {
                        // Preserve entries from other pages and searches in the offline cache.
                        for dream in &data.dreams {
                            let _ = save_dream_offline(dream);
                        }

                        // Create pagination information from the response
                        let server_pagination = Pagination {
                            current_page: data.page,
                            total_pages: data.total_pages,
                            items_per_page: data.limit,
                        };

                        on_success(data.dreams, server_pagination);
                    }
                    Err(e) => {
                        error!("Failed to parse response from server: {}", e);
                        // Try to load from local storage as a fallback
                        fallback_to_offline_dreams(
                            &pagination_clone,
                            &filters_clone,
                            on_success,
                            on_error,
                        );
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch dreams from server: {}", e);
                // Try to load from local storage as a fallback
                fallback_to_offline_dreams(&pagination_clone, &filters_clone, on_success, on_error);
            }
        }
    });
}

// Fallback to offline dreams when API is unavailable
fn fallback_to_offline_dreams(
    pagination: &Pagination,
    filters: &DreamSearchFilters,
    on_success: impl Fn(Vec<Dream>, Pagination),
    on_error: impl Fn(String),
) {
    match get_dreams_from_storage() {
        Ok(dreams) => {
            if dreams.is_empty() {
                on_error(
                    "No dreams found in offline storage. Please connect to the server.".to_string(),
                );
            } else {
                // Filter dreams based on criteria
                let filtered_dreams = filter_dreams(&dreams, filters);

                // Calculate pagination
                let total_dreams = filtered_dreams.len();
                let items_per_page = pagination.items_per_page;
                let total_pages = (total_dreams + items_per_page - 1) / items_per_page;

                // Create pagination information
                let offline_pagination = Pagination {
                    current_page: pagination.current_page,
                    total_pages: total_pages.max(1),
                    items_per_page: pagination.items_per_page,
                };

                // Paginate the results
                let paginated_dreams = paginate_dreams(filtered_dreams, &offline_pagination);

                log!(
                    "Loaded {} dreams from offline storage",
                    paginated_dreams.len()
                );
                on_success(paginated_dreams, offline_pagination);
            }
        }
        Err(e) => {
            on_error(format!("Failed to load dreams from offline storage: {}", e));
        }
    }
}

// Toggle favorite status for a dream
pub fn toggle_favorite(
    dream_id: &str,
    is_favorite: bool,
    on_success: impl Fn() + 'static,
    on_error: impl Fn(String) + 'static,
) {
    let dream_id = dream_id.to_string();
    let url = format!("{}/{}/favorite", get_dreams_url(), dream_id);

    spawn_local(async move {
        match Request::post(&url)
            .json(&is_favorite)
            .expect("Failed to serialize favorite status")
            .send()
            .await
        {
            Ok(_) => {
                // Update local storage
                if let Ok(mut dreams) = get_dreams_from_storage() {
                    if let Some(dream) = dreams.iter_mut().find(|d| d.id == dream_id) {
                        dream.is_favorite = is_favorite;
                        let _ = save_dreams_to_storage(&dreams);
                    }
                }
                on_success();
            }
            Err(e) => {
                // Try to update local storage anyway
                if let Ok(mut dreams) = get_dreams_from_storage() {
                    if let Some(dream) = dreams.iter_mut().find(|d| d.id == dream_id) {
                        dream.is_favorite = is_favorite;
                        if save_dreams_to_storage(&dreams).is_ok() {
                            on_success();
                            return;
                        }
                    }
                }
                on_error(format!("Failed to update favorite status: {}", e));
            }
        }
    });
}
