use crate::components::journal::models::Dream;
use crate::components::markdown::render_markdown;
use leptos::prelude::*;

#[component]
pub fn JournalCard(
    #[prop(into)] dream: Dream,
    #[prop(into)] animation_delay: String,
) -> impl IntoView {
    // Format date from ISO string to readable format
    let format_date = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%B %d, %Y").to_string()
        } else {
            date_str.to_string()
        }
    };

    // Format time from ISO string
    let format_time = |date_str: &str| -> String {
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
            date.format("%H:%M").to_string()
        } else {
            "".to_string()
        }
    };

    // Truncate text to a specified length
    let truncate = |text: &str, max_len: usize| -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    // Clean interpretation preview (extract first paragraph without markdown)
    let clean_interpretation_preview = |text: &str, max_len: usize| -> String {
        // Remove markdown and HTML tags for cleaner preview
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

        // Truncate if needed
        if text.len() <= max_len {
            text
        } else {
            format!("{}...", &text[0..max_len])
        }
    };

    let date = format_date(&dream.created_at);
    let time = format_time(&dream.created_at);
    let clean_interp = clean_interpretation_preview(&dream.interpretation, 120);

    view! {
        <div class=format!("dream-card slide-up {}", animation_delay)>
            <div class="dream-card-header">
                <span class="badge dream">"Dream"</span>
                <div class="dream-meta">
                    <div class="dream-date">{date}</div>
                    <div class="dream-time">{time}</div>
                </div>
            </div>
            <h3 class="dream-title">{truncate(&dream.text, 60)}</h3>
            <p class="dream-interpretation">{clean_interp}</p>
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
                >
                    <path d="M5 12h14"></path>
                    <path d="m12 5 7 7-7 7"></path>
                </svg>
            </a>
        </div>
    }
}
