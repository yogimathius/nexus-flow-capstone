use leptos::prelude::*;

use super::footer::Footer;
use super::header::Header;

#[component]
pub fn Layout(
    #[prop(default = String::from("page"))] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class=format!("layout {}", class)>
            <Header />
            <main class="main-content">
                <div class="container">{children()}</div>
            </main>
            <Footer />
        </div>
    }
}
