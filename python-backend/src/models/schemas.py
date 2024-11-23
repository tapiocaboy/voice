from pydantic import BaseModel
from typing import List, Dict, Optional

class WordTimestamp(BaseModel):
    text: str
    start: float
    end: float
    confidence: float

class TranscriptionResult(BaseModel):
    text: str
    language: str
    words: List[WordTimestamp]
    confidence: float

class EmotionSegment(BaseModel):
    start_time: float
    end_time: float
    emotion: str
    confidence: float
    all_emotions: Dict[str, float]

class EmotionResult(BaseModel):
    segments: List[EmotionSegment]
    dominant_emotion: str

class SpeakerSegment(BaseModel):
    speaker_id: str
    start_time: float
    end_time: float
    confidence: float

class DiarizationResult(BaseModel):
    segments: List[SpeakerSegment]
    num_speakers: int

class AudioAnalysisResponse(BaseModel):
    transcription: TranscriptionResult
    emotions: EmotionResult
    speakers: DiarizationResult 