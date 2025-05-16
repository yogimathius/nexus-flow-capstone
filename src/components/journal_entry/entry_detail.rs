use crate::components::journal_entry::models::Dream;
use crate::components::journal_entry::EntryNavigation;
use crate::components::markdown::{DreamText, InterpretationText};
use leptos::prelude::*;

#[component]
pub fn EntryDetail(
    #[prop(into)] dream: Dream,
    #[prop(into)] formatted_date: String,
) -> impl IntoView {
    view! {
        <div class="stack gap-6">
            <EntryNavigation date=formatted_date />

            <div class="card slide-up animation-delay-1">
                <h2 class="mb-4">"Dream"</h2>
                <DreamText content=dream.text.clone() />
            </div>

            <div class="card slide-up animation-delay-2">
                <h2 class="mb-4">"Interpretation"</h2>
                <InterpretationText content=dream.interpretation.clone() />
            </div>

            <div class="flex gap-4 justify-center slide-up animation-delay-3">
                <a href="/interpret" class="button">
                    "Interpret Another Dream"
                </a>
                <a href="/journal" class="button-outline">
                    "Return to Journal"
                </a>
            </div>
        </div>
    }
}
