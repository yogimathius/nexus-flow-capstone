use crate::components::dream::{
    constants::get_theme_sentence,
    DreamForm, DreamResult, DreamThemes, DreamTips
};
use leptos::prelude::*;
use leptos::logging::log;

#[component]
pub fn DreamInterpreter() -> impl IntoView {
    let (dream_text, set_dream_text) = signal(String::new());

    let (interpretation, set_interpretation) = signal(Option::<String>::None);

    let (is_loading, set_is_loading) = signal(false);

    let (error, set_error) = signal(Option::<String>::None);

    let (button_disabled, set_button_disabled) = signal(true);

    let (auto_save, set_auto_save) = signal(true);

    let (saved_dream_id, set_saved_dream_id) = signal(Option::<String>::None);

    Effect::new(move |_| {
        let text = dream_text.get();
        let loading = is_loading.get();

        log!("Dream text: '{}', is_loading: {}", text, loading);

        let should_be_disabled = text.trim().is_empty() || loading;
        log!("Button should be disabled: {}", should_be_disabled);

        set_button_disabled.set(should_be_disabled);
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

        log!("Badge '{}' clicked, text updated", theme);
    };

    view! {
        <div class="stack gap-6 fade-in">
            <DreamForm
                dream_text=dream_text
                set_dream_text=set_dream_text
                is_loading=is_loading
                set_is_loading=set_is_loading
                auto_save=auto_save
                set_auto_save=set_auto_save
                button_disabled=button_disabled
                set_button_disabled=set_button_disabled
                error=error
                set_error=set_error
                set_interpretation=set_interpretation
                set_saved_dream_id=set_saved_dream_id
            />

            <DreamThemes on_theme_click=handle_badge_click />

            <DreamResult 
                interpretation=interpretation.into()
                saved_dream_id=saved_dream_id.into()
            />

            <DreamTips />
        </div>
    }
}
