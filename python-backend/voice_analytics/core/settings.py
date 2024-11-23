from pydantic_settings import BaseSettings
from functools import lru_cache

class Settings(BaseSettings):
    # Server settings
    PORT: int = 6500
    WORKERS: int = 4
    DEBUG: bool = False

    # AI Model settings
    WHISPER_MODEL: str = "base"
    EMOTION_MODEL: str = "superb/wav2vec2-base-superb-er"
    DIARIZATION_MODEL: str = "speechbrain/spkrec-ecapa-voxceleb"

    # Processing settings
    MAX_AUDIO_LENGTH: int = 300  # seconds
    SAMPLE_RATE: int = 16000
    CHUNK_SIZE: int = 4096

    class Config:
        env_file = ".env"

@lru_cache()
def get_settings() -> Settings:
    return Settings()

settings = get_settings() 