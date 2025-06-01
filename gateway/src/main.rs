mod errors;
mod handlers;
mod middleware;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use reqwest::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env

    let port: u16 = env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8000".into())
        .parse()
        .expect("GATEWAY_PORT must be a number");

    let client_id = env::var("GITHUB_CLIENT_ID").unwrap_or_default();
    let client_secret = env::var("GITHUB_CLIENT_SECRET").unwrap_or_default();
    let redirect_url = env::var("OAUTH_REDIRECT_URL").unwrap_or_else(|_| format!("http://localhost:{}/auth/github/callback", port));

    let oauth_client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://github.com/login/oauth/authorize".into()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".into()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    let http_client = Client::new();
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

    println!("Gateway starting on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(oauth_client.clone()))
            .app_data(web::Data::new(http_client.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .wrap(middleware::logging())
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
