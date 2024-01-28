use std::sync::Arc;

use leptos::LeptosOptions;
use seqlines::{app::HomePage, sequence::Sequence};
use seqlines::seqserv::SequenceRef;
use axum::{extract::State, response::Html, routing::get, Router};

#[derive(Clone, Debug, axum::extract::FromRef)]
struct AppState {
    leptos_options : LeptosOptions,
    sequence_ref : SequenceRef,
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::{Mutex, Arc};

    use axum::{routing::post, routing::get, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use seqlines::app::*;
    use seqlines::fileserv::file_and_error_handler;
    use seqlines::sequence::Sequence;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let sequence_ref = Arc::new(Mutex::new(Sequence::empty()));
    let app_state = AppState { leptos_options, sequence_ref };

    // build our application with a route
    let app = Router::new()
        .route("/state", get(seqlines::seqserv::display_sequence))
        .route("/state", post(seqlines::seqserv::update_sequence))
        // .route("/", get(get_leptos_component))
        .route("/test", get(test_route))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&app_state, routes, App)
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_route() -> &'static str {
    "A test on the server."
}

async fn get_leptos_component(State(seq): State<SequenceRef>) -> Html<String> {
    leptos::ssr::render_to_string(HomePage).to_string().into()
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
