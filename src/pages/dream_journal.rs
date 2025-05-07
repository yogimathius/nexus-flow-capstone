use crate::components::markdown::render_markdown;
use crate::components::typography::SectionTitle;
use gloo_net::http::Request;
use leptos::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Pagination {
    current_page: usize,
    total_pages: usize,
    items_per_page: usize,
}

#[component]
pub fn DreamJournalPage() -> impl IntoView {
    // ----- State -----
    // Dreams list
    let (dreams, set_dreams) = create_signal(Vec::<Dream>::new());
    // Loading state
    let (is_loading, set_is_loading) = create_signal(false);
    // Error state
    let (error, set_error) = create_signal(Option::<String>::None);
    // Pagination state
    let (pagination, set_pagination) = create_signal(Pagination {
        current_page: 1,
        total_pages: 1,
        items_per_page: 10,
    });
    // Search query
    let (search_query, set_search_query) = create_signal(String::new());
    // Date filter
    let (date_filter, set_date_filter) = create_signal(String::new());
    // Trigger for refreshing dreams
    let (refresh_trigger, set_refresh_trigger) = create_signal(0);

    // ----- API endpoint -----
    // Try localhost first, fallback to IP
    let get_api_url = move || {
        // Try checking if we're running on localhost or IP
        let host = if window().location().hostname().unwrap_or_default() == "127.0.0.1" {
            "127.0.0.1"
        } else {
            "localhost"
        };
        format!("http://{}:3003/api/dreams", host)
    };

    // Create a memo to track dependencies for fetching
    let deps = create_memo(move |_| {
        (
            refresh_trigger.get(),
            pagination.get(),
            search_query.get(),
            date_filter.get(),
        )
    });

    // Create an effect to handle fetching
    create_effect(move |_| {
        let (_, pagination, search_query, date_filter) = deps.get();
        set_is_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
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

            log::debug!("Fetching dreams from: {}", page_url);

            match Request::get(&page_url).send().await {
                Ok(response) => {
                    match response.json::<Vec<Dream>>().await {
                        Ok(data) => {
                            // Calculate pagination
                            let total_dreams = data.len();
                            let items_per_page = pagination.items_per_page;
                            let total_pages = (total_dreams + items_per_page - 1) / items_per_page;

                            set_pagination.update(|p| {
                                p.total_pages = total_pages.max(1);
                            });

                            set_dreams.set(data);
                            set_is_loading.set(false);
                        }
                        Err(e) => {
                            set_error.set(Some(format!("Failed to parse response: {}", e)));
                            set_is_loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to fetch dreams: {}", e)));
                    set_is_loading.set(false);
                }
            }
        });
    });

    // Simplified load_dreams function that just triggers a refresh
    let load_dreams = move || {
        set_refresh_trigger.update(|count| *count += 1);
    };

    // Check if server is running
    let check_server_status = move || {
        let health_url = format!(
            "http://{}:3003/health",
            if window().location().hostname().unwrap_or_default() == "127.0.0.1" {
                "127.0.0.1"
            } else {
                "localhost"
            }
        );

        spawn_local(async move {
            match Client::builder().build() {
                Ok(client) => {
                    match client.get(&health_url).send().await {
                        Ok(_) => {
                            // Server is running, try loading dreams again
                            load_dreams();
                        }
                        Err(_) => {
                            set_error.set(Some(format!(
                                "The backend server appears to be offline. Please ensure it's running at {}.",
                                health_url
                            )));
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to create HTTP client: {}", e)));
                }
            }
        });
    };

    // Format date from ISO string to readable format
    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y").to_string()
        } else {
            date_str.to_string()
        }
    };

    // Format time from ISO string
    let format_time = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%H:%M").to_string()
        } else {
            "".to_string()
        }
    };

    // Handle page change
    let handle_page_change = move |new_page: usize| {
        set_pagination.update(|p| {
            p.current_page = new_page;
        });
    };

    // Handle search
    let handle_search = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_pagination.update(|p| {
            p.current_page = 1; // Reset to first page when searching
        });
        load_dreams(); // Trigger search immediately
    };

    // Truncate text to a specified length
    let truncate = |text: &str, max_len: usize| -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    // Clean interpretation preview (extract first paragraph without markdown)
    let clean_interpretation_preview = |text: &str, max_len: usize| -> String {
        // Remove markdown and HTML tags for cleaner preview
        let text = render_markdown(text);
        let text = text
            .replace(r#"<h3 class="md-heading">"#, "")
            .replace(r#"<h4 class="md-subheading">"#, "")
            .replace("</h3>", "")
            .replace("</h4>", "")
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("<em>", "")
            .replace("</em>", "")
            .replace("<br/>", " ");

        // Inline truncate logic instead of calling the truncate function
        if text.len() <= max_len {
            text
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    // ----- Render -----
    view! {
        <div class="dream-journal fade-in">
            <div class="journal-header bg-gradient">
                <div class="container">
                    <h1 class="slide-up">"Dream Journal"</h1>
                    <p class="intro-text slide-up animation-delay-1">
                        "Your personal collection of dreams and their interpretations."
                    </p>
                </div>
            </div>

            <div class="container">
                // Search and filters
                <div class="filters-container slide-up animation-delay-2">
                    <form on:submit=handle_search class="search-form">
                        <div class="search-input-wrapper">
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
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="20"
                                    height="20"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <circle cx="11" cy="11" r="8"></circle>
                                    <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                                </svg>
                            </button>
                        </div>
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

                    <button
                        type="button"
                        class="refresh-button"
                        title="Refresh dreams"
                        on:click=move |_| load_dreams()
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
                            <path d="M3 3v5h5"></path>
                        </svg>
                    </button>
                </div>

                // Content - Loading, Error, or Dreams
                <div class="journal-content">
                    // Show loading indicator
                    <Show when=move || is_loading.get()>
                        <div class="loading-container slide-up">
                            <div class="loading">
                                <div></div>
                                <div></div>
                                <div></div>
                            </div>
                            <p class="loading-text pulse">"Loading your dreams..."</p>
                        </div>
                    </Show>

                    // Show error if any
                    <Show when=move || error.get().is_some()>
                        {move || error.get().map(|err| view! {
                            <div class="error-message slide-up">
                                <h3>"Error Loading Dreams"</h3>
                                <p>{err}</p>
                                <div class="error-actions">
                                    <button
                                        type="button"
                                        class="retry-button"
                                        on:click=move |_| check_server_status()
                                    >
                                        "Check Server & Retry"
                                    </button>
                                    <a href="/interpret" class="interpret-link">
                                        "Create New Dream Instead"
                                    </a>
                                </div>
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
                                    <div class="empty-state slide-up">
                                        <div class="empty-state-content">
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="60"
                                                height="60"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="1"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                class="empty-icon"
                                            >
                                                <path d="M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-1.5"></path>
                                                <path d="M16 2v4"></path>
                                                <path d="M8 2v4"></path>
                                                <path d="M3 10h18"></path>
                                                <path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z"></path>
                                                <path d="m22 22-1.5-1.5"></path>
                                            </svg>
                                            <p>"No dreams found in your journal."</p>
                                            <a href="/interpret" class="interpret-link button-outline">
                                                "Interpret a new dream"
                                            </a>
                                        </div>
                                    </div>
                                }
                            }
                        >
                            <div class="dreams-grid">
                                {move || {
                                    dreams.get().into_iter().enumerate().map(|(index, dream)| {
                                        let dream_id = dream.id.clone();
                                        let date = format_date(&dream.created_at);
                                        let time = format_time(&dream.created_at);
                                        let clean_interp = clean_interpretation_preview(&dream.interpretation, 120);

                                        // Calculate animation delay based on index
                                        let delay_class = match index % 3 {
                                            0 => "animation-delay-1",
                                            1 => "animation-delay-2",
                                            _ => "animation-delay-3",
                                        };

                                        view! {
                                            <div class=format!("dream-card slide-up {}", delay_class)>
                                                <div class="dream-card-header">
                                                    <span class="badge dream">"Dream"</span>
                                                    <div class="dream-meta">
                                                        <div class="dream-date">{date}</div>
                                                        <div class="dream-time">{time}</div>
                                                    </div>
                                                </div>
                                                <h3 class="dream-title">
                                                    {truncate(&dream.text, 60)}
                                                </h3>
                                                <p class="dream-interpretation">
                                                    {clean_interp}
                                                </p>
                                                <a
                                                    href=format!("/journal/{}", dream_id)
                                                    class="view-dream-link"
                                                >
                                                    "View Full Dream"
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        width="16"
                                                        height="16"
                                                        viewBox="0 0 24 24"
                                                        fill="none"
                                                        stroke="currentColor"
                                                        stroke-width="2"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                    >
                                                        <path d="M5 12h14"></path>
                                                        <path d="m12 5 7 7-7 7"></path>
                                                    </svg>
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
                            <div class="pagination slide-up animation-delay-4">
                                <button
                                    class="page-button"
                                    disabled=p.current_page == 1
                                    on:click=move |_| {
                                        if p.current_page > 1 {
                                            handle_page_change(p.current_page - 1);
                                        }
                                    }
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <path d="m15 18-6-6 6-6"></path>
                                    </svg>
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
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <path d="m9 18 6-6-6-6"></path>
                                    </svg>
                                </button>
                            </div>
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}
