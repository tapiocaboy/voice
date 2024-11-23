use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMetadata {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
}

#[derive(Debug)]
pub struct AudioChunk {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub metadata: AudioMetadata,
}

#[derive(Debug, Serialize)]
pub struct ProcessedAudio {
    pub data: Vec<f32>,
    pub metadata: AudioMetadata,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub max_chunk_size: usize,
    pub noise_reduction: bool,
    pub normalize: bool,
} 