use axum::{http::Uri, response::IntoResponse};

mod health;
mod static_files;
mod calendar;

pub use {health::health, static_files::static_files, calendar::calendar};

pub async fn index() -> impl IntoResponse {
    static_files(Uri::from_static("/index.html")).await
}
