import type { 
  Corporation, 
  QueryBuildingsFilters, 
  QueryBuildingsResult,
  QueryBusinessesFilters,
  QueryBusinessesResult,
  QueryBusinessListingsFilters,
  QueryBusinessListingsResult,
  AcquireBusinessResult
} from './economy.types';

// The contract for any repository that can fetch economy data.
export interface EconomyRepository {
  getCorporation(ipAddress: string, jwt: string): Promise<Corporation>;
  queryBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult>;
  queryBusinesses(filters: QueryBusinessesFilters, ipAddress: string, jwt: string): Promise<QueryBusinessesResult>;
  queryBusinessListings(filters: QueryBusinessListingsFilters, ipAddress: string, jwt: string): Promise<QueryBusinessListingsResult>;
  acquireListedBusiness(businessListingUuid: string, ipAddress: string, jwt: string): Promise<AcquireBusinessResult>;
}
