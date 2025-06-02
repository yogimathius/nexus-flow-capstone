use leptos::{prelude::*, *};
use web_sys::MouseEvent;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Warning,
    Danger,
}

impl ButtonVariant {
    fn to_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Warning => "warning",
            ButtonVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    fn to_class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] class: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] aria_pressed: Option<bool>,
    #[prop(optional)] aria_expanded: Option<bool>,
    #[prop(optional)] aria_controls: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant_class = variant.to_class();
    let size_class = size.to_class();

    let combined_class = format!(
        "btn {} {} {}",
        variant_class,
        size_class,
        class.unwrap_or_default()
    );

    // Create a handler for click events
    let handle_click = move |ev: MouseEvent| {
        if let Some(handler) = &on_click {
            handler(ev);
        }
    };

    view! {
        <button
            class=combined_class
            disabled=disabled
            aria-disabled=if disabled { Some("true") } else { None }
            aria-label=aria_label
            aria-pressed=aria_pressed.map(|p| if p { "true" } else { "false" })
            aria-expanded=aria_expanded.map(|e| if e { "true" } else { "false" })
            aria-controls=aria_controls
            on:click=handle_click
        >
            {children()}
        </button>
    }
}
