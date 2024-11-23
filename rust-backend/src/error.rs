use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Internal Server Error: {0}")]
    InternalError(String),

    #[error("Authentication Error: {0}")]
    AuthenticationError(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Audio Processing Error: {0}")]
    AudioProcessing(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Not Found: {0}")]
    NotFound(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalError(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            ServiceError::AuthenticationError(_) => {
                HttpResponse::Unauthorized().json(self.to_string())
            }
            ServiceError::ValidationError(_) => {
                HttpResponse::BadRequest().json(self.to_string())
            }
            ServiceError::AudioProcessing(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_string())
            }
            ServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            ServiceError::NotFound(_) => {
                HttpResponse::NotFound().json(self.to_string())
            }
        }
    }
} 