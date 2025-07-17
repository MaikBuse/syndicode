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

export const createBuildingsLayer = (ownedBusinessGmlIds: Set<string>, time?: number) => {
  // Pre-calculate animation values to avoid repeated trigonometric calculations
  const hasTime = time !== undefined;
  const fillPulse = hasTime ? Math.sin(time * 4) * 0.2 + 0.8 : 0;
  const linePulse = hasTime ? Math.sin(time * 3) * 0.3 + 0.7 : 0;
  const animatedFillGreen = hasTime ? Math.floor(215 * fillPulse) : 0;
  const animatedLineGreen = hasTime ? Math.floor(223 * linePulse) : 0;

  // Pre-defined color arrays to avoid repeated array creation
  const ownedStaticFill: [number, number, number, number] = [255, 215, 0, 255];
  const ownedAnimatedFill: [number, number, number, number] = [255, animatedFillGreen, 0, 255];
  const notOwnedFill: [number, number, number, number] = [150, 150, 150, 255];

  const ownedStaticLine: [number, number, number] = [255, 223, 0];
  const ownedAnimatedLine: [number, number, number, number] = [255, animatedLineGreen, 0, 255];
  const notOwnedLine: [number, number, number] = [60, 60, 60];

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
      const isBusinessHeadquarter = ownedBusinessGmlIds.has(d.properties.gml_id);
      if (isBusinessHeadquarter) {
        return hasTime ? ownedAnimatedFill : ownedStaticFill;
      }
      return notOwnedFill;
    },
    getLineColor: (d: { properties: BuildingProperties }) => {
      const isBusinessHeadquarter = ownedBusinessGmlIds.has(d.properties.gml_id);
      if (isBusinessHeadquarter) {
        return hasTime ? ownedAnimatedLine : ownedStaticLine;
      }
      return notOwnedLine;
    },
    lineWidthMinPixels: 1,
    lineWidthMaxPixels: 3,
    getLineWidth: (d: { properties: BuildingProperties }) => {
      return ownedBusinessGmlIds.has(d.properties.gml_id) ? 2 : 1;
    },
    updateTriggers: {
      getLineColor: [ownedBusinessGmlIds, time],
      getFillColor: [ownedBusinessGmlIds, time],
      getLineWidth: [ownedBusinessGmlIds]
    },
  });
};
