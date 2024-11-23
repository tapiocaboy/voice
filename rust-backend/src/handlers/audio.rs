use actix_web::{post, web, Error, HttpResponse};
use actix_multipart::Multipart;
use futures::StreamExt;
use uuid::Uuid;
use crate::models::audio::{AudioChunk, AudioMetadata};
use crate::services::audio_processor::AudioProcessor;

#[post("/upload")]
pub async fn upload(
    mut payload: Multipart,
    processor: web::Data<AudioProcessor>,
) -> Result<HttpResponse, Error> {
    let mut chunks = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .unwrap_or("unnamed.wav")
            .to_string();

        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            data.extend_from_slice(&chunk?);
        }

        let size = data.len();

        let audio_chunk = AudioChunk {
            id: Uuid::new_v4(),
            data,
            metadata: AudioMetadata {
                filename,
                content_type: field.content_type()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "audio/wav".to_string()),
                size,
            },
        };

        chunks.push(audio_chunk);
    }

    let processed_chunks = futures::future::join_all(
        chunks.into_iter().map(|chunk| processor.process_chunk(chunk))
    ).await;

    let results: Vec<_> = processed_chunks
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    Ok(HttpResponse::Ok().json(results))
}

#[post("/process")]
pub async fn process(
    _app_state: web::Data<crate::models::AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Process endpoint placeholder",
        "status": "success"
    })))
}

#[post("/stream")]
pub async fn stream(
    _app_state: web::Data<crate::models::AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Stream endpoint placeholder",
        "status": "success"
    })))
} 