use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use crate::db::PgPool;
use super::product::{Product, ProductInput};
use super::user::{User, UserInput};

pub struct Context {
    pub dbpool: PgPool
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let conn= context.dbpool.clone();
        let users = sqlx::query_as!(User,
            "SELECT id, name, email FROM users"
        ).fetch_all(&conn).await?;

        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    async fn user(context: &Context, id: i32) -> FieldResult<User> {
        let conn = context.dbpool.clone();
        let user = sqlx::query_as!(User,
            "SELECT id, name, email FROM users WHERE id = $1",
            id
        ).fetch_one(&conn).await?;

        Ok(user)
    }

    #[graphql(description = "List of all product")]
    async fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let conn = context.dbpool.clone();
        let products = sqlx::query_as!( Product,
            "SELECT id, user_id, name, price::integer FROM product"
        ).fetch_all(&conn).await?;

        Ok(products)
    }

    #[graphql(description = "Get Single product reference by user ID")]
    async fn product(context: &Context, id: i32) -> FieldResult<Product> {
        let conn = context.dbpool.clone();
        let product = sqlx::query_as!(Product,
            "SELECT id, user_id, name, price::integer FROM product WHERE id = $1",
            id
        ).fetch_one(&conn).await?;
        Ok(product)
    }

}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let conn = context.dbpool.clone();
        let insert = sqlx::query!(
            "INSERT INTO users (name, email) VALUES ($1,$2)
            RETURNING id, name, email",
            user.name, user.email
        ).fetch_one(&conn).await?;

        let data = User {
            id: insert.id,
            name: insert.name,
            email: insert.email
        };

        Ok(data)
    }

    async fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        let conn = context.dbpool.clone();
        let insert = sqlx::query!(
            "INSERT INTO product (user_id, name, price) VALUES ($1,$2,$3)
            RETURNING id, user_id, name, price",
            product.user_id, product.name, product.price
        ).fetch_one(&conn).await?;

        let data = Product {
            id: insert.id,
            user_id: insert.user_id,
            name: insert.name,
            price: insert.price
        };

        Ok(data)
    }

}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}