use crate::components::badges::{Badge, BadgeVariant};
use leptos::prelude::*;

#[component]
pub fn DreamResult(
    interpretation: Signal<Option<String>>,
    saved_dream_id: Signal<Option<String>>,
) -> impl IntoView {
    move || interpretation.get().map(|text| view! {
        <div class="card fade-in">
            <h3 class="card-header">"Dream Interpretation"</h3>
            <div class="prose" inner_html=text></div>
            {saved_dream_id.get().map(|id| view! {
                <div class="card-footer">
                    <div class="flex items-center gap-2">
                        <Badge variant=BadgeVariant::Dream>"Saved"</Badge>
                        <span class="text-secondary">{"Dream saved to journal with ID: "}{id}</span>
                    </div>
                </div>
            })}
        </div>
    })
} 