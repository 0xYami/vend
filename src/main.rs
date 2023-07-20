mod config;
mod entities;
mod handlers;
mod jwt;

use axum::Router;
use config::Config;
use jwt::Jwt;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::entities::{ArticleEntity, UserEntity};

pub struct AppState {
    jwt: Jwt,
    user_entity: UserEntity,
    article_entity: ArticleEntity,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ichiba=debug,axum::rejection=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::try_from_env().expect("Failed to load configuration");

    let pool = PgPoolOptions::new()
        .max_connections(config.db.max_connections)
        .connect(config.db.url.as_str())
        .await
        .expect("Failed to connect to database");

    let user_entity = UserEntity::new(pool.clone());
    let article_entity = ArticleEntity::new(pool.clone());
    let jwt = Jwt::new(config.jwt.clone());
    let state = Arc::new(AppState {
        jwt,
        user_entity,
        article_entity,
    });

    let app = Router::new()
        .merge(handlers::router(state))
        .layer(
            CorsLayer::new()
                .allow_origin(config.cors.allowed_origin)
                .allow_methods(config.cors.allowed_methods)
                .allow_headers(config.cors.allowed_headers),
        )
        .layer(TraceLayer::new_for_http());

    info!("Server listening on {}", config.addr);

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
