use actix_web::web;
use crate::handlers::{register_user, login_user, health_check, index};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/health").route(web::get().to(health_check)))
        .service(web::resource("/register").route(web::post().to(register_user)))
        .service(web::resource("/login").route(web::post().to(login_user)));
}