import * as grpc from '@grpc/grpc-js';

// Import the generated client constructor
import { AuthServiceClient } from './generated/interface/v1/auth_grpc_pb';

const GRPC_SERVER_URL = process.env.GRPC_SERVER_URL || 'localhost:50051';

// Create a singleton instance of the client
// We use a lazy-initialized singleton to avoid creating the client on module load
let clientInstance: AuthServiceClient | null = null;

export const getAuthServiceClient = (): AuthServiceClient => {
  if (!clientInstance) {
    clientInstance = new AuthServiceClient(
      GRPC_SERVER_URL,
      // Use createInsecure for local development without TLS
      // For production, you would use grpc.credentials.createSsl()
      grpc.credentials.createInsecure()
    );
  }
  return clientInstance;
};

// Also export the generated message types for convenience
export * from './generated/interface/v1/auth_pb';
