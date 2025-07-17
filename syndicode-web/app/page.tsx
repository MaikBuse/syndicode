'use client';

import React, { useRef, useState } from 'react';
import { Map, MapRef, ViewStateChangeEvent } from 'react-map-gl/maplibre';
import { DeckGLOverlay } from '@/components/map/deck-gl-overlay';
import { AuthDialog } from '@/components/auth/auth-dialog';
import { AppSidebar } from '@/components/app-sidebar';
import { BusinessInfoCard } from '@/components/map/business-info-card';
import { useAnimationTime } from '@/hooks/use-animation-time';
import { useTokyoBoundary } from '@/hooks/use-tokyo-boundary';
import { useOwnedBusinesses } from '@/hooks/use-owned-businesses';
import { useBusinessBuildings } from '@/hooks/use-business-buildings';
import { useMapLayers } from '@/hooks/use-map-layers';
import type { BusinessDetails } from '@/domain/economy/economy.types';
import type { PickingInfo } from '@deck.gl/core';
import {
  TOKYO_BOUNDS,
  TOKYO_INITIAL_VIEW_STATE,
  MAP_STYLE
} from '@/lib/map/constants';
import {
  SidebarProvider,
} from "@/components/ui/sidebar";

function App() {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<MapRef | null>(null);
  const [zoom, setZoom] = useState(TOKYO_INITIAL_VIEW_STATE.zoom);
  const [hoveredBusiness, setHoveredBusiness] = useState<BusinessDetails | null>(null);
  const [selectedBusiness, setSelectedBusiness] = useState<BusinessDetails | null>(null);

  const time = useAnimationTime();
  const tokyoBoundary = useTokyoBoundary();
  const ownedBusinesses = useOwnedBusinesses();
  const { buildings: selectedBusinessBuildings } = useBusinessBuildings(selectedBusiness);
  const layers = useMapLayers(ownedBusinesses, time, tokyoBoundary, zoom, selectedBusiness, selectedBusinessBuildings);

  const handleViewStateChange = (evt: ViewStateChangeEvent) => {
    setZoom(evt.viewState.zoom);
  };

  const handleHover = (info: PickingInfo) => {
    if (info.layer?.id === 'headquarters-hex' && info.object?.properties?.business) {
      setHoveredBusiness(info.object.properties.business);
    } else {
      setHoveredBusiness(null);
    }
  };

  const handleClick = (info: PickingInfo) => {
    if (info.layer?.id === 'headquarters-hex' && info.object?.properties?.business) {
      const business = info.object.properties.business;
      // Toggle selection - if already selected, deselect
      setSelectedBusiness(selectedBusiness?.businessUuid === business.businessUuid ? null : business);
    } else {
      // Click on empty space deselects
      setSelectedBusiness(null);
    }
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
            onHover={handleHover}
            onClick={handleClick}
          />
        </Map>
        <AppSidebar />

        {/* Business Info Card - positioned in bottom right */}
        {(hoveredBusiness || selectedBusiness) && (
          <div className="absolute bottom-4 right-4 z-10 animate-in fade-in-0 slide-in-from-bottom-2 duration-200">
            <BusinessInfoCard business={selectedBusiness || hoveredBusiness} />
          </div>
        )}
      </div>
      <AuthDialog />
    </SidebarProvider>
  );
}

export default App;
