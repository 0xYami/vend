use crate::AppState;
use axum::{
    extract::{Path, State},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateUser {
    name: String,
}

pub fn router(state: Arc<AppState>) -> Router {
    async fn get_user(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i32>,
    ) -> Result<Json<User>, StatusCode> {
        if id < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let tx = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&state.pool)
            .await;

        match tx {
            Ok(tx) => Ok(Json(tx)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn create_user(
        State(state): State<Arc<AppState>>,
        Json(user): Json<CreateUser>,
    ) -> Result<Json<User>, StatusCode> {
        let tx = sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1")
            .bind(user.name.clone())
            .fetch_optional(&state.pool)
            .await;

        match tx {
            Ok(Some(_)) => return Err(StatusCode::BAD_REQUEST),
            Ok(None) => (),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let token = match state.jwt.generate(user.name.clone()) {
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            Ok(token) => token,
        };

        let tx = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, jwt) VALUES ($1, $2) RETURNING id, name, jwt",
        )
        .bind(user.name)
        .bind(token)
        .fetch_one(&state.pool)
        .await;

        match tx {
            Ok(tx) => Ok(Json(tx)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn update_user(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i32>,
        TypedHeader(headers): TypedHeader<Authorization<Bearer>>,
        Json(user): Json<UpdateUser>,
    ) -> Result<Json<User>, StatusCode> {
        if id < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let token = headers.token();
        if state.jwt.validate(String::from(token)).is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let tx = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND jwt = $2")
            .bind(id)
            .bind(token)
            .fetch_optional(&state.pool)
            .await;

        match tx {
            Ok(Some(_)) => (),
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let tx = sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name, jwt",
        )
        .bind(user.name)
        .bind(id)
        .fetch_one(&state.pool)
        .await;

        match tx {
            Ok(tx) => Ok(Json(tx)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    Router::new()
        .route("/users/:id", get(get_user).put(update_user))
        .route("/users", post(create_user))
        .with_state(state)
}
