use crate::components::button::{Button, ButtonSize, ButtonVariant};
use leptos::{prelude::*, *};
use web_sys::MouseEvent;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (message, set_message) = signal(String::new());

    let increment: Box<dyn Fn(MouseEvent)> = Box::new(move |_: MouseEvent| {
        set_count.update(|c| *c += 1);
        set_message.set(format!("Count incremented to {}", count.get()));
    });

    let decrement: Box<dyn Fn(MouseEvent)> = Box::new(move |_: MouseEvent| {
        set_count.update(|c| *c -= 1);
        set_message.set(format!("Count decremented to {}", count.get()));
    });

    let reset: Box<dyn Fn(MouseEvent)> = Box::new(move |_: MouseEvent| {
        set_count.set(0);
        set_message.set("Count reset to 0".to_string());
    });

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
                        <Button variant=ButtonVariant::Primary on_click=Some(increment)>
                            "Increment"
                        </Button>

                        <Button variant=ButtonVariant::Secondary on_click=Some(decrement)>
                            "Decrement"
                        </Button>

                        <Button variant=ButtonVariant::Danger on_click=Some(reset)>
                            "Reset"
                        </Button>
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
