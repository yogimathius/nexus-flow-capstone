use crate::components::api::check_server_status;
use crate::components::journal::{
    models::{Dream, Pagination},
    service::fetch_dreams,
    JournalCard, JournalEmptyState, JournalError, JournalFilters, JournalLoading,
    JournalPagination, OfflineBanner, OfflineIndicator,
};
use leptos::prelude::*;

#[component]
pub fn JournalList() -> impl IntoView {
    let (dreams, set_dreams) = signal(Vec::<Dream>::new());
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (pagination, set_pagination) = signal(Pagination {
        current_page: 1,
        total_pages: 1,
        items_per_page: 10,
    });
    let (search_query, set_search_query) = signal(String::new());
    let (is_offline, set_is_offline) = signal(false);
    let (refresh_trigger, set_refresh_trigger) = signal(0);

    let fetch_deps = Memo::new(move |_| {
        (refresh_trigger.get(), pagination.get(), search_query.get())
    });

    Effect::new(move |_| {
        let (_, pagination, query) = fetch_deps.get();
        set_is_loading.set(true);
        set_error.set(None);

        check_server_status(move |is_online| {
            set_is_offline.set(!is_online);

            fetch_dreams(
                &pagination,
                &query,
                move |data, server_pagination| {
                    set_dreams.set(data);

                    set_pagination.set(server_pagination);

                    set_is_loading.set(false);
                },
                move |err_msg| {
                    set_error.set(Some(err_msg));
                    set_is_loading.set(false);
                },
            );
        });
    });

    let load_dreams = move || {
        set_refresh_trigger.update(|count| *count += 1);
    };

    let check_server = move || {
        check_server_status(move |is_online| {
            set_is_offline.set(!is_online);
            if is_online {
                load_dreams();
            } else {
                set_error.set(Some(
                    "The backend server appears to be offline. Please ensure it's running.".into(),
                ));
            }
        });
    };

    let handle_page_change = Callback::new(move |new_page: usize| {
        set_pagination.update(|p| {
            p.current_page = new_page;
        });
    });

    let handle_search = Callback::new(move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_pagination.update(|p| {
            p.current_page = 1;
        });
    });

    let handle_refresh = Callback::new(move |()| {
        load_dreams();
    });

    let handle_retry = Callback::new(move |()| {
        check_server();
    });

    view! {
        <div class="container">
            <Show when=move || is_offline.get()>
                <OfflineIndicator />
            </Show>

            <JournalFilters
                search_query=search_query
                set_search_query=set_search_query
                on_search=handle_search
                on_refresh=handle_refresh
            />

            <div class="journal-content">
                <Show when=move || is_loading.get()>
                    <JournalLoading />
                </Show>

                <Show when=move || {
                    error.get().is_some()
                }>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <JournalError error_message=err on_retry=handle_retry /> }
                            })
                    }}
                </Show>

                <Show when=move || !is_loading.get() && error.get().is_none() fallback=|| view! {}>
                    <Show
                        when=move || !dreams.get().is_empty()
                        fallback=move || {
                            view! { <JournalEmptyState /> }
                        }
                    >
                        <div class="dreams-grid">
                            {move || {
                                dreams
                                    .get()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, dream)| {
                                        let delay_class = match index % 3 {
                                            0 => "animation-delay-1",
                                            1 => "animation-delay-2",
                                            _ => "animation-delay-3",
                                        };

                                        view! {
                                            <JournalCard dream=dream animation_delay=delay_class />
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </div>
                    </Show>
                </Show>
            </div>

            <Show when=move || { pagination.get().total_pages > 1 }>
                <JournalPagination pagination=pagination on_page_change=handle_page_change />
            </Show>

            <Show when=move || is_offline.get()>
                <OfflineBanner retry_action=handle_retry />
            </Show>
        </div>
    }
}
