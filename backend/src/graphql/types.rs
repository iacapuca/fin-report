use crate::models::product::Product as ProductModel;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A product in the store")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

impl From<ProductModel> for Product {
    fn from(product: ProductModel) -> Self {
        Product {
            id: product.id,
            name: product.name,
            price: product.price,
        }
    }
}
