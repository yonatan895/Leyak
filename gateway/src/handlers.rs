use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::fs;

const CLIENT_ID_PATH: &str = "/run/secrets/google_client_id";
const CLIENT_SECRET_PATH: &str = "/run/secrets/google_client_secret";
const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const REDIRECT_URI: &str = "http://localhost:8000/auth/google/callback";

fn read_secret(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Health check handler.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Gateway is up")
}

/// Example handler that would forward a request to the user service.
pub async fn proxy_users() -> impl Responder {
    // In a complete implementation, you might use reqwest to forward the request:
    // let response = reqwest::get("http://user-service:8001/profile").await;
    HttpResponse::Ok().body("Forwarding to user service (stub)")
}

/// Start Google OAuth login by redirecting to Google's authorization endpoint.
pub async fn google_login() -> impl Responder {
    let client_id = read_secret(CLIENT_ID_PATH);
    let url = format!(
        "{auth}?client_id={client_id}&response_type=code&scope=openid%20email%20profile&redirect_uri={redir}",
        auth = GOOGLE_AUTH_URL,
        client_id = urlencoding::encode(&client_id),
        redir = urlencoding::encode(REDIRECT_URI)
    );
    HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, url))
        .finish()
}

#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: String,
}

/// Handle the OAuth callback. In a real implementation, this would exchange the
/// code for an access token.
pub async fn google_callback(query: web::Query<AuthQuery>) -> impl Responder {
    let _client_secret = read_secret(CLIENT_SECRET_PATH);
    HttpResponse::Ok().body(format!("Received code: {}", query.code))
}

