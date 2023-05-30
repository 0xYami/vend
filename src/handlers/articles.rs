use crate::{entities::NewArticle, AppState};
use axum::{
    extract::{Path, State},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Article {
    id: i32,
    pub title: String,
    pub description: String,
    pub owner_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct CreateArticle {
    title: String,
    description: String,
    owner_id: i32,
}

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

        let user = match tx {
            Ok(Some(user)) => user,
            Ok(None) => return Err(StatusCode::UNAUTHORIZED),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let new_article = NewArticle {
            title: article.title,
            description: article.description,
            owner_id: user.id,
        };

        let tx = state.article_entity.create(new_article).await;
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
