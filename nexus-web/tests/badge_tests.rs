use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

mod setup;
// Explicitly import the functions we need from our setup module
use setup::{
    create_click_event, create_test_signal, has_class, mount_to_container, query_selector,
    with_test_container,
};

// Import badge components
use client_dream_ai::components::badges::{Badge, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn badge_renders_with_default_variant() {
    with_test_container(|container| {
        // Create a test component
        let badge_view = view! { <Badge>"Test Badge"</Badge> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, badge_view);

        // Test it
        let badge = query_selector(container, ".badge").expect("Badge should be rendered");

        // Check if default variant class is applied
        assert!(
            has_class(&badge, "default"),
            "Badge should have the default variant class"
        );

        // Check content
        assert_eq!(badge.text_content().unwrap(), "Test Badge");
    });
}

#[wasm_bindgen_test]
fn badge_renders_with_custom_variant() {
    with_test_container(|container| {
        // Create a test component
        let badge_view = view! { <Badge variant=BadgeVariant::Nature>"Nature Badge"</Badge> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, badge_view);

        // Test it
        let badge = query_selector(container, ".badge").expect("Badge should be rendered");

        // Check if nature variant class is applied
        assert!(
            has_class(&badge, "nature"),
            "Badge should have the nature variant class"
        );

        // Check content
        assert_eq!(badge.text_content().unwrap(), "Nature Badge");
    });
}

#[wasm_bindgen_test]
fn badge_handles_click_event() {
    with_test_container(|container| {
        // Create a signal to track clicks
        let clicked = create_test_signal(false);
        let clicked_for_check = clicked.clone();

        // Create a test component with click handler
        let badge_view = view! {
            <Badge variant=BadgeVariant::Emotion on_click=Box::new(move || clicked.set(true))>
                "Click Me"
            </Badge>
        };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, badge_view);

        // Test it
        let badge = query_selector(container, ".badge").expect("Badge should be rendered");

        // Badge should have clickable class
        assert!(
            has_class(&badge, "clickable"),
            "Badge should have the clickable class"
        );

        // Simulate a click
        let click_event = create_click_event();
        let _ = badge.dispatch_event(&click_event);

        // Check if click handler was called
        assert!(
            clicked_for_check.get(),
            "Click handler should have been called"
        );
    });
}

#[wasm_bindgen_test]
fn badge_applies_custom_class() {
    with_test_container(|container| {
        // Create a test component
        let badge_view =
            view! { <Badge class="custom-class".to_string()>"Custom Class Badge"</Badge> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, badge_view);

        // Test it
        let badge = query_selector(container, ".badge").expect("Badge should be rendered");

        // Check if custom class is applied
        assert!(
            has_class(&badge, "custom-class"),
            "Badge should have the custom class"
        );
    });
}

#[wasm_bindgen_test]
fn badge_applies_correct_accessibility_attributes() {
    with_test_container(|container| {
        // Create a test component
        let badge_view = view! {
            <Badge variant=BadgeVariant::Animal on_click=Box::new(|| {})>
                "Accessible Badge"
            </Badge>
        };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, badge_view);

        // Test it
        let badge = query_selector(container, ".badge").expect("Badge should be rendered");

        // Check if role is button for clickable badges
        assert_eq!(badge.get_attribute("role").unwrap(), "button");

        // Check if tabindex is set for keyboard navigation
        assert_eq!(badge.get_attribute("tabindex").unwrap(), "0");

        // Check aria-label
        assert_eq!(badge.get_attribute("aria-label").unwrap(), "animal");
    });
}

#[wasm_bindgen_test]
fn non_clickable_badge_has_no_interactive_attributes() {
    with_test_container(|container| {
        // Create a test component
        let badge_view = view! { <Badge>"Non-Clickable Badge"</Badge> };

        // Mount it
        let div = document().create_element("div").unwrap();
        container.append_child(&div).unwrap();

        mount_to_container(&div, badge_view);

        // Test it
        let badge = query_selector(container, ".badge").expect("Badge should be rendered");

        // Should not have clickable class
        assert!(
            !has_class(&badge, "clickable"),
            "Badge should not have the clickable class"
        );

        // Should not have role attribute
        assert!(badge.get_attribute("role").is_none());

        // Should not have tabindex attribute
        assert!(badge.get_attribute("tabindex").is_none());
    });
}

// Helper to get the document
fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}
