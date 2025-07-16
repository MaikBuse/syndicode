import type { EconomyRepository } from '@/domain/economy/economy-repository';
import type { 
  Corporation, 
  QueryBuildingsFilters, 
  QueryBuildingsResult,
  QueryBusinessesFilters,
  QueryBusinessesResult
} from '@/domain/economy/economy.types';
import { GrpcEconomyRepository } from '@/infrastructure/grpc/grpc-economy-repository';

class EconomyService {
  constructor(private economyRepository: EconomyRepository) { }
  async getCurrentCorporation(ipAddress: string, jwt: string): Promise<Corporation> {
    return this.economyRepository.getCorporation(ipAddress, jwt);
  }

  /**
   * The "Get Buildings" use case.
   * It takes filter data and uses the repository to fetch the results.
   */
  async getBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult> {
    return this.economyRepository.queryBuildings(filters, ipAddress, jwt);
  }

  /**
   * The "Get Businesses" use case.
   * It takes filter data and uses the repository to fetch the results.
   */
  async getBusinesses(filters: QueryBusinessesFilters, ipAddress: string, jwt: string): Promise<QueryBusinessesResult> {
    return this.economyRepository.queryBusinesses(filters, ipAddress, jwt);
  }

}

// Dependency Injection: Create a single instance of the service
// with the concrete gRPC repository implementation.
const economyService = new EconomyService(new GrpcEconomyRepository());

export default economyService;
