use {
    crate::{error::Error, models::DbEvent, validate_group, AppState},
    axum::{
        extract::{Path, State},
        headers::{CacheControl, ContentType},
        response::IntoResponse,
        TypedHeader,
    },
    ics::ICalendar,
    mime_guess::mime::Mime,
    std::str::FromStr,
};

pub async fn calendar(
    State(AppState { db, .. }): State<AppState>,
    Path(groups): Path<String>,
) -> Result<impl IntoResponse, Error> {
    // retrieve requested event groups from path
    let groups = groups
        .split('+')
        .map(|g| validate_group(g).map(ToOwned::to_owned))
        .collect::<Result<Vec<_>, Error>>()?;

    // for each group, pull all it's events from db, put into ical and return
    let events = sqlx::query_as!(
        DbEvent,
        r#"SELECT * FROM events WHERE "group" = ANY($1)"#,
        &groups
    )
    .fetch_all(&db)
    .await?;

    let body = {
        let mut calendar = ICalendar::new("2.0", "khronos");

        events
            .iter()
            .for_each(|event| calendar.add_event(event.into()));

        let mut body = vec![];
        calendar.write(&mut body).unwrap();
        body
    };

    Ok((
        TypedHeader(ContentType::from(
            Mime::from_str("text/calendar").expect("Failed to parse text/calendar MIME type"),
        )),
        TypedHeader(CacheControl::new().with_no_cache()),
        body,
    ))
}
