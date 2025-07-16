'use client';

import React, { useState, useRef } from 'react';
import { Map, MapRef } from 'react-map-gl/maplibre';
import type { ViewState } from 'react-map-gl/maplibre';
import { DeckGLOverlay } from '@/components/map/deck-gl-overlay';
import { AuthOverlay } from '@/components/map/auth-overlay';
import { useAnimationTime } from '@/hooks/use-animation-time';
import { useTokyoBoundary } from '@/hooks/use-tokyo-boundary';
import { useOwnedBusinesses } from '@/hooks/use-owned-businesses';
import { useMapLayers } from '@/hooks/use-map-layers';
import {
  TOKYO_BOUNDS,
  TOKYO_INITIAL_VIEW_STATE,
  MAP_STYLE
} from '@/lib/map/constants';

function App() {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<MapRef | null>(null);
  const [currentViewState, setCurrentViewState] = useState<ViewState>(TOKYO_INITIAL_VIEW_STATE);

  const time = useAnimationTime();
  const tokyoBoundary = useTokyoBoundary();
  const ownedBusinessGmlIds = useOwnedBusinesses();
  const layers = useMapLayers(ownedBusinessGmlIds, time, tokyoBoundary);

  return (
    <div ref={containerRef} style={{ width: '100%', height: '100%', position: 'relative' }}>
      <AuthOverlay />

      <Map
        ref={mapRef}
        initialViewState={TOKYO_INITIAL_VIEW_STATE}
        onMove={evt => {
          setCurrentViewState(evt.viewState);
        }}
        mapStyle={MAP_STYLE}
        style={{ width: '100%', height: '100%' }}
        maxBounds={TOKYO_BOUNDS}
        minZoom={12}
        maxZoom={18}
      >
        <DeckGLOverlay
          layers={layers}
          useDevicePixels={true}
          pickingRadius={5}
        />
      </Map>
    </div>
  );
}

export default App;
