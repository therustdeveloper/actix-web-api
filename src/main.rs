mod model;
mod schema;
mod handler;

use actix_web::{get, Responder, HttpResponse, HttpServer, App};
use serde_json::json;
use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[get("/v1/healthcheck")]
async fn healthcheck() -> impl Responder {
    const MESSAGE: &str = "💻 Application working";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}