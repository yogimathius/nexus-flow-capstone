use crate::components::journal::models::Dream;
use crate::components::markdown::render_markdown;
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
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    let clean_interpretation_preview = |text: &str, max_len: usize| -> String {
        let text = render_markdown(text);
        let text = text
            .replace(r#"<h3 class="md-heading">"#, "")
            .replace(r#"<h4 class="md-subheading">"#, "")
            .replace("</h3>", "")
            .replace("</h4>", "")
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("<em>", "")
            .replace("</em>", "")
            .replace("<br/>", " ");

        if text.len() <= max_len {
            text
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    let date = format_date(&dream.created_at);
    let time = format_time(&dream.created_at);
    let clean_interp = clean_interpretation_preview(&dream.interpretation, 120);

    let (is_favorite, set_is_favorite) = signal(dream.is_favorite);

    let (dream_tags, _) = signal(dream.tags.clone());

    let toggle_favorite = move |_| {
        set_is_favorite.update(|f| *f = !*f);
    };

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

                <button
                    type="button"
                    class=move || {
                        format!("favorite-btn {}", if is_favorite.get() { "active" } else { "" })
                    }
                    on:click=toggle_favorite
                    aria-label=move || {
                        if is_favorite.get() { "Remove from favorites" } else { "Add to favorites" }
                    }
                    aria-pressed=move || is_favorite.get().to_string()
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill=move || if is_favorite.get() { "currentColor" } else { "none" }
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon>
                    </svg>
                </button>
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
