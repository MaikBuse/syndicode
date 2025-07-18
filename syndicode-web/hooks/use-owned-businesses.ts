import { useState, useEffect } from 'react';
import { useAuthStore } from '@/stores/use-auth-store';
import { useUserDataStore } from '@/stores/use_user_data_store';
import { useMapLoadingStore } from '@/stores/use-map-loading-store';
import { queryBusinessesAction } from '@/app/actions/economy.actions';
import { toast } from 'sonner';
import type { BusinessDetails } from '@/domain/economy/economy.types';
import type { MapMode } from '@/components/map/map-layer-controls';
import { MAP_MODES } from '@/components/map/map-layer-controls';

export const useOwnedBusinesses = (mapMode: MapMode) => {
  const [ownedBusinesses, setOwnedBusinesses] = useState<BusinessDetails[]>([]);
  
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const corporation = useUserDataStore((state) => state.data?.corporation);
  const setBusinessesLoading = useMapLoadingStore((state) => state.setBusinessesLoading);

  useEffect(() => {
    const shouldFetchOwnedBusinesses = isAuthenticated && corporation?.uuid && mapMode === MAP_MODES.OWNED;
    
    const fetchOwnedBusinesses = async () => {
      if (!shouldFetchOwnedBusinesses) {
        if (ownedBusinesses.length > 0) {
          setOwnedBusinesses([]);
        }
        return;
      }

      setBusinessesLoading(true);
      
      try {
        const payload = {
          owningCorporationUuid: corporation.uuid,
          limit: 100 // Maximum allowed limit
        };

        const response = await queryBusinessesAction(payload);

        if (response.success) {
          setOwnedBusinesses(response.data.businesses);
        } else {
          console.error("Failed to fetch owned businesses:", response.message);
          toast.error("Could not load your businesses.", { description: response.message });
        }
      } finally {
        setBusinessesLoading(false);
      }
    };

    fetchOwnedBusinesses();
  }, [isAuthenticated, corporation?.uuid, mapMode, ownedBusinesses.length, setBusinessesLoading]);

  return ownedBusinesses;
};