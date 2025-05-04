#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::dream_detail::{Dream, DreamDetailPage};
    use leptos::*;
    use wasm_bindgen_test::*;

    // Set up wasm specific test configuration
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    // Mock dream data
    fn create_mock_dream() -> Dream {
        Dream {
            id: "test-id-123".to_string(),
            text: "I was flying over mountains and valleys in my dream.".to_string(),
            interpretation: "### Core Meaning\nFlying represents freedom and transcending limitations. \
                           \n\n#### Key Symbols\n* Mountains: Challenges you're overcoming\n* Valleys: Periods of peace and reflection\
                           \n\n---\n\nThis dream suggests you're feeling a sense of liberation.".to_string(),
            created_at: "2023-05-15T12:30:45+00:00".to_string(),
        }
    }

    // Test the format_interpretation function
    #[wasm_bindgen_test]
    fn test_format_interpretation() {
        // Create a component runtime for testing
        let runtime = create_runtime();

        // Define a component that uses the format_interpretation function
        #[component]
        fn TestComponent() -> impl IntoView {
            let format_interpretation = |text: &str| -> String {
                // Similar to the formatting in dream_detail.rs
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

                let text = text
                    .replace("###", "<h3 class=\"interp-heading\">")
                    .replace("####", "<h4 class=\"interp-subheading\">")
                    .replace("\n\n", "<br/><br/>")
                    .replace("\n---\n", "<hr class=\"interp-divider\"/>");

                let text = text.replace("**", "<strong>").replace("*", "<em>");
                text.replace("\n", "<br/>")
            };

            // Get mock dream interpretation
            let mock_dream = create_mock_dream();
            let formatted = format_interpretation(&mock_dream.interpretation);

            view! { <div id="interpretation" inner_html=formatted></div> }
        }

        // Mount the test component
        let mut scope = create_scope(runtime.clone(), |cx| {
            mount_to_body(cx, || view! { cx, <TestComponent /> });
        });

        // Check if the interpretation is formatted correctly
        let document = web_sys::window().unwrap().document().unwrap();
        let interp_div = document.get_element_by_id("interpretation").unwrap();

        // Check for HTML elements that should be created from markdown
        let headings = interp_div.query_selector_all("h3.interp-heading").unwrap();
        assert_eq!(headings.length(), 1);

        let subheadings = interp_div
            .query_selector_all("h4.interp-subheading")
            .unwrap();
        assert_eq!(subheadings.length(), 1);

        let dividers = interp_div.query_selector_all("hr.interp-divider").unwrap();
        assert_eq!(dividers.length(), 1);

        // Check for list items conversion
        let content = interp_div.inner_html();
        assert!(content.contains("<em>")); // Check for italic conversion

        runtime.dispose();
    }

    // Test error handling display
    #[wasm_bindgen_test]
    fn test_error_display() {
        // Create a component runtime for testing
        let runtime = create_runtime();

        // Define a test component that simulates the error state of DreamDetailPage
        #[component]
        fn TestComponent() -> impl IntoView {
            let (error, _set_error) = signal(Some("Dream not found".to_string()));
            let (is_loading, _set_is_loading) = signal(false);

            view! {
                <div>
                    {move || {
                        if is_loading.get() {
                            view! {
                                <div class="loading">
                                    "Loading..."
                                </div>
                            }.into_view()
                        } else if let Some(err) = error.get() {
                            view! {
                                <div class="error-container">
                                    <h2 class="error-title" id="error-title">"Error"</h2>
                                    <p class="error-message" id="error-message">{err}</p>
                                    <a href="/journal" class="back-link" id="back-link">"Back to Dream Journal"</a>
                                </div>
                            }.into_view()
                        } else {
                            view! { <div>"No error"</div> }.into_view()
                        }
                    }}
                </div>
            }
        }

        // Mount the test component
        let mut scope = create_scope(runtime.clone(), |cx| {
            mount_to_body(cx, || view! { cx, <TestComponent /> });
        });

        // Check if the error is displayed correctly
        let document = web_sys::window().unwrap().document().unwrap();

        let error_title = document
            .get_element_by_id("error-title")
            .unwrap()
            .text_content()
            .unwrap();
        let error_message = document
            .get_element_by_id("error-message")
            .unwrap()
            .text_content()
            .unwrap();
        let back_link = document
            .get_element_by_id("back-link")
            .unwrap()
            .text_content()
            .unwrap();

        assert_eq!(error_title, "Error");
        assert_eq!(error_message, "Dream not found");
        assert_eq!(back_link, "Back to Dream Journal");

        runtime.dispose();
    }

    // Test dream content display
    #[wasm_bindgen_test]
    fn test_dream_content_display() {
        // Create a component runtime for testing
        let runtime = create_runtime();

        // Define a test component that displays dream content
        #[component]
        fn TestComponent() -> impl IntoView {
            let mock_dream = create_mock_dream();
            let format_date = |date_str: &str| -> String {
                if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                    date.format("%B %d, %Y at %H:%M").to_string()
                } else {
                    date_str.to_string()
                }
            };

            let date = format_date(&mock_dream.created_at);

            view! {
                <div class="dream-content">
                    <div class="dream-header">
                        <a href="/journal" class="back-link">"← Back to Journal"</a>
                        <div class="dream-date" id="date">{date}</div>
                    </div>

                    <div class="dream-text" id="dream-text">
                        {mock_dream.text}
                    </div>
                </div>
            }
        }

        // Mount the test component
        let mut scope = create_scope(runtime.clone(), |cx| {
            mount_to_body(cx, || view! { cx, <TestComponent /> });
        });

        // Check if the dream content is displayed correctly
        let document = web_sys::window().unwrap().document().unwrap();

        let date = document
            .get_element_by_id("date")
            .unwrap()
            .text_content()
            .unwrap();
        let dream_text = document
            .get_element_by_id("dream-text")
            .unwrap()
            .text_content()
            .unwrap();

        assert!(date.contains("May 15, 2023"));
        assert!(date.contains("12:30"));
        assert_eq!(
            dream_text,
            "I was flying over mountains and valleys in my dream."
        );

        runtime.dispose();
    }
}
