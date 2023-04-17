//! GET, PUT, and DELETE event handlers

use {
    crate::{
        error::Error,
        models::{DbEvent, EventResponse, NewEvent},
    },
    axum::{
        extract::{Json, Path, State},
        response::IntoResponse,
    },
    sqlx::{Pool, Postgres},
};

/// Auth required
pub async fn get_events(State(db): State<Pool<Postgres>>) -> Result<impl IntoResponse, Error> {
    let events = sqlx::query_as!(DbEvent, "SELECT * FROM events")
        .fetch_all(&db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<EventResponse>>();

    Ok(Json(events))
}

/// Auth required
pub async fn add_event(
    State(_db): State<Pool<Postgres>>,
    Json(_event): Json<NewEvent>,
) -> Result<impl IntoResponse, Error> {
    Ok(())
}

/// Auth required post Event (no id), get DbEvent in response
pub async fn update_event(
    State(_db): State<Pool<Postgres>>,
    Path(_event_id): Path<String>,
    Json(_event): Json<NewEvent>,
) -> Result<impl IntoResponse, Error> {
    Ok(())
}

/// Auth required
pub async fn delete_event(
    State(_db): State<Pool<Postgres>>,
    Path(_event_id): Path<String>,
) -> Result<impl IntoResponse, Error> {
    Ok(())
}
