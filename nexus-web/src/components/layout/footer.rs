use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-content">
                    <p>"© 2025 Dream Interpreter AI · Powered by Leptos"</p>
                </div>
            </div>
        </footer>
    }
}
