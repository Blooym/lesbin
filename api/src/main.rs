mod cryptography;
mod database;
mod middleware;
mod routes;

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
    http::{HeaderValue, header},
    middleware::{self as axum_middleware, Next},
    routing::{delete, get, post},
};
use bytesize::ByteSize;
use chrono::Utc;
use clap::Parser;
use clap_duration::duration_range_value_parse;
use database::Database;
use dotenvy::dotenv;
use duration_human::{DurationHuman, DurationHumanValidator};
use email_address::EmailAddress;
use middleware::api_auth_middleware;
use routes::{get_config_handler, paste_create_handler, paste_delete_handler, paste_get_handler};
use sqlx::query;
use std::{convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, signal, task::JoinHandle};
use tower_http::{
    catch_panic::CatchPanicLayer,
    normalize_path::NormalizePathLayer,
    trace::{self, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, debug, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Parser)]
#[clap(author, about, version)]
struct Arguments {
    /// Internet socket address that the server should be ran on.
    #[arg(
        long = "address",
        env = "LESBIN_API_ADDRESS",
        default_value = "127.0.0.1:8255"
    )]
    address: SocketAddr,

    /// The SQLite database URL to use for persistent storage.
    #[arg(long = "database-url", env = "DATABASE_URL")]
    database_url: String,

    /// The access token that must be provided alongside all requests to execute any action on this server.
    #[arg(
        long = "access-token",
        env = "LESBIN_API_ACCESS_TOKEN",
        required = true
    )]
    access_token: String,

    /// The maximum allowed size of a paste. Paste size is calculated by combining the sizes of the title and content.
    #[arg(
        long = "paste-max-size",
        env = "LESBIN_API_PASTE_MAX_SIZE",
        default_value = "512kb"
    )]
    paste_max_size: ByteSize,

    /// Whether pastes are required to have an expiry time attached.
    #[arg(
        long = "paste-expiry-required",
        env = "LESBIN_API_PASTE_EXPIRY_REQUIRED",
        default_value_t = false
    )]
    paste_expiry_required: bool,

    /// The maximum expiry time of a paste, calculated by adding this time to the time of the paste creation.
    #[clap(long = "paste-max-expiry", env = "LESBIN_API_PASTE_MAX_EXPIRY", default_value = "1year", value_parser = duration_range_value_parse!(min: 60min, max: 10years))]
    paste_max_expiry: DurationHuman,

    /// The email
    #[clap(long = "report-email", env = "LESBIN_API_REPORT_EMAIL")]
    report_email: Option<EmailAddress>,
}

#[derive(Clone)]
struct AppState {
    database: Arc<Database>,
    access_token: String,

    paste_max_size: ByteSize,
    paste_max_expiry: Duration,
    paste_expiry_required: bool,

    report_email: Option<EmailAddress>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .init();
    let args = Arguments::parse();

    let app_state = AppState {
        access_token: args.access_token,
        database: Arc::new(Database::new(&args.database_url).await?),
        paste_max_size: args.paste_max_size,
        paste_expiry_required: args.paste_expiry_required,
        paste_max_expiry: Duration::from(&args.paste_max_expiry),
        report_email: args.report_email,
    };
    spawn_expiry_task(app_state.database.clone());

    let router = Router::new()
        .nest(
            "/instance",
            Router::new().route("/config", get(get_config_handler)),
        )
        .nest(
            "/paste",
            Router::new()
                .route(
                    "/",
                    post(paste_create_handler).layer(DefaultBodyLimit::max(
                        args.paste_max_size
                            .0
                            .try_into()
                            .context("max paste size does not fit into usize")?,
                    )),
                )
                .route("/{id}", delete(paste_delete_handler))
                .route("/{id}", get(paste_get_handler)),
        )
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            api_auth_middleware,
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::default().level(Level::INFO))
                .on_response(DefaultOnResponse::default().level(Level::INFO))
                .on_failure(DefaultOnFailure::default().level(Level::INFO)),
        )
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CatchPanicLayer::new())
        .layer(axum_middleware::from_fn(
            async |req: Request, next: Next| {
                let mut res = next.run(req).await;
                let res_headers = res.headers_mut();
                res_headers.insert(
                    header::SERVER,
                    HeaderValue::from_static(env!("CARGO_PKG_NAME")),
                );
                res_headers.insert("X-Robots-Tag", HeaderValue::from_static("none"));
                res_headers.insert(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_static("*"),
                );
                res
            },
        ))
        .with_state(app_state);

    let tcp_listener = TcpListener::bind(args.address).await?;
    info!(
        "Internal server started - listening on: http://{}",
        args.address,
    );
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn spawn_expiry_task(database: Arc<Database>) -> JoinHandle<Infallible> {
    tokio::spawn(async move {
        loop {
            debug!("Running paste expiry check");
            let now = Utc::now().timestamp();
            match query!("DELETE FROM pastes WHERE expires_at < ?1", now)
                .execute(database.pool())
                .await
            {
                Ok(result) => {
                    debug!(
                        "Expiry check removed {} expired pastes",
                        result.rows_affected()
                    )
                }
                Err(err) => {
                    warn!("Expiry check failed to remove expired pastes: {err:?}");
                }
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    })
}

// https://github.com/tokio-rs/axum/blob/15917c6dbcb4a48707a20e9cfd021992a279a662/examples/graceful-shutdown/src/main.rs#L55
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
