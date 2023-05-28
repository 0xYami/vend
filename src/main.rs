mod config;
mod handlers;

use axum::Router;
use config::Config;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let app = Router::new()
        .merge(handlers::router(pool))
        .layer(TraceLayer::new_for_http());

    info!("Server listening on {}", config.addr);

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
