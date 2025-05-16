use leptos::prelude::*;

#[component]
pub fn JournalLoading() -> impl IntoView {
    view! {
        <div class="loading-container slide-up">
            <div class="loading">
                <div></div>
                <div></div>
                <div></div>
            </div>
            <p class="loading-text pulse">"Loading your dreams..."</p>
        </div>
    }
}
