import { MVTLayer } from '@deck.gl/geo-layers';
import { GeoJsonLayer } from '@deck.gl/layers';
import type { TokyoBoundaryGeoJSON, BuildingProperties } from './types';
import type { BusinessDetails } from '@/domain/economy/economy.types';
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

export const createBuildingsLayer = (ownedBusinessGmlIds: Set<string>, updateTrigger: string) => {
  // Pre-defined color arrays to avoid repeated array creation
  // Lighter orange: brighter than the previous dark orange but not as bright as gold
  const ownedFill: [number, number, number, number] = [255, 150, 30, 255];
  const notOwnedFill: [number, number, number, number] = [150, 150, 150, 255];

  const ownedLine: [number, number, number] = [255, 170, 50];
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
      return ownedBusinessGmlIds.has(d.properties.gml_id) ? ownedFill : notOwnedFill;
    },
    getLineColor: (d: { properties: BuildingProperties }) => {
      return ownedBusinessGmlIds.has(d.properties.gml_id) ? ownedLine : notOwnedLine;
    },
    lineWidthMinPixels: 1,
    lineWidthMaxPixels: 3,
    getLineWidth: (d: { properties: BuildingProperties }) => {
      return ownedBusinessGmlIds.has(d.properties.gml_id) ? 2 : 1;
    },
    updateTriggers: {
      getLineColor: [updateTrigger],
      getFillColor: [updateTrigger],
      getLineWidth: [updateTrigger]
    },
  });
};

// Generate hexagon vertices around a center point
const generateHexagonVertices = (center: [number, number], radius: number): [number, number][] => {
  const vertices: [number, number][] = [];
  for (let i = 0; i < 6; i++) {
    const angle = (i * Math.PI) / 3; // 60-degree intervals
    const x = center[0] + radius * Math.cos(angle);
    const y = center[1] + radius * Math.sin(angle);
    vertices.push([x, y]);
  }
  return vertices;
};

export const createHeadquarterHexLayer = (businesses: BusinessDetails[], time: number, zoom: number) => {
  // Convert businesses to hexagon geometries centered on their exact coordinates
  const hexagonData = businesses.flatMap((business, index) => {
    const center: [number, number] = [business.headquarterLongitude, business.headquarterLatitude];

    // Zoom-dependent radius: larger at lower zoom levels, smaller at higher zoom levels
    // At zoom 12: ~200m radius, at zoom 15: ~100m radius, at zoom 18: ~50m radius
    const baseRadius = 0.0018; // Base radius in decimal degrees
    const zoomFactor = Math.pow(0.7, zoom - 12); // Exponential scaling
    const radiusInDegrees = baseRadius * zoomFactor;
    const vertices = generateHexagonVertices(center, radiusInDegrees);

    // Increased height for better visibility
    const height = 2000;

    // Each headquarters has its own animation offset based on its index for color pulsing
    const animationOffset = index * 0.7;

    // Improved magenta color with better contrast
    const fillColor: [number, number, number] = [200, 50, 180]; // Brighter magenta

    // Bright magenta outline with pulsing effect
    const linePulse = Math.sin(time * 4 + animationOffset) * 0.2 + 0.8;
    const lineColor: [number, number, number, number] = [Math.floor(255 * linePulse), 80, 220, 255];

    return {
      polygon: [vertices],
      height,
      fillColor,
      lineColor,
      animationOffset,
      business,
      radiusInDegrees
    };
  });

  return new GeoJsonLayer({
    id: 'headquarters-hex',
    data: {
      type: 'FeatureCollection',
      features: hexagonData.map((hex, index) => ({
        type: 'Feature',
        geometry: {
          type: 'Polygon',
          coordinates: hex.polygon
        },
        properties: {
          height: hex.height,
          fillColor: hex.fillColor,
          lineColor: hex.lineColor,
          index,
          animationOffset: hex.animationOffset,
          business: hex.business
        }
      }))
    },
    pickable: true,
    extruded: true,
    wireframe: true,
    filled: true,
    stroked: true,
    getElevation: (d: { properties: { height: number } }) => d.properties.height,
    getFillColor: (d: { properties: { animationOffset: number; fillColor: [number, number, number] } }) => {
      // Calculate dynamic transparency based on zoom and time
      const animationOffset = d.properties.animationOffset;
      const basePulse = Math.sin(time * 3 + animationOffset) * 0.2 + 0.8; // Smoother pulse: 0.6 to 1

      // Improved transparency scaling: more visible at low zoom, less at high zoom
      // At zoom 12: ~50% opacity, at zoom 15: ~25% opacity, at zoom 18: ~10% opacity
      const minOpacity = 0.1;
      const maxOpacity = 0.5;
      const zoomRange = 18 - 12; // Total zoom range
      const normalizedZoom = Math.max(0, Math.min(1, (zoom - 12) / zoomRange));
      const zoomFactor = maxOpacity - (normalizedZoom * (maxOpacity - minOpacity));

      const alpha = Math.floor(255 * basePulse * zoomFactor);

      const [r, g, b] = d.properties.fillColor;
      return [r, g, b, alpha];
    },
    getLineColor: (d: { properties: { lineColor: [number, number, number, number] } }) => d.properties.lineColor,
    lineWidthMinPixels: 1,
    lineWidthMaxPixels: 6,
    getLineWidth: () => {
      // Zoom-dependent line width: thicker at lower zoom, thinner at higher zoom
      return Math.max(1, Math.min(4, 6 - (zoom - 12) * 0.4));
    },
    updateTriggers: {
      getFillColor: [time, zoom],
      getLineColor: [time]
    },
  });
};
