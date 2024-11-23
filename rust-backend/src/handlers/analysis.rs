use actix_web::{get, web, HttpResponse, Error};
use uuid::Uuid;

/// Get analysis results
#[utoipa::path(
    get,
    path = "/api/analysis/{id}",
    responses(
        (status = 200, description = "Analysis retrieved successfully"),
        (status = 404, description = "Analysis not found"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Analysis ID")
    ),
    tag = "analysis"
)]
#[get("/analysis/{id}")]
pub async fn get_analysis(id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "status": "processing"
    })))
}

/// Get transcription results
#[utoipa::path(
    get,
    path = "/api/transcription/{id}",
    responses(
        (status = 200, description = "Transcription retrieved successfully"),
        (status = 404, description = "Transcription not found"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Analysis ID")
    ),
    tag = "analysis"
)]
#[get("/transcription/{id}")]
pub async fn get_transcription(id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "status": "processing"
    })))
}

/// Get emotion analysis results
#[utoipa::path(
    get,
    path = "/api/emotions/{id}",
    responses(
        (status = 200, description = "Emotions retrieved successfully"),
        (status = 404, description = "Emotions not found"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Analysis ID")
    ),
    tag = "analysis"
)]
#[get("/emotions/{id}")]
pub async fn get_emotions(id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "status": "processing"
    })))
} 