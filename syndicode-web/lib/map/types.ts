import type { FeatureCollection, Polygon } from 'geojson';

export interface TokyoBoundaryProperties {
  buildings_analyzed: number;
  total_corner_points: number;
  points_used_for_hull: number;
  alpha_value: number;
  sampling_cell_size: number;
  smoothing_tolerance: number;
  buffer_distance: number;
}

export type TokyoBoundaryGeoJSON = FeatureCollection<Polygon, TokyoBoundaryProperties>;

export interface BuildingProperties {
  gml_id: string;
  cal_height_m: number;
}

export interface BuildingQueryPayload {
  owningCorporationUuid: string;
  minLon: number;
  maxLon: number;
  minLat: number;
  maxLat: number;
  limit: number;
}