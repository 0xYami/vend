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
    #[serde(rename = "cap")]
    Cap,
    #[serde(rename = "kimono")]
    Kimono,
    #[serde(rename = "jacket")]
    Jacket,
    #[serde(rename = "hoodie")]
    Hoodie,
    #[serde(rename = "t_shirt")]
    TShirt,
    #[serde(rename = "shoes")]
    Shoes,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "size", rename_all = "snake_case")]
pub enum ArticleSize {
    #[serde(rename = "very_large")]
    VeryLarge,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "very_small")]
    VerySmall,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
pub enum ArticleGender {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "unisex")]
    Unisex,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum ArticleStatus {
    #[serde(rename = "in_sale")]
    InSale,
    #[serde(rename = "inactive")]
    Inactive,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "snake_case")]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub owner_id: i32,
    pub image_id: String,
    pub size: ArticleSize,
    pub gender: ArticleGender,
    pub price: i32,
    pub status: ArticleStatus,
    pub article_type: ArticleType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
