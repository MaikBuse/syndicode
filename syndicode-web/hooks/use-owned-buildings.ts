import { useState, useEffect } from 'react';
import type { ViewState, MapRef } from 'react-map-gl/maplibre';
import { useAuthStore } from '@/stores/use-auth-store';
import { useUserDataStore } from '@/stores/use_user_data_store';
import { queryBuildingsAction } from '@/app/actions/economy.actions';
import { toast } from 'sonner';
import { QUERY_ZOOM_LEVEL_THRESHOLD } from '@/lib/map/constants';

export const useOwnedBuildings = (
  currentViewState: ViewState,
  mapRef: React.RefObject<MapRef | null>
) => {
  const [ownedBuildingGmlId, setOwnedBuildingGmlId] = useState<Set<string>>(new Set());
  
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const corporation = useUserDataStore((state) => state.data?.corporation);

  useEffect(() => {
    const debounceTimer = setTimeout(() => {
      const fetchOwnedBuildings = async () => {
        if (!isAuthenticated || !corporation?.uuid || currentViewState.zoom < QUERY_ZOOM_LEVEL_THRESHOLD) {
          if (ownedBuildingGmlId.size > 0) {
            setOwnedBuildingGmlId(new Set());
          }
          return;
        }

        const map = mapRef.current?.getMap();
        if (!map) return;

        const bounds = map.getBounds();
        const payload = {
          owningCorporationUuid: corporation.uuid,
          minLon: bounds.getWest(),
          maxLon: bounds.getEast(),
          minLat: bounds.getSouth(),
          maxLat: bounds.getNorth(),
          limit: 100
        };

        const response = await queryBuildingsAction(payload);

        if (response.success) {
          const newOwnedIds = new Set(response.data.buildings.map(b => b.gmlId));
          setOwnedBuildingGmlId(newOwnedIds);
        } else {
          console.error("Failed to fetch owned buildings:", response.message);
          toast.error("Could not load your properties.", { description: response.message });
        }
      };

      fetchOwnedBuildings();
    }, 500);

    return () => clearTimeout(debounceTimer);
  }, [currentViewState, isAuthenticated, corporation, ownedBuildingGmlId.size, mapRef]);

  return ownedBuildingGmlId;
};