import { useState, useRef, useCallback } from 'react';

interface AudioRecorderOptions {
  maxDuration?: number;
  sampleRate?: number;
  channels?: number;
}

export const useAudioRecorder = ({
  maxDuration = 300,
  sampleRate = 44100,
  channels = 1
}: AudioRecorderOptions = {}) => {
  const [isRecording, setIsRecording] = useState(false);
  const [duration, setDuration] = useState(0);
  const [audioLevel, setAudioLevel] = useState(0);

  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const audioChunks = useRef<Blob[]>([]);
  const intervalRef = useRef<number>();

  const startRecording = useCallback(async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      
      mediaRecorder.current = new MediaRecorder(stream, {
        mimeType: 'audio/webm'
      });

      mediaRecorder.current.ondataavailable = (event) => {
        audioChunks.current.push(event.data);
      };

      mediaRecorder.current.start(1000);
      setIsRecording(true);

      // Start duration timer
      intervalRef.current = window.setInterval(() => {
        setDuration(d => {
          if (d >= maxDuration) {
            stopRecording();
            return d;
          }
          return d + 1;
        });
      }, 1000);

      // Audio level analyzer
      const audioContext = new AudioContext();
      const analyser = audioContext.createAnalyser();
      const microphone = audioContext.createMediaStreamSource(stream);
      microphone.connect(analyser);
      
      analyser.fftSize = 256;
      const dataArray = new Uint8Array(analyser.frequencyBinCount);
      
      const updateLevel = () => {
        if (!isRecording) return;
        
        analyser.getByteFrequencyData(dataArray);
        const average = dataArray.reduce((a, b) => a + b) / dataArray.length;
        setAudioLevel(average / 255);
        
        requestAnimationFrame(updateLevel);
      };
      
      updateLevel();

    } catch (error) {
      console.error('Error starting recording:', error);
    }
  }, [maxDuration]);

  const stopRecording = useCallback(() => {
    if (!mediaRecorder.current) return;

    mediaRecorder.current.stop();
    mediaRecorder.current.stream.getTracks().forEach(track => track.stop());
    
    clearInterval(intervalRef.current);
    setIsRecording(false);
    
    const audioBlob = new Blob(audioChunks.current, { type: 'audio/webm' });
    audioChunks.current = [];
    
    return audioBlob;
  }, []);

  return {
    isRecording,
    duration,
    audioLevel,
    startRecording,
    stopRecording
  };
}; 