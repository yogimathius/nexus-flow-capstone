use crate::components::typography::{SectionTitle, SubTitle};
use leptos::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen_futures::spawn_local;

// Import CSS module
const _: &str = include_str!("./dream_journal.module.css");

// Dream model from API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dream {
    pub id: String, // Keep as String for frontend to handle UUID
    pub text: String,
    pub interpretation: String,
    #[serde(rename = "created_at")]
    pub created_at: String, // ISO 8601 date string
}

// Pagination model
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Pagination {
    current_page: usize,
    total_pages: usize,
    items_per_page: usize,
}

#[component]
pub fn DreamJournalPage() -> impl IntoView {
    // ----- State -----
    // Dreams list
    let (dreams, set_dreams) = signal(Vec::<Dream>::new());
    // Loading state
    let (is_loading, set_is_loading) = signal(false);
    // Error state
    let (error, set_error) = signal(Option::<String>::None);
    // Pagination state
    let (pagination, set_pagination) = signal(Pagination {
        current_page: 1,
        total_pages: 1,
        items_per_page: 10,
    });
    // Search query
    let (search_query, set_search_query) = signal(String::new());
    // Date filter
    let (date_filter, set_date_filter) = signal(String::new());

    // ----- API endpoint -----
    let api_url = "http://localhost:3003/api/dreams";

    // ----- Functions -----
    // Load dreams with current filters and pagination
    let load_dreams = move || {
        set_is_loading.set(true);
        set_error.set(None);

        // URL with query parameters for pagination
        let page_url = format!(
            "{}?page={}&limit={}",
            api_url,
            pagination.get().current_page,
            pagination.get().items_per_page
        );

        spawn_local(async move {
            let client = Client::new();
            match client.get(&page_url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        // Log the raw response before trying to parse it
                        let response_text = response.text().await;
                        match response_text {
                            Ok(text) => {
                                log::debug!("API Response text: {}", text);
                                // Parse the JSON manually to debug
                                match serde_json::from_str::<Vec<Dream>>(&text) {
                                    Ok(data) => {
                                        log::debug!("Successfully parsed {} dreams", data.len());

                                        // Log the first dream for debugging
                                        if !data.is_empty() {
                                            let first_dream = &data[0];
                                            log::debug!(
                                                "First dream: id={}, text={}, created_at={}",
                                                first_dream.id,
                                                first_dream
                                                    .text
                                                    .chars()
                                                    .take(30)
                                                    .collect::<String>(),
                                                first_dream.created_at
                                            );
                                        } else {
                                            log::debug!("No dreams returned from API");
                                        }

                                        // For now, we don't have actual pagination in the API
                                        // so just setting total_pages based on dreams length
                                        let total_dreams = data.len();
                                        let items_per_page = pagination.get().items_per_page;
                                        let total_pages =
                                            (total_dreams + items_per_page - 1) / items_per_page;

                                        set_pagination.update(|p| {
                                            p.total_pages = total_pages.max(1);
                                        });
                                        set_dreams.set(data);
                                    }
                                    Err(e) => {
                                        let error_msg = format!(
                                            "Failed to parse JSON: {} - Raw response: {}",
                                            e, text
                                        );
                                        log::error!("{}", error_msg);
                                        set_error.set(Some(error_msg));
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to get response text: {}", e);
                                set_error.set(Some(format!("Failed to read response: {}", e)));
                            }
                        }
                    } else {
                        set_error.set(Some(format!(
                            "Server error: {}",
                            response.status().as_str()
                        )));
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to connect to the server: {}", e)));
                }
            }
            set_is_loading.set(false);
        });
    };

    // Format date from ISO string to readable format
    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y at %H:%M").to_string()
        } else {
            date_str.to_string()
        }
    };

    // Handle page change
    let handle_page_change = move |new_page: usize| {
        set_pagination.update(|p| {
            p.current_page = new_page;
        });
        load_dreams();
    };

    // Handle search
    let handle_search = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_pagination.update(|p| {
            p.current_page = 1; // Reset to first page when searching
        });
        load_dreams();
    };

    // Truncate text to a specified length
    let truncate = |text: &str, max_len: usize| -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    // Load dreams once on component mount instead of using effect which causes infinite re-renders
    let _ = Effect::new(move |_| {
        load_dreams();
    });

    // ----- Render -----
    view! {
        <div class="dream-journal">
            <SectionTitle text="Dream Journal" />
            <p class="intro-text">
                "Your personal collection of dreams and their interpretations."
            </p>

            // Search and filters
            <div class="filters-container">
                <form on:submit=handle_search class="search-form">
                    <input
                        type="text"
                        placeholder="Search dreams..."
                        class="search-input"
                        prop:value=search_query
                        on:input=move |ev| {
                            set_search_query.set(event_target_value(&ev));
                        }
                    />
                    <button type="submit" class="search-button">
                        "Search"
                    </button>
                </form>

                <div class="date-filter">
                    <select
                        class="date-select"
                        on:change=move |ev| {
                            set_date_filter.set(event_target_value(&ev));
                            set_pagination.update(|p| {
                                p.current_page = 1;
                            });
                            load_dreams();
                        }
                    >
                        <option value="">"All time"</option>
                        <option value="day">"Last 24 hours"</option>
                        <option value="week">"Last week"</option>
                        <option value="month">"Last month"</option>
                    </select>
                </div>

                // Manual refresh button for debugging
                <button
                    type="button"
                    class="refresh-button"
                    on:click=move |_| load_dreams()
                >
                    "Refresh"
                </button>
            </div>

            // Content - Loading, Error, or Dreams
            <div class="journal-content">
                // Show loading indicator
                <Show when=move || is_loading.get()>
                    <div class="loading">
                        <div></div>
                        <div></div>
                        <div></div>
                    </div>
                </Show>

                // Show error if any
                <Show when=move || error.get().is_some()>
                    {move || error.get().map(|err| view! {
                        <div class="error-message">
                            <h3>"Error Loading Dreams"</h3>
                            <p>{err}</p>
                            <button
                                type="button"
                                class="retry-button"
                                on:click=move |_| load_dreams()
                            >
                                "Try Again"
                            </button>
                        </div>
                    })}
                </Show>

                // Show dreams or empty state
                <Show
                    when=move || !is_loading.get() && error.get().is_none()
                    fallback=|| view! {}
                >
                    <Show
                        when=move || !dreams.get().is_empty()
                        fallback=move || {
                            view! {
                                <div class="empty-state">
                                    <p>"No dreams found in your journal."</p>
                                    <a href="/interpret" class="interpret-link">
                                        "Interpret a new dream"
                                    </a>
                                </div>
                            }
                        }
                    >
                        <div class="dreams-grid">
                            {move || {
                                dreams.get().into_iter().map(|dream| {
                                    let dream_id = dream.id.clone();
                                    let date = format_date(&dream.created_at);

                                    view! {
                                        <div class="dream-card">
                                            <div class="dream-date">{date}</div>
                                            <h3 class="dream-title">
                                                {truncate(&dream.text, 60)}
                                            </h3>
                                            <p class="dream-interpretation">
                                                {truncate(&dream.interpretation, 120)}
                                            </p>
                                            <a
                                                href=format!("/dream/{}", dream_id)
                                                class="view-dream-link"
                                            >
                                                "View Full Dream"
                                            </a>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    </Show>
                </Show>
            </div>

            // Pagination controls
            <Show when=move || (pagination.get().total_pages > 1)>
                {move || {
                    let p = pagination.get();
                    view! {
                        <div class="pagination">
                            <button
                                class="page-button"
                                disabled=p.current_page == 1
                                on:click=move |_| {
                                    if p.current_page > 1 {
                                        handle_page_change(p.current_page - 1);
                                    }
                                }
                            >
                                "Previous"
                            </button>

                            <span class="page-info">
                                "Page " {p.current_page} " of " {p.total_pages}
                            </span>

                            <button
                                class="page-button"
                                disabled=p.current_page == p.total_pages
                                on:click=move |_| {
                                    if p.current_page < p.total_pages {
                                        handle_page_change(p.current_page + 1);
                                    }
                                }
                            >
                                "Next"
                            </button>
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
