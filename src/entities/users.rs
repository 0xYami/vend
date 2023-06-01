use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct UserEntity {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub jwt: String,
    pub balance: i32,
}

pub struct NewUser {
    pub name: String,
    pub jwt: String,
}

impl UserEntity {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<User> {
        let tx = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(tx)
    }

    pub async fn get_by_name(&self, name: String) -> Result<Option<User>> {
        let tx = sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(tx)
    }

    pub async fn get_by_id_and_jwt(&self, id: i32, jwt: String) -> Result<Option<User>> {
        let tx = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND jwt = $2")
            .bind(id)
            .bind(jwt)
            .fetch_optional(&self.pool)
            .await?;
        Ok(tx)
    }

    pub async fn create(&self, user: NewUser) -> Result<User> {
        let tx = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, jwt) VALUES ($1, $2) RETURNING id, name, jwt, balance",
        )
        .bind(user.name)
        .bind(user.jwt)
        .fetch_one(&self.pool)
        .await?;
        Ok(tx)
    }

    pub async fn update(&self, id: i32, name: String) -> Result<User> {
        let tx = sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name, jwt, balance",
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(tx)
    }
}
