use crate::models::product::Product as ProductModel;
use crate::models::transaction::Transaction as TransactionModel;
use chrono::NaiveDate;
use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "An Imperial Product")]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}

#[derive(GraphQLEnum)]
#[graphql(description = "Type of Transaction")]

pub enum TransactionType {
    Buy,
    Sale,
}

#[derive(GraphQLObject)]
#[graphql(description = "An Imperial Transaction")]
pub struct Transaction {
    pub id: String,
    pub product_id: Option<String>,
    pub amount: f64,
    pub description: Option<String>,
    pub date: NaiveDate,
}

impl From<ProductModel> for Product {
    fn from(product: ProductModel) -> Self {
        Product {
            id: product.id.to_string(),
            name: product.name,
            price: product.price,
        }
    }
}

impl From<TransactionModel> for Transaction {
    fn from(transaction: TransactionModel) -> Self {
        Transaction {
            id: transaction.id.to_string(),
            product_id: transaction.product_id.map(|id| id.to_string()),
            amount: transaction.amount,
            description: transaction.description,
            date: transaction.date,
        }
    }
}
