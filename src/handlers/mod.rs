mod health;
mod users;

use crate::AppState;
use axum::{routing::MethodRouter, Router};
use std::sync::Arc;

pub fn router(pool: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::router())
        .merge(users::router(pool))
}

fn route(path: &str, method: MethodRouter<()>) -> Router {
    Router::new().route(path, method)
}
