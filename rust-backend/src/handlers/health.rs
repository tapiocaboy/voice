use actix_web::{get, HttpResponse};
use utoipa::OpenApi;

/// Check API health status
#[utoipa::path(
    get,
    path = "/api/health",
    tag = "health",
    responses(
        (status = 200, description = "API is healthy", body = String),
        (status = 500, description = "API is unhealthy", body = String)
    )
)]
#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
} 