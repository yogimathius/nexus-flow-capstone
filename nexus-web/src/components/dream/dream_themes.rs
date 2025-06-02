use crate::components::badges::{Badge, BadgeVariant};
use crate::components::dream::constants::COMMON_DREAM_THEMES;
use leptos::prelude::*;

#[component]
pub fn DreamThemes(
    on_theme_click: impl Fn(String) + 'static + Clone,
) -> impl IntoView {
    view! {
        <div class="card">
            <h3 class="card-header">"Common Dream Themes"</h3>
            <p class="text-secondary mb-4">
                "Click on any theme to add a template to your dream description."
            </p>
            <div class="cluster">
                {COMMON_DREAM_THEMES.iter().map(|&theme| {
                    let theme_str = theme.to_string();
                    let variant = match theme {
                        "Water" | "Fire" => BadgeVariant::Nature,
                        "Flying" | "Falling" | "Being chased" | "Being trapped" =>
                            BadgeVariant::Emotion,
                        "Snakes" | "Animals" => BadgeVariant::Animal,
                        "Teeth falling out" | "Money" | "Naked in public" | "Finding a new room" =>
                            BadgeVariant::Object,
                        "School or work" | "Celebrity" | "Childhood home" =>
                            BadgeVariant::Person,
                        _ => BadgeVariant::Dream
                    };
                    let on_click_callback = on_theme_click.clone();
                    let theme_for_callback = theme_str.clone();
                    view! {
                        <Badge
                            variant=variant
                            on_click=Box::new(move || on_click_callback(theme_for_callback.clone()))
                        >
                            {theme}
                        </Badge>
                    }
                }).collect_view()}
            </div>
        </div>
    }
} 