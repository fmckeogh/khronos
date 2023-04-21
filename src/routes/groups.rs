use {
    crate::{auth::AdminAuth, error::Error, models::DbGroup, validate_group, AppState},
    axum::{
        extract::{Path, State},
        headers::CacheControl,
        http::StatusCode,
        response::{IntoResponse, Json},
        TypedHeader,
    },
};

/// Get all groups
pub async fn get_groups(
    State(AppState { db, .. }): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let groups = sqlx::query_as!(DbGroup, "SELECT * FROM groups")
        .fetch_all(&db)
        .await?;

    Ok((
        TypedHeader(CacheControl::new().with_no_cache()),
        Json(groups.into_iter().map(|g| g.name).collect::<Vec<_>>()),
    ))
}

/// Add a new group
pub async fn add_group(
    _: AdminAuth,
    State(AppState { db, .. }): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, Error> {
    validate_group(&name)?;

    sqlx::query!("INSERT INTO groups VALUES ($1)", name)
        .execute(&db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Delete a group
pub async fn delete_group(
    _: AdminAuth,
    State(AppState { db, .. }): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, Error> {
    validate_group(&name)?;

    if sqlx::query!("DELETE FROM groups WHERE name=$1", name)
        .execute(&db)
        .await?
        .rows_affected()
        == 0
    {
        return Ok(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
