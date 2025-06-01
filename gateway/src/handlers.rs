
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

use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope};
use serde::{Deserialize, Serialize};
use serde_json::json;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct GithubUser { id: u64, login: String }

#[derive(Serialize)]
struct Claims { sub: String, username: String, exp: usize }

/// Health check handler.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Gateway is up")
}

/// Root endpoint to confirm the gateway is running.
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Gateway service")
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

/// Show login page
pub async fn login_page() -> impl Responder {
    actix_files::NamedFile::open_async("gateway/static/index.html").await
}

pub async fn profile_page() -> impl Responder {
    actix_files::NamedFile::open_async("gateway/static/profile.html").await
}

pub async fn github_auth(oauth: web::Data<BasicClient>) -> impl Responder {
    let (auth_url, _csrf) = oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".into()))
        .url();
    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
}

pub async fn github_callback(
    oauth: web::Data<BasicClient>,
    req: HttpRequest,
    params: web::Query<AuthRequest>,
    client: web::Data<Client>,
    jwt_key: web::Data<String>,
) -> impl Responder {
    let token = oauth
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    let user: GithubUser = client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .header("User-Agent", "leyak")
        .send()
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .json()
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    let new_user = json!({"github_id": user.id.to_string(), "username": user.login});
    let _ = client
        .post("http://user-service:8001/register")
        .json(&new_user)
        .send()
        .await;

    let claims = Claims { sub: user.id.to_string(), username: user.login, exp: 2000000000 };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_key.as_bytes())).unwrap();

    HttpResponse::Found()
        .append_header((header::LOCATION, "/profile"))
        .append_header((header::SET_COOKIE, format!("token={}; HttpOnly; Path=/", token)))
        .finish()
}

pub async fn profile_api(req: HttpRequest, client: web::Data<Client>) -> impl Responder {
    if let Some(cookie) = req.cookie("token") {
        if let Ok(token_data) = jsonwebtoken::decode::<Claims>(
            cookie.value(),
            &jsonwebtoken::DecodingKey::from_secret(b"placeholder"),
            &jsonwebtoken::Validation::default(),
        ) {
            let user_id = token_data.claims.sub;
            if let Ok(resp) = client
                .get(format!("http://user-service:8001/users/{}", user_id))
                .send()
                .await
            {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    return HttpResponse::Ok().json(data);
                }
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}

