use crate::components::{Button, ButtonSize, ButtonVariant};
use leptos::prelude::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (message, set_message) = signal(String::new());

    // Get message box class
    let message_box_class = move || {
        if message.get().is_empty() {
            "messageBox".to_string()
        } else {
            "messageBox messageActive".to_string()
        }
    };

    view! {
        <div class="container">
            <h1 class="title">Button Component Demo</h1>

            <section class="section">
                <h2>Counter Example</h2>

                <div class="counter">
                    <div class="counterValue">{count}</div>
                    <div class="counterControls">
                        <div on:click=move |_| {
                            set_count.update(|c| *c += 1);
                            set_message.set(format!("Count incremented to {}", count.get()));
                        }>
                            <Button variant=ButtonVariant::Primary>"Increment"</Button>
                        </div>

                        <div on:click=move |_| {
                            set_count.update(|c| *c -= 1);
                            set_message.set(format!("Count decremented to {}", count.get()));
                        }>
                            <Button variant=ButtonVariant::Secondary>"Decrement"</Button>
                        </div>

                        <div on:click=move |_| {
                            set_count.set(0);
                            set_message.set("Count reset to 0".to_string());
                        }>
                            <Button variant=ButtonVariant::Danger>"Reset"</Button>
                        </div>
                    </div>
                </div>

                <div class=message_box_class>
                    {move || {
                        if message.get().is_empty() {
                            "Click a button to see a message".to_string()
                        } else {
                            message.get()
                        }
                    }}
                </div>
            </section>

            <section class="section">
                <h2>Button Variants</h2>
                <p>Click on different buttons to see their styles and interactions.</p>

                <div class="buttonRow">
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>

                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>

                    <Button variant=ButtonVariant::Warning>"Warning"</Button>

                    <Button variant=ButtonVariant::Danger>"Danger"</Button>
                </div>
            </section>

            <section class="section">
                <h2>Button Sizes</h2>

                <div class="grid">
                    <div class="labelCard">
                        <span class="labelName">Small</span>
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Small>
                            "Small"
                        </Button>
                    </div>

                    <div class="labelCard">
                        <span class="labelName">Medium (Default)</span>
                        <Button variant=ButtonVariant::Primary>"Medium"</Button>
                    </div>

                    <div class="labelCard">
                        <span class="labelName">Large</span>
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Large>
                            "Large"
                        </Button>
                    </div>
                </div>
            </section>

            <section class="section">
                <h2>Disabled State</h2>
                <div class="buttonRow">
                    <Button variant=ButtonVariant::Primary disabled=true>
                        "Disabled Primary"
                    </Button>

                    <Button variant=ButtonVariant::Secondary disabled=true>
                        "Disabled Secondary"
                    </Button>

                    <Button variant=ButtonVariant::Warning disabled=true>
                        "Disabled Warning"
                    </Button>

                    <Button variant=ButtonVariant::Danger disabled=true>
                        "Disabled Danger"
                    </Button>
                </div>
            </section>
        </div>
    }
}
