use std::{fs::File, io::{Write, Read, BufReader}};
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
    use crate::sequence::Sequence;

    

    pub type SequenceRef = Arc<Mutex<Sequence>>;

    pub async fn update_sequence(uri: Uri, State(seq): State<SequenceRef>, axum::Json(new_seq) : axum::Json<String>) -> axum::response::Response {
        // let mut file = File::create("test.json").unwrap();
        // file.write_all(new_seq.as_bytes()).unwrap();
        println!("Updating content: {}", &new_seq);
        (*seq.clone().lock().unwrap()).update_from_json(&new_seq);
        "Hey! I got it!".into_response()
    }

    pub async fn display_sequence(uri: Uri, State(seq): State<SequenceRef>, req: Request<Body>) -> axum::response::Response {
        let seq_inner = seq.lock().unwrap();
        seq_inner.into_json().into_response()
    }

    pub async fn display_plot_content() {

    }
}}