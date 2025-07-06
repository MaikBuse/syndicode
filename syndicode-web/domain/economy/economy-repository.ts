import type { Corporation, QueryBuildingsFilters, QueryBuildingsResult } from './economy.types';

// The contract for any repository that can fetch economy data.
export interface EconomyRepository {
  getCorporation(ipAddress: string, jwt: string): Promise<Corporation>;
  queryBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult>;
}
