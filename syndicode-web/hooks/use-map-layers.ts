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
  // Memoize the Set creation to avoid recreating it on every render
  const ownedBusinessGmlIds = useMemo(() => {
    return new Set(ownedBusinesses.map(b => b.headquarterBuildingGmlId));
  }, [ownedBusinesses]);

  // Memoize the update trigger hash to avoid expensive operations
  const ownedBusinessesUpdateTrigger = useMemo(() => {
    return ownedBusinessGmlIds.size > 0 ? 
      Array.from(ownedBusinessGmlIds).sort().join(',') : 
      'empty';
  }, [ownedBusinessGmlIds]);

  return useMemo(() => {
    const layersList = [];

    if (tokyoBoundary) {
      layersList.push(createTokyoBoundaryLayer(tokyoBoundary, time));
      layersList.push(createTokyoBoundaryGlowLayer(tokyoBoundary, time));
    }

    // Use memoized GML ID set for buildings layer
    layersList.push(createBuildingsLayer(ownedBusinessGmlIds, ownedBusinessesUpdateTrigger));

    // Add hex layer for headquarters (visible from far away)
    if (ownedBusinesses.length > 0) {
      layersList.push(createHeadquarterHexLayer(ownedBusinesses, time, zoom));
    }

    return layersList;
  }, [ownedBusinessGmlIds, ownedBusinessesUpdateTrigger, ownedBusinesses, time, tokyoBoundary, zoom]);
};
