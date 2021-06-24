use serde::{Serialize};

use crate::handler::product::Product;
use crate::handler::root::Context;
use juniper::{GraphQLInputObject};

#[derive(Debug, Default, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String
}

#[juniper::graphql_object(Context = Context)]
impl User{
    fn id(&self) -> &i32 {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    async fn products(&self, context: &Context) -> Vec<Product> {
        let conn = context.dbpool.clone();
        let product = sqlx::query_as!(Product,
            "SELECT id, user_id, name, price::integer FROM product WHERE user_id = $1",
            &self.id
        ).fetch_all(&conn).await;

        product.expect("User list product error")
    }
}