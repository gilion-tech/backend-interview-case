use std::convert::Infallible;

use axum::{
    body::{Body, Bytes},
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};
use futures::{stream, StreamExt};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/v1/data-stream", get(get_data_stream));

    let addr = "localhost:3000";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "
        Welcome to Quantum Innovations Inc.

        We are a company that specializes in quantum computing and data science.
        We are currently working on a project that involves generating synthetic data.
        This project is still in the early stages, but we are making progress.
        Our goal is to create a data stream that can be used for training forecasting models.
        But before we can do that, we need a middle layer which feeds the data to our visualizer app.

        The data stream can be accessed at /api/v1/data-stream
    "
}

async fn get_data_stream() -> Response<Body> {
    let stream = stream::iter(0..1_000_000_000).map(|_| {
        let random_date = get_random_data();
        Ok::<_, Infallible>(random_date)
    });

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .body(Body::from_stream(stream))
        .unwrap()
}

fn get_random_data() -> axum::body::Bytes {
    let mut rng = rand::thread_rng();
    let random_date = format!(
        "{:04}{:02}{:02}{:03}{:03}",
        rng.gen_range(1960..=2025),
        rng.gen_range(1..=12),
        rng.gen_range(1..=31),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255)
    );
    Bytes::from(random_date)
}
