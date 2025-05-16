use crate::components::dream::constants::DREAM_TIPS;
use leptos::prelude::*;

#[component]
pub fn DreamTips() -> impl IntoView {
    view! {
        <div class="card">
            <h3 class="card-header">"Dream Interpretation Tips"</h3>
            <ul class="stack gap-2">
                {DREAM_TIPS.iter().map(|tip| view! {
                    <li class="flex gap-2">
                        <span class="text-primary">{"•"}</span>
                        {tip.to_string()}
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
} 