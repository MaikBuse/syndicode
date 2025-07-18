'use client';

import { useAuthStore } from '@/stores/use-auth-store';
import { getCurrentUser } from '@/app/actions/auth';

export function handleExpiredToken() {
  const { logoutExpired } = useAuthStore.getState();
  logoutExpired();
}

export async function verifyTokenAndHandleErrors() {
  try {
    const user = await getCurrentUser();
    return user;
  } catch {
    // Any JWT verification error should trigger logout and login dialog
    handleExpiredToken();
    return null;
  }
}