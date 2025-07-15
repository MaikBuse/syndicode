import { useMemo } from 'react';
import type { TokyoBoundaryGeoJSON } from '@/lib/map/types';
import { 
  createTokyoBoundaryLayer, 
  createTokyoBoundaryGlowLayer, 
  createBuildingsLayer 
} from '@/lib/map/layers';

export const useMapLayers = (
  ownedBuildingGmlId: Set<string>,
  time: number,
  tokyoBoundary: TokyoBoundaryGeoJSON | null
) => {
  return useMemo(() => {
    const layersList = [];

    if (tokyoBoundary) {
      layersList.push(createTokyoBoundaryLayer(tokyoBoundary, time));
      layersList.push(createTokyoBoundaryGlowLayer(tokyoBoundary, time));
    }

    layersList.push(createBuildingsLayer(ownedBuildingGmlId));

    return layersList;
  }, [ownedBuildingGmlId, time, tokyoBoundary]);
};