use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct ArticleEntity {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "article_type", rename_all = "snake_case")]
pub enum ArticleType {
    Cap,
    Kimono,
    Jacket,
    Hoodie,
    TShirt,
    Shoes,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "size", rename_all = "snake_case")]
pub enum ArticleSize {
    VeryLarge,
    Large,
    Medium,
    Small,
    VerySmall,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
pub enum ArticleGender {
    Male,
    Female,
    Unisex,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum ArticleStatus {
    InSale,
    Inactive,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    id: i32,
    title: String,
    description: String,
    owner_id: i32,
    image_id: String,
    size: ArticleSize,
    gender: ArticleGender,
    price: i32,
    status: ArticleStatus,
    article_type: ArticleType,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub description: String,
    pub owner_id: i32,
    pub image_id: String,
    pub size: ArticleSize,
    pub gender: ArticleGender,
    pub price: i32,
    pub status: ArticleStatus,
    pub article_type: ArticleType,
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

    pub async fn create(&self, article: CreateArticle) -> Result<Article> {
        let tx = sqlx::query_as::<_, Article>(
            "INSERT INTO articles (title, description, owner_id, image_id, size, gender, price, status, article_type) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        )
        .bind(article.title)
        .bind(article.description)
        .bind(article.owner_id)
        .bind(article.image_id)
        .bind(article.size)
        .bind(article.gender)
        .bind(article.price)
        .bind(article.status)
        .bind(article.article_type)
        .fetch_one(&self.pool)
        .await?;
        Ok(tx)
    }
}
