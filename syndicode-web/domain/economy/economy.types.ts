// Represents the filters that can be used to query buildings.
export type QueryBuildingsFilters = {
  owningCorporationUuid?: string | null;
  minLon?: number | null;
  maxLon?: number | null;
  minLat?: number | null;
  maxLat?: number | null;
  limit?: number | null;
};

// Represents a single building's details.
export type BuildingDetails = {
  gmlId: string;
};

// Represents the complete result of a building query.
export type QueryBuildingsResult = {
  gameTick: number;
  buildings: BuildingDetails[];
  totalCount: number;
};
