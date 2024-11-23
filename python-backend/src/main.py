from fastapi import FastAPI, UploadFile, File, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import asyncio
import logging
from typing import List, Optional

from src.services.whisper_service import WhisperService
from src.services.emotion_service import EmotionService
from src.services.diarization_service import DiarizationService
from src.models.schemas import (
    AudioAnalysisResponse,
    TranscriptionResult,
    EmotionResult,
    DiarizationResult
)
from src.utils.audio_utils import AudioPreprocessor

app = FastAPI(title="Voice Analytics AI Service")

# Configure CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize services
whisper_service = WhisperService()
emotion_service = EmotionService()
diarization_service = DiarizationService()
audio_preprocessor = AudioPreprocessor()

@app.post("/analyze", response_model=AudioAnalysisResponse)
async def analyze_audio(
    file: UploadFile = File(...),
    language: Optional[str] = None
) -> AudioAnalysisResponse:
    try:
        # Preprocess audio
        audio_data = await audio_preprocessor.process(file)
        
        # Run analysis tasks concurrently
        transcription_task = asyncio.create_task(
            whisper_service.transcribe(audio_data, language)
        )
        emotion_task = asyncio.create_task(
            emotion_service.analyze(audio_data)
        )
        diarization_task = asyncio.create_task(
            diarization_service.process(audio_data)
        )
        
        # Wait for all tasks to complete
        transcription, emotions, speakers = await asyncio.gather(
            transcription_task,
            emotion_task,
            diarization_task
        )
        
        return AudioAnalysisResponse(
            transcription=transcription,
            emotions=emotions,
            speakers=speakers
        )
        
    except Exception as e:
        logging.error(f"Analysis failed: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/transcribe", response_model=TranscriptionResult)
async def transcribe_audio(
    file: UploadFile = File(...),
    language: Optional[str] = None
) -> TranscriptionResult:
    try:
        audio_data = await audio_preprocessor.process(file)
        return await whisper_service.transcribe(audio_data, language)
    except Exception as e:
        logging.error(f"Transcription failed: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/emotions", response_model=EmotionResult)
async def analyze_emotions(
    file: UploadFile = File(...)
) -> EmotionResult:
    try:
        audio_data = await audio_preprocessor.process(file)
        return await emotion_service.analyze(audio_data)
    except Exception as e:
        logging.error(f"Emotion analysis failed: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/diarize", response_model=DiarizationResult)
async def diarize_speakers(
    file: UploadFile = File(...)
) -> DiarizationResult:
    try:
        audio_data = await audio_preprocessor.process(file)
        return await diarization_service.process(audio_data)
    except Exception as e:
        logging.error(f"Speaker diarization failed: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e)) 