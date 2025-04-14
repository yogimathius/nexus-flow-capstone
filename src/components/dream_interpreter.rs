use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen::{JsCast, JsValue};
use log::{info, warn, error};

#[derive(Deserialize, Clone, Debug)]
struct DreamResponse {
    interpretation: String,
    status: String,
}

#[component]
pub fn DreamInterpreter() -> impl IntoView {
    let (dream_text, set_dream_text) = signal(String::new());
    let (interpretation, set_interpretation) = signal(None::<String>);
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (api_url, set_api_url) = signal("http://localhost:3001/api/interpret_dream".to_string());
    let (debug_log, set_debug_log) = signal(String::new());
    let (use_mock, set_use_mock) = signal(false);
    let (cors_mode, set_cors_mode) = signal("cors".to_string()); // "cors" or "no-cors"

    let add_log = move |message: &str| {
        let timestamp = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();
        let log_entry = format!("[{}] {}\n", timestamp, message);
        info!("{}", log_entry.trim());
        set_debug_log.update(|log| *log = format!("{}{}", log, log_entry));
    };

    let generate_mock_interpretation = move |dream_text: &str| -> String {
        add_log("Using MOCK mode - generating local interpretation");
        
        let dream_keywords = [
            "flying", "fall", "falling", "chase", "chased", "teeth", "water", 
            "ocean", "mountain", "school", "exam", "test", "naked", "bird", "dog", 
            "cat", "house", "car", "death", "dying", "gold", "silver", "glass"
        ];
        
        let contains_keyword = dream_keywords.iter()
            .any(|&keyword| dream_text.to_lowercase().contains(keyword));
            
        if !contains_keyword {
            return format!(
                "Your dream about \"{}\" is unique! Dreams often reflect our subconscious mind processing daily experiences and emotions. Consider what symbols in this dream might represent in your waking life.",
                dream_text.chars().take(40).collect::<String>()
            );
        }
        
        if dream_text.to_lowercase().contains("flying") {
            return "Dreams of flying often represent feelings of freedom, power, or escaping from something in your waking life. They can indicate a desire to rise above your circumstances or gain a new perspective on a situation.".to_string();
        } else if dream_text.to_lowercase().contains("fall") || dream_text.to_lowercase().contains("falling") {
            return "Dreams about falling typically represent insecurity, anxiety, or feeling overwhelmed. They may indicate you're afraid of losing control in some aspect of your life.".to_string();
        } else if dream_text.to_lowercase().contains("chase") || dream_text.to_lowercase().contains("chased") {
            return "Being chased in dreams often reflects avoiding a person or issue that makes you feel threatened. Consider what you might be running from in your waking life.".to_string();
        } else if dream_text.to_lowercase().contains("teeth") {
            return "Dreams involving teeth (falling out, breaking, etc.) commonly symbolize anxiety about your appearance or how others perceive you. They may also represent communication challenges.".to_string();
        } else if dream_text.to_lowercase().contains("water") || dream_text.to_lowercase().contains("ocean") {
            return "Water in dreams often represents your emotional state. Calm water might suggest peace, while turbulent water could indicate emotional turmoil. The ocean specifically can symbolize the unconscious mind.".to_string();
        } else if dream_text.to_lowercase().contains("mountain") {
            return "Mountains in dreams typically symbolize challenges, obstacles, or aspirations. Climbing a mountain may represent your journey toward a goal, while being at the summit could indicate achievement.".to_string();
        } else if dream_text.to_lowercase().contains("school") || dream_text.to_lowercase().contains("exam") || dream_text.to_lowercase().contains("test") {
            return "Dreams about school, exams, or tests often reflect feelings of being unprepared or evaluated. They might indicate performance anxiety or fear of failure in your current life.".to_string();
        } else if dream_text.to_lowercase().contains("naked") {
            return "Dreams of being naked in public typically represent vulnerability or fear of exposure. You might be worried about revealing your true self or being judged by others.".to_string();
        } else if dream_text.to_lowercase().contains("bird") {
            return "Birds in dreams often symbolize freedom, perspective, and aspirations. They can represent your thoughts or desires to transcend limitations.".to_string();
        } else if dream_text.to_lowercase().contains("dog") {
            return "Dogs in dreams frequently represent loyalty, friendship, and protection. Consider your relationship with dogs and what specific qualities they embody for you.".to_string();
        } else if dream_text.to_lowercase().contains("cat") {
            return "Cats in dreams often symbolize independence, intuition, and femininity. They may represent aspects of yourself that are mysterious or difficult to control.".to_string();
        } else if dream_text.to_lowercase().contains("house") {
            return "Houses in dreams typically represent the self, with different rooms corresponding to different aspects of your personality or life. The condition of the house often reflects how you perceive yourself.".to_string();
        } else if dream_text.to_lowercase().contains("car") {
            return "Cars in dreams often symbolize your direction in life and the control you feel. Problems with a car might indicate obstacles in your path or uncertainty about your direction.".to_string();
        } else if dream_text.to_lowercase().contains("death") || dream_text.to_lowercase().contains("dying") {
            return "Dreams about death rarely predict literal death. Instead, they typically symbolize transformation, endings, and new beginnings. Something in your life may be changing significantly.".to_string();
        } else if dream_text.to_lowercase().contains("gold") || dream_text.to_lowercase().contains("silver") {
            return "Precious metals like gold or silver in dreams often represent value, wealth, or spiritual riches. They might indicate something you treasure or aspects of yourself that you value highly.".to_string();
        } else if dream_text.to_lowercase().contains("glass") {
            return "Glass in dreams can symbolize fragility, transparency, or barriers. Consider whether you're feeling vulnerable or if there's something separating you from what you desire.".to_string();
        } else {
            return format!(
                "Your dream about \"{}\" contains symbols that reflect your personal experiences and emotions. Dreams often process our daily thoughts and feelings in symbolic form. Consider what these specific symbols mean to you personally.",
                dream_text.chars().take(40).collect::<String>()
            );
        }
    };

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        // Don't submit if already loading or empty text
        if is_loading() || dream_text().trim().is_empty() {
            add_log("Submission canceled: form is either loading or text is empty");
            return;
        }

        set_is_loading(true);
        set_error(None);
        set_interpretation(None);
        set_debug_log(String::new());
        
        let current_dream_text = dream_text().clone();
        let current_api_url = api_url().clone();
        let should_use_mock = use_mock();
        let current_cors_mode = cors_mode().clone();

        add_log(&format!("Mode: {}", if should_use_mock { "MOCK" } else { "LIVE API" }));
        
        if should_use_mock {
            // Use mock mode - generate interpretation locally
            let mock_interpretation = generate_mock_interpretation(&current_dream_text);
            add_log("Setting mock interpretation result");
            set_interpretation(Some(mock_interpretation));
            set_is_loading(false);
            return;
        }

        add_log(&format!("Starting API request to: {}", current_api_url));

        spawn_local(async move {
            // Create request
            add_log("Creating request options...");
            let mut opts = RequestInit::new();
            #[allow(deprecated)]
            {
                opts.method("POST");
                let body_json = format!(
                    r#"{{"dream_text": "{}"}}"#,
                    current_dream_text.replace(r#"""#, r#"\""#)
                );
                add_log(&format!("Request body: {}", body_json));
                opts.body(Some(&JsValue::from_str(&body_json)));
                
                // Set the CORS mode
                add_log(&format!("Setting CORS mode to: {}", current_cors_mode));
                match current_cors_mode.as_str() {
                    "no-cors" => opts.mode(web_sys::RequestMode::NoCors),
                    _ => opts.mode(web_sys::RequestMode::Cors), // Default to "cors" mode
                };
            }

            add_log(&format!("Initializing request to URL: {}", current_api_url));
            let request = match Request::new_with_str_and_init(
                &current_api_url,
                &opts
            ) {
                Ok(req) => {
                    add_log("Request created successfully, setting headers...");
                    match req.headers().set("Content-Type", "application/json") {
                        Ok(_) => add_log("Content-Type header set successfully"),
                        Err(e) => add_log(&format!("Failed to set headers: {:?}", e))
                    }
                    req
                },
                Err(e) => {
                    let err_msg = format!("Failed to create request: {:?}", e);
                    add_log(&err_msg);
                    error!("{}", err_msg);
                    set_error(Some("Failed to create request. Check the API URL.".to_string()));
                    set_is_loading(false);
                    return;
                }
            };

            // Send the request
            add_log("Sending request...");
            let window = web_sys::window().unwrap();
            let fetch_promise = window.fetch_with_request(&request);
            add_log("Fetch initiated, awaiting response...");
            
            let resp_value = match wasm_bindgen_futures::JsFuture::from(fetch_promise).await {
                Ok(val) => {
                    add_log("Received response from server");
                    val
                },
                Err(e) => {
                    let error_string = format!("{:?}", e);
                    let err_msg = if error_string.contains("Failed to fetch") || error_string.contains("NetworkError") || error_string.contains("ERR_FAILED") {
                        if error_string.contains("CORS") || error_string.contains("cross-origin") || error_string.contains("Access-Control-Allow-Origin") {
                            add_log("CORS error detected - cross-origin request blocked");
                            format!(
                                "CORS Error: The browser blocked the request from port 3000 to port 3001 due to security restrictions. \
                                Try these solutions:\n\
                                1. Enable the 'Use Mock Mode' toggle above to test without an API\n\
                                2. Choose 'no-cors' mode in API settings (but response data will be limited)\n\
                                3. Run the API server on the same port as the frontend\n\
                                4. Configure your API server to include CORS headers"
                            )
                        } else {
                            add_log("Network error - failed to connect to API server");
                            format!("Failed to connect to the API server at: {}. \
                                   The server might be down or unreachable.", current_api_url)
                        }
                    } else {
                        format!("Network error: {:?}", e)
                    };
                    
                    add_log(&err_msg);
                    error!("{}", err_msg);
                    set_error(Some(err_msg));
                    set_is_loading(false);
                    return;
                }
            };

            // Convert to Response and check status
            add_log("Converting response to Response object...");
            let resp: Response = match resp_value.dyn_into() {
                Ok(resp) => resp,
                Err(e) => {
                    let err_msg = format!("Failed to parse response as Response object: {:?}", e);
                    add_log(&err_msg);
                    error!("{}", err_msg);
                    set_error(Some("Failed to parse response".to_string()));
                    set_is_loading(false);
                    return;
                }
            };

            // Check if response is ok
            let status = resp.status();
            add_log(&format!("Response status: {}", status));
            
            // If we're using no-cors mode, we can't access the response details
            // No-cors responses are "opaque" - we can't see status, headers, or body
            if current_cors_mode == "no-cors" {
                // For no-cors mode, we can't reliably determine if the request succeeded
                // So we just display a message that we received an opaque response
                add_log("Received opaque response from no-cors request - unable to read response details");
                set_error(Some("Received an opaque response using no-cors mode. We can't verify if the API call succeeded. Try enabling mock mode instead, or configure your API server with proper CORS headers.".to_string()));
                set_is_loading(false);
                return;
            }
            
            if !resp.ok() {
                let err_msg = match status {
                    405 => "Server returned 405 Method Not Allowed. The API endpoint does not accept POST requests. Please verify the API URL and ensure the server supports POST requests.".to_string(),
                    404 => "Server returned 404 Not Found. The API endpoint could not be found. Please verify the API URL is correct.".to_string(),
                    403 => "Server returned 403 Forbidden. The request was denied access. Authentication may be required.".to_string(),
                    500 => "Server returned 500 Internal Server Error. There's an issue with the API server.".to_string(),
                    502 => "Server returned 502 Bad Gateway. There might be an issue with the API server or proxy.".to_string(),
                    _ => format!("Server returned error status: {}", status)
                };
                add_log(&err_msg);
                error!("{}", err_msg);
                set_error(Some(err_msg));
                set_is_loading(false);
                return;
            }

            // Parse JSON response
            add_log("Parsing response body as text...");
            let text = match wasm_bindgen_futures::JsFuture::from(resp.text().unwrap()).await {
                Ok(text) => {
                    let text_str = text.as_string().unwrap_or_default();
                    add_log(&format!("Response text received: {}", text_str));
                    text_str
                },
                Err(e) => {
                    let err_msg = format!("Failed to get response text: {:?}", e);
                    add_log(&err_msg);
                    error!("{}", err_msg);
                    set_error(Some("Failed to get response text".to_string()));
                    set_is_loading(false);
                    return;
                }
            };

            // Parse the text as JSON
            add_log("Parsing text as JSON...");
            match serde_json::from_str::<DreamResponse>(&text) {
                Ok(data) => {
                    add_log(&format!("JSON parsed successfully: {:?}", data));
                    if data.status == "success" {
                        add_log("Interpretation successful, updating UI");
                        set_interpretation(Some(data.interpretation));
                    } else {
                        let err_msg = format!("API returned error status: {}", data.status);
                        add_log(&err_msg);
                        warn!("{}", err_msg);
                        set_error(Some("The server returned an error".to_string()));
                    }
                },
                Err(e) => {
                    let err_msg = format!("Failed to parse response data: {:?}", e);
                    add_log(&err_msg);
                    error!("{}", err_msg);
                    set_error(Some(err_msg));
                }
            }
            
            add_log("Request processing complete");
            set_is_loading(false);
        });
    };

    view! {
        <div class="dream-interpreter">
            <h2>"Dream Interpreter"</h2>

            <div class="form-group api-settings">
                <div class="mode-toggle">
                    <label for="use-mock" class="toggle-label">
                        <input
                            type="checkbox"
                            id="use-mock"
                            prop:checked=use_mock
                            on:change=move |ev| {
                                set_use_mock(event_target_checked(&ev));
                            }
                        />
                        <span>"Use Mock Mode (no API required)"</span>
                    </label>
                </div>

                <div
                    class="url-input"
                    style:display=move || if use_mock() { "none" } else { "block" }
                >
                    <label for="api-url">"API URL:"</label>
                    <input
                        type="text"
                        id="api-url"
                        prop:value=api_url
                        on:input=move |ev| {
                            set_api_url(event_target_value(&ev));
                        }
                        style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;"
                    />

                    <div class="cors-settings">
                        <label>"CORS Mode:"</label>
                        <div class="radio-group">
                            <label class="radio-label">
                                <input
                                    type="radio"
                                    name="cors-mode"
                                    value="cors"
                                    checked=move || cors_mode() == "cors"
                                    on:change=move |_| set_cors_mode("cors".to_string())
                                />
                                <span>"cors"</span>
                                <small class="help-text">
                                    "Standard mode (requires server CORS headers)"
                                </small>
                            </label>
                            <label class="radio-label">
                                <input
                                    type="radio"
                                    name="cors-mode"
                                    value="no-cors"
                                    checked=move || cors_mode() == "no-cors"
                                    on:change=move |_| set_cors_mode("no-cors".to_string())
                                />
                                <span>"no-cors"</span>
                                <small class="help-text">
                                    "Opaque response (may help with CORS errors but limits response data)"
                                </small>
                            </label>
                        </div>
                    </div>
                </div>
            </div>

            <form on:submit=handle_submit>
                <div class="form-group">
                    <label for="dream-text">"Describe your dream:"</label>
                    <textarea
                        id="dream-text"
                        placeholder="I was flying over mountains made of glass while a golden bird guided me through clouds..."
                        prop:value=dream_text
                        on:input=move |ev| {
                            set_dream_text(event_target_value(&ev));
                        }
                        rows="5"
                    ></textarea>
                </div>
                <button type="submit" class="submit-btn" disabled=move || is_loading()>
                    {move || if is_loading() { "Interpreting..." } else { "Interpret Dream" }}
                </button>
            </form>

            <div class="results">
                {move || {
                    error()
                        .map(|err| {
                            let is_cors_error = err.contains("CORS Error");

                            view! {
                                <div class="error-message">
                                    <p>{err}</p>
                                    <div class="error-hint">
                                        <p>
                                            {if is_cors_error {
                                                "CORS issues can be bypassed by using Mock Mode for testing."
                                            } else {
                                                "API connection failed. Consider using Mock Mode to test without an API."
                                            }}
                                        </p>

                                        <div class="error-actions">
                                            <button
                                                class="enable-mock-btn"
                                                on:click=move |_| set_use_mock(true)
                                                style:display=move || {
                                                    if use_mock() { "none" } else { "block" }
                                                }
                                            >
                                                "Enable Mock Mode"
                                            </button>
                                        </div>

                                        // Only show the No-CORS button for CORS errors
                                        <div
                                            class="error-actions"
                                            style:display=move || {
                                                if is_cors_error && !use_mock() && cors_mode() != "no-cors"
                                                {
                                                    "block"
                                                } else {
                                                    "none"
                                                }
                                            }
                                        >
                                            <button
                                                class="enable-nocors-btn"
                                                on:click=move |_| set_cors_mode("no-cors".to_string())
                                            >
                                                "Try No-CORS Mode"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                }}
                {move || {
                    interpretation()
                        .map(|interp| {
                            view! {
                                <div class="interpretation">
                                    <h3>"Your Dream Interpretation"</h3>
                                    <p class="interpretation-text">{interp}</p>
                                </div>
                            }
                        })
                }} <details class="debug-logs">
                    <summary>Debug Logs</summary>
                    <pre style="text-align: left; white-space: pre-wrap; overflow-x: auto; background-color: #f0f0f0; padding: 1rem; border-radius: 4px; font-size: 0.8rem;">
                        {move || debug_log()}
                    </pre>
                </details>
            </div>
        </div>
    }
}

// Helper to render CORS error actions
fn render_cors_error(
    set_use_mock: WriteSignal<bool>,
    set_cors_mode: WriteSignal<String>,
    use_mock: ReadSignal<bool>,
    cors_mode: ReadSignal<String>
) -> impl IntoView {
    view! {
        <div class="cors-error-actions">
            <h4>"Quick Actions:"</h4>
            <button
                class="enable-mock-btn"
                on:click=move |_| set_use_mock(true)
                style:display=move || if use_mock() { "none" } else { "block" }
            >
                "Enable Mock Mode"
            </button>

            <button
                class="enable-nocors-btn"
                on:click=move |_| set_cors_mode("no-cors".to_string())
                style:display=move || {
                    if cors_mode() == "no-cors" || use_mock() { "none" } else { "block" }
                }
            >
                "Try No-CORS Mode"
            </button>
        </div>
    }
}

// Helper to render general error actions
fn render_general_error(
    set_use_mock: WriteSignal<bool>,
    use_mock: ReadSignal<bool>
) -> impl IntoView {
    view! {
        <div>
            <p>
                "API connection failed. Consider using the " <strong>"Mock Mode"</strong>
                " toggle above to test the dream interpreter without requiring an API server."
            </p>
            <button
                class="enable-mock-btn"
                on:click=move |_| set_use_mock(true)
                style:display=move || if use_mock() { "none" } else { "block" }
            >
                "Enable Mock Mode"
            </button>
        </div>
    }
} 