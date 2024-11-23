from transformers import pipeline
import torch
import numpy as np
from typing import List
from src.models.schemas import EmotionResult, EmotionSegment

class EmotionService:
    def __init__(self):
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        self.model = pipeline(
            "audio-classification",
            model="audeering/wav2vec2-large-emotion",
            device=self.device
        )
        
    async def analyze(self, audio_data: np.ndarray) -> EmotionResult:
        try:
            # Split audio into segments
            segments = self._segment_audio(audio_data)
            
            # Analyze emotions for each segment
            emotion_segments = []
            for start, end, segment in segments:
                predictions = self.model(segment)
                emotion_segments.append(
                    EmotionSegment(
                        start_time=start,
                        end_time=end,
                        emotion=predictions[0]["label"],
                        confidence=predictions[0]["score"],
                        all_emotions={
                            p["label"]: p["score"] for p in predictions
                        }
                    )
                )
            
            return EmotionResult(
                segments=emotion_segments,
                dominant_emotion=self._get_dominant_emotion(emotion_segments)
            )
            
        except Exception as e:
            raise RuntimeError(f"Emotion analysis failed: {str(e)}")
            
    def _segment_audio(self, audio_data: np.ndarray, segment_length: float = 3.0):
        """Split audio into overlapping segments"""
        sample_rate = 16000
        segment_samples = int(segment_length * sample_rate)
        overlap = segment_samples // 2
        
        segments = []
        for start in range(0, len(audio_data) - overlap, overlap):
            end = start + segment_samples
            if end > len(audio_data):
                end = len(audio_data)
            
            segment = audio_data[start:end]
            segments.append((
                start / sample_rate,
                end / sample_rate,
                segment
            ))
            
        return segments
        
    def _get_dominant_emotion(self, segments: List[EmotionSegment]) -> str:
        emotion_counts = {}
        for segment in segments:
            emotion = segment.emotion
            emotion_counts[emotion] = emotion_counts.get(emotion, 0) + 1
        
        return max(emotion_counts.items(), key=lambda x: x[1])[0] 