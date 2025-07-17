import { useState, useEffect } from 'react';
import { queryBuildingsAction } from '@/app/actions/economy.actions';
import type { BusinessDetails, BuildingDetails } from '@/domain/economy/economy.types';
import { useAuthStore } from '@/stores/use-auth-store';

export const useBusinessBuildings = (selectedBusiness: BusinessDetails | null) => {
  const [buildings, setBuildings] = useState<BuildingDetails[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const user = useAuthStore((state) => state.user);

  useEffect(() => {
    if (!selectedBusiness || !user) {
      setBuildings([]);
      setLoading(false);
      setError(null);
      return;
    }

    const fetchBuildings = async () => {
      setLoading(true);
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
      }
    };

    fetchBuildings();
  }, [selectedBusiness?.businessUuid, user]);

  return { buildings, loading, error };
};