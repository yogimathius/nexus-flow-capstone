use leptos::prelude::*;
use wasm_bindgen_test::*;
use web_sys::MouseEvent;

mod setup;
// Explicitly import the functions we need from our setup module
use setup::{
    create_click_event, create_test_signal, has_class, mount_to_container, query_selector,
    with_test_container,
};

// Import button components
use client_dream_ai::components::buttons::{Button, ButtonSize, ButtonVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn button_renders_with_default_variant() {
    with_test_container(|container| {
        // Create a test component
        let button_view = view! { <Button>"Test Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if primary variant class is applied (primary is default)
        assert!(
            has_class(&button, "primary"),
            "Button should have the primary variant class"
        );

        // Check content
        assert_eq!(button.text_content().unwrap(), "Test Button");
    });
}

#[wasm_bindgen_test]
fn button_renders_with_custom_variant() {
    with_test_container(|container| {
        // Create a test component
        let button_view =
            view! { <Button variant=ButtonVariant::Secondary>"Secondary Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if secondary variant class is applied
        assert!(
            has_class(&button, "secondary"),
            "Button should have the secondary variant class"
        );

        // Check content
        assert_eq!(button.text_content().unwrap(), "Secondary Button");
    });
}

#[wasm_bindgen_test]
fn button_renders_with_custom_size() {
    with_test_container(|container| {
        // Create a test component
        let button_view = view! { <Button size=ButtonSize::Large>"Large Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if large size class is applied
        assert!(
            has_class(&button, "large"),
            "Button should have the large size class"
        );

        // Check content
        assert_eq!(button.text_content().unwrap(), "Large Button");
    });
}

#[wasm_bindgen_test]
fn button_handles_click_event() {
    with_test_container(|container| {
        // Create a signal to track clicks
        let clicked = create_test_signal(false);
        let clicked_for_check = clicked.clone();

        // Create a handler function
        let handler = Box::new(move |_: MouseEvent| {
            clicked.set(true);
        }) as Box<dyn Fn(MouseEvent)>;

        // Create a test component with click handler - pass handler directly
        let button_view = view! { <Button on_click=handler>"Click Me"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Simulate a click
        let click_event = create_click_event();
        let _ = button.dispatch_event(&click_event);

        // Check if click handler was called
        assert!(
            clicked_for_check.get(),
            "Click handler should have been called"
        );
    });
}

#[wasm_bindgen_test]
fn button_applies_custom_class() {
    with_test_container(|container| {
        // Create a test component
        let button_view =
            view! { <Button class="custom-class".to_string()>"Custom Class Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if custom class is applied
        assert!(
            has_class(&button, "custom-class"),
            "Button should have the custom class"
        );
    });
}

#[wasm_bindgen_test]
fn button_respects_disabled_state() {
    with_test_container(|container| {
        // Create a signal to track clicks
        let clicked = create_test_signal(false);
        let clicked_for_check = clicked.clone();

        // Create a handler function
        let handler = Box::new(move |_: MouseEvent| {
            clicked.set(true);
        }) as Box<dyn Fn(MouseEvent)>;

        // Create a test component with click handler but disabled - pass handler directly
        let button_view = view! {
            <Button on_click=handler disabled=true>
                "Disabled Button"
            </Button>
        };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if disabled attribute is applied
        assert_eq!(
            button.get_attribute("disabled").unwrap(),
            "",
            "Button should have the disabled attribute"
        );

        // Simulate a click
        let click_event = create_click_event();
        let _ = button.dispatch_event(&click_event);

        // Click handler should not be called when button is disabled
        assert_eq!(
            clicked_for_check.get(),
            false,
            "Click handler should not have been called for disabled button"
        );
    });
}

#[wasm_bindgen_test]
fn button_with_danger_variant() {
    with_test_container(|container| {
        // Create a test component
        let button_view = view! { <Button variant=ButtonVariant::Danger>"Danger Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if danger variant class is applied
        assert!(
            has_class(&button, "danger"),
            "Button should have the danger variant class"
        );

        // Check content
        assert_eq!(button.text_content().unwrap(), "Danger Button");
    });
}

#[wasm_bindgen_test]
fn button_with_warning_variant() {
    with_test_container(|container| {
        // Create a test component
        let button_view =
            view! { <Button variant=ButtonVariant::Warning>"Warning Button"</Button> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check if warning variant class is applied
        assert!(
            has_class(&button, "warning"),
            "Button should have the warning variant class"
        );

        // Check content
        assert_eq!(button.text_content().unwrap(), "Warning Button");
    });
}

#[wasm_bindgen_test]
fn button_with_aria_attributes() {
    with_test_container(|container| {
        // Create a test component with aria attributes
        let button_view = view! {
            <Button
                aria_label="Test button".to_string()
                aria_pressed=true
                aria_expanded=false
                aria_controls="test-content".to_string()
            >
                "ARIA Button"
            </Button>
        };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, button_view);

        // Test it
        let button = query_selector(container, ".btn").expect("Button should be rendered");

        // Check aria attributes
        assert_eq!(
            button.get_attribute("aria-label").unwrap(),
            "Test button",
            "Button should have the correct aria-label"
        );
        assert_eq!(
            button.get_attribute("aria-pressed").unwrap(),
            "true",
            "Button should have the correct aria-pressed"
        );
        assert_eq!(
            button.get_attribute("aria-expanded").unwrap(),
            "false",
            "Button should have the correct aria-expanded"
        );
        assert_eq!(
            button.get_attribute("aria-controls").unwrap(),
            "test-content",
            "Button should have the correct aria-controls"
        );
    });
}

// Helper to get the document
fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}
