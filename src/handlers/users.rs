use axum::{
    extract::{Path, State},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    entities::{NewUser, User},
    AppState,
};

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

        let tx = state.user_entity.get_by_id(id).await;
        match tx {
            Ok(tx) => Ok(Json(tx)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn create_user(
        State(state): State<Arc<AppState>>,
        Json(user): Json<CreateUser>,
    ) -> Result<Json<User>, StatusCode> {
        let tx = state.user_entity.get_by_name(user.name.clone()).await;
        match tx {
            Ok(Some(_)) => return Err(StatusCode::BAD_REQUEST),
            Ok(None) => (),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let token = match state.jwt.generate(user.name.clone()) {
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            Ok(token) => token,
        };

        let new_user = NewUser {
            name: user.name,
            jwt: token.clone(),
        };

        let tx = state.user_entity.create(new_user).await;
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

        let token = headers.token().to_string();
        if state.jwt.validate(token.clone()).is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let tx = state.user_entity.get_by_id_and_jwt(id, token).await;
        match tx {
            Ok(Some(_)) => (),
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }

        let tx = state.user_entity.update(id, user.name).await;
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
