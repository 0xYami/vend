use axum::{
    extract::{Path, State},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use std::sync::Arc;

use crate::{
    entities::{Article, CreateArticle},
    AppState,
};

pub fn router(state: Arc<AppState>) -> Router {
    async fn get_article(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i32>,
    ) -> Result<Json<Article>, StatusCode> {
        if id < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let tx = state.article_entity.get_by_id(id).await;
        match tx {
            Ok(Some(article)) => Ok(Json(article)),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn create_article(
        State(state): State<Arc<AppState>>,
        TypedHeader(headers): TypedHeader<Authorization<Bearer>>,
        Json(article): Json<CreateArticle>,
    ) -> Result<Json<Article>, StatusCode> {
        let token = headers.token().to_string();
        if state.jwt.validate(token.clone()).is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let tx = state
            .user_entity
            .get_by_id_and_jwt(article.owner_id, token)
            .await;

        match tx {
            Ok(Some(_)) => (),
            Ok(None) => return Err(StatusCode::UNAUTHORIZED),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let tx = state.image_entity.get_by_id(article.image_id).await;
        let image = match tx {
            Ok(tx) => tx,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let tx = state.image_entity.create(image).await;
        match tx {
            Ok(_) => (),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let tx = state.article_entity.create(article).await;
        match tx {
            Ok(tx) => Ok(Json(tx)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    Router::new()
        .route("/articles", post(create_article))
        .route("/articles/:id", get(get_article))
        .with_state(state)
}
