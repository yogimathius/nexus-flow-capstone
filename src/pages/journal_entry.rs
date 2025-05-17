use crate::components::journal_entry::{
    use_dream_data, EntryDetail, EntryError, EntryFallback, EntryHeader, EntryLoading,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn JournalEntryPage() -> impl IntoView {
    let params = use_params_map();
    let dream_id = params.with(|params| {
        params
            .get("entry")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let state = use_dream_data(dream_id);

    view! {
        <div class="fade-in">
            <EntryHeader
                title="Dream Journal Entry"
                subtitle="Explore the interpretation of your dream in detail."
            />

            <section class="section">
                <div class="container">
                    // Loading state
                    <Show when=move || state.is_loading.get() fallback=|| ()>
                        <EntryLoading />
                    </Show>

                    // Error state
                    <Show
                        when=move || !state.is_loading.get() && state.error.get().is_some()
                        fallback=|| ()
                    >
                        {move || {
                            let err = state.error.get().unwrap_or_default();
                            view! { <EntryError error_message=err /> }
                        }}
                    </Show>

                    // Dream data state
                    <Show
                        when=move || {
                            !state.is_loading.get() && state.error.get().is_none()
                                && state.dream.get().is_some()
                        }
                        fallback=|| ()
                    >
                        {move || {
                            let dream_data = state.dream.get().unwrap();
                            let formatted_date = (state.format_date)(&dream_data.created_at);
                            view! { <EntryDetail dream=dream_data formatted_date=formatted_date /> }
                        }}
                    </Show>

                    // Fallback state (no data, no error, not loading)
                    <Show
                        when=move || {
                            !state.is_loading.get() && state.error.get().is_none()
                                && state.dream.get().is_none()
                        }
                        fallback=|| ()
                    >
                        <EntryFallback />
                    </Show>
                </div>
            </section>
        </div>
    }
}
