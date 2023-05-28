use crate::handlers::route;
use axum::{routing::get, Router};

pub fn router() -> Router {
    async fn handler() -> &'static str {
        "healthy"
    }

    route("/_health", get(handler))
}
