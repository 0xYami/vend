mod health;

use axum::{routing::MethodRouter, Router};
use sqlx::PgPool;

pub fn router(pool: PgPool) -> Router {
    Router::new().merge(health::router())
}

fn route(path: &str, method: MethodRouter<()>) -> Router {
    Router::new().route(path, method)
}
