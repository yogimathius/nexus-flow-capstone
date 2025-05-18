use crate::components::dream::{
    constants::get_theme_sentence, DreamForm, DreamResult, DreamThemes, DreamTips,
};
use crate::components::feedback::LoadingIndicator;
use leptos::prelude::*;

#[component]
pub fn DreamInterpreter() -> impl IntoView {
    let (dream_text, set_dream_text) = signal(String::new());
    let (interpretation, set_interpretation) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (button_disabled, set_button_disabled) = signal(true);
    let (auto_save, set_auto_save) = signal(true);
    let (saved_dream_id, set_saved_dream_id) = signal(Option::<String>::None);

    // Calculate if the result should be visible
    let show_result = move || interpretation.get().is_some();

    // Update button state based on form state
    Effect::new(move |_| {
        let text = dream_text.get();
        let loading = is_loading.get();
        set_button_disabled.set(text.trim().is_empty() || text.trim().len() < 20 || loading);
    });

    let handle_badge_click = move |theme: String| {
        let sentence = get_theme_sentence(&theme);

        let current = dream_text.get();
        let new_text = if current.is_empty() {
            sentence.to_string()
        } else if current.trim_end().ends_with('.')
            || current.trim_end().ends_with('!')
            || current.trim_end().ends_with('?')
        {
            format!("{} {}", current, sentence)
        } else {
            format!("{}. {}", current, sentence)
        };

        set_dream_text.set(new_text);
        set_button_disabled.set(false);
    };

    let interpretation_id = "dream-interpretation-result";
    let loading_id = "loading-indicator";

    view! {
        <div class="dream-interpreter-container fade-in" role="main" aria-live="polite">
            <h1 class="visually-hidden">Dream Interpreter</h1>

            // Two-column layout on larger screens
            <div class="dream-form-layout">
                // Dream form column
                <div class="dream-form" role="form" aria-labelledby="dream-form-heading">
                    <h2 id="dream-form-heading" class="visually-hidden">
                        Dream Description Form
                    </h2>
                    <DreamForm
                        dream_text=dream_text
                        set_dream_text=set_dream_text
                        is_loading=is_loading
                        set_is_loading=set_is_loading
                        auto_save=auto_save
                        set_auto_save=set_auto_save
                        _button_disabled=button_disabled
                        _set_button_disabled=set_button_disabled
                        error=error
                        set_error=set_error
                        set_interpretation=set_interpretation
                        set_saved_dream_id=set_saved_dream_id
                    />
                </div>

                // Common themes column
                <div class="dream-themes" role="region" aria-labelledby="themes-heading">
                    <h2 id="themes-heading" class="visually-hidden">
                        Common Dream Themes
                    </h2>
                    <DreamThemes on_theme_click=handle_badge_click />
                </div>
            </div>

            // Loading indicator (visible only during loading)
            <Show when=move || is_loading.get() fallback=|| ()>
                <div
                    class="loading-container"
                    id=loading_id
                    role="status"
                    aria-live="assertive"
                    aria-label="Interpreting your dream"
                >
                    <LoadingIndicator />
                    <span class="loading-text">Interpreting your dream...</span>
                </div>
            </Show>

            // Interpretation results (full width)
            <div
                class="dream-interpretation"
                id=interpretation_id
                role="region"
                aria-labelledby="interpretation-heading"
                aria-live="polite"
            >
                <h2 id="interpretation-heading" class="visually-hidden">
                    Dream Interpretation Result
                </h2>
                <Show
                    when=show_result
                    fallback=move || {
                        let loading = is_loading.get();
                        view! {
                            <Show when=move || !loading fallback=|| ()>
                                <DreamTips />
                            </Show>
                        }
                    }
                >
                    <DreamResult
                        interpretation=interpretation.into()
                        saved_dream_id=saved_dream_id.into()
                    />
                </Show>
            </div>
        </div>
    }
}
