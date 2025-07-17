import { useState, useEffect, useRef } from 'react';

interface UseAnimationTimeOptions {
  paused?: boolean;
  increment?: number;
}

export const useAnimationTime = (options: UseAnimationTimeOptions = {}) => {
  const { paused = false, increment = 0.01 } = options;
  const [time, setTime] = useState(0);
  const animationIdRef = useRef<number | null>(null);
  const lastTimeRef = useRef(0);

  useEffect(() => {
    if (paused) {
      if (animationIdRef.current !== null) {
        cancelAnimationFrame(animationIdRef.current);
        animationIdRef.current = null;
      }
      return;
    }

    const animate = (currentTime: number) => {
      // Use high-resolution timestamp for smoother animations
      const deltaTime = currentTime - lastTimeRef.current;
      if (deltaTime >= 16) { // ~60fps throttling
        setTime(prev => prev + increment);
        lastTimeRef.current = currentTime;
      }
      animationIdRef.current = requestAnimationFrame(animate);
    };

    animationIdRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationIdRef.current !== null) {
        cancelAnimationFrame(animationIdRef.current);
        animationIdRef.current = null;
      }
    };
  }, [paused, increment]);

  return time;
};