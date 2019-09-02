use std::env;

use env_logger;
use log::info;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("actix-web run");

    HttpServer::new(|| App::new().route("/", web::to(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .unwrap();
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
