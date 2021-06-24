use serde::{Serialize};
use crate::handler::root::Context;
use crate::handler::user::User;
use juniper::{GraphQLInputObject};

#[derive(Debug, Default, Serialize)]
pub struct Product {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub price: i32
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub user_id: i32,
    pub name: String,
    pub price: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Product {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn user_id(&self) -> &i32 {
        &self.user_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> &i32 {
        &self.price
    }

    async fn user(&self, context: &Context) -> Option<User> {
        let conn = context.dbpool.clone();
        let data;
        if let Ok(user) = sqlx::query_as!(User,
            "SELECT * FROM users WHERE id = $1",
            &self.user_id
        ).fetch_one(&conn).await{
            data = Some ( User {
                    id: user.id,
                    name: user.name,
                    email: user.email
                })
        }else{
            data = None
        }
        data
    }
}