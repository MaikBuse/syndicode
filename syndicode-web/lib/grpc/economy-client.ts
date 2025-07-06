import { EconomyServiceClient } from './generated/interface/v1/economy_grpc_pb';
import { contextMetadataInterceptor } from '@/infrastructure/grpc/interceptor';
import { serverConfig } from '@/config/server';

// Create a singleton instance of the client
// We use a lazy-initialized singleton to avoid creating the client on module load
let clientInstance: EconomyServiceClient | null = null;

export const getEconomyServiceClient = (): EconomyServiceClient => {
  if (!clientInstance) {
    clientInstance = new EconomyServiceClient(
      serverConfig.grpcServerUrl,
      serverConfig.grpcCredentials,
      {
        interceptors: [contextMetadataInterceptor]
      }

    );
  }
  return clientInstance;
};

// Also export the generated message types for convenience
export * from './generated/interface/v1/economy_pb';
