use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    static ref CONFIG: Vec<String> = {
        let config_data = fs::read_to_string("config.json")
            .expect("Failed to read configuration file");
        serde_json::from_str(&config_data)
            .expect("Failed to parse configuration file")
    };
}

#[derive(Deserialize)]
struct CheckParams {
    uuid: String,
}

#[derive(Serialize)]
struct UUIDResponse {
    success: bool
}

async fn manual_hello(query: web::Query<CheckParams>) -> impl Responder {
    //HttpResponse::Ok().body(format!("Hey there uuid {}", query.uuid))
    if CONFIG.contains(&query.uuid) {
        return HttpResponse::Ok().body(serde_json::to_string(&UUIDResponse { success: true }).unwrap())
    }
    HttpResponse::Ok().body(serde_json::to_string(&UUIDResponse { success: false }).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/check", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3797))?
    .run()
    .await
}
