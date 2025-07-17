import { MVTLayer } from '@deck.gl/geo-layers';
import { GeoJsonLayer } from '@deck.gl/layers';
import type { TokyoBoundaryGeoJSON, BuildingProperties } from './types';
import type { BusinessDetails } from '@/domain/economy/economy.types';
import { TILE_URL } from './constants';

// Pre-calculate animation values to avoid Math.sin calculations on every render
const calculateBoundaryAnimationValues = (time: number) => {
  const colorPulse = Math.sin(time * 3) * 0.3 + 0.7;
  const widthPulse = Math.sin(time * 2) * 2;
  const glowColorPulse = Math.sin(time * 2.5 + Math.PI) * 0.2 + 0.3;
  const glowWidthPulse = Math.sin(time * 1.5) * 3;

  return {
    boundaryColor: [0, 255, 255, Math.floor(255 * colorPulse)] as [number, number, number, number],
    boundaryWidth: 4 + widthPulse,
    glowColor: [255, 0, 255, Math.floor(255 * glowColorPulse * 0.4)] as [number, number, number, number],
    glowWidth: 8 + glowWidthPulse
  };
};

export const createTokyoBoundaryLayer = (
  tokyoBoundary: TokyoBoundaryGeoJSON,
  time: number
) => {
  const animationValues = calculateBoundaryAnimationValues(time);

  return new GeoJsonLayer({
    id: 'tokyo-boundary',
    data: tokyoBoundary,
    pickable: true,
    stroked: true,
    filled: false,
    lineWidthMinPixels: 3,
    lineWidthMaxPixels: 8,
    getLineColor: animationValues.boundaryColor,
    getLineWidth: animationValues.boundaryWidth,
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
  const animationValues = calculateBoundaryAnimationValues(time);

  return new GeoJsonLayer({
    id: 'tokyo-boundary-glow',
    data: tokyoBoundary,
    pickable: false,
    stroked: true,
    filled: false,
    lineWidthMinPixels: 6,
    lineWidthMaxPixels: 12,
    getLineColor: animationValues.glowColor,
    getLineWidth: animationValues.glowWidth,
    updateTriggers: {
      getLineColor: [time],
      getLineWidth: [time]
    }
  });
};

// Optimized function to create both boundary layers with shared animation calculations
export const createBoundaryLayersWithSharedAnimation = (
  tokyoBoundary: TokyoBoundaryGeoJSON,
  time: number
) => {
  const animationValues = calculateBoundaryAnimationValues(time);

  const boundaryLayer = new GeoJsonLayer({
    id: 'tokyo-boundary',
    data: tokyoBoundary,
    pickable: true,
    stroked: true,
    filled: false,
    lineWidthMinPixels: 3,
    lineWidthMaxPixels: 8,
    getLineColor: animationValues.boundaryColor,
    getLineWidth: animationValues.boundaryWidth,
    updateTriggers: {
      getLineColor: [time],
      getLineWidth: [time]
    }
  });

  const glowLayer = new GeoJsonLayer({
    id: 'tokyo-boundary-glow',
    data: tokyoBoundary,
    pickable: false,
    stroked: true,
    filled: false,
    lineWidthMinPixels: 6,
    lineWidthMaxPixels: 12,
    getLineColor: animationValues.glowColor,
    getLineWidth: animationValues.glowWidth,
    updateTriggers: {
      getLineColor: [time],
      getLineWidth: [time]
    }
  });

  return [boundaryLayer, glowLayer];
};

export const createBuildingsLayer = (
  ownedBusinessGmlIds: Set<string>, 
  updateTrigger: string, 
  selectedBusinessBuildingGmlIds: Set<string> = new Set(), 
  selectedUpdateTrigger: string = 'empty'
) => {
  // Pre-defined color arrays to avoid repeated array creation
  const ownedFill: [number, number, number, number] = [255, 150, 30, 255]; // Orange/gold for owned headquarters
  const selectedFill: [number, number, number, number] = [147, 51, 234, 255]; // Purple for selected business buildings
  const notOwnedFill: [number, number, number, number] = [150, 150, 150, 255]; // Gray for not owned

  const ownedLine: [number, number, number] = [255, 170, 50]; // Orange outline
  const selectedLine: [number, number, number] = [168, 85, 247]; // Purple outline
  const notOwnedLine: [number, number, number] = [60, 60, 60]; // Gray outline

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
      const gmlId = d.properties.gml_id;
      // Priority: Selected business buildings (purple) > Owned headquarters (gold) > Not owned (gray)
      if (selectedBusinessBuildingGmlIds.has(gmlId)) return selectedFill;
      if (ownedBusinessGmlIds.has(gmlId)) return ownedFill;
      return notOwnedFill;
    },
    getLineColor: (d: { properties: BuildingProperties }) => {
      const gmlId = d.properties.gml_id;
      if (selectedBusinessBuildingGmlIds.has(gmlId)) return selectedLine;
      if (ownedBusinessGmlIds.has(gmlId)) return ownedLine;
      return notOwnedLine;
    },
    lineWidthMinPixels: 1,
    lineWidthMaxPixels: 3,
    getLineWidth: (d: { properties: BuildingProperties }) => {
      const gmlId = d.properties.gml_id;
      if (selectedBusinessBuildingGmlIds.has(gmlId)) return 3; // Thicker outline for selected business buildings
      if (ownedBusinessGmlIds.has(gmlId)) return 2; // Medium outline for owned headquarters
      return 1; // Thin outline for not owned
    },
    updateTriggers: {
      getLineColor: [updateTrigger, selectedUpdateTrigger],
      getFillColor: [updateTrigger, selectedUpdateTrigger],
      getLineWidth: [updateTrigger, selectedUpdateTrigger]
    },
  });
};

