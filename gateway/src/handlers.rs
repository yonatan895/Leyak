use actix_web::{HttpResponse, Responder};

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

