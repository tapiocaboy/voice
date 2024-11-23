use actix_web::{get, web, HttpResponse, Error};
use uuid::Uuid;

#[get("/analysis/{id}")]
pub async fn get_analysis(id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "status": "processing"
    })))
}

#[get("/transcription/{id}")]
pub async fn get_transcription(id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "status": "processing"
    })))
}

#[get("/emotions/{id}")]
pub async fn get_emotions(id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "status": "processing"
    })))
} 