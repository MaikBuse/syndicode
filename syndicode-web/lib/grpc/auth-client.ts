import { serverConfig } from '@/config/server';
import { contextMetadataInterceptor } from '@/infrastructure/grpc/interceptor';
import { AuthServiceClient } from './generated/interface/v1/auth_grpc_pb';

// Create a singleton instance of the client
// We use a lazy-initialized singleton to avoid creating the client on module load
let clientInstance: AuthServiceClient | null = null;

export const getAuthServiceClient = (): AuthServiceClient => {
  if (!clientInstance) {
    clientInstance = new AuthServiceClient(
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
export * from './generated/interface/v1/auth_pb';
