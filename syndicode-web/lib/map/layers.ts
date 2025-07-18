import { MVTLayer } from '@deck.gl/geo-layers';
import { GeoJsonLayer, ArcLayer } from '@deck.gl/layers';
import type { TokyoBoundaryGeoJSON, BuildingProperties } from './types';
import type { BusinessDetails, BuildingDetails, BusinessListingDetails } from '@/domain/economy/economy.types';
import { TILE_URL } from './constants';

// Pre-calculate animation values to avoid Math.sin calculations on every render
const calculateBoundaryAnimationValues = (time: number) => {
  const colorPulse = Math.sin(time * 3) * 0.15 + 0.85; // Reduced range: 0.7 to 1.0
  const widthPulse = Math.sin(time * 2) * 2;
  const glowColorPulse = Math.sin(time * 2.5 + Math.PI) * 0.1 + 0.4; // Reduced range: 0.3 to 0.5
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
  selectedUpdateTrigger: string = 'empty',
  allListedBusinessGmlIds: Set<string> = new Set(),
  allListedUpdateTrigger: string = 'empty'
) => {
  // Pre-defined color arrays to avoid repeated array creation
  const ownedFill: [number, number, number, number] = [255, 150, 30, 255]; // Orange/gold for owned headquarters
  const selectedFill: [number, number, number, number] = [147, 51, 234, 255]; // Purple for selected business buildings
  const listedFill: [number, number, number, number] = [0, 255, 150, 255]; // Green for selected listed business headquarters
  const notOwnedFill: [number, number, number, number] = [150, 150, 150, 255]; // Gray for not owned

  const ownedLine: [number, number, number] = [255, 170, 50]; // Orange outline
  const selectedLine: [number, number, number] = [168, 85, 247]; // Purple outline
  const listedLine: [number, number, number] = [0, 255, 150]; // Green outline
  const notOwnedLine: [number, number, number] = [60, 60, 60]; // Gray outline

  return new MVTLayer({
    id: 'buildings',
    data: TILE_URL,
    minZoom: 12,
    maxZoom: 18,
    extruded: true,
    pickable: true,
    autoHighlight: true,
    beforeId: 'place_city',
    getElevation: (d: { properties: BuildingProperties }) => d.properties.cal_height_m,
    getFillColor: (d: { properties: BuildingProperties }) => {
      const gmlId = d.properties.gml_id;
      // Priority: All listed business headquarters (green) > Selected business buildings (purple) > Owned headquarters (gold) > Not owned (gray)
      if (allListedBusinessGmlIds.has(gmlId)) return listedFill;
      if (selectedBusinessBuildingGmlIds.has(gmlId)) return selectedFill;
      if (ownedBusinessGmlIds.has(gmlId)) return ownedFill;
      return notOwnedFill;
    },
    getLineColor: (d: { properties: BuildingProperties }) => {
      const gmlId = d.properties.gml_id;
      if (allListedBusinessGmlIds.has(gmlId)) return listedLine;
      if (selectedBusinessBuildingGmlIds.has(gmlId)) return selectedLine;
      if (ownedBusinessGmlIds.has(gmlId)) return ownedLine;
      return notOwnedLine;
    },
    lineWidthMinPixels: 1,
    lineWidthMaxPixels: 3,
    getLineWidth: (d: { properties: BuildingProperties }) => {
      const gmlId = d.properties.gml_id;
      if (allListedBusinessGmlIds.has(gmlId)) return 2; // Medium outline for all listed business headquarters
      if (selectedBusinessBuildingGmlIds.has(gmlId)) return 3; // Thicker outline for selected business buildings
      if (ownedBusinessGmlIds.has(gmlId)) return 2; // Medium outline for owned headquarters
      return 1; // Thin outline for not owned
    },
    updateTriggers: {
      getLineColor: [updateTrigger, selectedUpdateTrigger, allListedUpdateTrigger],
      getFillColor: [updateTrigger, selectedUpdateTrigger, allListedUpdateTrigger],
      getLineWidth: [updateTrigger, selectedUpdateTrigger, allListedUpdateTrigger]
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


// Generic hexagon layer configuration
type HexagonConfig = {
  id: string;
  baseRadius: number;
  height: number;
  fillColor: [number, number, number];
  lineColorBase: [number, number, number];
  animationSpeed: number;
  pulseIntensity: number;
};

// Generic function to create hexagon layers for different business types
const createBusinessHexLayer = <T extends { headquarterLongitude: number; headquarterLatitude: number }>(
  businesses: T[],
  time: number,
  zoom: number,
  config: HexagonConfig
) => {
  // Convert businesses to hexagon geometries centered on their exact coordinates
  const hexagonData = businesses.flatMap((business, index) => {
    const center: [number, number] = [business.headquarterLongitude, business.headquarterLatitude];

    // Each business has its own animation offset based on its index for color pulsing
    const animationOffset = index * 0.7;

    // Zoom-dependent radius: larger at lower zoom levels, smaller at higher zoom levels
    const zoomFactor = Math.pow(0.7, zoom - 12); // Exponential scaling
    const radiusInDegrees = config.baseRadius * zoomFactor;

    // Generate hexagon vertices
    const vertices = generateHexagonVertices(center, radiusInDegrees);

    // Color with pulsing effect
    const linePulse = Math.sin(time * config.animationSpeed + animationOffset) * config.pulseIntensity + 0.8;
    const lineColor: [number, number, number, number] = [
      Math.floor(config.lineColorBase[0] * linePulse),
      Math.floor(config.lineColorBase[1] * linePulse),
      Math.floor(config.lineColorBase[2] * linePulse),
      255
    ];

    return {
      polygon: [vertices],
      height: config.height,
      fillColor: config.fillColor,
      lineColor,
      animationOffset,
      business,
      radiusInDegrees
    };
  });

  return new GeoJsonLayer({
    id: config.id,
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

// Specific function for owned headquarters using the generic layer
export const createHeadquarterHexLayer = (
  businesses: BusinessDetails[],
  time: number,
  zoom: number
) => {
  return createBusinessHexLayer(businesses, time, zoom, {
    id: 'headquarters-hex',
    baseRadius: 0.0018,
    height: 2000,
    fillColor: [255, 215, 0], // Light gold
    lineColorBase: [255, 215, 0], // Light gold
    animationSpeed: 4,
    pulseIntensity: 0.2
  });
};

// Specific function for listed businesses using the generic layer
export const createListedBusinessHexLayer = (
  listings: BusinessListingDetails[],
  time: number,
  zoom: number
) => {
  return createBusinessHexLayer(listings, time, zoom, {
    id: 'listed-businesses-hex',
    baseRadius: 0.0018,
    height: 1500,
    fillColor: [0, 255, 150], // Bright green
    lineColorBase: [0, 255, 150], // Bright green
    animationSpeed: 3,
    pulseIntensity: 0.3
  });
};

// Create arc layer connecting headquarters to owned buildings
export const createHeadquarterArcLayer = (
  selectedBusiness: BusinessDetails,
  selectedBusinessBuildings: BuildingDetails[],
  time: number
) => {
  const arcData = selectedBusinessBuildings.map((building) => {
    return {
      sourcePosition: [selectedBusiness.headquarterLongitude, selectedBusiness.headquarterLatitude],
      targetPosition: [building.longitude, building.latitude],
      buildingId: building.gmlId
    };
  });

  return new ArcLayer({
    id: 'headquarters-arcs',
    data: arcData,
    pickable: true,
    getWidth: 4,
    getSourcePosition: (d: { sourcePosition: [number, number] }) => d.sourcePosition,
    getTargetPosition: (d: { targetPosition: [number, number] }) => d.targetPosition,
    getSourceColor: [255, 215, 0, 75], // Gold from headquarters
    getTargetColor: [147, 51, 234, 50], // Purple to buildings
    widthMinPixels: 2,
    widthMaxPixels: 8,
    // Arc height controls the curvature - needs to be much larger for visibility
    getHeight: 1.0, // Much larger height for visible arc curvature
    // Alternative: animated height
    // getHeight: (d: any) => Math.sin(time * 2 + d.sourcePosition[0] * 100) * 0.05 + 0.1,
    updateTriggers: {
      getHeight: [time]
    }
  });
};

// Create arc layer connecting listed business headquarters to owned buildings
export const createListedBusinessArcLayer = (
  selectedListedBusiness: BusinessListingDetails,
  selectedBusinessBuildings: BuildingDetails[],
  time: number
) => {
  const arcData = selectedBusinessBuildings.map((building) => {
    return {
      sourcePosition: [selectedListedBusiness.headquarterLongitude, selectedListedBusiness.headquarterLatitude],
      targetPosition: [building.longitude, building.latitude],
      buildingId: building.gmlId
    };
  });

  return new ArcLayer({
    id: 'listed-business-arcs',
    data: arcData,
    pickable: true,
    getWidth: 4,
    getSourcePosition: (d: { sourcePosition: [number, number] }) => d.sourcePosition,
    getTargetPosition: (d: { targetPosition: [number, number] }) => d.targetPosition,
    getSourceColor: [0, 255, 150, 75], // Green from listed business headquarters
    getTargetColor: [147, 51, 234, 50], // Purple to buildings
    widthMinPixels: 2,
    widthMaxPixels: 8,
    // Arc height controls the curvature - needs to be much larger for visibility
    getHeight: 1.0, // Much larger height for visible arc curvature
    updateTriggers: {
      getHeight: [time]
    }
  });
};
