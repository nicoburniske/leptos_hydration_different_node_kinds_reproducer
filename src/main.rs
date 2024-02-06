#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_image::ImageCacheRoute;
    use reproducer_node_mismatch::app::*;
    use reproducer_node_mismatch::fileserv::file_and_error_handler;

    #[derive(Clone, axum::extract::FromRef)]
    struct AppState {
        leptos_options: leptos::LeptosOptions,
        optimizer: leptos_image::ImageOptimizer,
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let state = AppState {
        leptos_options: leptos_options.clone(),
        optimizer: leptos_image::ImageOptimizer::new(
            "/__cache/image",
            leptos_options.site_root.clone(),
            1,
        ),
    };

    let addr = leptos_options.site_addr;

    leptos_query::suppress_query_load(true);
    let routes = generate_route_list(App);
    leptos_query::suppress_query_load(false);

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            axum::routing::post(leptos_axum::handle_server_fns),
        )
        .image_cache_route(&state)
        .leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), App)
        .fallback(file_and_error_handler)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
