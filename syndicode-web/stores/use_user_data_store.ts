import { create } from 'zustand';
import { Corporation } from '@/domain/economy/economy.types';
import { getCurrentCorporationAction } from '@/app/actions/economy.actions';
import { useMapLoadingStore } from './use-map-loading-store';

interface UserData {
  corporation: Corporation;
}

interface UserDataState {
  data: UserData | null;
  isLoading: boolean;
  fetchUserData: () => Promise<void>;
  fetchCorporation: () => Promise<void>;
  clearUserData: () => void;
}


export const useUserDataStore = create<UserDataState>((set) => ({
  data: null,
  isLoading: false,
  fetchUserData: async () => {
    set({ isLoading: true });
    useMapLoadingStore.getState().setCorporationLoading(true);
    
    try {
      const response = await getCurrentCorporationAction()

      if (response.success) {
        const userData = {
          corporation: response.data
        };

        set({ data: userData, isLoading: false });
      }
    } catch (error) {
      console.error("Failed to fetch user data", error);
      set({ isLoading: false, data: null }); // Handle error state
    } finally {
      useMapLoadingStore.getState().setCorporationLoading(false);
    }
  },
  fetchCorporation: async () => {
    try {
      const response = await getCurrentCorporationAction();

      if (response.success) {
        set((state) => ({
          data: state.data ? { ...state.data, corporation: response.data } : { corporation: response.data }
        }));
      }
    } catch (error) {
      console.error("Failed to fetch corporation data", error);
    }
  },
  clearUserData: () => {
    set({ data: null, isLoading: false });
  },
}));
