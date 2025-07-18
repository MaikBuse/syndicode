import { useMemo } from 'react';
import type { TokyoBoundaryGeoJSON } from '@/lib/map/types';
import type { BusinessDetails, BuildingDetails, BusinessListingDetails } from '@/domain/economy/economy.types';
import type { MapMode } from '@/components/map/map-layer-controls';
import { MAP_MODES } from '@/components/map/map-layer-controls';
import {
  createBuildingsLayer,
  createHeadquarterHexLayer,
  createListedBusinessHexLayer,
  createHeadquarterArcLayer,
  createListedBusinessArcLayer,
  createBoundaryLayersWithSharedAnimation
} from '@/lib/map/layers';

export const useMapLayers = (
  ownedBusinesses: BusinessDetails[],
  businessListings: BusinessListingDetails[],
  time: number,
  tokyoBoundary: TokyoBoundaryGeoJSON | null,
  zoom: number,
  mapMode: MapMode,
  selectedBusiness: BusinessDetails | BusinessListingDetails | null = null,
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

  // Memoize ALL listed business headquarters GML IDs for performance
  const allListedBusinessGmlIds = useMemo(() => {
    if (mapMode !== MAP_MODES.MARKET) return new Set<string>();
    return new Set(businessListings.map(b => b.headquarterBuildingGmlId));
  }, [businessListings, mapMode]);

  // Memoize all listed business update trigger
  const allListedBusinessUpdateTrigger = useMemo(() => {
    return allListedBusinessGmlIds.size > 0 ?
      Array.from(allListedBusinessGmlIds).sort().join(',') :
      'empty';
  }, [allListedBusinessGmlIds]);

  return useMemo(() => {
    const layersList = [];

    if (tokyoBoundary) {
      // Use optimized boundary layer creation with shared animation calculations
      const boundaryLayers = createBoundaryLayersWithSharedAnimation(tokyoBoundary, time);
      layersList.push(...boundaryLayers);
    }

    // Use memoized GML ID set for buildings layer with selection support
    layersList.push(createBuildingsLayer(
      ownedBusinessGmlIds,
      ownedBusinessesUpdateTrigger,
      selectedBusinessBuildingGmlIds,
      selectedBusinessBuildingUpdateTrigger,
      allListedBusinessGmlIds,
      allListedBusinessUpdateTrigger
    ));

    // Add business layers based on map mode
    if (mapMode === MAP_MODES.OWNED) {
      // Show owned businesses
      if (ownedBusinesses.length > 0) {
        // If a business is selected and has buildings, show arc layer instead of hexagon
        if (selectedBusiness && selectedBusinessBuildings && selectedBusinessBuildings.length > 0 && !('listingUuid' in selectedBusiness)) {
          layersList.push(createHeadquarterArcLayer(selectedBusiness as BusinessDetails, selectedBusinessBuildings, time));
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
    } else if (mapMode === MAP_MODES.MARKET) {
      // Show listed businesses available for purchase
      if (businessListings.length > 0) {
        // If a listed business is selected and has buildings, show arc layer instead of hexagon
        if (selectedBusiness && selectedBusinessBuildings && selectedBusinessBuildings.length > 0 && 'listingUuid' in selectedBusiness) {
          layersList.push(createListedBusinessArcLayer(selectedBusiness as BusinessListingDetails, selectedBusinessBuildings, time));
          // Add hexagons for non-selected businesses
          const nonSelectedBusinesses = businessListings.filter(b => b.businessUuid !== selectedBusiness.businessUuid);
          if (nonSelectedBusinesses.length > 0) {
            layersList.push(createListedBusinessHexLayer(nonSelectedBusinesses, time, zoom));
          }
        } else {
          // Show all businesses as hexagons
          layersList.push(createListedBusinessHexLayer(businessListings, time, zoom));
        }
      }
    }

    return layersList;
  }, [ownedBusinessGmlIds, ownedBusinessesUpdateTrigger, selectedBusinessBuildingGmlIds, selectedBusinessBuildingUpdateTrigger, allListedBusinessGmlIds, allListedBusinessUpdateTrigger, ownedBusinesses, businessListings, time, tokyoBoundary, zoom, mapMode, selectedBusiness, selectedBusinessBuildings]);
};
