use leptos::prelude::*;

/// Title component - Uses h1 styling with gradient effect
#[component]
pub fn Title(
    #[prop(into)] text: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    view! { <h1 class=class_value>{text}</h1> }
}

/// SubTitle component - Uses h2 styling
#[component]
pub fn SubTitle(
    #[prop(into)] text: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    view! { <h2 class=class_value>{text}</h2> }
}

/// Section Title component - Uses h3 styling
#[component]
pub fn SectionTitle(
    #[prop(into)] text: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    view! { <h3 class=class_value>{text}</h3> }
}

/// Text component - Standard paragraph text
#[component]
pub fn Text(
    #[prop(into)] children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    view! { <p class=class_value>{children()}</p> }
}

/// Small Text component - Smaller text, often used for captions or secondary information
#[component]
pub fn SmallText(
    #[prop(into)] children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = format!("text-sm {}", class.unwrap_or_default());

    view! { <span class=class_value>{children()}</span> }
}

/// Monospace Text component - For code, commands, and technical content
#[component]
pub fn MonoText(
    #[prop(into)] children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = format!("mono {}", class.unwrap_or_default());

    view! { <code class=class_value>{children()}</code> }
}

/// Link component - Styled anchor element
#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(into)] children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] external: Option<bool>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();
    let is_external = external.unwrap_or(false);

    let target = if is_external { "_blank" } else { "_self" };
    let rel = if is_external {
        "noopener noreferrer"
    } else {
        ""
    };

    view! {
        <a href=href class=class_value target=target rel=rel>
            {children()}
        </a>
    }
}

/// Label component - Form label with consistent styling
#[component]
pub fn Label(
    #[prop(into)] for_input: String,
    #[prop(into)] text: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_value = class.unwrap_or_default();

    view! {
        <label for=for_input class=class_value>
            {text}
        </label>
    }
}
