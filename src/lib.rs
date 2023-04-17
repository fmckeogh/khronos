use {
    crate::{
        log::{create_trace_layer, tracing_init},
        routes::{calendar, health, index, static_files},
    },
    axum::{routing::get, Router},
    color_eyre::eyre::Result,
    sqlx::postgres::PgPoolOptions,
    std::{net::SocketAddr, time::Duration},
    tokio::task::JoinHandle,
    tower_http::compression::CompressionLayer,
    tracing::{debug, info},
};

pub mod config;
pub mod error;
pub mod log;
pub mod routes;

pub use crate::config::Config;

/// Static files cached time in seconds
const STATIC_FILES_MAX_AGE: u64 = 300;

/// Cache time for calendar requests
const CALENDAR_MAX_AGE: u64 = 60;

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

    let compression = CompressionLayer::new().br(true).deflate(true).gzip(true);

    // create router with all routes and tracing layer
    let router = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/calendar/:groups", get(calendar))
        .fallback(static_files)
        .with_state(pool)
        .with_state(config.clone())
        .layer(compression)
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
