import { AuthButton } from '@/components/auth/auth-button';
import { useAuthStore } from '@/stores/use-auth-store';

export function AuthOverlay() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);

  if (isAuthenticated) {
    return null;
  }

  return (
    <div style={{ position: 'absolute', top: 20, right: 20, zIndex: 100 }}>
      <AuthButton />
    </div>
  );
}