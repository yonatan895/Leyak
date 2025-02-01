mod routes;
mod handlers;
mod models;
mod db;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("TWEET_SERVICE_PORT")
        .unwrap_or_else(|_| "8002".into())
        .parse()
        .expect("TWEET_SERVICE_PORT must be a number");

    println!("Tweet Service starting on port {}", port);

    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
