import uvicorn
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from voice_analytics.api.routes import router
from voice_analytics.core.settings import settings
from voice_analytics.core.logging import setup_logging

app = FastAPI(
    title="Voice Analytics AI Service",
    description="AI-powered voice analysis services",
    version="1.0.0"
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(router, prefix="/api")

def start():
    """Production server start"""
    setup_logging()
    uvicorn.run(
        "voice_analytics.main:app",
        host="0.0.0.0",
        port=settings.PORT,
        workers=settings.WORKERS,
        log_level="info"
    )

def dev():
    """Development server start"""
    setup_logging()
    uvicorn.run(
        "voice_analytics.main:app",
        host="0.0.0.0",
        port=settings.PORT,
        reload=True,
        log_level="debug"
    )

if __name__ == "__main__":
    dev() 