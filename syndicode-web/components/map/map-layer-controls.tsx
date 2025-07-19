'use client';

import { Building2, ShoppingCart } from 'lucide-react';
import { useAuthStore } from '@/stores/use-auth-store';
import { useIsMobile } from '@/hooks/use-mobile';

export const MAP_MODES = {
  OWNED: 'owned',
  MARKET: 'market'
} as const;

export type MapMode = typeof MAP_MODES[keyof typeof MAP_MODES];

interface MapLayerControlsProps {
  currentMode: MapMode;
  onModeChange: (mode: MapMode) => void;
}

export function MapLayerControls({ currentMode, onModeChange }: MapLayerControlsProps) {
  const { isAuthenticated } = useAuthStore();
  const isMobile = useIsMobile();

  if (!isAuthenticated) {
    return null;
  }

  return (
    <div className={`absolute ${isMobile ? 'bottom-2 right-2' : 'bottom-4 right-4'} flex flex-row gap-2`}>
      <button
        onClick={() => onModeChange(MAP_MODES.OWNED)}
        className={`${isMobile ? 'p-4 text-base' : 'p-3'} rounded-lg border transition-all touch-manipulation ${
          currentMode === MAP_MODES.OWNED
            ? 'bg-primary/20 border-primary text-primary'
            : 'bg-card/80 border-border hover:bg-card/60 active:bg-card/40'
        }`}
        title="View Owned Businesses"
      >
        <Building2 className={`${isMobile ? 'h-6 w-6' : 'h-5 w-5'}`} />
      </button>
      
      <button
        onClick={() => onModeChange(MAP_MODES.MARKET)}
        className={`${isMobile ? 'p-4 text-base' : 'p-3'} rounded-lg border transition-all touch-manipulation ${
          currentMode === MAP_MODES.MARKET
            ? 'bg-primary/20 border-primary text-primary'
            : 'bg-card/80 border-border hover:bg-card/60 active:bg-card/40'
        }`}
        title="View Market Listings"
      >
        <ShoppingCart className={`${isMobile ? 'h-6 w-6' : 'h-5 w-5'}`} />
      </button>
    </div>
  );
}