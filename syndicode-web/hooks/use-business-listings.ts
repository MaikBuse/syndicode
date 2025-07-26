import { useState, useEffect } from 'react';
import { useAuthStore } from '@/stores/use-auth-store';
import { useMapLoadingStore } from '@/stores/use-map-loading-store';
import { queryBusinessListings } from '@/app/actions/economy.actions';
import type { BusinessListingDetails } from '@/domain/economy/economy.types';
import type { MapMode } from '@/components/map/map-layer-controls';
import { MAP_MODES } from '@/components/map/map-layer-controls';

export const useBusinessListings = (mapMode: MapMode) => {
  const [listings, setListings] = useState<BusinessListingDetails[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { isAuthenticated } = useAuthStore();
  const setBusinessesLoading = useMapLoadingStore((state) => state.setBusinessesLoading);

  useEffect(() => {
    const shouldFetchListings = isAuthenticated && mapMode === MAP_MODES.MARKET;
    
    const fetchListings = async () => {
      if (!shouldFetchListings) {
        if (listings.length > 0) {
          setListings([]);
        }
        return;
      }

      setLoading(true);
      setBusinessesLoading(true);
      setError(null);
      
      try {
        const result = await queryBusinessListings({});
        setListings(result.listings);
      } catch (err) {
        setError('Failed to fetch business listings');
        console.error('Error fetching business listings:', err);
      } finally {
        setLoading(false);
        setBusinessesLoading(false);
      }
    };

    fetchListings();
  }, [isAuthenticated, mapMode, setBusinessesLoading, listings.length]);

  return { listings, loading, error };
};