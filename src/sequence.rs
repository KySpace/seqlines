use std::sync::{Arc, Mutex};

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        body::{boxed, Body, BoxBody},
        extract::State,
        response::IntoResponse,
        http::{Request, StatusCode, Uri},
    };

    use axum::response::Response as AxumResponse;
    use tower::ServiceExt;
    use tower_http::services::ServeDir;
    use leptos::*;
    use crate::app::App;

    #[derive(Clone)]
    pub struct RawSequence {
        pub content : String,
    }

    impl RawSequence {
        pub fn update(&mut self, val : &String) {
            self.content = val.clone();
        }
    }

    type Sequence = Arc<Mutex<RawSequence>>;

    pub async fn update_sequence(uri: Uri, State(seq): State<Sequence>, new_seq : String) -> axum::response::Response {
        (*seq.clone().lock().unwrap()).update(&new_seq);
        log::info!("Updating content: {}", &new_seq);
        ().into_response()
    }

    pub async fn display_sequence(uri: Uri, State(seq): State<Sequence>, req: Request<Body>) -> axum::response::Response {
        let seq_inner = seq.lock().unwrap().clone();
        seq_inner.content.into_response()
    }

    pub async fn display_plot_content() {

    }
}}