import { useState, useEffect } from 'react';
import type { TokyoBoundaryGeoJSON } from '@/lib/map/types';
import { loadTokyoBoundary } from '@/lib/map/tokyo-boundary';

export const useTokyoBoundary = () => {
  const [tokyoBoundary, setTokyoBoundary] = useState<TokyoBoundaryGeoJSON | null>(null);

  useEffect(() => {
    const loadBoundary = async () => {
      try {
        const boundary = await loadTokyoBoundary();
        setTokyoBoundary(boundary);
      } catch (error) {
        console.error('Failed to load Tokyo boundary:', error);
      }
    };
    loadBoundary();
  }, []);

  return tokyoBoundary;
};