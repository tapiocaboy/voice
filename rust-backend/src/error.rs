use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde_json::json;

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
            ServiceError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": msg
                }))
            }
            ServiceError::AuthenticationError(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Authentication Error",
                    "message": msg
                }))
            }
            ServiceError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Validation Error",
                    "message": msg
                }))
            }
            ServiceError::AudioProcessing(msg) => {
                HttpResponse::UnprocessableEntity().json(json!({
                    "error": "Audio Processing Error",
                    "message": msg
                }))
            }
            ServiceError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Database Error",
                    "message": msg
                }))
            }
            ServiceError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": "Not Found",
                    "message": msg
                }))
            }
        }
    }
} 