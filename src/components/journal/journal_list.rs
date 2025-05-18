use crate::components::journal::{
    models::{Dream, DreamSearchFilters, Pagination},
    service::{check_server_status, fetch_dreams},
    JournalCard, JournalEmptyState, JournalError, JournalFilters, JournalLoading,
    JournalPagination, OfflineBanner, OfflineIndicator,
};
use leptos::prelude::*;

#[component]
pub fn JournalList() -> impl IntoView {
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
    // Category filter
    let (category_filter, set_category_filter) = signal("All Categories".to_string());
    // Tags filter
    let (selected_tags, set_selected_tags) = signal(Vec::<String>::new());
    // Favorites only filter
    let (favorites_only, set_favorites_only) = signal(false);
    // Offline mode
    let (is_offline, set_is_offline) = signal(false);
    // Trigger for refreshing dreams
    let (refresh_trigger, set_refresh_trigger) = signal(0);

    // Create a memo to track dependencies for fetching
    let search_filters = Memo::new(move |_| DreamSearchFilters {
        search_query: search_query.get(),
        date_filter: date_filter.get(),
        category: category_filter.get(),
        tags: selected_tags.get(),
        favorites_only: favorites_only.get(),
    });

    let fetch_deps = Memo::new(move |_| {
        (
            refresh_trigger.get(),
            pagination.get(),
            search_filters.get(),
        )
    });

    // Create an effect to handle fetching
    Effect::new(move |_| {
        let (_, pagination, filters) = fetch_deps.get();
        set_is_loading.set(true);
        set_error.set(None);

        // Check if we're online or need to use offline mode
        check_server_status(move |is_online| {
            set_is_offline.set(!is_online);

            fetch_dreams(
                &pagination,
                &filters.search_query,
                &filters.date_filter,
                move |data, server_pagination| {
                    // Set dreams data
                    set_dreams.set(data);

                    // Update pagination with server data
                    set_pagination.set(server_pagination);

                    // Update loading state
                    set_is_loading.set(false);
                },
                move |err_msg| {
                    set_error.set(Some(err_msg));
                    set_is_loading.set(false);
                },
            );
        });
    });

    // Simplified load_dreams function that just triggers a refresh
    let load_dreams = move || {
        set_refresh_trigger.update(|count| *count += 1);
    };

    // Check server status and retry loading
    let check_server = move || {
        check_server_status(move |is_online| {
            set_is_offline.set(!is_online);
            if is_online {
                load_dreams();
            } else {
                set_error.set(Some(
                    "The backend server appears to be offline. Please ensure it's running.".into(),
                ));
            }
        });
    };

    // Handle page change
    let handle_page_change = Callback::new(move |new_page: usize| {
        set_pagination.update(|p| {
            p.current_page = new_page;
        });
    });

    // Handle search
    let handle_search = Callback::new(move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_pagination.update(|p| {
            p.current_page = 1; // Reset to first page when searching
        });
        load_dreams(); // Trigger search immediately
    });

    // Handle refresh
    let handle_refresh = Callback::new(move |()| {
        load_dreams();
    });

    // Handle retry
    let handle_retry = Callback::new(move |()| {
        check_server();
    });

    // Handle category filter change
    let handle_category_change = Callback::new(move |category: String| {
        set_category_filter.set(category);
        set_pagination.update(|p| {
            p.current_page = 1; // Reset to first page when filtering
        });
        load_dreams();
    });

    // Handle tags filter change
    let handle_tags_change = Callback::new(move |tags: Vec<String>| {
        set_selected_tags.set(tags);
        set_pagination.update(|p| {
            p.current_page = 1; // Reset to first page when filtering
        });
        load_dreams();
    });

    // Handle favorites filter toggle
    let handle_favorites_toggle = Callback::new(move |is_checked: bool| {
        set_favorites_only.set(is_checked);
        set_pagination.update(|p| {
            p.current_page = 1; // Reset to first page when filtering
        });
        load_dreams();
    });

    view! {
        <div class="container">
            // Show offline indicator
            <Show when=move || is_offline.get()>
                <OfflineIndicator />
            </Show>

            // Search and filters
            <JournalFilters
                search_query=search_query
                set_search_query=set_search_query
                date_filter=date_filter
                set_date_filter=set_date_filter
                category_filter=category_filter
                set_category_filter=set_category_filter
                selected_tags=selected_tags
                set_selected_tags=set_selected_tags
                favorites_only=favorites_only
                set_favorites_only=set_favorites_only
                on_search=handle_search
                on_refresh=handle_refresh
                on_category_change=handle_category_change
                on_tags_change=handle_tags_change
                on_favorites_toggle=handle_favorites_toggle
            />

            // Content - Loading, Error, or Dreams
            <div class="journal-content">
                // Show loading indicator
                <Show when=move || is_loading.get()>
                    <JournalLoading />
                </Show>

                // Show error if any
                <Show when=move || {
                    error.get().is_some()
                }>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <JournalError error_message=err on_retry=handle_retry /> }
                            })
                    }}
                </Show>

                // Show dreams or empty state
                <Show when=move || !is_loading.get() && error.get().is_none() fallback=|| view! {}>
                    <Show
                        when=move || !dreams.get().is_empty()
                        fallback=move || {
                            view! { <JournalEmptyState /> }
                        }
                    >
                        <div class="dreams-grid">
                            {move || {
                                dreams
                                    .get()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, dream)| {
                                        let delay_class = match index % 3 {
                                            0 => "animation-delay-1",
                                            1 => "animation-delay-2",
                                            _ => "animation-delay-3",
                                        };
                                        // Calculate animation delay based on index

                                        view! {
                                            <JournalCard dream=dream animation_delay=delay_class />
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </div>
                    </Show>
                </Show>
            </div>

            // Pagination controls
            <Show when=move || (pagination.get().total_pages > 1)>
                <JournalPagination pagination=pagination on_page_change=handle_page_change />
            </Show>

            // Offline banner that appears at the bottom of the screen
            <Show when=move || is_offline.get()>
                <OfflineBanner retry_action=handle_retry />
            </Show>
        </div>
    }
}
