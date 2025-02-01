use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum GatewayError {
    #[display(fmt = "Internal Server Error")]
    Internal,
}

impl ResponseError for GatewayError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body("Gateway error occurred")
    }
}
