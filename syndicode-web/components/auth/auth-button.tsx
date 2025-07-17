'use client';

import { LogIn } from 'lucide-react';
import { useAuthModal } from '@/stores/use-auth-modal';
import { SidebarMenuButton } from '@/components/ui/sidebar';

export function AuthButton() {
  const openModal = useAuthModal((state) => state.openModal);

  return (
    <SidebarMenuButton 
      size="lg"
      onClick={() => openModal('login')}
      className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
    >
      <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
        <LogIn className="size-4" />
      </div>
      <div className="grid flex-1 text-left text-sm leading-tight">
        <span className="truncate font-medium">Sign In</span>
        <span className="truncate text-xs">Access your account</span>
      </div>
    </SidebarMenuButton>
  );
}
