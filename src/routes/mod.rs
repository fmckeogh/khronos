use axum::{http::Uri, response::IntoResponse};

mod calendar;
mod events;
mod health;
mod static_files;

pub use {calendar::calendar, health::health, static_files::static_files};

pub async fn index() -> impl IntoResponse {
    static_files(Uri::from_static("/index.html")).await
}
