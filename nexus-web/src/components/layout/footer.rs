use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-content">
                    <p>"© 2026 Nexus Flow · Built with Leptos"</p>
                </div>
            </div>
        </footer>
    }
}
