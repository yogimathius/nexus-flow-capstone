use leptos::ev;
use leptos::prelude::*;

/// Button variants
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Warning,
    Danger,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Primary
    }
}

impl ButtonVariant {
    fn to_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Warning => "btn-warning",
            ButtonVariant::Danger => "btn-danger",
        }
    }
}

/// Button sizes
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Medium
    }
}

impl ButtonSize {
    fn to_class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "btn-sm",
            ButtonSize::Medium => "",
            ButtonSize::Large => "btn-lg",
        }
    }
}

/// Button function - Creates a reusable button with different variants and sizes
pub fn Button(
    children: impl IntoView,
    variant: ButtonVariant,
    size: ButtonSize,
    class: Option<String>,
    disabled: bool,
    on_click: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
    button_type: &'static str,
) -> impl IntoView {
    let variant_class = variant.to_class();
    let size_class = size.to_class();

    // Combine classes
    let combined_class = format!(
        "btn {} {} {}",
        variant_class,
        size_class,
        class.unwrap_or_default()
    );

    view! {
        <button
            type=button_type
            class=combined_class
            disabled=disabled
            on:click=move |ev| {
                if let Some(ref cb) = on_click {
                    (cb)(ev)
                }
            }
        >
            {children}
        </button>
    }
}

/// Submit Button function - Creates a specialized button for form submission
pub fn SubmitButton(
    text: String,
    class: Option<String>,
    disabled: bool,
    loading: bool,
) -> impl IntoView {
    let is_disabled = disabled || loading;
    let display_text = if loading {
        "Loading...".to_string()
    } else {
        text
    };

    let combined_class = format!("btn btn-primary {}", class.unwrap_or_default());

    view! {
        <button type="submit" class=combined_class disabled=is_disabled>
            {display_text}
        </button>
    }
}
