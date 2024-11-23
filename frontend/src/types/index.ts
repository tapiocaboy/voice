export interface TranscriptionData {
  text: string;
  words: Array<{
    word: string;
    startTime: number;
    endTime: number;
    confidence: number;
  }>;
}

export interface EmotionData {
  timestamp: number;
  emotion: string;
  confidence: number;
}

export interface SpeakerData {
  id: string;
  segments: Array<{
    startTime: number;
    endTime: number;
    confidence: number;
  }>;
}

export interface AnalysisResult {
  transcription: TranscriptionData;
  emotions: EmotionData[];
  speakers: SpeakerData[];
} 