import { useMemo } from 'react';
import type { TokyoBoundaryGeoJSON } from '@/lib/map/types';
import type { BusinessDetails } from '@/domain/economy/economy.types';
import {
  createTokyoBoundaryLayer,
  createTokyoBoundaryGlowLayer,
  createBuildingsLayer,
  createHeadquarterHexLayer
} from '@/lib/map/layers';

export const useMapLayers = (
  ownedBusinesses: BusinessDetails[],
  time: number,
  tokyoBoundary: TokyoBoundaryGeoJSON | null,
  zoom: number
) => {
  return useMemo(() => {
    const layersList = [];

    if (tokyoBoundary) {
      layersList.push(createTokyoBoundaryLayer(tokyoBoundary, time));
      layersList.push(createTokyoBoundaryGlowLayer(tokyoBoundary, time));
    }

    // Create GML ID set for buildings layer
    const ownedBusinessGmlIds = new Set(ownedBusinesses.map(b => b.headquarterBuildingGmlId));
    layersList.push(createBuildingsLayer(ownedBusinessGmlIds));

    // Add hex layer for headquarters (visible from far away)
    if (ownedBusinesses.length > 0) {
      layersList.push(createHeadquarterHexLayer(ownedBusinesses, time, zoom));
    }

    return layersList;
  }, [ownedBusinesses, time, tokyoBoundary, zoom]);
};
