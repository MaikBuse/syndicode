import { useState, useEffect } from 'react';

export const useAnimationTime = () => {
  const [time, setTime] = useState(0);

  useEffect(() => {
    const animate = () => {
      setTime(prev => prev + 0.01);
      requestAnimationFrame(animate);
    };
    const animationId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animationId);
  }, []);

  return time;
};