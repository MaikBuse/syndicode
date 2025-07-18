'use client';

import { Building2, ShoppingCart } from 'lucide-react';
import { useAuthStore } from '@/stores/use-auth-store';

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

  if (!isAuthenticated) {
    return null;
  }

  return (
    <div className="absolute bottom-4 right-4 flex flex-row gap-2">
      <button
        onClick={() => onModeChange(MAP_MODES.OWNED)}
        className={`p-3 rounded-lg border transition-all ${
          currentMode === MAP_MODES.OWNED
            ? 'bg-primary/20 border-primary text-primary'
            : 'bg-card/80 border-border hover:bg-card/60'
        }`}
        title="View Owned Businesses"
      >
        <Building2 className="h-5 w-5" />
      </button>
      
      <button
        onClick={() => onModeChange(MAP_MODES.MARKET)}
        className={`p-3 rounded-lg border transition-all ${
          currentMode === MAP_MODES.MARKET
            ? 'bg-primary/20 border-primary text-primary'
            : 'bg-card/80 border-border hover:bg-card/60'
        }`}
        title="View Market Listings"
      >
        <ShoppingCart className="h-5 w-5" />
      </button>
    </div>
  );
}