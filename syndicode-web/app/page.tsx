'use client';

import React, { useState, useMemo, useEffect, useRef } from 'react';
import { Map, useControl, MapRef } from 'react-map-gl/maplibre';
import type { ViewState } from 'react-map-gl/maplibre';
import { MapboxOverlay } from '@deck.gl/mapbox';
import type { DeckProps } from '@deck.gl/core';
import { MVTLayer } from '@deck.gl/geo-layers';
import { GeoJsonLayer } from '@deck.gl/layers';
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
  [139.545306, 35.508716], // Southwest coordinates
  [139.935272, 35.832124]  // Northeast coordinates
];

const TOKYO_INITIAL_VIEW_STATE: ViewState = {
  longitude: 139.740289,
  latitude: 35.670420,
  zoom: 15,
  pitch: 50,
  bearing: 0,
  padding: { top: 0, bottom: 0, left: 0, right: 0 },
};

const TILE_URL = 'https://assets.syndicode.dev/tokyo-buildings/{z}/{x}/{y}.pbf';

// Import the generated Tokyo boundary
const TOKYO_BOUNDARY = async () => {
  try {
    const response = await fetch('/data/tokyo-boundary.geojson');
    return await response.json();
  } catch (error) {
    console.error('Failed to load Tokyo boundary:', error);
    return null;
  }
};

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

  // Animation state for cyberpunk boundary effect
  const [time, setTime] = useState(0);

  // State for Tokyo boundary
  const [tokyoBoundary, setTokyoBoundary] = useState<any>(null);

  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const corporation = useUserDataStore((state) => state.data?.corporation);

  // Load Tokyo boundary on component mount
  useEffect(() => {
    const loadTokyoBoundary = async () => {
      try {
        const boundary = await TOKYO_BOUNDARY();
        setTokyoBoundary(boundary);
      } catch (error) {
        console.error('Failed to load Tokyo boundary:', error);
      }
    };
    loadTokyoBoundary();
  }, []);

  // Animation loop for cyberpunk effects
  useEffect(() => {
    const animate = () => {
      setTime(prev => prev + 0.01);
      requestAnimationFrame(animate);
    };
    const animationId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animationId);
  }, []);

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

  const layers = useMemo(() => {
    const layersList = [];

    // Add Tokyo boundary layers if data is loaded
    if (tokyoBoundary) {
      // Main Tokyo boundary layer
      layersList.push(
        new GeoJsonLayer({
          id: 'tokyo-boundary',
          data: tokyoBoundary,
          pickable: true,
          stroked: true,
          filled: false,
          lineWidthMinPixels: 3,
          lineWidthMaxPixels: 8,
          getLineColor: () => {
            const pulse = Math.sin(time * 3) * 0.3 + 0.7;
            return [0, 255, 255, Math.floor(255 * pulse)]; // Cyan with alpha pulse
          },
          getLineWidth: () => {
            return 4 + Math.sin(time * 2) * 2; // Animated width
          },
          updateTriggers: {
            getLineColor: [time],
            getLineWidth: [time]
          }
        })
      );

      // Secondary glow layer for Tokyo boundary
      layersList.push(
        new GeoJsonLayer({
          id: 'tokyo-boundary-glow',
          data: tokyoBoundary,
          pickable: false,
          stroked: true,
          filled: false,
          lineWidthMinPixels: 6,
          lineWidthMaxPixels: 12,
          getLineColor: () => {
            const pulse = Math.sin(time * 2.5 + Math.PI) * 0.2 + 0.3;
            return [255, 0, 255, Math.floor(255 * pulse * 0.4)]; // Magenta glow
          },
          getLineWidth: () => {
            return 8 + Math.sin(time * 1.5) * 3;
          },
          updateTriggers: {
            getLineColor: [time],
            getLineWidth: [time]
          }
        })
      );
    }

    // Buildings layer
    layersList.push(
      new MVTLayer({
        id: 'buildings',
        data: TILE_URL,
        minZoom: 10,
        maxZoom: 16,
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
    );

    return layersList;
  }, [ownedBuildingGmlId, time, tokyoBoundary]); // Re-create layers when owned buildings, animation time, or Tokyo boundary change

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
        onMove={evt => {
          console.log('Current zoom level:', evt.viewState.zoom);
          setCurrentViewState(evt.viewState);
        }}
        mapStyle={mapStyle}
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
