import type { EconomyRepository } from '@/domain/economy/economy-repository';
import type { 
  Corporation, 
  QueryBuildingsFilters, 
  QueryBuildingsResult,
  QueryBusinessesFilters,
  QueryBusinessesResult,
  QueryBusinessListingsFilters,
  QueryBusinessListingsResult,
  AcquireBusinessResult
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

  /**
   * The "Get Business Listings" use case.
   * It takes filter data and uses the repository to fetch the results.
   */
  async getBusinessListings(filters: QueryBusinessListingsFilters, ipAddress: string, jwt: string): Promise<QueryBusinessListingsResult> {
    return this.economyRepository.queryBusinessListings(filters, ipAddress, jwt);
  }

  /**
   * The "Acquire Listed Business" use case.
   * It takes a business listing UUID and uses the repository to acquire it.
   */
  async acquireListedBusiness(businessListingUuid: string, ipAddress: string, jwt: string): Promise<AcquireBusinessResult> {
    return this.economyRepository.acquireListedBusiness(businessListingUuid, ipAddress, jwt);
  }

}

// Dependency Injection: Create a single instance of the service
// with the concrete gRPC repository implementation.
const economyService = new EconomyService(new GrpcEconomyRepository());

export default economyService;
