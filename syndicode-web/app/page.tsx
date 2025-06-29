'use client';

import React, { useState, useCallback, useMemo } from 'react';
import { Map, useControl } from 'react-map-gl/maplibre';
import { MapboxOverlay } from '@deck.gl/mapbox';
import type { DeckProps, PickingInfo } from '@deck.gl/core';
import { MVTLayer } from '@deck.gl/geo-layers';
import 'maplibre-gl/dist/maplibre-gl.css';

function DeckGLOverlay(props: DeckProps) {
  const overlay = useControl<MapboxOverlay>(() => {
    return new MapboxOverlay({
      ...props,
      id: 'deckgl-overlay',
      interleaved: true,
    });
  });

  // On every re-render, update the dynamic props like layers and onClick
  overlay.setProps(props);
  return null;
}

const TOKYO_BOUNDS: [[number, number], [number, number]] = [
  [139.3, 35.4], // Southwest coordinates
  [140.1, 35.9]  // Northeast coordinates
];

const TOKYO_INITIAL_VIEW_STATE = {
  longitude: 139.6917,
  latitude: 35.6895,
  zoom: 12, // Adjusted zoom to see buildings initially
  pitch: 50, // Increased pitch for a better 3D view
  bearing: 0
};

const TILE_URL = 'https://syndicode-web-map-assets.s3.eu-central-1.amazonaws.com/tokyo-tiles/{z}/{x}/{y}.pbf';

function App() {
  const stadiaApiKey = process.env.NEXT_PUBLIC_STADIA_API_KEY;
  const mapStyle = `https://tiles.stadiamaps.com/styles/alidade_smooth_dark.json?api_key=${stadiaApiKey}`;

  // State to track which buildings are "owned" or selected
  const [ownedBuildingIds, setOwnedBuildingIds] = useState<string[]>([]);

  const handleLayerClick = useCallback((info: PickingInfo) => {
    if (info && info.object) {
      const clickedId = info.object.properties.gml_id;
      console.log('Clicked building properties:', info.object.properties);

      setOwnedBuildingIds(prevIds =>
        prevIds.includes(clickedId)
          ? prevIds.filter(id => id !== clickedId)
          : [...prevIds, clickedId]
      );
    }
  }, []);

  // Define the layers to be rendered by Deck.gl
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
      getElevation: (d: any) => d.properties.cal_height_m,
      getFillColor: (d: any) => {
        const isOwned = ownedBuildingIds.includes(d.properties.gml_id);
        return isOwned ? [255, 0, 128, 255] : [150, 150, 150, 255];
      },
      updateTriggers: {
        getFillColor: ownedBuildingIds
      },
      getLineColor: [60, 60, 60],
      lineWidthMinPixels: 1,
    })
  ], [ownedBuildingIds]);

  if (!stadiaApiKey) {
    return (
      <div style={{ padding: '20px', fontFamily: 'sans-serif' }}>
        <h2>Configuration Error</h2>
        <p>
          Stadia Maps API key is not configured. Please add{' '}
          <code>NEXT_PUBLIC_STADIA_API_KEY</code> to your{' '}
          <code>.env.local</code> file and restart the development server.
        </p>
      </div>
    );
  }

  return (
    <div style={{ position: 'relative', width: '100vw', height: '100vh' }}>
      <Map
        initialViewState={TOKYO_INITIAL_VIEW_STATE}
        mapStyle={mapStyle}
        style={{ width: '100%', height: '100%' }}
        maxBounds={TOKYO_BOUNDS}
        minZoom={9}
        maxZoom={19}
      >
        <DeckGLOverlay
          layers={layers}
          onClick={handleLayerClick}
          useDevicePixels={true}
          pickingRadius={5}
        />
      </Map>
    </div>
  );
}

export default App;
