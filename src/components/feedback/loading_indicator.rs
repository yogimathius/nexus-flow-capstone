use leptos::prelude::*;

#[component]
pub fn LoadingIndicator() -> impl IntoView {
    view! {
        <div class="loading-spinner-container">
            <div class="loading" aria-label="Loading"></div>
        </div>
    }
} 