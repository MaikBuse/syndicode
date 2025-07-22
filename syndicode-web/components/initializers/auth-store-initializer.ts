'use client';

import { User } from '@/domain/auth/auth.types';
import { useAuthStore } from '@/stores/use-auth-store';
import { useRef, useEffect } from 'react';
import { clearExpiredAuthToken } from '@/app/actions/auth';
import { useAuthModal } from '@/stores/use-auth-modal';

// This component's sole purpose is to initialize the store
function AuthStoreInitializer({ user }: { user: User | null }) {
  // Use a ref to ensure initialization only happens once
  const initialized = useRef(false);
  // Track if we've detected an expired session
  const expiredSessionHandled = useRef(false);

  useEffect(() => {
    if (!initialized.current) {
      useAuthStore.getState().initialize({
        isAuthenticated: !!user,
        user: user,
      });
      initialized.current = true;

      // Check if there's an auth_token cookie but user is null (expired token)
      if (!user && document.cookie.includes('auth_token=') && !expiredSessionHandled.current) {
        expiredSessionHandled.current = true;
        // Clear the expired token and show login dialog
        clearExpiredAuthToken().then(() => {
          useAuthModal.getState().openModal('login');
        });
      }
    }
  }, [user]); // Depend on user prop to re-init if it changes (e.g., on revalidation)

  return null; // This component doesn't render anything
}

export default AuthStoreInitializer;
