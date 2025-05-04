#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::dream_journal::{Dream, DreamJournalPage, Pagination};
    use leptos::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use wasm_bindgen_test::*;

    // Set up wasm specific test configuration
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    // Helper function to create a test dream
    fn create_test_dream(id: usize) -> Dream {
        // Create a timestamp that's `id` days ago
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days_ago = id as u64 * 24 * 60 * 60;
        let timestamp = now - days_ago;
        let date_string = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
            .unwrap()
            .to_rfc3339();

        Dream {
            id: format!("dream-{}", id),
            text: format!("This is test dream number {}", id),
            interpretation: format!(
                "Interpretation for dream {}: This dream symbolizes testing.",
                id
            ),
            created_at: date_string,
        }
    }

    // Test dream formatting functions
    #[wasm_bindgen_test]
    fn test_format_date() {
        // Create a component runtime for testing
        let runtime = create_runtime();

        // Define a component that uses the format_date function
        #[component]
        fn TestComponent() -> impl IntoView {
            let format_date = |date_str: &str| -> String {
                if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                    date.format("%B %d, %Y at %H:%M").to_string()
                } else {
                    date_str.to_string()
                }
            };

            // Use a fixed date for testing
            let test_date = "2023-01-15T12:30:45+00:00";
            let formatted = format_date(test_date);

            view! { <div>{formatted}</div> }
        }

        // Mount the test component
        let mut scope = create_scope(runtime.clone(), |cx| {
            mount_to_body(cx, || view! { cx, <TestComponent /> });
        });

        // Check if the date is formatted correctly
        let document = web_sys::window().unwrap().document().unwrap();
        let div_text = document
            .query_selector("div")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap();

        assert!(div_text.contains("January 15, 2023"));
        assert!(div_text.contains("12:30"));

        runtime.dispose();
    }

    // Test truncate text function
    #[wasm_bindgen_test]
    fn test_truncate() {
        // Create a component runtime for testing
        let runtime = create_runtime();

        // Define a component that uses the truncate function
        #[component]
        fn TestComponent() -> impl IntoView {
            let truncate = |text: &str, max_len: usize| -> String {
                if text.len() <= max_len {
                    text.to_string()
                } else {
                    format!("{}...", &text[0..max_len])
                }
            };

            let long_text = "This is a very long text that should be truncated";
            let short_text = "Short";

            let truncated_long = truncate(long_text, 10);
            let truncated_short = truncate(short_text, 10);

            view! {
                <div id="long">{truncated_long}</div>
                <div id="short">{truncated_short}</div>
            }
        }

        // Mount the test component
        let mut scope = create_scope(runtime.clone(), |cx| {
            mount_to_body(cx, || view! { cx, <TestComponent /> });
        });

        // Check if the texts are truncated correctly
        let document = web_sys::window().unwrap().document().unwrap();
        let long_div = document
            .get_element_by_id("long")
            .unwrap()
            .text_content()
            .unwrap();
        let short_div = document
            .get_element_by_id("short")
            .unwrap()
            .text_content()
            .unwrap();

        assert_eq!(long_div, "This is a ...");
        assert_eq!(short_div, "Short");

        runtime.dispose();
    }

    // Test pagination logic
    #[wasm_bindgen_test]
    fn test_pagination() {
        // Create a component runtime for testing
        let runtime = create_runtime();

        // Define a test component that simulates pagination
        #[component]
        fn TestComponent() -> impl IntoView {
            let (pagination, set_pagination) = signal(Pagination {
                current_page: 1,
                total_pages: 5,
                items_per_page: 10,
            });

            let handle_page_change = move |new_page: usize| {
                set_pagination.update(|p| {
                    p.current_page = new_page;
                });
            };

            // Create a simple pagination UI
            view! {
                <div>
                    <button
                        id="prev"
                        on:click=move |_| {
                            let current = pagination.get().current_page;
                            if current > 1 {
                                handle_page_change(current - 1);
                            }
                        }
                    >
                        "Previous"
                    </button>

                    <span id="page-info">
                        {"Page "}{move || pagination.get().current_page}{" of "}{move || pagination.get().total_pages}
                    </span>

                    <button
                        id="next"
                        on:click=move |_| {
                            let current = pagination.get().current_page;
                            let total = pagination.get().total_pages;
                            if current < total {
                                handle_page_change(current + 1);
                            }
                        }
                    >
                        "Next"
                    </button>
                </div>
            }
        }

        // Mount the test component
        let mut scope = create_scope(runtime.clone(), |cx| {
            mount_to_body(cx, || view! { cx, <TestComponent /> });
        });

        // Test the initial state
        let document = web_sys::window().unwrap().document().unwrap();
        let page_info = document
            .get_element_by_id("page-info")
            .unwrap()
            .text_content()
            .unwrap();
        assert_eq!(page_info, "Page 1 of 5");

        // Click the next button and check if page increases
        let next_button = document.get_element_by_id("next").unwrap();
        next_button
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();

        // Wait for the update
        std::thread::sleep(Duration::from_millis(10));

        let page_info = document
            .get_element_by_id("page-info")
            .unwrap()
            .text_content()
            .unwrap();
        assert_eq!(page_info, "Page 2 of 5");

        // Click the previous button and check if page decreases
        let prev_button = document.get_element_by_id("prev").unwrap();
        prev_button
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .click();

        // Wait for the update
        std::thread::sleep(Duration::from_millis(10));

        let page_info = document
            .get_element_by_id("page-info")
            .unwrap()
            .text_content()
            .unwrap();
        assert_eq!(page_info, "Page 1 of 5");

        runtime.dispose();
    }
}
