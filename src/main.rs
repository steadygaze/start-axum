cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use std::sync::Arc;

    use leptos::*;
    use axum::{
        extract::Extension,
        routing::post,
        Router,
    };
    use leptos_axum::{generate_route_list, LeptosRoutes};

    use start_axum::fileserv::file_and_error_handler;
    use start_axum::app::*;
}}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_address.clone();
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
