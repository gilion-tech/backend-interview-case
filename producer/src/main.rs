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
    tracing_subscriber::fmt::init();

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
        Welcome to AI Quantum Innovations co.

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
        let random_date = get_random_data_two();
        Ok::<_, Infallible>(random_date)
    });

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/octet-stream")
        .body(Body::from_stream(stream))
        .unwrap()
}

fn get_random_data_two() -> axum::body::Bytes {
    let mut rng = rand::thread_rng();

    let year: u16 = rng.gen_range(1960..=2025);
    let month: u8 = rng.gen_range(1..=12);
    let day: u8 = rng.gen_range(1..=31);
    let a: u8 = rng.gen_range(0..=255);
    let b: u8 = rng.gen_range(0..=255);

    let mut buffer = Vec::with_capacity(6);

    buffer.extend_from_slice(&year.to_be_bytes());
    buffer.push(month);
    buffer.push(day);
    buffer.push(a);
    buffer.push(b);

    axum::body::Bytes::from(buffer)
}
