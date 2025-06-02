use crate::components::journal::models::Pagination;
use leptos::prelude::*;

#[component]
pub fn JournalPagination(
    #[prop(into)] pagination: Signal<Pagination>,
    #[prop(into)] on_page_change: Callback<usize>,
) -> impl IntoView {
    let p = pagination.get();

    view! {
        <div class="pagination slide-up animation-delay-4">
            <button
                class="page-button"
                disabled=p.current_page == 1
                on:click=move |_| {
                    if p.current_page > 1 {
                        on_page_change.run(p.current_page - 1);
                    }
                }
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m15 18-6-6 6-6"></path>
                </svg>
                "Previous"
            </button>

            <span class="page-info">"Page " {p.current_page} " of " {p.total_pages}</span>

            <button
                class="page-button"
                disabled=p.current_page == p.total_pages
                on:click=move |_| {
                    if p.current_page < p.total_pages {
                        on_page_change.run(p.current_page + 1);
                    }
                }
            >
                "Next"
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m9 18 6-6-6-6"></path>
                </svg>
            </button>
        </div>
    }
}
