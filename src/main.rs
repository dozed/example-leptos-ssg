#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use example_leptos_ssg::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};
    use std::time::Instant;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let (routes, static_routes) = generate_route_list_with_ssg(App);

    let now = Instant::now();
    println!("start: {now:?}");
    static_routes.generate(&leptos_options).await;
    let elapsed = now.elapsed();
    println!("done, took {s} secs", s = elapsed.as_secs());

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
