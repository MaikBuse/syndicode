'use client';

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useAuthModal } from '@/stores/use-auth-modal';
import { LoginForm } from './login-form';
import { RegisterForm } from './register-form';
import { VerifyForm } from './verify-form';

const titles: Record<ReturnType<typeof useAuthModal.getState>['view'], string> = {
  login: 'Log In',
  register: 'Create an Account',
  verify: 'Verify Your Email',
};

export function AuthDialog() {
  const { isOpen, view, closeModal } = useAuthModal();

  return (
    <Dialog open={isOpen} onOpenChange={closeModal}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{titles[view]}</DialogTitle>
        </DialogHeader>
        <div className="mt-4">
          {view === 'login' && <LoginForm />}
          {view === 'register' && <RegisterForm />}
          {view === 'verify' && <VerifyForm />}
        </div>
      </DialogContent>
    </Dialog>
  );
}
