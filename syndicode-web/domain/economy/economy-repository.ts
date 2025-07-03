import type { QueryBuildingsFilters, QueryBuildingsResult } from './economy.types';

// The contract for any repository that can fetch economy data.
export interface EconomyRepository {
  queryBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult>;
}
