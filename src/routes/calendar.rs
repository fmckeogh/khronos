use {
    crate::{error::Error, CALENDAR_MAX_AGE},
    axum::{
        extract::State,
        http::{
            header::{CACHE_CONTROL, CONTENT_TYPE},
            HeaderMap, HeaderValue,
        },
        response::IntoResponse,
    },
    ics::{
        escape_text,
        properties::{Categories, Description, DtEnd, DtStart, Organizer, Status, Summary},
        Event, ICalendar,
    },
    sqlx::{Pool, Postgres},
};

pub async fn calendar(State(_): State<Pool<Postgres>>) -> Result<impl IntoResponse, Error> {
    let mut body = vec![];

    let mut calendar = ICalendar::new("2.0", "ics-rs");

    let mut event = Event::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19960704T120000Z");
    // add properties
    event.push(Organizer::new("mailto:jsmith@example.com"));
    event.push(DtStart::new("19960918T143000Z"));
    event.push(DtEnd::new("19960920T220000Z"));
    event.push(Status::confirmed());
    event.push(Categories::new("CONFERENCE"));
    event.push(Summary::new("Networld+Interop Conference"));
    // Values that are "TEXT" must be escaped (only if the text contains a comma,
    // semicolon, backslash or newline).
    event.push(Description::new(escape_text(
        "Networld+Interop Conference and Exhibit\n\
         Atlanta World Congress Center\n\
         Atlanta, Georgia",
    )));
    // add event to calendar
    calendar.add_event(event);

    calendar.write(&mut body).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("calendar/text"));
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&format!("public, max-age={CALENDAR_MAX_AGE}, immutable")).unwrap(),
    );

    Ok((headers, body))
}
