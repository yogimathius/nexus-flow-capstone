use leptos::prelude::*;

#[component]
pub fn JournalEmptyState() -> impl IntoView {
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