// Cached hexagon geometry for performance
const hexagonGeometryCache = new Map<string, [number, number][]>();

// Generate hexagon vertices around a center point with caching
const generateHexagonVertices = (center: [number, number], radius: number): [number, number][] => {
  // Create a cache key based on center and radius (rounded for cache efficiency)
  const cacheKey = `${center[0].toFixed(6)}_${center[1].toFixed(6)}_${radius.toFixed(6)}`;

  if (hexagonGeometryCache.has(cacheKey)) {
    return hexagonGeometryCache.get(cacheKey)!;
  }

  const vertices: [number, number][] = [];
  for (let i = 0; i < 6; i++) {
    const angle = (i * Math.PI) / 3; // 60-degree intervals
    const x = center[0] + radius * Math.cos(angle);
    const y = center[1] + radius * Math.sin(angle);
    vertices.push([x, y]);
  }

  // Cache the result and implement simple LRU by limiting cache size
  if (hexagonGeometryCache.size > 1000) {
    const firstKey = hexagonGeometryCache.keys().next().value;
    if (firstKey) {
      hexagonGeometryCache.delete(firstKey);
    }
  }
  hexagonGeometryCache.set(cacheKey, vertices);

  return vertices;
};

// Rotate vertices around a center point
const rotateVertices = (vertices: [number, number][], center: [number, number], angle: number): [number, number][] => {
  const cos = Math.cos(angle);
  const sin = Math.sin(angle);
  
  return vertices.map(([x, y]) => {
    // Translate to origin
    const translatedX = x - center[0];
    const translatedY = y - center[1];
    
    // Rotate
    const rotatedX = translatedX * cos - translatedY * sin;
    const rotatedY = translatedX * sin + translatedY * cos;
    
    // Translate back
    return [rotatedX + center[0], rotatedY + center[1]] as [number, number];
  });
};

export const createHeadquarterHexLayer = (
  businesses: BusinessDetails[], 
  time: number, 
  zoom: number,
  selectedBusiness: BusinessDetails | null = null
) => {
  // Convert businesses to hexagon geometries centered on their exact coordinates
  const hexagonData = businesses.flatMap((business, index) => {
    const center: [number, number] = [business.headquarterLongitude, business.headquarterLatitude];

    // Check if this business is selected
    const isSelected = selectedBusiness?.businessUuid === business.businessUuid;

    // Each headquarters has its own animation offset based on its index for color pulsing
    const animationOffset = index * 0.7;

    // Rotation angle for selected hexagon
    const rotationAngle = isSelected ? time * 2 : 0; // Rotate selected hexagon

    // Zoom-dependent radius: larger at lower zoom levels, smaller at higher zoom levels
    // At zoom 12: ~200m radius, at zoom 15: ~100m radius, at zoom 18: ~50m radius
    const baseRadius = 0.0018; // Base radius in decimal degrees
    const zoomFactor = Math.pow(0.7, zoom - 12); // Exponential scaling
    const radiusInDegrees = baseRadius * zoomFactor;
    
    // Generate base hexagon vertices
    const baseVertices = generateHexagonVertices(center, radiusInDegrees);
    
    // Apply rotation if selected
    const vertices = isSelected ? 
      rotateVertices(baseVertices, center, rotationAngle) : 
      baseVertices;

    // Increased height for better visibility
    const height = 2000;

    // Light gold color for headquarters
    const fillColor: [number, number, number] = [255, 215, 0]; // Light gold

    // Light gold outline with pulsing effect
    const linePulse = Math.sin(time * 4 + animationOffset) * 0.2 + 0.8;
    const lineColor: [number, number, number, number] = [Math.floor(255 * linePulse), Math.floor(215 * linePulse), 0, 255];

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
      getFillColor: [time, zoom, selectedBusiness?.businessUuid],
      getLineColor: [time, selectedBusiness?.businessUuid]
    },
  });
};
