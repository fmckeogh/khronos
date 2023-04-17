use {
    crate::{error::Error, models::DbEvent, validate_group, CALENDAR_MAX_AGE},
    axum::{
        extract::{Path, State},
        http::{
            header::{CACHE_CONTROL, CONTENT_TYPE},
            HeaderMap, HeaderValue,
        },
        response::IntoResponse,
    },
    ics::ICalendar,
    sqlx::{Pool, Postgres},
};

pub async fn calendar(
    State(db): State<Pool<Postgres>>,
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

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("calendar/text"));
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&format!("public, max-age={CALENDAR_MAX_AGE}, immutable")).unwrap(),
    );

    Ok((headers, body))
}
