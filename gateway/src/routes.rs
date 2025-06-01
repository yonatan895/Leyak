use actix_web::web;

use crate::handlers::{
    health_check,
    proxy_users,
    google_login,
    google_callback,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)))
        .service(web::resource("/users").route(web::get().to(proxy_users)))
        .service(web::resource("/auth/google").route(web::get().to(google_login)))
        .service(web::resource("/auth/google/callback").route(web::get().to(google_callback)));
}
