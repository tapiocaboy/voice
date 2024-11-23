use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AudioMetadata {
    #[schema(example = "recording.wav")]
    pub filename: String,
    #[schema(example = "audio/wav")]
    pub content_type: String,
    #[schema(example = 1024)]
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AudioChunk {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(format = "binary")]
    pub data: Vec<u8>,
    pub metadata: AudioMetadata,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProcessedAudio {
    #[schema(format = "binary")]
    pub data: Vec<f32>,
    pub metadata: AudioMetadata,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AudioConfig {
    #[schema(example = 1048576)]
    pub max_chunk_size: usize,
    #[schema(example = true)]
    pub noise_reduction: bool,
    #[schema(example = true)]
    pub normalize: bool,
} 