use super::render_markdown;
use leptos::prelude::*;

/// A component that renders text with markdown formatting
#[component]
pub fn Markdown(
    #[prop(into)] content: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let formatted_content = render_markdown(&content);
    let class_value = format!("prose {}", class.unwrap_or_default());

    view! {
        <div class={class_value} inner_html={formatted_content}></div>
    }
}

/// A component specifically for rendering dream text
#[component]
pub fn DreamText(
    #[prop(into)] content: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = format!("dream-text {}", class.unwrap_or_default());

    view! {
        <div class={class_value}>
            {content}
        </div>
    }
}

/// A component specifically for rendering dream interpretations
#[component]
pub fn InterpretationText(
    #[prop(into)] content: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let formatted_content = render_markdown(&content);
    let class_value = format!(
        "markdown-container interpretation-markdown {}",
        class.unwrap_or_default()
    );

    view! {
        <div class={class_value} inner_html={formatted_content}></div>
    }
}
