use {
    crate::{error::Error, CALENDAR_MAX_AGE},
    axum::{
        extract::{Path, State},
        http::{
            header::{CACHE_CONTROL, CONTENT_TYPE},
            HeaderMap, HeaderValue,
        },
        response::IntoResponse,
    },
    chrono::{DateTime, Utc},
    ics::{
        escape_text,
        properties::{Categories, Description, DtEnd, DtStart, Organizer, Summary},
        Event, ICalendar,
    },
    once_cell::sync::Lazy,
    regex::Regex,
    sqlx::{Pool, Postgres},
    uuid::Uuid,
};

static GROUP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[0-9a-z]+$").unwrap());

pub async fn calendar(
    State(db): State<Pool<Postgres>>,
    Path(groups): Path<String>,
) -> Result<impl IntoResponse, Error> {
    // retrieve requested event groups from path
    let groups = groups
        .split('+')
        .map(|s| match GROUP_REGEX.is_match(s) {
            true => Ok(s.to_owned()),
            false => Err(Error::InvalidGroupFormat(groups.clone())),
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // for each group, pull all it's events from db, put into ical and return
    let events = sqlx::query_as!(
        DbEvent,
        r#"SELECT * FROM events WHERE "group" = ANY($1)"#,
        &groups
    )
    .fetch_all(&db)
    .await?;

    let mut calendar = ICalendar::new("2.0", "khronos");

    events
        .iter()
        .for_each(|event| calendar.add_event(event.into()));

    let mut body = vec![];
    calendar.write(&mut body).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("calendar/text"));
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&format!("public, max-age={CALENDAR_MAX_AGE}, immutable")).unwrap(),
    );

    Ok((headers, body))
}

#[derive(Debug)]
struct DbEvent {
    name: String,
    email: String,
    description: String,
    group: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl<'a> From<&'a DbEvent> for Event<'a> {
    fn from(
        DbEvent {
            name,
            email,
            description,
            group,
            start,
            end,
        }: &'a DbEvent,
    ) -> Self {
        let mut event = Event::new(
            Uuid::new_v4().as_hyphenated().to_string(),
            format_datetime(start),
        );

        event.push(Summary::new(name));
        event.push(Description::new(escape_text(description)));
        event.push(Organizer::new(format!("mailto:{email}")));
        event.push(DtStart::new(format_datetime(start)));
        event.push(DtEnd::new(format_datetime(end)));
        event.push(Categories::new(group));

        event
    }
}

fn format_datetime(datetime: &DateTime<Utc>) -> String {
    datetime.format("%Y%m%dT%H%M%SZ").to_string()
}
