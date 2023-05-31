use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct ArticleEntity {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    id: i32,
    pub title: String,
    pub description: String,
    pub owner_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct NewArticle {
    pub title: String,
    pub description: String,
    pub owner_id: i32,
}

impl ArticleEntity {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Article>> {
        let tx = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(tx)
    }

    pub async fn create(&self, article: NewArticle) -> Result<Article> {
        let tx = sqlx::query_as::<_, Article>(
            "INSERT INTO articles (title, description, owner_id) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(article.title)
        .bind(article.description)
        .bind(article.owner_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(tx)
    }
}
