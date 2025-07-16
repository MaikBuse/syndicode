import type { 
  Corporation, 
  QueryBuildingsFilters, 
  QueryBuildingsResult,
  QueryBusinessesFilters,
  QueryBusinessesResult
} from './economy.types';

// The contract for any repository that can fetch economy data.
export interface EconomyRepository {
  getCorporation(ipAddress: string, jwt: string): Promise<Corporation>;
  queryBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult>;
  queryBusinesses(filters: QueryBusinessesFilters, ipAddress: string, jwt: string): Promise<QueryBusinessesResult>;
}
