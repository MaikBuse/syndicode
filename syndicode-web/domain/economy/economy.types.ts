export type Corporation = {
  uuid: string;
  user_uuid: string;
  name: string;
  balance: number;
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
  marketName: string;
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

// Represents a business listing (for sale)
export type BusinessListingDetails = {
  listingUuid: string;
  businessUuid: string;
  businessName: string;
  sellerCorporationUuid?: string | null;
  marketUuid: string;
  marketName: string;
  askingPrice: number;
  operationalExpenses: number;
  // Extended fields for map display
  headquarterLongitude: number;
  headquarterLatitude: number;
  headquarterBuildingGmlId: string;
};

// Represents the filters that can be used to query business listings.
export type QueryBusinessListingsFilters = {
  minAskingPrice?: number | null;
  maxAskingPrice?: number | null;
  sellerCorporationUuid?: string | null;
  marketUuid?: string | null;
  minOperationalExpenses?: number | null;
  maxOperationalExpenses?: number | null;
  sortBy?: BusinessListingSortBy | null;
  sortDirection?: SortDirection | null;
  limit?: number | null;
  offset?: number | null;
};

export enum BusinessListingSortBy {
  SORT_BY_UNSPECIFIED = 0,
  PRICE = 1,
  NAME = 2,
  OPERATION_EXPENSES = 3,
  MARKET_VOLUME = 4,
}

// Represents the complete result of a business listing query.
export type QueryBusinessListingsResult = {
  listings: BusinessListingDetails[];
  totalCount: number;
};

// Represents a business entity
export type Business = {
  uuid: string;
  marketUuid: string;
  owningCorporationUuid: string;
  name: string;
  operationalExpenses: number;
};

// Represents the result of acquiring a business (action init response)
export type AcquireBusinessResult = {
  requestUuid: string;
};
