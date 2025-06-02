use leptos::prelude::*;

#[component]
pub fn EntryFallback() -> impl IntoView {
    view! {
        <div class="card text-center slide-up">
            <p class="text-error mb-4">"Something went wrong. Please try again."</p>
            <a href="/journal" class="button">
                "Back to Journal"
            </a>
        </div>
    }
}
