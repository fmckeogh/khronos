use {
    crate::{error::Error, models::DbGroup, validate_group},
    axum::{
        extract::{Path, State},
        http::StatusCode,
        response::{IntoResponse, Json},
    },
    sqlx::{Pool, Postgres},
};

pub async fn get_groups(State(db): State<Pool<Postgres>>) -> Result<impl IntoResponse, Error> {
    let groups = sqlx::query_as!(DbGroup, "SELECT * FROM groups")
        .fetch_all(&db)
        .await?;

    Ok(Json(groups.into_iter().map(|g| g.name).collect::<Vec<_>>()))
}

/// requires auth
pub async fn add_group(
    State(db): State<Pool<Postgres>>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, Error> {
    validate_group(&name)?;

    sqlx::query!("INSERT INTO groups VALUES ($1)", name)
        .execute(&db)
        .await?;

    Ok(())
}

/// requires auth
pub async fn delete_group(
    State(db): State<Pool<Postgres>>,
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
