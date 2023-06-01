use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct ImageEntity {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Image {
    pub id: i32,
    pub article_id: i32,
    pub filename: String,
    pub data: Vec<u8>,
}

impl ImageEntity {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Image> {
        let tx = sqlx::query_as::<_, Image>("SELECT * FROM images WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(tx)
    }

    pub async fn create(&self, image: Image) -> Result<Image> {
        let tx = sqlx::query_as::<_, Image>(
            "INSERT INTO images (article_id, filename, data) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(image.article_id)
        .bind(image.filename)
        .bind(image.data)
        .fetch_one(&self.pool)
        .await?;
        Ok(tx)
    }
}
