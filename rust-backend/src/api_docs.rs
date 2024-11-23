use utoipa::OpenApi;
use crate::models::audio::{AudioMetadata, ProcessedAudio, AudioChunk, AudioConfig};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
        crate::handlers::audio::upload,
        crate::handlers::audio::process,
        crate::handlers::audio::stream,
        crate::handlers::analysis::get_analysis,
        crate::handlers::analysis::get_transcription,
        crate::handlers::analysis::get_emotions,
    ),
    components(
        schemas(
            AudioMetadata,
            AudioChunk,
            ProcessedAudio,
            AudioConfig
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "audio", description = "Audio processing endpoints"),
        (name = "analysis", description = "Audio analysis endpoints")
    ),
    info(
        title = "Voice Analytics API",
        version = "1.0.0",
        description = "Voice Recognition and Signal Analytics API"
    )
)]
pub struct ApiDoc; 