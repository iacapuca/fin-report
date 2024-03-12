use chrono::{Duration, NaiveDate};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::transaction::TransactionType;

pub async fn seed_products(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let products_data = vec![
        ("Spice Melange", 129.99),
        ("Water of Life", 199.99),
        ("Krys Knife", 249.99),
        ("Ornithopter Model", 99.99),
        ("Fremen Sietch Tapestry", 59.99),
        ("Sardaukar Battle Gear", 349.99),
        ("Arrakeen Sand Coffee", 14.99),
        ("Navigators Guild Incense", 23.99),
        ("Caladan Wine", 45.99),
        ("Giedi Prime Cigar", 30.99),
        ("Duncan Idaho Action Figure", 75.99),
        ("House Atreides Banner", 19.99),
        ("House Harkonnen Flag", 19.99),
        ("Bene Gesserit Cloak", 89.99),
        ("Guild Navigator Fish Tank", 500.99),
        ("Piter De Vries Brain Teaser Puzzle", 35.99),
        ("Stillsuit", 150.99),
        ("Fremen Sand Boots", 120.99),
        ("Paul Atreides Desert Fatigues", 175.99),
        ("Chani's Fremkit", 210.99),
    ];

    for (name, price) in products_data.iter() {
        let id = Uuid::new_v4(); // Generate a new UUID for each product

        let _ = sqlx::query!(
            "INSERT INTO products (id, name, price) VALUES (?, ?, ?)",
            id,
            name,
            price
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn seed_transactions(
    pool: &SqlitePool,
    num_transactions: usize,
) -> Result<(), sqlx::Error> {
    let product_exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM products LIMIT 1)")
        .fetch_one(pool)
        .await?;

    if !product_exists {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut rng = rand::thread_rng();

    for _ in 0..num_transactions {
        let id = Uuid::new_v4();
        let product_id: Option<Uuid> =
            sqlx::query_scalar("SELECT id FROM products ORDER BY RANDOM() LIMIT 1")
                .fetch_optional(pool)
                .await?;
        let amount = rng.gen_range(10.0..=1000.0);
        let date = generate_random_date();
        let description: Option<String> = Some(
            rng.clone()
                .sample_iter(&Alphanumeric)
                .take(rng.gen_range(10..=20))
                .map(char::from)
                .collect(),
        );

        let transaction_type = if rng.gen_bool(0.5) {
            TransactionType::Sale
        } else {
            TransactionType::Buy
        };

        let transaction_type_str = match transaction_type {
            TransactionType::Sale => "sale",
            TransactionType::Buy => "buy",
        };

        sqlx::query(
            "INSERT INTO transactions (id, product_id, amount, description, date, transaction_type)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(product_id)
        .bind(amount)
        .bind(description)
        .bind(date)
        .bind(transaction_type_str)
        .execute(pool)
        .await?;
    }

    Ok(())
}

fn generate_random_date() -> NaiveDate {
    let start_date = NaiveDate::from_ymd_opt(2022, 1, 1);
    let end_date = NaiveDate::from_ymd_opt(2024, 1, 1);
    let range = end_date.unwrap().signed_duration_since(start_date.unwrap());
    let mut rng = rand::thread_rng();
    let offset = Duration::try_days(rng.gen_range(0..range.num_days()));
    start_date.unwrap() + offset.unwrap()
}
