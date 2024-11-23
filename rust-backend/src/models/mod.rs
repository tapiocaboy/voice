pub mod audio;

use sqlx::PgPool;
use redis::Client as RedisClient;
use crate::config::Config;

pub struct AppState {
    pub db: PgPool,
    pub redis: RedisClient,
    pub config: Config,
}

pub struct AudioConfig {
    pub max_chunk_size: usize,
    pub noise_reduction: bool,
    pub normalize: bool,
} 