use leptos::prelude::*;

#[component]
pub fn EntryLoading() -> impl IntoView {
    view! {
        <div class="loading-container">
            <div class="loading"></div>
            <p class="loading-text pulse">"Loading dream details..."</p>
        </div>
    }
}
