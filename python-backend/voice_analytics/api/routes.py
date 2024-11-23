from fastapi import APIRouter
from voice_analytics.api.endpoints import audio, health

router = APIRouter()

router.include_router(health.router, tags=["health"])
router.include_router(audio.router, prefix="/audio", tags=["audio"]) 