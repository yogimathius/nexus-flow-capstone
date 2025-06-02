use leptos::prelude::*;

#[component]
pub fn EntryHeader(#[prop(into)] title: String, #[prop(into)] subtitle: String) -> impl IntoView {
    view! {
        <section class="section-sm bg-gradient">
            <div class="container">
                <div class="stack items-center text-center">
                    <h1 class="slide-up">{title}</h1>
                    <p class="slide-up animation-delay-1">{subtitle}</p>
                </div>
            </div>
        </section>
    }
}
