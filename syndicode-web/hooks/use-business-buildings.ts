import { useState, useEffect } from 'react';
import { queryBuildingsAction } from '@/app/actions/economy.actions';
import type { BusinessDetails, BuildingDetails } from '@/domain/economy/economy.types';
import { useAuthStore } from '@/stores/use-auth-store';
import { useMapLoadingStore } from '@/stores/use-map-loading-store';

export const useBusinessBuildings = (selectedBusiness: BusinessDetails | null) => {
  const [buildings, setBuildings] = useState<BuildingDetails[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const user = useAuthStore((state) => state.user);
  const setBuildingsLoading = useMapLoadingStore((state) => state.setBuildingsLoading);

  useEffect(() => {
    // Immediately clear buildings when no business is selected
    if (!selectedBusiness) {
      setBuildings([]);
      setLoading(false);
      setError(null);
      return;
    }

    if (!user) {
      setBuildings([]);
      setLoading(false);
      setError(null);
      return;
    }

    const fetchBuildings = async () => {
      // Clear previous buildings immediately when switching between businesses
      setBuildings([]);
      setLoading(true);
      setBuildingsLoading(true);
      setError(null);

      try {
        const result = await queryBuildingsAction({
          owningBusinessUuid: selectedBusiness.businessUuid,
          limit: 100 // Max limit allowed by validation schema
        });

        if (result.success) {
          setBuildings(result.data.buildings);
        } else {
          setError(result.message || 'Failed to fetch buildings');
          setBuildings([]);
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
        setBuildings([]);
      } finally {
        setLoading(false);
        setBuildingsLoading(false);
      }
    };

    fetchBuildings();
  }, [selectedBusiness?.businessUuid, user, setBuildingsLoading]);

  return { buildings, loading, error };
};