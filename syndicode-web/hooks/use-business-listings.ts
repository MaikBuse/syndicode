import { useState, useEffect } from 'react';
import { useAuthStore } from '@/stores/use-auth-store';
import { queryBusinessListings } from '@/app/actions/economy.actions';
import type { BusinessListingDetails } from '@/domain/economy/economy.types';

export const useBusinessListings = () => {
  const [listings, setListings] = useState<BusinessListingDetails[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { isAuthenticated } = useAuthStore();

  useEffect(() => {
    if (!isAuthenticated) {
      setListings([]);
      return;
    }

    const fetchListings = async () => {
      setLoading(true);
      setError(null);
      
      try {
        const result = await queryBusinessListings({});
        setListings(result.listings);
      } catch (err) {
        setError('Failed to fetch business listings');
        console.error('Error fetching business listings:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchListings();
  }, [isAuthenticated]);

  return { listings, loading, error };
};