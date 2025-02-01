mod errors;
mod handlers;
mod middleware;
mod routes;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env

    let port: u16 = env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8000".into())
        .parse()
        .expect("GATEWAY_PORT must be a number");

    println!("Gateway starting on port {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::logging()) // Example middleware for logging
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
