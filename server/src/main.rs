pub mod mutool;

use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use mutool::Resolution;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// These should be input params, but they are hardcoded for simplicity
const INPUT_IMAGE: &str = "./data/test1.png";
const INPUT_PDF: &str = "./data/test.pdf";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = State::default();

    // Compose the routes
    let app = Router::new().route("/blank", get(is_blank)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(state))
            .into_inner(),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type State = Arc<Mutex<HashMap<Resolution, Box<String>>>>;

#[derive(Debug, Serialize, Clone)]
struct Response {
    blank_hash: String,
    image_hash: String,
    is_blank: bool,
}

async fn is_blank(
    Query(resolution): Query<Resolution>,
    Extension(state): Extension<State>,
) -> impl IntoResponse {
    let blank_hash = compute_blank_hash(resolution, state);

    let image_hash = sha256(INPUT_IMAGE.to_string());

    if image_hash == *blank_hash {
        Json(Response {
            image_hash,
            blank_hash: *blank_hash,
            is_blank: !mutool::check_images(INPUT_PDF.to_string()),
        })
    } else {
        Json(Response {
            image_hash,
            blank_hash: *blank_hash,
            is_blank: false,
        })
    }
}

fn compute_blank_hash(resolution: Resolution, state: State) -> Box<String> {
    let mut data = state.lock().unwrap();

    match data.get(&resolution) {
        Some(hash) => {
            tracing::info!("Hash found!");
            hash.clone()
        }
        None => {
            tracing::info!("Hash not found, computing it...");

            // Create blank image using the given resolution
            // and save the hash in global state
            mutool::create_blank_pdf();
            mutool::create_blank_png(&resolution);

            let filename = format!(
                "/tmp/blank-{}-{}1.png",
                resolution.density, resolution.height
            );
            let hash = Box::new(sha256(filename));
            data.insert(resolution, hash.clone());

            hash
        }
    }
}

fn sha256(filename: String) -> String {
    let input = File::open(Path::new(filename.as_str())).unwrap();
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };

    format!("{:X}", digest)
}
