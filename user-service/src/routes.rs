use actix_web::web;
use crate::handlers::{register_user, get_user};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register").route(web::post().to(register_user)),
    )
    .service(web::resource("/users/{id}").route(web::get().to(get_user)));
}
