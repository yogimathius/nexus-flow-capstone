use crate::components::buttons::{Button, ButtonVariant};
use crate::components::dream::service;
use leptos::prelude::*;
use leptos::logging::log;
use wasm_bindgen_futures::spawn_local;

const MIN_DREAM_LENGTH: usize = 20; // Minimum characters for a meaningful dream description

#[component]
pub fn DreamForm(
    dream_text: ReadSignal<String>,
    set_dream_text: WriteSignal<String>,
    is_loading: ReadSignal<bool>,
    set_is_loading: WriteSignal<bool>,
    auto_save: ReadSignal<bool>,
    set_auto_save: WriteSignal<bool>, 
    _button_disabled: ReadSignal<bool>,
    _set_button_disabled: WriteSignal<bool>,
    error: ReadSignal<Option<String>>,
    set_error: WriteSignal<Option<String>>,
    set_interpretation: WriteSignal<Option<String>>,
    set_saved_dream_id: WriteSignal<Option<String>>
) -> impl IntoView {
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        set_interpretation.set(None);
        set_error.set(None);
        set_saved_dream_id.set(None);

        let dream = dream_text.get();

        if dream.trim().is_empty() {
            set_error.set(Some("Please enter your dream description".to_string()));
            return;
        }

        if dream.trim().len() < MIN_DREAM_LENGTH {
            set_error.set(Some(format!("Your dream description needs to be at least {} characters for a meaningful interpretation", MIN_DREAM_LENGTH)));
            return;
        }

        set_is_loading.set(true);
        log!("Submitting dream interpretation request, loading state set to true");

        let dream_clone = dream.clone();
        let save = auto_save.get();
        
        spawn_local(async move {
            match service::interpret_dream(dream_clone, save).await {
                Ok(data) => {
                    let formatted_interpretation = service::format_interpretation(&data.interpretation);
                    set_interpretation.set(Some(formatted_interpretation));

                    if let Some(id) = data.id {
                        set_saved_dream_id.set(Some(id));
                    }

                    log!("Received successful interpretation response");
                }
                Err(error_msg) => {
                    set_error.set(Some(error_msg));
                }
            }

            set_is_loading.set(false);            
            log!("Request completed, loading state set to false");
        });
    };

    let char_count = Signal::derive(move || dream_text.get().len());
    
    let get_validation_status = move || {
        let text = dream_text.get();
        let count = text.trim().len();
        
        if count == 0 {
            "empty"
        } else if count < MIN_DREAM_LENGTH {
            "too-short"
        } else {
            "valid"
        }
    };

    view! {
        <div class="card">
            <form on:submit=handle_submit class="stack gap-6">
                <div class="stack gap-2">
                    <label for="dream-text" class="label">
                        "Describe your dream"
                        <span class="text-sm text-muted ml-2">
                            {"(minimum "}{MIN_DREAM_LENGTH}{" characters)"}
                        </span>
                    </label>
                    <div class=move || {
                        let mut classes = "textarea-container".to_string();
                        if error.get().is_some() {
                            classes.push_str(" error");
                        }
                        classes
                    }>
                        <textarea
                            id="dream-text"
                            class="textarea"
                            placeholder="Enter your dream description in detail..."
                            prop:value=dream_text
                            aria-invalid=move || error.get().is_some().to_string()
                            aria-describedby="dream-validation-message char-counter"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                if error.get().is_some() {
                                    set_error.set(None);
                                }
                                set_dream_text.set(value);
                                log!("Textarea input updated");
                            }
                        />
                        <div class="char-counter" id="char-counter">
                            <span class=move || {
                                let validation = get_validation_status();
                                let mut classes = "counter-text".to_string();
                                if validation == "empty" {
                                    classes.push_str(" text-error");
                                } else if validation == "too-short" {
                                    classes.push_str(" text-warning");
                                } else if validation == "valid" {
                                    classes.push_str(" text-success");
                                }
                                classes
                            }>
                                {move || char_count.get()} " / " {MIN_DREAM_LENGTH} " characters"
                            </span>
                        </div>
                    </div>
                    <div
                        id="dream-validation-message"
                        class="validation-message"
                        aria-live="polite"
                    >
                        <Show
                            when=move || error.get().is_some()
                            fallback=move || {
                                let status = get_validation_status();
                                view! {
                                    <Show
                                        when=move || status == "empty"
                                        fallback=move || {
                                            view! {
                                                <Show
                                                    when=move || status == "too-short"
                                                    fallback=move || {
                                                        view! { <span class="text-success"></span> }
                                                    }
                                                >
                                                    <span class="text-warning">
                                                        "Your dream is too short for meaningful interpretation. Please add more details."
                                                    </span>
                                                </Show>
                                            }
                                        }
                                    >
                                        <span class="text-error">"Please enter your dream"</span>
                                    </Show>
                                }
                            }
                        >
                            <span class="text-error">
                                {move || error.get().unwrap_or_default()}
                            </span>
                        </Show>
                    </div>
                </div>

                <div class="flex items-center gap-2">
                    <input
                        type="checkbox"
                        id="auto-save"
                        checked=auto_save
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_auto_save.set(checked);
                        }
                    />
                    <label for="auto-save" class="label m-0">
                        "Save to dream journal"
                    </label>
                </div>

                <Button variant=ButtonVariant::Primary class="full-width".to_string()>
                    {move || if is_loading.get() { "Interpreting..." } else { "Interpret Dream" }}
                </Button>
            </form>
        </div>
    }
} 