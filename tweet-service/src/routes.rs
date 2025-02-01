use crate::handlers::{get_tweets, post_tweet};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tweets").route(web::post().to(post_tweet)))
        .service(web::resource("/tweets").route(web::get().to(get_tweets)));
}
