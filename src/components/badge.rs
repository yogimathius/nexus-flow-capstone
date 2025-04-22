use leptos::{prelude::*, *};
use web_sys::KeyboardEvent;

#[derive(Clone, PartialEq, Debug)]
pub enum BadgeVariant {
    Nature,
    Emotion,
    Animal,
    Object,
    Person,
    Mythological,
    Dream,
    Default,
}

impl BadgeVariant {
    fn to_class(&self) -> &'static str {
        match self {
            BadgeVariant::Nature => "nature",
            BadgeVariant::Emotion => "emotion",
            BadgeVariant::Animal => "animal",
            BadgeVariant::Object => "object",
            BadgeVariant::Person => "person",
            BadgeVariant::Mythological => "mythological",
            BadgeVariant::Dream => "dream",
            BadgeVariant::Default => "default",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional)] class: Option<String>,
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    let variant_class = variant.to_class();
    let combined_class = format!("badge {} {}", variant_class, class.unwrap_or_default());
    let is_clickable = on_click.is_some();
    let clickable_class = if is_clickable { "clickable" } else { "" };
    let final_class = format!("{} {}", combined_class, clickable_class);

    // Only make it keyboard accessible if it's clickable
    let tab_index = if is_clickable { Some(0) } else { None };

    // Create a signal to handle badge click events
    let (clicked, set_clicked) = signal(false);

    // Trigger the callback when clicked changes to true
    Effect::new(move |_| {
        if clicked.get() && on_click.is_some() {
            // Call the callback
            if let Some(callback) = &on_click {
                callback();
            }
            // Reset the clicked state
            set_clicked.set(false);
        }
    });

    view! {
        <span
            class=final_class
            tabindex=tab_index
            role=if is_clickable { Some("button") } else { None }
            aria-label=variant_class
            on:click=move |_| {
                if is_clickable {
                    set_clicked.set(true);
                }
            }
            on:keydown=move |e: KeyboardEvent| {
                if is_clickable && (e.key() == "Enter" || e.key() == " ") {
                    e.prevent_default();
                    set_clicked.set(true);
                }
            }
        >
            {children()}
        </span>
    }
}
