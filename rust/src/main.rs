use std::time::Instant;

use anyhow::Context;
use axum::{
    Router,
    extract::{MatchedPath, Request},
    middleware::from_fn,
};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app_state::AppState, config::AppConfig, routes::ticket_routes,
    schemas::app_error::log_app_errors,
};

mod app_state;
mod config;
mod extractors;
mod models;
mod routes;
mod schema;
mod schemas;
mod services;
mod repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let start = Instant::now();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "info,{}=debug,tower_http=debug,diesel=debug,diesel_async=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_config = AppConfig::build().context("Failed to load config.")?;

    let app_state = AppState::build(&app_config)
        .await
        .context("Failed to connect to the database.")?;

    let app = app(app_state);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_config.server.port)).await?;

    tracing::info!("Started rust-app in {:?}.", start.elapsed());
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub fn app(app_state: AppState) -> Router {
    Router::new()
        .merge(ticket_routes::router())
        .layer(from_fn(log_app_errors))
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();

                    // axum automatically adds this extension.
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .with_state(app_state)
}

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
