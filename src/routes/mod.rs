use axum::{http::Uri, response::IntoResponse};

mod calendar;
mod events;
mod groups;
mod health;
mod static_files;

pub use {
    calendar::calendar,
    events::{add_event, delete_event, get_events, update_event},
    groups::{add_group, delete_group, get_groups},
    health::health,
    static_files::static_files,
};

pub async fn index() -> impl IntoResponse {
    static_files(Uri::from_static("/index.html")).await
}
