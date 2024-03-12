use sqlx::SqlitePool;
use std::collections::HashSet;

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
        let _ = sqlx::query!(
            "INSERT INTO products (name, price) VALUES (?, ?)",
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

    let product_id: Option<i32> =
        sqlx::query_scalar("SELECT id FROM products ORDER BY RANDOM() LIMIT 1")
            .fetch_optional(pool)
            .await?;

    Ok(())
}
