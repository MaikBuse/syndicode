import { create } from 'zustand';

interface MapLoadingState {
  // Individual operation states
  businessesLoading: boolean;
  buildingsLoading: boolean;
  corporationLoading: boolean;
  boundaryLoading: boolean;
  
  // Computed state - true if any map operation is loading
  isMapLoading: boolean;
  
  // Actions
  setBusinessesLoading: (loading: boolean) => void;
  setBuildingsLoading: (loading: boolean) => void;
  setCorporationLoading: (loading: boolean) => void;
  setBoundaryLoading: (loading: boolean) => void;
}

const calculateMapLoading = (state: Pick<MapLoadingState, 'businessesLoading' | 'buildingsLoading' | 'corporationLoading' | 'boundaryLoading'>) => {
  return state.businessesLoading || state.buildingsLoading || state.corporationLoading || state.boundaryLoading;
};

export const useMapLoadingStore = create<MapLoadingState>((set, get) => ({
  // Initial state
  businessesLoading: false,
  buildingsLoading: false,
  corporationLoading: false,
  boundaryLoading: false,
  isMapLoading: false,
  
  // Actions
  setBusinessesLoading: (loading: boolean) => {
    set((state) => {
      const newState = { ...state, businessesLoading: loading };
      return { ...newState, isMapLoading: calculateMapLoading(newState) };
    });
  },
  
  setBuildingsLoading: (loading: boolean) => {
    set((state) => {
      const newState = { ...state, buildingsLoading: loading };
      return { ...newState, isMapLoading: calculateMapLoading(newState) };
    });
  },
  
  setCorporationLoading: (loading: boolean) => {
    set((state) => {
      const newState = { ...state, corporationLoading: loading };
      return { ...newState, isMapLoading: calculateMapLoading(newState) };
    });
  },
  
  setBoundaryLoading: (loading: boolean) => {
    set((state) => {
      const newState = { ...state, boundaryLoading: loading };
      return { ...newState, isMapLoading: calculateMapLoading(newState) };
    });
  },
}));