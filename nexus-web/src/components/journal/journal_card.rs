use crate::components::journal::models::Dream;
use leptos::prelude::*;

#[component]
pub fn JournalCard(
    #[prop(into)] dream: Dream,
    #[prop(into)] animation_delay: String,
) -> impl IntoView {
    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y").to_string()
        } else {
            date_str.to_string()
        }
    };

    let format_time = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%H:%M").to_string()
        } else {
            "".to_string()
        }
    };

    let truncate = |text: &str, max_len: usize| -> String {
        let mut chars = text.chars();
        let prefix: String = chars.by_ref().take(max_len).collect();
        if chars.next().is_some() { format!("{prefix}…") } else { prefix }
    };

    let clean_interpretation_preview = |text: &str, max_len: usize| -> String {
        let plain = text.replace('*', "").replace('#', "").replace('\n', " ");
        truncate(&plain, max_len)
    };

    let date = format_date(&dream.created_at);
    let time = format_time(&dream.created_at);
    let clean_interp = clean_interpretation_preview(&dream.interpretation, 120);

    let (dream_tags, _) = signal(dream.tags.clone());

    view! {
        <div class=format!("dream-card slide-up {}", animation_delay)>
            <div class="dream-card-header">
                <div class="dream-card-badges">
                    <span class="badge dream">"Dream"</span>

                    {dream
                        .category
                        .clone()
                        .map(|category| {
                            view! { <span class="badge category">{category}</span> }
                        })}
                </div>

                <div class="dream-meta">
                    <div class="dream-date">{date}</div>
                    <div class="dream-time">{time}</div>
                </div>

            </div>

            <h3 class="dream-title">{truncate(&dream.text, 60)}</h3>
            <p class="dream-interpretation">{clean_interp}</p>

            <Show when=move || !dream_tags.get().is_empty()>
                <div class="dream-tags">
                    {move || {
                        dream_tags
                            .get()
                            .iter()
                            .map(|tag| {
                                let tag = tag.clone();
                                view! { <span class="dream-tag">{tag}</span> }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </Show>

            <a href=format!("/journal/{}", dream.id) class="view-dream-link">
                "View Full Dream"
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <path d="M5 12h14"></path>
                    <path d="m12 5 7 7-7 7"></path>
                </svg>
            </a>
        </div>
    }
}
