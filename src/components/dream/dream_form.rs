use crate::components::buttons::{Button, ButtonVariant};
use crate::components::dream::service;
use leptos::prelude::*;
use leptos::logging::log;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn DreamForm(
    dream_text: ReadSignal<String>,
    set_dream_text: WriteSignal<String>,
    is_loading: ReadSignal<bool>,
    set_is_loading: WriteSignal<bool>,
    auto_save: ReadSignal<bool>,
    set_auto_save: WriteSignal<bool>, 
    button_disabled: ReadSignal<bool>,
    set_button_disabled: WriteSignal<bool>,
    error: ReadSignal<Option<String>>,
    set_error: WriteSignal<Option<String>>,
    set_interpretation: WriteSignal<Option<String>>,
    set_saved_dream_id: WriteSignal<Option<String>>
) -> impl IntoView {
    // Handle form submission
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        // Reset previous interpretation, error and saved ID
        set_interpretation.set(None);
        set_error.set(None);
        set_saved_dream_id.set(None);

        let dream = dream_text.get();

        // Validate input
        if dream.trim().is_empty() {
            set_error.set(Some("Please enter your dream description".to_string()));
            return;
        }

        // Set loading state to true
        set_is_loading.set(true);
        log!("Submitting dream interpretation request, loading state set to true");

        // Use the service to interpret the dream
        let dream_clone = dream.clone();
        let save = auto_save.get();
        
        spawn_local(async move {
            match service::interpret_dream(dream_clone, save).await {
                Ok(data) => {
                    // Process the interpretation to improve formatting
                    let formatted_interpretation = service::format_interpretation(&data.interpretation);
                    set_interpretation.set(Some(formatted_interpretation));

                    // Set the saved dream ID if available
                    if let Some(id) = data.id {
                        set_saved_dream_id.set(Some(id));
                    }

                    log!("Received successful interpretation response");
                }
                Err(error_msg) => {
                    set_error.set(Some(error_msg));
                }
            }

            // Set loading state back to false
            set_is_loading.set(false);
            log!("Request completed, loading state set to false");
        });
    };

    view! {
      <div class="card">
        <form 
            on:submit=handle_submit
            class="stack gap-6"
        >
            <div class="stack gap-2">
                <label for="dream-text" class="label">"Describe your dream"</label>
                <textarea
                    id="dream-text"
                    class="textarea"
                    placeholder="Enter your dream description..."
                    prop:value=dream_text
                    on:input=move |ev| {
                        let value = event_target_value(&ev);

                        // Directly update button state on input
                        let should_disable = value.trim().is_empty() || is_loading.get();
                        set_button_disabled.set(should_disable);

                        // Set the dream text after checking the value
                        set_dream_text.set(value);

                        log!("Textarea input updated, button disabled: {}", should_disable);
                    }
                />
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
                <label for="auto-save" class="label m-0">"Save to dream journal"</label>
            </div>

            <Button
                variant=ButtonVariant::Primary
                // disabled=button_disabled.get()
                class="full-width".to_string()
            >
                {move || if is_loading.get() { "Interpreting..." } else { "Interpret Dream" }}
            </Button>

            {move || error.get().map(|err| view! {
                <div class="text-error">{err}</div>
            })}
        </form>
    </div>
    }
} 