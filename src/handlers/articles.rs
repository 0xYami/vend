use axum::{
    extract::{Path, State},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    entities::{Article, ArticleGender, ArticleSize, ArticleStatus, ArticleType, CreateArticle},
    AppState,
};

#[derive(Serialize, Deserialize)]
struct ArticleResponse {
    id: i32,
    title: String,
    description: String,
    owner_id: i32,
    image_url: String,
    size: ArticleSize,
    gender: ArticleGender,
    price: i32,
    status: ArticleStatus,
    article_type: ArticleType,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ArticleResponse {
    fn from_article(article: Article, s3_base_url: String) -> Self {
        Self {
            id: article.id,
            title: article.title,
            description: article.description,
            owner_id: article.owner_id,
            image_url: format!("{}/{}.png", s3_base_url, article.image_id),
            size: article.size,
            gender: article.gender,
            price: article.price,
            status: article.status,
            article_type: article.article_type,
            created_at: article.created_at,
            updated_at: article.updated_at,
        }
    }
}

pub fn router(state: Arc<AppState>) -> Router {
    async fn get_article(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i32>,
    ) -> Result<Json<ArticleResponse>, StatusCode> {
        if id < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let tx = state.article_entity.get_by_id(id).await;
        let article = match tx {
            Ok(Some(article)) => article,
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let img_base_url = state.config.s3_bucket.base_url.clone();
        let response = ArticleResponse::from_article(article, img_base_url);
        Ok(Json(response))
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
