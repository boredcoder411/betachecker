use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref CONFIG: HashMap<String, Vec<String>> = {
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
    success: bool,
}

async fn check_project(path: web::Path<String>, query: web::Query<CheckParams>) -> impl Responder {
    let project_name = path.into_inner();
    if let Some(uuids) = CONFIG.get(&project_name) {
        if uuids.contains(&query.uuid) {
            return HttpResponse::Ok().json(UUIDResponse { success: true });
        }
    }
    HttpResponse::Ok().json(UUIDResponse { success: false })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{project_name}", web::get().to(check_project))
    })
    .bind(("127.0.0.1", 3797))?
    .run()
    .await
}

