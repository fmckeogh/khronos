use {
    crate::{
        error::Error,
        log::{create_trace_layer, tracing_init},
        routes::{
            add_event, add_group, calendar, delete_event, delete_group, get_events, get_groups,
            health, index, static_files, update_event,
        },
    },
    axum::{
        routing::{get, put},
        Router,
    },
    color_eyre::eyre::Result,
    jsonwebtoken::DecodingKey,
    once_cell::sync::Lazy,
    regex::Regex,
    sqlx::postgres::{PgPoolOptions, Postgres},
    std::{net::SocketAddr, time::Duration},
    tokio::task::JoinHandle,
    tower_http::compression::CompressionLayer,
    tracing::{debug, info},
};

pub mod auth;
pub mod config;
pub mod error;
pub mod log;
pub mod models;
pub mod routes;

use sqlx::Pool;

pub use crate::config::Config;

/// Static files cached time
const STATIC_FILES_MAX_AGE: Duration = Duration::from_secs(300);

/// Cache time for calendar requests
const CALENDAR_MAX_AGE: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
    key: DecodingKey,
}

/// Starts a new instance of the contractor returning a handle
pub async fn start(config: &Config) -> Result<Handle> {
    // initialize global tracing subscriber
    tracing_init()?;

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await?;

    debug!("running migrations");
    sqlx::migrate!().run(&pool).await?;

    let key = DecodingKey::from_base64_secret(&config.jwt_secret)?;

    let state = AppState { db: pool, key };

    // create router with all routes and tracing layer
    let router = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/calendar/:groups", get(calendar))
        .route("/groups", get(get_groups))
        .route("/groups/:name", put(add_group).delete(delete_group))
        .route("/events", get(get_events).post(add_event))
        .route("/events/:id", put(update_event).delete(delete_event))
        .fallback(static_files)
        .with_state(state)
        .layer(CompressionLayer::new().br(true).deflate(true).gzip(true))
        .layer(create_trace_layer());

    // bind axum server to socket address and use router to create a service factory
    let server = axum::Server::bind(&config.address).serve(router.into_make_service());

    // get address server is bound to (may be different to address passed to Server::bind)
    let address = server.local_addr();

    // spawn server on new tokio task
    let handle = tokio::spawn(async { server.await.map_err(Into::into) });

    info!("khronos started on http://{}", address);

    // return handles
    Ok(Handle { address, handle })
}

/// Handle for running an instance
pub struct Handle {
    // Socket address instance is bound to
    address: SocketAddr,
    // JoinHandle for server task
    handle: JoinHandle<Result<()>>,
}

impl Handle {
    /// Gets the socket address the running instance is bound to
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Awaits on the instance's task
    pub async fn join(self) -> Result<()> {
        self.handle.await??;
        Ok(())
    }
}

pub fn validate_group(name: &str) -> Result<&str, Error> {
    /// Group name validator
    static GROUP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[0-9a-z]+$").unwrap());

    if !GROUP_REGEX.is_match(name) {
        return Err(Error::InvalidGroupFormat(name.to_owned()));
    }

    Ok(name)
}
