use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params,
    params::Params,
    path,
    static_routes::StaticRoute,
    SsrMode,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=path!("/books/:book_id")
                        view=BookPage
                        ssr=SsrMode::Static(
                            StaticRoute::new()
                                .prerender_params(|| async move {
                                    println!("Calling prerender_params");
                                    let book_ids = list_book_ids();
                                    println!("book_ids: {l}", l=book_ids.len());

                                    [("book_id".into(), book_ids)]
                                        .into_iter()
                                        .collect()
                                })
                        )
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookPageError {
    #[error("Invalid book ID.")]
    InvalidId,
    #[error("Book not found.")]
    BookNotFound,
    #[error("Server error: {0}.")]
    ServerError(String),
}

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct BookPageParams {
    book_id: Option<String>,
}

#[component]
fn BookPage() -> impl IntoView {
    let query = use_params::<BookPageParams>();
    let book_id = move || {
        query
            .get()
            .map(|q| q.book_id.unwrap_or_default())
            .map_err(|_| BookPageError::InvalidId)
    };

    let book_resource = Resource::new_blocking(book_id, |book_id| async move {
        println!("book_id: {book_id:?}");
        match book_id {
            Err(e) => Err(e),
            Ok(book_id) => Ok(book_id),
        }
    });

    let book_view = move || {
        Suspend::new(async move {
            match book_resource.await {
                Ok(book_id) => Ok(view! {
                    <h1>Book {book_id.clone()}</h1>
                }),
                Err(e) => Err(BookPageError::ServerError(e.to_string())),
            }
        })
    };

    view! {
        <Suspense fallback=move || view! { <p>"Loading person page..."</p> }>
            <ErrorBoundary fallback=|errors| {
                #[cfg(feature = "ssr")]
                expect_context::<leptos_axum::ResponseOptions>()
                    .set_status(http::StatusCode::NOT_FOUND);
                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}

                        </ul>
                    </div>
                }
            }>{book_view}</ErrorBoundary>
        </Suspense>
    }
}

fn list_book_ids() -> Vec<String> {
    let book_ids = vec![
        "bk101".to_owned(),
        "bk102".to_owned(),
        "bk103".to_owned(),
        "bk104".to_owned(),
        "bk105".to_owned(),
        "bk106".to_owned(),
        "bk107".to_owned(),
        "bk108".to_owned(),
        "bk109".to_owned(),
        "bk110".to_owned(),
        "bk111".to_owned(),
        "bk112".to_owned(),
    ];

    book_ids.iter().cycle().take(4_000_000).cloned().collect()
}
