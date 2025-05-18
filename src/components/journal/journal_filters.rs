use leptos::html::Input;
use leptos::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent, MouseEvent};

// Define a list of common dream categories
const DREAM_CATEGORIES: [&str; 6] = [
    "All Categories",
    "Flying",
    "Water",
    "Transformation",
    "Meeting Guides",
    "Cosmic",
];

// Debounce function to avoid too many search requests
fn use_debounce(callback: Callback<String>, ms: u32) -> Callback<String> {
    let (timer_id, set_timer_id) = signal(None::<i32>);

    Callback::new(move |value: String| {
        // Clear previous timer if it exists
        if let Some(id) = timer_id.get() {
            let window = web_sys::window().unwrap();
            window.clear_timeout_with_handle(id);
        }

        // Clone value and callback for the closure
        let value_clone = value.clone();
        let callback_clone = callback.clone();

        // Set a new timer
        let window = web_sys::window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            callback_clone.run(value_clone.clone());
        }) as Box<dyn FnMut()>);

        let new_timer_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                ms as i32,
            )
            .unwrap();

        // Need to forget the closure to keep it alive
        closure.forget();

        set_timer_id.set(Some(new_timer_id));
    })
}

#[component]
pub fn JournalFilters(
    #[prop(into)] search_query: Signal<String>,
    #[prop(into)] set_search_query: WriteSignal<String>,
    #[prop(into)] date_filter: Signal<String>,
    #[prop(into)] set_date_filter: WriteSignal<String>,
    #[prop(into, optional)] category_filter: Option<Signal<String>>,
    #[prop(into, optional)] set_category_filter: Option<WriteSignal<String>>,
    #[prop(into, optional)] selected_tags: Option<Signal<Vec<String>>>,
    #[prop(into, optional)] set_selected_tags: Option<WriteSignal<Vec<String>>>,
    #[prop(into, optional)] favorites_only: Option<Signal<bool>>,
    #[prop(into, optional)] set_favorites_only: Option<WriteSignal<bool>>,
    #[prop(into)] on_search: Callback<web_sys::SubmitEvent>,
    #[prop(into)] on_refresh: Callback<()>,
    #[prop(into, optional)] on_category_change: Option<Callback<String>>,
    #[prop(into, optional)] on_tags_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] on_favorites_toggle: Option<Callback<bool>>,
) -> impl IntoView {
    // Create refs for the input elements
    let search_input_ref = NodeRef::<Input>::new();

    // Default signals for optional props
    let (local_category_filter, local_set_category_filter) = signal("All Categories".to_string());
    let (local_selected_tags, local_set_selected_tags) = signal(Vec::<String>::new());
    let (local_favorites_only, local_set_favorites_only) = signal(false);

    // Use provided signals or locals
    let category_filter =
        category_filter.unwrap_or_else(|| Signal::derive(move || local_category_filter.get()));
    let set_category_filter = set_category_filter.unwrap_or(local_set_category_filter);

    let selected_tags =
        selected_tags.unwrap_or_else(|| Signal::derive(move || local_selected_tags.get()));
    let set_selected_tags = set_selected_tags.unwrap_or(local_set_selected_tags);

    let favorites_only =
        favorites_only.unwrap_or_else(|| Signal::derive(move || local_favorites_only.get()));
    let set_favorites_only = set_favorites_only.unwrap_or(local_set_favorites_only);

    // New tag input
    let (tag_input, set_tag_input) = signal(String::new());

    // Advanced search mode
    let (advanced_mode, set_advanced_mode) = signal(false);

    // Create a debounced search handler
    let debounced_search = use_debounce(
        Callback::new(move |value: String| {
            set_search_query.set(value);
            // Trigger search after debounce
            let ev = web_sys::SubmitEvent::new("submit").unwrap();
            on_search.run(ev);
        }),
        500, // 500ms debounce delay
    );

    // Handle input with debounce
    let handle_search_input = move |ev: Event| {
        let value = event_target_value(&ev);
        debounced_search.run(value);
    };

    // Handle adding a tag
    let add_tag = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();

            let tag = tag_input.get().trim().to_string();
            if !tag.is_empty() && !selected_tags.get().contains(&tag) {
                set_selected_tags.update(|tags| {
                    tags.push(tag.clone());
                });
                set_tag_input.set("".to_string());

                // Notify parent component if callback exists
                if let Some(callback) = on_tags_change.as_ref() {
                    callback.run(selected_tags.get());
                }
            }
        }
    };

    // Handle removing a tag
    let remove_tag = move |tag: String| {
        set_selected_tags.update(|tags| {
            tags.retain(|t| t != &tag);
        });

        // Notify parent component if callback exists
        if let Some(callback) = on_tags_change.as_ref() {
            callback.run(selected_tags.get());
        }
    };

    // Toggle advanced search mode
    let toggle_advanced_mode = move |_: MouseEvent| {
        set_advanced_mode.update(|mode| *mode = !*mode);
    };

    // Clear all filters
    let clear_filters = move |_: MouseEvent| {
        set_search_query.set("".to_string());
        set_date_filter.set("".to_string());
        set_category_filter.set("All Categories".to_string());
        set_selected_tags.set(Vec::new());
        set_favorites_only.set(false);

        // Reset input field
        if let Some(input) = search_input_ref.get() {
            input.set_value("");
        }

        // Notify parent components if callbacks exist
        if let Some(callback) = on_category_change.as_ref() {
            callback.run("All Categories".to_string());
        }

        if let Some(callback) = on_tags_change.as_ref() {
            callback.run(Vec::new());
        }

        if let Some(callback) = on_favorites_toggle.as_ref() {
            callback.run(false);
        }

        // Refresh the list
        on_refresh.run(());
    };

    // Handle category change
    let handle_category_select = move |ev: Event| {
        let value = event_target_value(&ev);
        set_category_filter.set(value.clone());

        if let Some(callback) = on_category_change.as_ref() {
            callback.run(value);
        } else {
            on_refresh.run(());
        }
    };

    // Handle favorites toggle
    let handle_favorites = move |ev: Event| {
        let checked = event_target_checked(&ev);
        set_favorites_only.set(checked);

        if let Some(callback) = on_favorites_toggle.as_ref() {
            callback.run(checked);
        } else {
            on_refresh.run(());
        }
    };

    view! {
        <div class="filters-container slide-up animation-delay-2">
            <form on:submit=move |ev| on_search.run(ev) class="search-form">
                <div class="search-input-wrapper">
                    <input
                        type="text"
                        placeholder="Search dreams..."
                        class="search-input"
                        prop:value=search_query
                        on:input=handle_search_input
                        node_ref=search_input_ref
                        aria-label="Search dreams"
                    />
                    <button type="submit" class="search-button" aria-label="Search">
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
                            aria-hidden="true"
                        >
                            <circle cx="11" cy="11" r="8"></circle>
                            <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                        </svg>
                    </button>
                </div>

                <button
                    type="button"
                    class="advanced-search-toggle"
                    on:click=toggle_advanced_mode
                    aria-expanded=move || advanced_mode.get().to_string()
                    aria-controls="advanced-search-panel"
                >
                    {move || if advanced_mode.get() { "Simple Search" } else { "Advanced Search" }}
                </button>

                <Show when=move || advanced_mode.get()>
                    <div id="advanced-search-panel" class="advanced-search-panel slide-down">
                        <div class="filter-row">
                            <div class="filter-group">
                                <label for="category-filter" class="filter-label">
                                    Category
                                </label>
                                <select
                                    id="category-filter"
                                    class="filter-select"
                                    on:change=handle_category_select
                                >
                                    {DREAM_CATEGORIES
                                        .iter()
                                        .map(|category| {
                                            let category_str = category.to_string();
                                            let category_for_value = category_str.clone();
                                            let category_for_comparison = category_str.clone();
                                            view! {
                                                <option
                                                    value=category_for_value
                                                    selected=move || {
                                                        category_filter.get() == category_for_comparison
                                                    }
                                                >
                                                    {category_str}
                                                </option>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </select>
                            </div>

                            <div class="filter-group">
                                <label for="date-filter" class="filter-label">
                                    Time Period
                                </label>
                                <select
                                    id="date-filter"
                                    class="filter-select"
                                    prop:value=date_filter
                                    on:change=move |ev| {
                                        set_date_filter.set(event_target_value(&ev));
                                        on_refresh.run(());
                                    }
                                >
                                    <option value="">"All time"</option>
                                    <option value="day">"Last 24 hours"</option>
                                    <option value="week">"Last week"</option>
                                    <option value="month">"Last month"</option>
                                    <option value="year">"Last year"</option>
                                </select>
                            </div>
                        </div>

                        <div class="filter-row mt-4">
                            <div class="filter-group">
                                <div class="flex items-center gap-2 mb-2">
                                    <input
                                        type="checkbox"
                                        id="favorites-filter"
                                        checked=favorites_only
                                        on:change=handle_favorites
                                    />
                                    <label for="favorites-filter" class="filter-label m-0">
                                        "Favorites Only"
                                    </label>
                                </div>
                            </div>
                        </div>

                        <div class="tags-section">
                            <label for="tag-input" class="filter-label">
                                Tags
                            </label>
                            <div class="tag-input-container">
                                <input
                                    type="text"
                                    id="tag-input"
                                    class="tag-input"
                                    placeholder="Add tag and press Enter"
                                    prop:value=tag_input
                                    on:input=move |ev| set_tag_input.set(event_target_value(&ev))
                                    on:keydown=add_tag
                                />
                            </div>

                            <div class="selected-tags">
                                {move || {
                                    selected_tags
                                        .get()
                                        .iter()
                                        .map(|tag| {
                                            let tag_clone = tag.clone();
                                            view! {
                                                <div class="selected-tag">
                                                    <span>{tag_clone.clone()}</span>
                                                    <button
                                                        type="button"
                                                        class="remove-tag-btn"
                                                        on:click={
                                                            let tag_for_removal = tag_clone.clone();
                                                            move |_| remove_tag(tag_for_removal.clone())
                                                        }
                                                        aria-label=format!("Remove tag {}", tag_clone.clone())
                                                    >
                                                        "×"
                                                    </button>
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }}
                            </div>
                        </div>

                        <button type="button" class="clear-filters-btn" on:click=clear_filters>
                            "Clear All Filters"
                        </button>
                    </div>
                </Show>
            </form>

            <button
                type="button"
                class="refresh-button"
                title="Refresh dreams"
                on:click=move |_| on_refresh.run(())
                aria-label="Refresh dreams"
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
                    aria-hidden="true"
                >
                    <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
                    <path d="M3 3v5h5"></path>
                </svg>
            </button>
        </div>
    }
}
