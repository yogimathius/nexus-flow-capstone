use crate::components::badges::{Badge, BadgeVariant};
use crate::components::markdown::InterpretationText;
use leptos::prelude::*;

#[component]
pub fn DreamResult(
    interpretation: Signal<Option<String>>,
    saved_dream_id: Signal<Option<String>>,
) -> impl IntoView {
    move || {
        interpretation.get().map(|text| view! {
            <div class="dream-interpretation-results slide-up" role="region" aria-label="Dream interpretation result">
                <div class="dream-interpretation-header">
                    <h2 class="text-xl font-bold" id="result-heading">Dream Interpretation</h2>
                    {saved_dream_id
                        .get()
                        .map(|id| {
                            view! {
                                <div class="flex items-center gap-2 mt-2 slide-in-right" 
                                     role="status" 
                                     aria-live="polite">
                                    <Badge variant=BadgeVariant::Dream>"Saved to Journal"</Badge>
                                    <span class="text-secondary text-sm">ID: {id}</span>
                                </div>
                            }
                        })}
                </div>

                <div role="article" aria-labelledby="result-heading">
                    <InterpretationText content=text />
                </div>

                <div class="text-center mt-6 fade-in animation-delay-3">
                    <div class="interpretation-actions">
                        <a href="/journal" 
                           class="button button-outline"
                           aria-label="View your dream journal">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="18"
                                height="18"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="mr-2"
                                aria-hidden="true"
                                role="presentation"
                            >
                                <path d="M3 6v18h18"></path>
                                <path d="M3 12h18"></path>
                                <path d="M3 18h18"></path>
                                <path d="M18 3v18"></path>
                                <path d="M12 3v18"></path>
                                <path d="M6 3v18"></path>
                            </svg>
                            "View Dream Journal"
                        </a>
                        
                        <button 
                            class="button-outline ml-4"
                            on:click=move |_| { 
                                // Scroll to top - this doesn't actually scroll but reloads the page
                                let window = window();
                                let _ = window.location().reload();
                            }
                            title="Create a new dream interpretation"
                            aria-label="Create a new dream interpretation"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="18"
                                height="18"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="mr-2"
                                aria-hidden="true"
                                role="presentation"
                            >
                                <path d="M12 5v14M5 12h14"></path>
                            </svg>
                            "New Interpretation"
                        </button>
                    </div>
                </div>
            </div>
        })
    }
}
