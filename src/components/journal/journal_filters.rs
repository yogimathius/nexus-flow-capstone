use leptos::prelude::*;

#[component]
pub fn JournalFilters(
    #[prop(into)] search_query: Signal<String>,
    #[prop(into)] set_search_query: WriteSignal<String>,
    #[prop(into)] date_filter: Signal<String>,
    #[prop(into)] set_date_filter: WriteSignal<String>,
    #[prop(into)] on_search: Callback<web_sys::SubmitEvent>,
    #[prop(into)] on_refresh: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="filters-container slide-up animation-delay-2">
            <form on:submit=move |ev| on_search.run(ev) class="search-form">
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
                </select>
            </div>

            <button
                type="button"
                class="refresh-button"
                title="Refresh dreams"
                on:click=move |_| on_refresh.run(())
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
    }
}
