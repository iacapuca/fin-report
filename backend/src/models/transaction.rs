use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Type};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionType {
    Sale,
    Buy,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub amount: f64,
    pub description: Option<String>,
    pub date: NaiveDate,
    pub transaction_type: TransactionType,
}

impl Transaction {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Transaction>, sqlx::Error> {
        sqlx::query_as::<_, Transaction>("SELECT * FROM transactions")
            .fetch_all(pool)
            .await
    }
}
