use std::env;
use std::sync::Mutex;

use env_logger;
use log::info;
use serde::Serialize;

use actix_web::{
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    Result as ActixResult,
};

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("actix-web run");

    HttpServer::new(|| {
        let data = web::Data::new(SharedFood::new());

        App::new()
            .register_data(data.clone())
            .route("/", web::to(index))
            .service(
                web::scope("/app")
                    .route("/add/{name}/{sugar}", web::get().to(app_index)),
            )
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}

fn index(data: web::Data<SharedFood>) -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    let mut sugar = data.sugar_contents.lock().unwrap();
    *sugar += 1;
    Food {
        name: data.name,
        made_in: data.made_in,
        category: data.category,
        // sugar_contents: sugar.clone(),
        sugar_contents: *sugar,
    }
}

fn app_index(info: web::Path<(String, u32)>) -> ActixResult<String> {
    Ok(format!("add fruits: {}, sugar: {}", info.0, info.1))
}

#[derive(Serialize)]
struct Food {
    name: &'static str,
    made_in: &'static str,
    category: &'static str,
    sugar_contents: i32,
}

struct SharedFood {
    name: &'static str,
    made_in: &'static str,
    category: &'static str,
    sugar_contents: Mutex<i32>,
}

impl SharedFood {
    fn new() -> Self {
        Self {
            name: "Apple",
            made_in: "America",
            category: "fruits",
            sugar_contents: Mutex::new(10),
        }
    }
}

impl Responder for Food {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
