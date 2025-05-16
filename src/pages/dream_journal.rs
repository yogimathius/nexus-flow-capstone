use crate::components::journal::JournalList;
use leptos::prelude::*;

#[component]
pub fn DreamJournalPage() -> impl IntoView {
    view! {
        <div class="dream-journal fade-in">
            <div class="journal-header bg-gradient">
                <div class="container">
                    <h1 class="slide-up">"Dream Journal"</h1>
                    <p class="intro-text slide-up animation-delay-1">
                        "Your personal collection of dreams and their interpretations."
                    </p>
                </div>
            </div>

            <JournalList />
        </div>
    }
}
