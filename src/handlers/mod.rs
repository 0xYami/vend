mod articles;
mod health;
mod users;

use crate::AppState;
use axum::{routing::MethodRouter, Router};
use std::sync::Arc;

pub use users::User;

pub fn router(pool: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::router())
        .merge(users::router(pool.clone()))
        .merge(articles::router(pool))
}

fn route(path: &str, method: MethodRouter<()>) -> Router {
    Router::new().route(path, method)
}
