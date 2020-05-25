use std::env;
use std::sync::Arc;

use env_logger;
use log::info;

use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web::get;

use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod graphql;
use graphql::schema::{create_schema, Context, Fruit, Schema};



#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let port = 8080;

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = std::sync::Arc::new(create_schema());

    info!("actix-web run");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .service(graphiql)
            .route("/graphql", web::post().to(graphql))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().json("{\"message\":\"Hello world again!\"}")
}

#[get("/macro-path")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("this is index3")
}


async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(
            &st,
            &Context {
                fruits: vec![
                    Fruit::new("1".to_string(),"apple".to_string(),"America".to_string(),
                    ),
                    Fruit::new("2".to_string(), "banana".to_string(), "Brazil".to_string()),
                    Fruit::new("3".to_string(), "candy".to_string(), "China".to_string()),

                ],
            }
        );
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok().content_type("application/json")
        .body(user))
}

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok().content_type("text/html; charset=utf-8")
        .body(html)
}

