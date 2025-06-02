use leptos::prelude::*;

#[component]
pub fn EntryError(#[prop(into)] error_message: String) -> impl IntoView {
    view! {
        <div class="card text-center slide-up">
            <h2 class="text-error mb-4">"Error"</h2>
            <p class="mb-4">{error_message}</p>
            <a href="/journal" class="button">
                "Back to Journal"
            </a>
        </div>
    }
}
