use leptos::prelude::*;
use leptos::{attr::Scope, mount::mount_to};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::__rt::detect::Runtime;
use web_sys::{Element, HtmlElement, MouseEvent, Node};

// Helper function to create a test environment
pub fn setup_test() -> web_sys::Element {
    let test_container = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    test_container
}

// Mount a component to a container
// pub fn mount_to<F, N>(container: web_sys::Element, f: F)
// where
//     F: FnOnce() -> N + 'static,
//     N: IntoView,
// {
//     let container = container.dyn_into::<HtmlElement>().unwrap();
//     let _ = mount_to(container, f);
// }

/// A simple signal implementation for testing purposes
#[derive(Clone)]
pub struct TestSignal<T: Clone + 'static> {
    inner: Rc<RefCell<T>>,
}

impl<T: Clone + 'static> TestSignal<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(value)),
        }
    }

    pub fn get(&self) -> T {
        (*self.inner.borrow()).clone()
    }

    pub fn set(&self, value: T) {
        *self.inner.borrow_mut() = value;
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut *self.inner.borrow_mut());
    }
}

/// Creates a test signal with the given initial value
pub fn create_test_signal<T: Clone + 'static>(initial: T) -> TestSignal<T> {
    TestSignal::new(initial)
}

/// Mounts a component to an element
pub fn mount_to_container<IV: IntoView + 'static>(container: &web_sys::Element, component: IV) {
    let container_clone = container.clone();
    let container_html = container_clone.dyn_into::<HtmlElement>().unwrap();
    let _ = leptos::prelude::mount_to(container_html, move || component);
}

/// Checks if an element has a specific class
pub fn has_class(element: &Element, class: &str) -> bool {
    element.class_list().contains(class)
}

/// Selects a single element matching the given selector
pub fn query_selector(parent: &Node, selector: &str) -> Option<Element> {
    parent
        .owner_document()
        .unwrap()
        .query_selector(selector)
        .ok()
        .flatten()
}

/// Selects all elements matching the given selector
pub fn query_selector_all(parent: &Node, selector: &str) -> Vec<Element> {
    // Get all elements using multiple query_selector calls
    // since web_sys doesn't expose querySelectorAll in a convenient way
    let document = parent.owner_document().unwrap();

    // This is a simplified implementation that will work for basic cases
    // In a real-world scenario, you'd want to implement a more robust solution
    let mut elements = Vec::new();

    if let Ok(Some(element)) = document.query_selector(selector) {
        elements.push(element.clone());

        // Try to find more elements with siblings
        let mut current = element;
        while let Some(next) = current.next_element_sibling() {
            if let Ok(true) = next.matches(selector) {
                elements.push(next.clone());
            }
            current = next;
        }
    }

    elements
}

// Creates a test container and mounts the provided component
pub fn render_component<T, F>(component: F) -> Element
where
    T: IntoView,
    F: FnOnce() -> T + 'static,
{
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    let container_html = container.clone().dyn_into::<HtmlElement>().unwrap();

    // Mount the component to our test container
    let _ = leptos::prelude::mount_to(container_html, component);

    container
}

/// Executes a test with a temporary container
pub fn with_test_container<F>(test_fn: F)
where
    F: FnOnce(&Element),
{
    let window = web_sys::window().expect("Window not found");
    let document = window.document().expect("Document not found");
    let body = document.body().expect("Body not found");

    // Create a temporary container for the test
    let container = document
        .create_element("div")
        .expect("Failed to create container");
    body.append_child(&container)
        .expect("Failed to append container");

    // Run the test with the container
    test_fn(&container);

    // Clean up
    body.remove_child(&container)
        .expect("Failed to remove container");
}

/// Creates a mouse click event
pub fn create_click_event() -> web_sys::Event {
    let window = web_sys::window().expect("Window not found");
    let document = window.document().expect("Document not found");

    let event = document
        .create_event("MouseEvent")
        .expect("Failed to create event");

    let mouse_event: &MouseEvent = event.unchecked_ref();
    mouse_event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg(
        "click", true, // bubbles
        true, // cancelable
    );

    event
}

// Helper to track event calls
pub struct EventTracker<E> {
    pub calls: Rc<RefCell<Vec<E>>>,
}

impl<E: Clone + 'static> EventTracker<E> {
    pub fn new() -> Self {
        Self {
            calls: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn callback(&self) -> Box<dyn Fn(E)> {
        let calls = self.calls.clone();
        Box::new(move |event| {
            calls.borrow_mut().push(event.clone());
        })
    }

    pub fn callback_fn<F>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(E) + 'static,
    {
        let calls = self.calls.clone();
        Box::new(move |event| {
            calls.borrow_mut().push(event.clone());
            f(event);
        })
    }

    pub fn was_called(&self) -> bool {
        !self.calls.borrow().is_empty()
    }

    pub fn call_count(&self) -> usize {
        self.calls.borrow().len()
    }
}

// Simple test helper to attach a component to a container div for testing
pub fn mount_for_test<T, F>(f: F) -> Element
where
    T: IntoView,
    F: FnOnce() -> T + 'static,
{
    // Create a container for our test
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    // Set up the leptos runtime and render our component
    let container_html = container.clone().dyn_into::<HtmlElement>().unwrap();
    let _ = leptos::prelude::mount_to(container_html, f);

    container
}
