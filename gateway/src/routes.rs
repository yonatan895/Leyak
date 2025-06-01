use actix_web::web;


use crate::handlers::{
    github_auth, github_callback, health_check, login_page, profile_api, profile_page, proxy_users,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)))
        .service(web::resource("/users").route(web::get().to(proxy_users)))
        .service(web::resource("/").route(web::get().to(login_page)))
        .service(web::resource("/profile").route(web::get().to(profile_page)))
        .service(web::resource("/api/profile").route(web::get().to(profile_api)))
        .service(web::resource("/auth/github").route(web::get().to(github_auth)))
        .service(
            web::resource("/auth/github/callback").route(web::get().to(github_callback)),
        );

use crate::handlers::{health_check, proxy_users, index};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/health").route(web::get().to(health_check)))
        .service(web::resource("/users").route(web::get().to(proxy_users)));

}
