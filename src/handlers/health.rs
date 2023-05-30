use axum::{routing::get, Router};

pub fn router() -> Router {
    async fn handler() -> &'static str {
        "healthy"
    }

    Router::new().route("/_health", get(handler))
}
