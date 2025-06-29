'use client';

import { useAuthModal } from '@/stores/use-auth-modal';
import { Button } from '@/components/ui/button';

export function AuthButton() {
  const openModal = useAuthModal((state) => state.openModal);

  return <Button onClick={() => openModal('login')}>Login / Sign Up</Button>;
}
