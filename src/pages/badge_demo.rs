use crate::components::badge::{Badge, BadgeVariant};
use leptos::{prelude::*, *};

#[component]
pub fn BadgeDemo() -> impl IntoView {
    let (clicked_badge, set_clicked_badge) = signal(String::new());

    let handle_badge_click = move |badge_name: &str| {
        let name = badge_name.to_string();
        Box::new(move || {
            set_clicked_badge.set(format!("Clicked on {} badge", name));
        })
    };

    view! {
        <div class="container">
            <h1 class="title">Badge Component Demo</h1>

            <section class="section">
                <h2>Badge Variants</h2>
                <p>Different badge styles based on symbol categories.</p>

                <div class="badgeContainer">
                    <Badge variant=BadgeVariant::Default on_click=handle_badge_click("Default")>
                        "Default"
                    </Badge>

                    <Badge variant=BadgeVariant::Nature on_click=handle_badge_click("Nature")>
                        "Nature"
                    </Badge>

                    <Badge variant=BadgeVariant::Emotion on_click=handle_badge_click("Emotion")>
                        "Emotion"
                    </Badge>

                    <Badge variant=BadgeVariant::Animal on_click=handle_badge_click("Animal")>
                        "Animal"
                    </Badge>

                    <Badge variant=BadgeVariant::Object on_click=handle_badge_click("Object")>
                        "Object"
                    </Badge>

                    <Badge variant=BadgeVariant::Person on_click=handle_badge_click("Person")>
                        "Person"
                    </Badge>

                    <Badge
                        variant=BadgeVariant::Mythological
                        on_click=handle_badge_click("Mythological")
                    >
                        "Mythological"
                    </Badge>

                    <Badge variant=BadgeVariant::Dream on_click=handle_badge_click("Dream")>
                        "Dream"
                    </Badge>
                </div>

                <div class="messageBox">
                    {move || {
                        if clicked_badge.get().is_empty() {
                            "Click a badge to see interaction".to_string()
                        } else {
                            clicked_badge.get()
                        }
                    }}
                </div>
            </section>

            <section class="section">
                <h2>Badge Usage Examples</h2>
                <p>Badges can be used in different contexts:</p>

                <div class="exampleCard">
                    <h3>Symbol Categories</h3>
                    <div class="badgeContainer">
                        <Badge variant=BadgeVariant::Nature>"Mountain"</Badge>
                        <Badge variant=BadgeVariant::Nature>"Water"</Badge>
                        <Badge variant=BadgeVariant::Nature>"Fire"</Badge>
                        <Badge variant=BadgeVariant::Nature>"Tree"</Badge>
                    </div>
                </div>

                <div class="exampleCard">
                    <h3>Emotions</h3>
                    <div class="badgeContainer">
                        <Badge variant=BadgeVariant::Emotion>"Joy"</Badge>
                        <Badge variant=BadgeVariant::Emotion>"Fear"</Badge>
                        <Badge variant=BadgeVariant::Emotion>"Anger"</Badge>
                        <Badge variant=BadgeVariant::Emotion>"Love"</Badge>
                    </div>
                </div>

                <div class="exampleCard">
                    <h3>Animals</h3>
                    <div class="badgeContainer">
                        <Badge variant=BadgeVariant::Animal>"Wolf"</Badge>
                        <Badge variant=BadgeVariant::Animal>"Snake"</Badge>
                        <Badge variant=BadgeVariant::Animal>"Eagle"</Badge>
                        <Badge variant=BadgeVariant::Animal>"Tiger"</Badge>
                    </div>
                </div>

                <div class="exampleCard">
                    <h3>Mythological</h3>
                    <div class="badgeContainer">
                        <Badge variant=BadgeVariant::Mythological>"Dragon"</Badge>
                        <Badge variant=BadgeVariant::Mythological>"Phoenix"</Badge>
                        <Badge variant=BadgeVariant::Mythological>"Unicorn"</Badge>
                        <Badge variant=BadgeVariant::Mythological>"Mermaid"</Badge>
                    </div>
                </div>
            </section>

            <section class="section">
                <h2>Non-clickable Badges</h2>
                <p>Badges can also be used as static display elements:</p>

                <div class="badgeContainer">
                    <Badge variant=BadgeVariant::Default>"Label"</Badge>
                    <Badge variant=BadgeVariant::Mythological>"Category"</Badge>
                    <Badge variant=BadgeVariant::Dream>"Type"</Badge>
                </div>
            </section>
        </div>
    }
}
