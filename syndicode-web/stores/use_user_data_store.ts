import { create } from 'zustand';
import { Corporation } from '@/domain/economy/economy.types';
import { getCurrentCorporationAction } from '@/app/actions/economy.actions';

interface UserData {
  corporation: Corporation;
}

interface UserDataState {
  data: UserData | null;
  isLoading: boolean;
  fetchUserData: () => Promise<void>;
}


export const useUserDataStore = create<UserDataState>((set) => ({
  data: null,
  isLoading: false,
  fetchUserData: async () => {
    set({ isLoading: true });
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
    }
  },
}));
