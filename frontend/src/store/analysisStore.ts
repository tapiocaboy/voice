import create from 'zustand';
import { AnalysisService } from '@/services/analysisService';

interface AnalysisState {
  transcription: TranscriptionData | null;
  emotions: EmotionData[];
  speakers: SpeakerData[];
  isLoading: boolean;
  error: string | null;
  fetchAnalysis: (audioId: string) => Promise<void>;
}

export const useAnalysisStore = create<AnalysisState>((set) => ({
  transcription: null,
  emotions: [],
  speakers: [],
  isLoading: false,
  error: null,

  fetchAnalysis: async (audioId: string) => {
    try {
      set({ isLoading: true, error: null });
      
      const analysis = await AnalysisService.getAnalysis(audioId);
      
      set({
        transcription: analysis.transcription,
        emotions: analysis.emotions,
        speakers: analysis.speakers,
        isLoading: false
      });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Unknown error',
        isLoading: false
      });
    }
  }
})); 