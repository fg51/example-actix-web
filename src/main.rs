use std::env;

use env_logger;
use log::info;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let port = 8080;

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    info!("actix-web run");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
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
