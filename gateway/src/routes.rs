use actix_web::web;

use crate::handlers::{health_check, proxy_users};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)))
        .service(web::resource("/users").route(web::get().to(proxy_users)));
}
