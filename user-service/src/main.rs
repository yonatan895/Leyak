mod db;
mod handlers;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use db::{create_pool, init_db};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("USER_SERVICE_PORT")
        .unwrap_or_else(|_| "8001".into())
        .parse()
        .expect("USER_SERVICE_PORT must be a number");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url).await;
    init_db(&pool).await.expect("Failed to initialize DB");

    println!("User Service starting on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
