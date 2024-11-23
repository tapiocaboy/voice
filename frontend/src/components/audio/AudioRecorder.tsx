import { useState, useRef } from 'react';
import { useAudioRecorder } from '@/hooks/useAudioRecorder';
import { Button, Timer, VolumeIndicator } from '@/components/common';

interface AudioRecorderProps {
  onRecordingComplete: (blob: Blob) => void;
  maxDuration?: number; // in seconds
}

export const AudioRecorder: React.FC<AudioRecorderProps> = ({
  onRecordingComplete,
  maxDuration = 300 // 5 minutes default
}) => {
  const {
    startRecording,
    stopRecording,
    isRecording,
    duration,
    audioLevel
  } = useAudioRecorder({ maxDuration });

  return (
    <div className="flex flex-col items-center gap-4 p-4 border rounded-lg">
      <VolumeIndicator level={audioLevel} isActive={isRecording} />
      
      <Timer
        duration={duration}
        maxDuration={maxDuration}
        isActive={isRecording}
      />
      
      <div className="flex gap-4">
        {!isRecording ? (
          <Button 
            onClick={startRecording}
            variant="primary"
            icon="microphone"
          >
            Start Recording
          </Button>
        ) : (
          <Button
            onClick={stopRecording}
            variant="danger"
            icon="stop"
          >
            Stop Recording
          </Button>
        )}
      </div>
    </div>
  );
}; 