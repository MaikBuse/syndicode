import { User } from '@/domain/auth/auth.types';
import { create } from 'zustand';

type AuthState = {
  isAuthenticated: boolean;
  user: User | null;
  // Action to set the user and authenticated status (e.g., on login)
  login: (user: User) => void;
  // Action to clear the user and status (e.g., on logout)
  logout: () => void;
  // Action for initialization from the server
  initialize: (initialState: { isAuthenticated: boolean; user: User | null }) => void;
};

export const useAuthStore = create<AuthState>((set) => ({
  isAuthenticated: false,
  user: null,
  login: (user) => set({ isAuthenticated: true, user }),
  logout: () => set({ isAuthenticated: false, user: null }),
  initialize: (initialState) => set(initialState),
}));
