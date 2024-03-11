use chrono::NaiveDate;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct SaleOrder {
    pub id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub total_price: f64,
    pub order_date: NaiveDate,
}
