mod articles;
mod health;
mod users;

use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub use users::User;

pub fn router(pool: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::router())
        .merge(users::router(pool.clone()))
        .merge(articles::router(pool))
}
