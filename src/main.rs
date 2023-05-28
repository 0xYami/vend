mod config;
mod handlers;
mod jwt;

use axum::Router;
use config::Config;
use jwt::Jwt;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    jwt: Jwt,
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vend=debug,axum::rejection=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::try_from_env().expect("Failed to load configuration");

    let pool = PgPoolOptions::new()
        .max_connections(config.db.max_connections)
        .connect(config.db.url.as_str())
        .await
        .expect("Failed to connect to database");

    let jwt = Jwt::new(config.jwt);

    let state = Arc::new(AppState { jwt, pool });

    let app = Router::new()
        .merge(handlers::router(state))
        .layer(TraceLayer::new_for_http());

    info!("Server listening on {}", config.addr);

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
