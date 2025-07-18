import { User } from '@/domain/auth/auth.types';
import { create } from 'zustand';
import { logout as logoutAction } from '@/app/actions/auth';
import { useAuthModal } from '@/stores/use-auth-modal';

type AuthState = {
  isAuthenticated: boolean;
  user: User | null;
  // Action to set the user and authenticated status (e.g., on login)
  login: (user: User) => void;
  // Action to clear the user and status (e.g., on logout)
  logout: () => Promise<void>;
  // Action to logout due to expired token and open login dialog
  logoutExpired: () => Promise<void>;
  // Action for initialization from the server
  initialize: (initialState: { isAuthenticated: boolean; user: User | null }) => void;
};

export const useAuthStore = create<AuthState>((set) => ({
  isAuthenticated: false,
  user: null,
  login: (user) => set({ isAuthenticated: true, user }),
  logout: async () => {
    await logoutAction();
    set({ isAuthenticated: false, user: null });
  },
  logoutExpired: async () => {
    await logoutAction();
    set({ isAuthenticated: false, user: null });
    useAuthModal.getState().openModal('login');
  },
  initialize: (initialState) => set(initialState),
}));
