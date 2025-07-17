'use client';

import React, { useRef, useState } from 'react';
import { Map, MapRef, ViewStateChangeEvent } from 'react-map-gl/maplibre';
import { DeckGLOverlay } from '@/components/map/deck-gl-overlay';
import { AuthDialog } from '@/components/auth/auth-dialog';
import { AppSidebar } from '@/components/app-sidebar';
import { useAnimationTime } from '@/hooks/use-animation-time';
import { useTokyoBoundary } from '@/hooks/use-tokyo-boundary';
import { useOwnedBusinesses } from '@/hooks/use-owned-businesses';
import { useMapLayers } from '@/hooks/use-map-layers';
import {
  TOKYO_BOUNDS,
  TOKYO_INITIAL_VIEW_STATE,
  MAP_STYLE
} from '@/lib/map/constants';
import {
  SidebarInset,
  SidebarProvider,
} from "@/components/ui/sidebar";

function App() {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<MapRef | null>(null);
  const [zoom, setZoom] = useState(TOKYO_INITIAL_VIEW_STATE.zoom);

  const time = useAnimationTime();
  const tokyoBoundary = useTokyoBoundary();
  const ownedBusinesses = useOwnedBusinesses();
  const layers = useMapLayers(ownedBusinesses, time, tokyoBoundary, zoom);

  const handleViewStateChange = (evt: ViewStateChangeEvent) => {
    setZoom(evt.viewState.zoom);
  };

  return (
    <SidebarProvider>
      <div ref={containerRef} style={{ width: '100%', height: '100vh', position: 'relative' }}>
        <Map
          ref={mapRef}
          initialViewState={TOKYO_INITIAL_VIEW_STATE}
          mapStyle={MAP_STYLE}
          style={{ width: '100%', height: '100%' }}
          maxBounds={TOKYO_BOUNDS}
          minZoom={12}
          maxZoom={18}
          onMove={handleViewStateChange}
        >
          <DeckGLOverlay
            layers={layers}
            useDevicePixels={true}
            pickingRadius={5}
          />
        </Map>
        <AppSidebar />
      </div>
      <AuthDialog />
    </SidebarProvider>
  );
}

export default App;
