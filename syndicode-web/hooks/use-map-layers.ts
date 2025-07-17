import { useMemo } from 'react';
import type { TokyoBoundaryGeoJSON } from '@/lib/map/types';
import type { BusinessDetails, BuildingDetails } from '@/domain/economy/economy.types';
import {
  createBuildingsLayer,
  createHeadquarterHexLayer,
  createHeadquarterArcLayer,
  createBoundaryLayersWithSharedAnimation
} from '@/lib/map/layers';

export const useMapLayers = (
  ownedBusinesses: BusinessDetails[],
  time: number,
  tokyoBoundary: TokyoBoundaryGeoJSON | null,
  zoom: number,
  selectedBusiness: BusinessDetails | null = null,
  selectedBusinessBuildings: BuildingDetails[] = []
) => {
  // Memoize the Set creation to avoid recreating it on every render
  const ownedBusinessGmlIds = useMemo(() => {
    return new Set(ownedBusinesses.map(b => b.headquarterBuildingGmlId));
  }, [ownedBusinesses]);

  // Memoize selected business buildings GML IDs for performance
  const selectedBusinessBuildingGmlIds = useMemo(() => {
    if (!selectedBusinessBuildings || selectedBusinessBuildings.length === 0) return new Set<string>();
    return new Set(selectedBusinessBuildings.map(b => b.gmlId));
  }, [selectedBusinessBuildings]);

  // Memoize the update trigger hash to avoid expensive operations
  const ownedBusinessesUpdateTrigger = useMemo(() => {
    return ownedBusinessGmlIds.size > 0 ?
      Array.from(ownedBusinessGmlIds).sort().join(',') :
      'empty';
  }, [ownedBusinessGmlIds]);

  // Memoize selected business buildings update trigger
  const selectedBusinessBuildingUpdateTrigger = useMemo(() => {
    return selectedBusinessBuildingGmlIds.size > 0 ?
      Array.from(selectedBusinessBuildingGmlIds).sort().join(',') :
      'empty';
  }, [selectedBusinessBuildingGmlIds]);

  return useMemo(() => {
    const layersList = [];

    if (tokyoBoundary) {
      // Use optimized boundary layer creation with shared animation calculations
      const boundaryLayers = createBoundaryLayersWithSharedAnimation(tokyoBoundary, time);
      layersList.push(...boundaryLayers);
    }

    // Use memoized GML ID set for buildings layer with selection support
    layersList.push(createBuildingsLayer(ownedBusinessGmlIds, ownedBusinessesUpdateTrigger, selectedBusinessBuildingGmlIds, selectedBusinessBuildingUpdateTrigger));

    // Add hex layer for headquarters (visible from far away) with selection support
    if (ownedBusinesses.length > 0) {
      // If a business is selected and has buildings, show arc layer instead of hexagon
      if (selectedBusiness && selectedBusinessBuildings && selectedBusinessBuildings.length > 0) {
        layersList.push(createHeadquarterArcLayer(selectedBusiness, selectedBusinessBuildings, time));
        // Add hexagons for non-selected businesses
        const nonSelectedBusinesses = ownedBusinesses.filter(b => b.businessUuid !== selectedBusiness.businessUuid);
        if (nonSelectedBusinesses.length > 0) {
          layersList.push(createHeadquarterHexLayer(nonSelectedBusinesses, time, zoom));
        }
      } else {
        // Show all businesses as hexagons
        layersList.push(createHeadquarterHexLayer(ownedBusinesses, time, zoom));
      }
    }

    return layersList;
  }, [ownedBusinessGmlIds, ownedBusinessesUpdateTrigger, selectedBusinessBuildingGmlIds, selectedBusinessBuildingUpdateTrigger, ownedBusinesses, time, tokyoBoundary, zoom, selectedBusiness, selectedBusinessBuildings]);
};
