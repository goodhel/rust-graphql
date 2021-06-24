use actix_web::{Error, HttpResponse, web};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::db::PgPool;
use crate::handler::root::{create_schema, Context, Schema};

pub async fn graphql(
    pool: web::Data<PgPool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        dbpool: pool.get_ref().to_owned()
    };
    // let res = web::block(move || {
    //     let res = data.execute_sync(&schema, &ctx);
    //     // let res = data.execute(&schema, &ctx).await;
    //     Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    // }).await.map_err(Error::from)?;
    // let result =  data.execute(&schema, &ctx).await;
    // let json;
    // match result {
    //     Ok(dat) => {
    //         json = serde_json::to_string(&dat)?;
    //     },
    //     Err(err) => panic!("Problem opening the file: {:?}", err)
    // };
    let res = data.execute(&schema, &ctx).await;
    let json = serde_json::to_string(&res)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

pub fn init(app: &mut web::ServiceConfig) {
    app
        .data(create_schema())
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
    
}