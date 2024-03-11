use crate::graphql::types::Product;
use crate::models::product::Product as ProductModel;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};
use sqlx::SqlitePool;

pub struct QueryRoot;

#[juniper::graphql_object(Context = SqlitePool)]
impl QueryRoot {
    async fn product_by_id(context: &SqlitePool, id: i32) -> FieldResult<Option<Product>> {
        let result = ProductModel::find_by_id(context, id).await;
        match result {
            Ok(product) => Ok(Some((product).into())),
            Err(sqlx::Error::RowNotFound) => {
                log::info!("Product with ID {} not found.", id);
                Ok(None)
            }
            Err(e) => {
                log::error!("Database error: {:?}", e);
                Err(juniper::FieldError::from(
                    "An error occurred while retrieving the product.",
                ))
            }
        }
    }

    async fn all_products(context: &SqlitePool) -> FieldResult<Vec<Product>> {
        let result = ProductModel::find_all(context).await;
        match result {
            Ok(products) if products.is_empty() => Ok(vec![]),
            Ok(products) => Ok(products.into_iter().map(Into::into).collect()),
            Err(e) => {
                log::error!("Database error: {:?}", e);
                Err(juniper::FieldError::from(
                    "An error occurred while retrieving the products.",
                ))
            }
        }
    }
}

pub type Schema =
    RootNode<'static, QueryRoot, EmptyMutation<SqlitePool>, EmptySubscription<SqlitePool>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
