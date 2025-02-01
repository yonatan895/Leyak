use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::middleware::Logger;
use futures::future::{ok, Ready};

/// Simple logging middleware
pub fn logging() -> Logger {
    Logger::default()
}
