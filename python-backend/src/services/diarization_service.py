from pyannote.audio import Pipeline
import torch
import numpy as np
from typing import List
from src.models.schemas import DiarizationResult, SpeakerSegment

class DiarizationService:
    def __init__(self):
        self.pipeline = Pipeline.from_pretrained(
            "pyannote/speaker-diarization",
            use_auth_token=True
        )
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        self.pipeline = self.pipeline.to(self.device)
        
    async def process(self, audio_data: np.ndarray) -> DiarizationResult:
        try:
            # Run diarization
            diarization = self.pipeline(audio_data)
            
            # Process results
            speakers = []
            for turn, _, speaker in diarization.itertracks(yield_label=True):
                speakers.append(
                    SpeakerSegment(
                        speaker_id=speaker,
                        start_time=turn.start,
                        end_time=turn.end,
                        confidence=turn.confidence if hasattr(turn, 'confidence') else 1.0
                    )
                )
            
            return DiarizationResult(
                segments=speakers,
                num_speakers=len(set(s.speaker_id for s in speakers))
            )
            
        except Exception as e:
            raise RuntimeError(f"Speaker diarization failed: {str(e)}")
            
    def _merge_segments(
        self,
        segments: List[SpeakerSegment],
        threshold: float = 0.5
    ) -> List[SpeakerSegment]:
        """Merge close segments from the same speaker"""
        if not segments:
            return segments
            
        merged = []
        current = segments[0]
        
        for next_segment in segments[1:]:
            if (
                next_segment.speaker_id == current.speaker_id and
                next_segment.start_time - current.end_time <= threshold
            ):
                current.end_time = next_segment.end_time
            else:
                merged.append(current)
                current = next_segment
                
        merged.append(current)
        return merged 