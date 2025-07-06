'use client';

import { User } from '@/domain/auth/auth.types';
import { useAuthStore } from '@/stores/use-auth-store';
import { useRef, useEffect } from 'react';

// This component's sole purpose is to initialize the store
function AuthStoreInitializer({ user }: { user: User | null }) {
  // Use a ref to ensure initialization only happens once
  const initialized = useRef(false);

  useEffect(() => {
    if (!initialized.current) {
      useAuthStore.getState().initialize({
        isAuthenticated: !!user,
        user: user,
      });
      initialized.current = true;
    }
  }, [user]); // Depend on user prop to re-init if it changes (e.g., on revalidation)

  return null; // This component doesn't render anything
}

export default AuthStoreInitializer;
