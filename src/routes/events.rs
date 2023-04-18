//! GET, PUT, and DELETE event handlers

use {
    crate::{
        auth::{AdminAuth, UserAuth},
        error::Error,
        models::{DbEvent, EventResponse, NewEvent},
        AppState,
    },
    axum::{
        extract::{Path, State},
        headers::CacheControl,
        response::{IntoResponse, Json},
        TypedHeader,
    },
};

/// Get all events
pub async fn get_events(
    _: AdminAuth,
    State(AppState { db, .. }): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let events = sqlx::query_as!(DbEvent, "SELECT * FROM events")
        .fetch_all(&db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<EventResponse>>();

    Ok((
        TypedHeader(CacheControl::new().with_no_cache()),
        Json(events),
    ))
}

/// Add a new event
pub async fn add_event(
    _: UserAuth,
    State(AppState { .. }): State<AppState>,
    Json(_event): Json<NewEvent>,
) -> Result<impl IntoResponse, Error> {
    Ok(())
}

/// Update existing event
pub async fn update_event(
    _: UserAuth,
    State(AppState { .. }): State<AppState>,
    Path(_event_id): Path<String>,
    Json(_event): Json<NewEvent>,
) -> Result<impl IntoResponse, Error> {
    Ok(())
}

/// Delete an event
pub async fn delete_event(
    _: AdminAuth,
    State(AppState { .. }): State<AppState>,
    Path(_event_id): Path<String>,
) -> Result<impl IntoResponse, Error> {
    Ok(())
}
