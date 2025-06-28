use crate::components::journal::JournalList;
use leptos::prelude::*;

#[component]
pub fn DreamJournalPage() -> impl IntoView {
    view! {
        <div class="dream-journal fade-in">
            <section class="section bg-gradient">
                <div class="container">
                    <div class="stack items-center text-center">
                        <h1 class="slide-up">"Dream Journal"</h1>
                        <p class="slide-up animation-delay-1">
                            "Your personal collection of dreams and their interpretations."
                        </p>
                    </div>
                </div>
            </section>

            <JournalList />
        </div>
    }
}
