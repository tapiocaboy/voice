import whisper
import numpy as np
import torch
from typing import Optional, List
from src.models.schemas import TranscriptionResult, WordTimestamp

class WhisperService:
    def __init__(self):
        self.model = whisper.load_model("medium")
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        
    async def transcribe(
        self,
        audio_data: np.ndarray,
        language: Optional[str] = None
    ) -> TranscriptionResult:
        try:
            # Prepare options
            options = {
                "language": language,
                "task": "transcribe",
                "word_timestamps": True
            }
            
            # Run transcription
            result = self.model.transcribe(
                audio_data,
                **options
            )
            
            # Process word-level timestamps
            words = self._process_word_timestamps(result)
            
            return TranscriptionResult(
                text=result["text"],
                language=result["language"],
                words=words,
                confidence=result.get("confidence", 0.0)
            )
            
        except Exception as e:
            raise RuntimeError(f"Transcription failed: {str(e)}")
            
    def _process_word_timestamps(self, result: dict) -> List[WordTimestamp]:
        words = []
        for segment in result["segments"]:
            for word in segment.get("words", []):
                words.append(
                    WordTimestamp(
                        text=word["text"],
                        start=word["start"],
                        end=word["end"],
                        confidence=word.get("confidence", 0.0)
                    )
                )
        return words 