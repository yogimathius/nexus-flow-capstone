use leptos::prelude::*;

#[component]
pub fn StyledInterpretation(#[prop(into)] content: String) -> impl IntoView {
    view! {
        <div class="interpretation-container">
            <div class="interpretation-content">{render_interpretation(content)}</div>
        </div>
    }
}

// Process and render interpretation content with styled sections
fn render_interpretation(content: String) -> impl IntoView {
    // Split content into sections
    let parts: Vec<&str> = content.split("\n\n").collect();

    let mut sections = Vec::new();
    let mut current_heading = None;
    let mut current_items = Vec::new();

    for part in parts {
        if part.starts_with("# ") || part.starts_with("## ") || part.starts_with("### ") {
            // If we were building a section, finish it
            if !current_items.is_empty() {
                sections.push((current_heading.clone(), current_items.clone()));
                current_items = Vec::new();
            }
            current_heading = Some(part.trim_start_matches('#').trim().to_string());
        } else if part.starts_with("- ") || part.starts_with("* ") {
            // Process list items
            let item = part.trim_start_matches(|c| c == '-' || c == '*').trim();
            current_items.push(item.to_string());
        } else if !part.trim().is_empty() {
            // Regular paragraph
            if current_heading.is_none() {
                // This is the intro paragraph
                sections.push((
                    Some("Introduction".to_string()),
                    vec![part.trim().to_string()],
                ));
            } else {
                current_items.push(part.trim().to_string());
            }
        }
    }

    // Add the last section if not empty
    if !current_items.is_empty() {
        sections.push((current_heading, current_items));
    }

    // Render the sections
    view! {
        <div class="interpretation-sections">
            {sections
                .into_iter()
                .map(|(heading, items)| {
                    let heading_text = heading.unwrap_or_default();
                    view! {
                        <div class="interpretation-section fade-in">
                            <h2 class="interpretation-heading">{heading_text}</h2>
                            <div class="interpretation-content">
                                {items
                                    .into_iter()
                                    .map(|item| {
                                        if item.contains(":") && !item.starts_with("http") {
                                            let parts: Vec<&str> = item.splitn(2, ':').collect();
                                            if parts.len() == 2 {
                                                let term = parts[0].trim().to_string();
                                                let definition = parts[1].trim().to_string();
                                                let html = format!(
                                                    "<div class=\"interpretation-term-group\"><span class=\"interpretation-term\">{}</span><span class=\"interpretation-definition\">{}</span></div>",
                                                    term,
                                                    definition,
                                                );

                                                view! { <div inner_html=html></div> }
                                            } else {
                                                let html = format!(
                                                    "<div class=\"interpretation-text-wrapper\"><p class=\"interpretation-text\">{}</p></div>",
                                                    item,
                                                );

                                                view! { <div inner_html=html></div> }
                                            }
                                        } else {
                                            let html = format!(
                                                "<div class=\"interpretation-text-wrapper\"><p class=\"interpretation-text\">{}</p></div>",
                                                item,
                                            );

                                            view! { <div inner_html=html></div> }
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
