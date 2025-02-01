use actix_web::{dev::ServiceRequest, Error, HttpMessage, dev::ServiceResponse};
use actix_web::middleware::{Logger, Transform};
use futures::future::{ok, Ready};

/// Simple logging middleware
pub fn logging() -> Logger {
    Logger::default()
}
