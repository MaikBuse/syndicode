'use client';

import React, { useRef, useState, useEffect } from 'react';
import { Map, MapRef, ViewStateChangeEvent } from 'react-map-gl/maplibre';
import { DeckGLOverlay } from '@/components/map/deck-gl-overlay';
import { AuthDialog } from '@/components/auth/auth-dialog';
import { AppSidebar } from '@/components/app-sidebar';
import { InfoSidebar } from '@/components/ui/info-sidebar';
import { BusinessInfoContent } from '@/components/map/business-info-content';
import { MapLoadingIndicator } from '@/components/map/map-loading-indicator';
import { MapLayerControls, MapMode, MAP_MODES } from '@/components/map/map-layer-controls';
import { useAnimationTime } from '@/hooks/use-animation-time';
import { useTokyoBoundary } from '@/hooks/use-tokyo-boundary';
import { useOwnedBusinesses } from '@/hooks/use-owned-businesses';
import { useBusinessBuildings } from '@/hooks/use-business-buildings';
import { useBusinessListings } from '@/hooks/use-business-listings';
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
  useSidebar,
} from "@/components/ui/sidebar";
import { useAuthStore } from '@/stores/use-auth-store';
import { useIsMobile } from '@/hooks/use-mobile';
import { MobileSidebarTrigger } from '@/components/ui/mobile-sidebar-trigger';

function AppContent() {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<MapRef | null>(null);
  const [zoom, setZoom] = useState(TOKYO_INITIAL_VIEW_STATE.zoom);
  const [selectedBusiness, setSelectedBusiness] = useState<BusinessDetails | null>(null);
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const [mapMode, setMapMode] = useState<MapMode>(MAP_MODES.OWNED);

  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const isMobile = useIsMobile();
  const { setOpen } = useSidebar();
  const [prevAuthState, setPrevAuthState] = useState(isAuthenticated);

  // Close sidebar when user transitions from not authenticated to authenticated
  useEffect(() => {
    if (!prevAuthState && isAuthenticated) {
      setOpen(false);
    }
    setPrevAuthState(isAuthenticated);
  }, [isAuthenticated, setOpen, prevAuthState]);

  // Close right sidebar and clear selected business when user logs out
  useEffect(() => {
    if (!isAuthenticated) {
      setIsSidebarOpen(false);
      setSelectedBusiness(null);
    }
  }, [isAuthenticated]);

  const time = useAnimationTime();
  const tokyoBoundary = useTokyoBoundary();
  const ownedBusinesses = useOwnedBusinesses(mapMode);
  const { listings: businessListings } = useBusinessListings(mapMode);
  const { buildings: selectedBusinessBuildings } = useBusinessBuildings(selectedBusiness);
  const layers = useMapLayers(ownedBusinesses, businessListings, time, tokyoBoundary, zoom, mapMode, selectedBusiness, selectedBusinessBuildings);

  const handleViewStateChange = (evt: ViewStateChangeEvent) => {
    setZoom(evt.viewState.zoom);
  };


  const handleClick = (info: PickingInfo) => {
    if ((info.layer?.id === 'headquarters-hex' || info.layer?.id === 'listed-businesses-hex') && info.object?.properties?.business) {
      const business = info.object.properties.business;
      // Toggle selection - if already selected, deselect
      const isAlreadySelected = selectedBusiness?.businessUuid === business.businessUuid;
      setSelectedBusiness(isAlreadySelected ? null : business);
      setIsSidebarOpen(!isAlreadySelected);
    } else {
      // Click on empty space deselects
      setSelectedBusiness(null);
      setIsSidebarOpen(false);
    }
  };

  return (
    <div ref={containerRef} style={{ width: '100%', height: '100vh', position: 'relative' }}>
      <Map
        ref={mapRef}
        initialViewState={TOKYO_INITIAL_VIEW_STATE}
        mapStyle={MAP_STYLE}
        style={{ width: '100%', height: '100%', backgroundColor: 'hsl(0, 0%, 20%)' }}
        maxBounds={TOKYO_BOUNDS}
        minZoom={12}
        maxZoom={18}
        onMove={handleViewStateChange}
        attributionControl={false}
      >
        <DeckGLOverlay
          layers={layers}
          useDevicePixels={true}
          pickingRadius={isMobile ? 12 : 5}
          onClick={handleClick}
        />
      </Map>
      <AppSidebar />
      <MobileSidebarTrigger />
      <MapLoadingIndicator />
      <MapLayerControls
        currentMode={mapMode}
        onModeChange={setMapMode}
      />

      {/* Info Sidebar */}
      <InfoSidebar
        isOpen={isSidebarOpen}
        onClose={() => {
          setIsSidebarOpen(false);
          setSelectedBusiness(null);
        }}
        title="Business Details"
      >
        <BusinessInfoContent business={selectedBusiness} />
      </InfoSidebar>
      <AuthDialog />
    </div>
  );
}

function App() {
  return (
    <SidebarProvider>
      <AppContent />
    </SidebarProvider>
  );
}

export default App;
