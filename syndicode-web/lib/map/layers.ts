import { MVTLayer } from '@deck.gl/geo-layers';
import { GeoJsonLayer } from '@deck.gl/layers';
import type { TokyoBoundaryGeoJSON, BuildingProperties } from './types';
import { TILE_URL } from './constants';

export const createTokyoBoundaryLayer = (
  tokyoBoundary: TokyoBoundaryGeoJSON,
  time: number
) => {
  return new GeoJsonLayer({
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
  });
};

export const createTokyoBoundaryGlowLayer = (
  tokyoBoundary: TokyoBoundaryGeoJSON,
  time: number
) => {
  return new GeoJsonLayer({
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
  });
};

export const createBuildingsLayer = (ownedBuildingGmlId: Set<string>) => {
  return new MVTLayer({
    id: 'buildings',
    data: TILE_URL,
    minZoom: 12,
    maxZoom: 18,
    extruded: true,
    pickable: true,
    autoHighlight: true,
    getElevation: (d: { properties: BuildingProperties }) => d.properties.cal_height_m,
    getFillColor: (d: { properties: BuildingProperties }) => {
      const isOwned = ownedBuildingGmlId.has(d.properties.gml_id);
      return isOwned ? [255, 0, 128, 255] : [150, 150, 150, 255]; // Owned: Pink, Not Owned: Grey
    },
    updateTriggers: {
      getFillColor: [ownedBuildingGmlId]
    },
    getLineColor: [60, 60, 60],
    lineWidthMinPixels: 1,
  });
};