'use client';

import React, { useState, useMemo, useEffect, useRef } from 'react';
import { Map, useControl, MapRef } from 'react-map-gl/maplibre';
import type { ViewState } from 'react-map-gl/maplibre';
import { MapboxOverlay } from '@deck.gl/mapbox';
import type { DeckProps } from '@deck.gl/core';
import { MVTLayer } from '@deck.gl/geo-layers';
import { AuthButton } from '@/components/auth/auth-button';
import { useAuthStore } from '@/stores/use-auth-store';
import { toast } from 'sonner';
import { useUserDataStore } from '@/stores/use_user_data_store';
import { queryBuildingsAction } from './actions/economy.actions';

function DeckGLOverlay(props: DeckProps) {
  const overlay = useControl<MapboxOverlay>(() => new MapboxOverlay(props));
  overlay.setProps(props);
  return null;
}

const TOKYO_BOUNDS: [[number, number], [number, number]] = [
  [139.3, 35.4], // Southwest coordinates
  [140.1, 35.9]  // Northeast coordinates
];

const TOKYO_INITIAL_VIEW_STATE: ViewState = {
  longitude: 139.6917,
  latitude: 35.6895,
  zoom: 12,
  pitch: 50,
  bearing: 0,
  padding: { top: 0, bottom: 0, left: 0, right: 0 },
};

const TILE_URL = 'https://assets.syndicode.dev/tokyo-tiles/{z}/{x}/{y}.pbf';

// Define a zoom level threshold to start querying for owned buildings.
// This prevents fetching thousands of buildings when zoomed out.
const QUERY_ZOOM_LEVEL_THRESHOLD = 15;

function App() {
  const containerRef = useRef<HTMLDivElement>(null);

  const mapStyle = `https://tiles.stadiamaps.com/styles/alidade_smooth_dark.json`;
  const mapRef = useRef<MapRef | null>(null);

  // Track the current view state for querying buildings
  const [currentViewState, setCurrentViewState] = useState<ViewState>(TOKYO_INITIAL_VIEW_STATE);

  // State to track which building IDs are owned by the user's corporation
  const [ownedBuildingGmlId, setOwnedBuildingGmlId] = useState<Set<string>>(new Set());

  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const corporation = useUserDataStore((state) => state.data?.corporation);

  // Effect to fetch owned buildings when the map view changes
  useEffect(() => {
    // Debounce the fetch request. This creates a timer that will be
    // reset every time the viewState changes. The API call only happens
    // 500ms after the user stops moving the map.
    const debounceTimer = setTimeout(() => {
      const fetchOwnedBuildings = async () => {
        // Condition 1: User must be logged in and have a corporation
        // Condition 2: Map must be zoomed in past the threshold
        if (!isAuthenticated || !corporation?.uuid || currentViewState.zoom < QUERY_ZOOM_LEVEL_THRESHOLD) {
          // If conditions aren't met, ensure the list of owned buildings is empty
          if (ownedBuildingGmlId.size > 0) {
            setOwnedBuildingGmlId(new Set());
          }
          return;
        }

        // Get the current map boundaries
        const map = mapRef.current?.getMap();
        if (!map) return;

        const bounds = map.getBounds();
        const payload = {
          owningCorporationUuid: corporation.uuid,
          minLon: bounds.getWest(),
          maxLon: bounds.getEast(),
          minLat: bounds.getSouth(),
          maxLat: bounds.getNorth(),
          limit: 100
        };

        const response = await queryBuildingsAction(payload);

        if (response.success) {
          // Extract the gml_id from each building and create a Set for fast lookups.
          const newOwnedIds = new Set(response.data.buildings.map(b => b.gmlId));
          setOwnedBuildingGmlId(newOwnedIds);
        } else {
          console.error("Failed to fetch owned buildings:", response.message);
          toast.error("Could not load your properties.", { description: response.message });
        }
      };

      fetchOwnedBuildings();
    }, 500); // 500ms debounce delay

    // Cleanup function: clear the timer if the component unmounts or viewState changes again
    return () => clearTimeout(debounceTimer);

  }, [currentViewState, isAuthenticated, corporation, ownedBuildingGmlId.size]);

  const layers = useMemo(() => [
    new MVTLayer({
      id: 'buildings',
      data: TILE_URL,
      minZoom: 12,
      maxZoom: 16,
      onTileError: () => { }, // Suppress edge tile errors
      extruded: true,
      pickable: true,
      autoHighlight: true,
      getElevation: (d: { properties: { cal_height_m: number } }) => d.properties.cal_height_m,
      getFillColor: (d: { properties: { gml_id: string } }) => {
        // Check if the building's ID is in our Set of owned IDs
        const isOwned = ownedBuildingGmlId.has(d.properties.gml_id);
        return isOwned ? [255, 0, 128, 255] : [150, 150, 150, 255]; // Owned: Pink, Not Owned: Grey
      },
      // This is crucial. It tells Deck.gl to re-evaluate getFillColor
      // whenever the `ownedBuildingIds` state changes.
      updateTriggers: {
        getFillColor: [ownedBuildingGmlId]
      },
      getLineColor: [60, 60, 60],
      lineWidthMinPixels: 1,
    })
  ], [ownedBuildingGmlId]); // Re-create layers only when ownedBuildingIds changes

  return (
    <div ref={containerRef} style={{ width: '100%', height: '100%', position: 'relative' }}>
      <div
        style={{ position: 'absolute', top: 20, right: 20, zIndex: 100 }}>
        {!isAuthenticated && <AuthButton />}
      </div>

      <Map
        ref={mapRef}
        initialViewState={TOKYO_INITIAL_VIEW_STATE}
        // Update our tracked view state when the user moves the map
        onMove={evt => setCurrentViewState(evt.viewState)}
        mapStyle={mapStyle}
        style={{ width: '100%', height: '100%' }}
        maxBounds={TOKYO_BOUNDS}
        minZoom={9}
        maxZoom={19}
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
