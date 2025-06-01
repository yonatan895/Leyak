use crate::models::User;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

// For demonstration, we use an in-memory store.
lazy_static::lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(vec![]);
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("User service")
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("User service healthy")
}

pub async fn register_user(new_user: web::Json<User>) -> impl Responder {
    let mut users = USERS.lock().unwrap();
    users.push(new_user.into_inner());
    HttpResponse::Ok().body("User registered")
}

pub async fn login_user() -> impl Responder {
    HttpResponse::Ok().body("User login stub")
}