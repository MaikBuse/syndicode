import { useMemo } from 'react';
import type { TokyoBoundaryGeoJSON } from '@/lib/map/types';
import { 
  createTokyoBoundaryLayer, 
  createTokyoBoundaryGlowLayer, 
  createBuildingsLayer 
} from '@/lib/map/layers';

export const useMapLayers = (
  ownedBusinessGmlIds: Set<string>,
  time: number,
  tokyoBoundary: TokyoBoundaryGeoJSON | null
) => {
  return useMemo(() => {
    const layersList = [];

    if (tokyoBoundary) {
      layersList.push(createTokyoBoundaryLayer(tokyoBoundary, time));
      layersList.push(createTokyoBoundaryGlowLayer(tokyoBoundary, time));
    }

    layersList.push(createBuildingsLayer(ownedBusinessGmlIds, time));

    return layersList;
  }, [ownedBusinessGmlIds, time, tokyoBoundary]);
};