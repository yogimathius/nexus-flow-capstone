use leptos::prelude::*;

/// The journal currently supports text search and refresh. Other filters can be
/// surfaced when they have matching API behavior and persistent data.
#[component]
pub fn JournalFilters(
    #[prop(into)] search_query: Signal<String>,
    #[prop(into)] set_search_query: WriteSignal<String>,
    #[prop(into)] on_search: Callback<web_sys::SubmitEvent>,
    #[prop(into)] on_refresh: Callback<()>,
) -> impl IntoView {
    let (input, set_input) = signal(search_query.get_untracked());

    view! {
        <div class="filters-container">
            <form
                class="search-form"
                on:submit=move |event| {
                    event.prevent_default();
                    set_search_query.set(input.get());
                    on_search.run(event);
                }
            >
                <div class="search-input-wrapper">
                    <input
                        type="search"
                        class="search-input"
                        placeholder="Search dreams..."
                        aria-label="Search dreams"
                        prop:value=move || input.get()
                        on:input=move |event| set_input.set(event_target_value(&event))
                    />
                    <button type="submit" class="search-button" aria-label="Search">
                        "Search"
                    </button>
                </div>
            </form>
            <button
                type="button"
                class="refresh-button"
                title="Refresh dreams"
                aria-label="Refresh dreams"
                on:click=move |_| on_refresh.run(())
            >
                "Refresh"
            </button>
        </div>
    }
}
