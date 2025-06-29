import { create } from 'zustand';

type AuthView = 'login' | 'register' | 'verify';

type AuthModalState = {
  isOpen: boolean;
  view: AuthView;
  userNameToVerify: string | null; // To pass username from register to verify
  openModal: (view?: AuthView) => void;
  closeModal: () => void;
  setView: (view: AuthView) => void;
  setUserNameToVerify: (name: string) => void;
};

export const useAuthModal = create<AuthModalState>((set) => ({
  isOpen: false,
  view: 'login',
  userNameToVerify: null,
  openModal: (view = 'login') => set({ isOpen: true, view }),
  closeModal: () => set({ isOpen: false, userNameToVerify: null }),
  setView: (view) => set({ view }),
  setUserNameToVerify: (name) => set({ userNameToVerify: name }),
}));
