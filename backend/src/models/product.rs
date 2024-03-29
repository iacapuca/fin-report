use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
}

impl Product {
    pub async fn find_by_id(
        pool: &SqlitePool,
        product_id: &String,
    ) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
            .bind(product_id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM products")
            .fetch_all(pool)
            .await
    }
}
