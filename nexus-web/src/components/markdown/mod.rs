// A reusable markdown rendering component for consistent styling
mod markdown;
mod styled_interpretation;

pub use markdown::{DreamText, InterpretationText, Markdown};
pub use styled_interpretation::StyledInterpretation;

/// Converts markdown text to styled HTML
///
/// This function handles:
/// - Headers (# ## ### and ####)
/// - Emphasis (*italic* and **bold**)
/// - Dividers (---)
/// - Removes <think> tags used by the AI
/// - Preserves line breaks
pub fn render_markdown(text: &str) -> String {
    // Remove <think> blocks used by the AI
    let text = if let Some(start_idx) = text.find("<think>") {
        if let Some(end_idx) = text.find("</think>") {
            let before = &text[0..start_idx];
            let after = &text[(end_idx + 8)..];
            format!("{}{}", before, after)
        } else {
            text.to_string()
        }
    } else {
        text.to_string()
    };

    // Process the text line by line for better header handling
    let mut result = String::new();
    let lines = text.lines();

    for line in lines {
        let processed_line = if line.starts_with("#### ") {
            format!("<h4 class=\"md-subheading\">{}</h4>", &line[5..])
        } else if line.starts_with("### ") {
            format!("<h3 class=\"md-heading\">{}</h3>", &line[4..])
        } else if line.starts_with("## ") {
            format!("<h2 class=\"md-heading\">{}</h2>", &line[3..])
        } else if line.starts_with("# ") {
            format!("<h2 class=\"md-heading\">{}</h2>", &line[2..])
        } else if line == "---" {
            "<hr class=\"md-divider\"/>".to_string()
        } else {
            // For regular lines, just process them normally
            line.to_string()
        };

        result.push_str(&processed_line);
        result.push_str("<br/>");
    }

    // Handle bold and italic
    let mut html_result = String::new();
    let mut chars = result.chars().peekable();
    let mut in_bold = false;
    let mut in_italic = false;

    while let Some(c) = chars.next() {
        if c == '*' {
            if chars.peek() == Some(&'*') {
                // Bold **
                chars.next(); // Skip the second *
                if in_bold {
                    html_result.push_str("</strong>");
                    in_bold = false;
                } else {
                    html_result.push_str("<strong>");
                    in_bold = true;
                }
            } else {
                // Italic *
                if in_italic {
                    html_result.push_str("</em>");
                    in_italic = false;
                } else {
                    html_result.push_str("<em>");
                    in_italic = true;
                }
            }
        } else {
            html_result.push(c);
        }
    }

    // Close any unclosed tags
    if in_bold {
        html_result.push_str("</strong>");
    }
    if in_italic {
        html_result.push_str("</em>");
    }

    // Fix consecutive line breaks to avoid too much spacing
    html_result
        .replace("<br/><br/>", "<br/>")
        .replace("<br/></h3>", "</h3>")
        .replace("<br/></h4>", "</h4>")
        .replace("<br/></h2>", "</h2>")
        .replace("</h3><br/>", "</h3>")
        .replace("</h4><br/>", "</h4>")
        .replace("</h2><br/>", "</h2>")
}
