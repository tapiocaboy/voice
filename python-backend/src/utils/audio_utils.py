import numpy as np
import soundfile as sf
import io
from fastapi import UploadFile

class AudioPreprocessor:
    def __init__(self, target_sr: int = 16000):
        self.target_sr = target_sr
        
    async def process(self, file: UploadFile) -> np.ndarray:
        try:
            # Read audio file
            content = await file.read()
            audio_data, sr = sf.read(io.BytesIO(content))
            
            # Convert to mono if stereo
            if len(audio_data.shape) > 1:
                audio_data = np.mean(audio_data, axis=1)
            
            # Resample if necessary
            if sr != self.target_sr:
                audio_data = self._resample(audio_data, sr, self.target_sr)
            
            # Normalize
            audio_data = self._normalize(audio_data)
            
            return audio_data
            
        except Exception as e:
            raise RuntimeError(f"Audio preprocessing failed: {str(e)}")
            
    def _normalize(self, audio_data: np.ndarray) -> np.ndarray:
        """Normalize audio to [-1, 1] range"""
        return audio_data / np.max(np.abs(audio_data))
        
    def _resample(
        self,
        audio_data: np.ndarray,
        orig_sr: int,
        target_sr: int
    ) -> np.ndarray:
        """Resample audio to target sample rate"""
        from scipy import signal
        return signal.resample(
            audio_data,
            int(len(audio_data) * target_sr / orig_sr)
        ) 