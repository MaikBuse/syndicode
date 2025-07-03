import type { EconomyRepository } from '@/domain/economy/economy-repository';
import type { QueryBuildingsFilters, QueryBuildingsResult } from '@/domain/economy/economy.types';
import { GrpcEconomyRepository } from '@/infrastructure/grpc/grpc-economy-repository';

class EconomyService {
  constructor(private economyRepository: EconomyRepository) { }

  /**
   * The "Get Buildings" use case.
   * It takes filter data and uses the repository to fetch the results.
   */
  async getBuildings(filters: QueryBuildingsFilters, ipAddress: string): Promise<QueryBuildingsResult> {
    return this.economyRepository.queryBuildings(filters, ipAddress);
  }
}

// Dependency Injection: Create a single instance of the service
// with the concrete gRPC repository implementation.
const economyService = new EconomyService(new GrpcEconomyRepository());

export default economyService;
