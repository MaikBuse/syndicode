import { useState, useEffect } from 'react';
import { useAuthStore } from '@/stores/use-auth-store';
import { useUserDataStore } from '@/stores/use_user_data_store';
import { queryBusinessesAction } from '@/app/actions/economy.actions';
import { toast } from 'sonner';

export const useOwnedBusinesses = () => {
  const [ownedBusinessGmlIds, setOwnedBusinessGmlIds] = useState<Set<string>>(new Set());
  
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const corporation = useUserDataStore((state) => state.data?.corporation);

  useEffect(() => {
    const fetchOwnedBusinesses = async () => {
      if (!isAuthenticated || !corporation?.uuid) {
        if (ownedBusinessGmlIds.size > 0) {
          setOwnedBusinessGmlIds(new Set());
        }
        return;
      }

      const payload = {
        owningCorporationUuid: corporation.uuid,
        limit: 1000 // Get all businesses owned by the corporation
      };

      const response = await queryBusinessesAction(payload);

      if (response.success) {
        const newOwnedIds = new Set(response.data.businesses.map(b => b.headquarterBuildingGmlId));
        setOwnedBusinessGmlIds(newOwnedIds);
      } else {
        console.error("Failed to fetch owned businesses:", response.message);
        toast.error("Could not load your businesses.", { description: response.message });
      }
    };

    fetchOwnedBusinesses();
  }, [isAuthenticated, corporation, ownedBusinessGmlIds.size]);

  return ownedBusinessGmlIds;
};