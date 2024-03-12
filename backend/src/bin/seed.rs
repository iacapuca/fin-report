use dotenvy::dotenv;
use fin_report::models::seeds;
use sqlx::SqlitePool;
use std::env;
use tokio;

async fn seed_db(
    pool: &SqlitePool,
    model: &str,
    num_transactions: Option<usize>,
) -> Result<(), sqlx::Error> {
    match model {
        "products" => seeds::seed_products(pool).await,
        "transactions" => {
            let count = num_transactions.unwrap_or_default();
            seeds::seed_transactions(pool, count).await
        }
        _ => {
            eprintln!("Model '{}' is not recognized for seeding.", model);
            Err(sqlx::Error::RowNotFound) // Using RowNotFound as a placeholder error
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin seed <model> [<num_transactions>]");
        std::process::exit(1);
    }

    let model_to_seed = &args[1];
    let num_transactions = if args.len() > 2 {
        Some(args[2].parse().expect("Invalid number of transactions"))
    } else {
        None
    };
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    if let Err(e) = seed_db(&pool, model_to_seed, num_transactions).await {
        eprintln!("Error seeding the database: {}", e);
    }
}
