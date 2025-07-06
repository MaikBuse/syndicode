'use client';

import { useEffect, useRef } from 'react';
import { useAuthStore } from '@/stores/use-auth-store';
import { useUserDataStore } from '@/stores/use_user_data_store';

export function SessionDataInitializer() {
  const { user } = useAuthStore();
  const { fetchUserData, data } = useUserDataStore();
  const hasFetched = useRef(false);

  useEffect(() => {
    // Check if:
    // 1. We have a logged-in user.
    // 2. We haven't already loaded their data.
    // 3. We haven't already tried to fetch the data in this session.
    if (user && !data && !hasFetched.current) {
      // Mark that we are initiating a fetch to prevent re-fetching on re-renders.
      hasFetched.current = true;
      fetchUserData();
    }

    // If the user logs out, reset the fetched status
    if (!user) {
      hasFetched.current = false;
      // You might also want to clear the user data store here
      // useUserDataStore.setState({ data: null });
    }
  }, [user, data, fetchUserData]);

  // This component renders nothing. It's purely for side effects.
  return null;
}
