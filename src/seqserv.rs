use std::{fs::File, io::{Write, Read, BufReader}};
use std::sync::{Arc, Mutex};
use axum::{Json, extract};
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
    use crate::sequence::Sequence;

    

    type SequenceRef = Arc<Mutex<Sequence>>;

    pub async fn update_sequence(uri: Uri, State(seq): State<SequenceRef>, extract::Json(new_seq) : extract::Json<Sequence>) -> axum::response::Response {
        (*seq.clone().lock().unwrap()).replace(new_seq);
        log::info!("Updating content");
        ().into_response()
    }

    pub async fn display_sequence(uri: Uri, State(seq): State<SequenceRef>, req: Request<Body>) -> axum::response::Response {
        let seq_inner = seq.lock().unwrap();
        seq_inner.into_json().into_response()
    }

    pub async fn display_plot_content() {

    }
}}