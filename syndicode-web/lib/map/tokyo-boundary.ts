import type { TokyoBoundaryGeoJSON } from './types';

export const loadTokyoBoundary = async (): Promise<TokyoBoundaryGeoJSON | null> => {
  try {
    const response = await fetch('/data/tokyo-boundary.geojson');
    return await response.json() as TokyoBoundaryGeoJSON;
  } catch (error) {
    console.error('Failed to load Tokyo boundary:', error);
    return null;
  }
};