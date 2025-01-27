use std::convert::Infallible;

use axum::{
    body::{Body, Bytes},
    http::Response,
    routing::get,
    Router,
};
use chrono::{Duration, NaiveDate};
use futures::stream::{self, StreamExt};
use rand::Rng;

fn gen_random_date() -> NaiveDate {
    let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let start = NaiveDate::from_ymd_opt(1950, 1, 1).unwrap();
    let days_in_range = (end - start).num_days();
    let random_days: i64 = Rng::gen_range(&mut rand::thread_rng(), 0..days_in_range);
    return start + Duration::days(random_days);
}

async fn get_data() -> Response<Body> {
    let data_stream = stream::iter(0..100).map(|num| {
        // Convert each number to a `Bytes` object
        let bytes = Bytes::from(format!("{num},"));
        Ok(bytes) as Result<Bytes, Infallible>
    });

    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from_stream(data_stream))
        .unwrap()
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/data", get(get_data));
}


