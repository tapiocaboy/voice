import { useEffect, useState } from 'react';
import { TranscriptionPanel } from './TranscriptionPanel';
import { EmotionGraph } from './EmotionGraph';
import { SpeakerTimeline } from './SpeakerTimeline';
import { useAnalysisStore } from '@/store/analysisStore';

interface AnalysisDashboardProps {
  audioId: string;
}

export const AnalysisDashboard: React.FC<AnalysisDashboardProps> = ({ audioId }) => {
  const { 
    transcription,
    emotions,
    speakers,
    fetchAnalysis,
    isLoading,
    error 
  } = useAnalysisStore();

  useEffect(() => {
    fetchAnalysis(audioId);
  }, [audioId]);

  if (isLoading) {
    return <div className="flex justify-center p-8">Loading analysis...</div>;
  }

  if (error) {
    return <div className="text-red-500 p-4">Error: {error}</div>;
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-6 p-4">
      <TranscriptionPanel 
        data={transcription}
        onWordClick={handleWordClick}
      />
      
      <div className="space-y-6">
        <EmotionGraph 
          data={emotions}
          height={200}
        />
        
        <SpeakerTimeline 
          speakers={speakers}
          currentTime={currentTime}
        />
      </div>
    </div>
  );
}; 