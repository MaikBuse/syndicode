import type { ViewState } from 'react-map-gl/maplibre';

export const TOKYO_BOUNDS: [[number, number], [number, number]] = [
  [139.545306, 35.508716], // Southwest coordinates
  [139.935272, 35.832124]  // Northeast coordinates
];

export const TOKYO_INITIAL_VIEW_STATE: ViewState = {
  longitude: 139.740289,
  latitude: 35.670420,
  zoom: 15,
  pitch: 50,
  bearing: 0,
  padding: { top: 0, bottom: 0, left: 0, right: 0 },
};

export const TILE_URL = 'https://assets.syndicode.dev/tokyo-buildings/{z}/{x}/{y}.pbf';

export const QUERY_ZOOM_LEVEL_THRESHOLD = 15;

export const MAP_STYLE = 'https://tiles.stadiamaps.com/styles/alidade_smooth_dark.json';