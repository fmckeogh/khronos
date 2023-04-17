use {
    chrono::{DateTime, Utc},
    ics::{
        self, escape_text,
        properties::{Categories, Description, DtEnd, DtStart, Organizer, Summary},
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub email: String,
    pub description: String,
    pub group: String,
    /// Start of event in milliseconds since UNIX epoch
    pub start: i64,
    /// End of event in milliseconds since UNIX epoch
    pub end: i64,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    /// UUID of event
    pub id: String,
    pub name: String,
    pub email: String,
    pub description: String,
    pub group: String,
    /// Start of event in milliseconds since UNIX epoch
    pub start: i64,
    /// End of event in milliseconds since UNIX epoch
    pub end: i64,
}

impl From<DbEvent> for EventResponse {
    fn from(
        DbEvent {
            id,
            name,
            email,
            description,
            group,
            start,
            end,
        }: DbEvent,
    ) -> Self {
        Self {
            id: id.as_hyphenated().to_string(),
            name,
            email,
            description,
            group,
            start: start.timestamp_millis(),
            end: end.timestamp_millis(),
        }
    }
}

#[derive(Debug)]
pub struct DbEvent {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub description: String,
    pub group: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl<'a> From<&'a DbEvent> for ics::Event<'a> {
    fn from(
        DbEvent {
            id,
            name,
            email,
            description,
            group,
            start,
            end,
        }: &'a DbEvent,
    ) -> Self {
        let mut event = ics::Event::new(id.as_hyphenated().to_string(), format_datetime(start));

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

pub struct DbGroup {
    pub name: String,
}
