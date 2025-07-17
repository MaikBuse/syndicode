export type Corporation = {
  uuid: string;
  name: string;
  cash_balance: number,
};

// Represents the filters that can be used to query buildings.
export type QueryBuildingsFilters = {
  owningCorporationUuid?: string | null;
  owningBusinessUuid?: string | null;
  minLon?: number | null;
  maxLon?: number | null;
  minLat?: number | null;
  maxLat?: number | null;
  limit?: number | null;
};

// Represents a single building's details.
export type BuildingDetails = {
  gmlId: string;
  longitude: number;
  latitude: number;
};

// Represents the complete result of a building query.
export type QueryBuildingsResult = {
  gameTick: number;
  buildings: BuildingDetails[];
  totalCount: number;
};

// Represents the filters that can be used to query businesses.
export type QueryBusinessesFilters = {
  owningCorporationUuid?: string | null;
  marketUuid?: string | null;
  minOperationalExpenses?: number | null;
  maxOperationalExpenses?: number | null;
  sortBy?: BusinessSortBy | null;
  sortDirection?: SortDirection | null;
  limit?: number | null;
  offset?: number | null;
};

export enum BusinessSortBy {
  UNSPECIFIED = 0,
  BUSINESS_NAME = 1,
  BUSINESS_OPERATION_EXPENSES = 2,
  BUSINESS_MARKET_VOLUME = 3,
}

export enum SortDirection {
  UNSPECIFIED = 0,
  ASCENDING = 1,
  DESCENDING = 2,
}

// Represents a single business's details.
export type BusinessDetails = {
  businessUuid: string;
  businessName: string;
  owningCorporationUuid?: string | null;
  marketUuid: string;
  operationalExpenses: number;
  headquarterBuildingUuid: string;
  headquarterBuildingGmlId: string;
  headquarterLongitude: number;
  headquarterLatitude: number;
};

// Represents the complete result of a business query.
export type QueryBusinessesResult = {
  businesses: BusinessDetails[];
  totalCount: number;
};
