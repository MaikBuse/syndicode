import type { FeatureCollection, Polygon } from 'geojson';

export interface TokyoBoundaryProperties {
  city_code: string;
  name: string;
  state_name: string;
  state_name_jp: string;
  station_name_jp: string | null;
  county_name_jp: string | null;
  city_name_jp: string | null;
  distric_name_jp: string | null;
  population: number;
  num_households: number;
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