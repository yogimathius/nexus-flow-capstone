use leptos::prelude::*;

/// Star Rating Component
///
/// A customizable 5-star rating component that supports:
/// - Whole and half-star ratings
/// - Multiple size options (small, medium, large)
/// - Custom CSS classes
#[component]
pub fn StarRating(
    #[prop(default = 5.0)] rating: f32,
    #[prop(default = "medium")] size: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = true)] allow_half: bool,
) -> impl IntoView {
    // Calculate the number of full, half, and empty stars
    let full_stars = rating.floor() as usize;
    let has_half_star = allow_half && rating - rating.floor() >= 0.5;
    let empty_stars = 5 - full_stars - if has_half_star { 1 } else { 0 };

    // Determine the size class
    let size_class = match size {
        "small" => "star_small",
        "large" => "star_large",
        _ => "star_medium",
    };

    // Build the combined class string
    let container_class = format!("star_rating {} {}", size_class, class);

    view! {
        <div class=container_class aria-label=format!("{} out of 5 stars", rating)>
            // Render full stars
            {(0..full_stars)
                .map(|_| {
                    let star_class = "star star_full";
                    view! {
                        <span class=star_class aria-hidden="true">
                            "★"
                        </span>
                    }
                })
                .collect_view()}

            // Render half star if needed
            {has_half_star
                .then(|| {
                    let star_class = "star star_half";
                    view! {
                        <span class=star_class aria-hidden="true">
                            "★"
                        </span>
                    }
                })}

            // Render empty stars
            {(0..empty_stars)
                .map(|_| {
                    let star_class = "star star_empty";
                    view! {
                        <span class=star_class aria-hidden="true">
                            "☆"
                        </span>
                    }
                })
                .collect_view()}
        </div>
    }
}
