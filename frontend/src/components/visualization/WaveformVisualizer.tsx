import { useEffect, useRef } from 'react';
import WaveSurfer from 'wavesurfer.js';

interface WaveformVisualizerProps {
  audioUrl: string;
  peaks?: number[];
  onReady?: () => void;
  onTimeUpdate?: (currentTime: number) => void;
}

export const WaveformVisualizer: React.FC<WaveformVisualizerProps> = ({
  audioUrl,
  peaks,
  onReady,
  onTimeUpdate
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const wavesurferRef = useRef<WaveSurfer | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    wavesurferRef.current = WaveSurfer.create({
      container: containerRef.current,
      waveColor: '#4F46E5',
      progressColor: '#818CF8',
      cursorColor: '#312E81',
      height: 128,
      normalize: true,
      responsive: true,
      peaks: peaks
    });

    wavesurferRef.current.load(audioUrl);

    wavesurferRef.current.on('ready', () => {
      onReady?.();
    });

    wavesurferRef.current.on('audioprocess', (time) => {
      onTimeUpdate?.(time);
    });

    return () => {
      wavesurferRef.current?.destroy();
    };
  }, [audioUrl, peaks]);

  return <div ref={containerRef} className="w-full h-32" />;
}; 