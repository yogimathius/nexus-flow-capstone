use leptos::prelude::*;

#[component]
pub fn JournalError(#[prop(into)] error_message: String, on_retry: Callback<()>) -> impl IntoView {
    view! {
        <div class="error-message slide-up">
            <h3>"Error Loading Dreams"</h3>
            <p>{error_message}</p>
            <div class="error-actions">
                <button type="button" class="retry-button" on:click=move |_| on_retry.run(())>
                    "Check Server & Retry"
                </button>
                <a href="/interpret" class="interpret-link">
                    "Create New Dream Instead"
                </a>
            </div>
        </div>
    }
}
