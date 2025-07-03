import * as grpc from '@grpc/grpc-js';

if (!process.env.PROXY_API_KEY) {
  throw new Error("FATAL: PROXY_API_KEY environment variable is not set.");
}

if (!process.env.JWT_SECRET) {
  throw new Error("FATAL: JWT_SECRET environment variable is not set.");
}

// --- Determine gRPC Credentials Based on Environment ---
let grpcCredentials;

if (process.env.NODE_ENV === 'production') {
  // In production, use SSL/TLS credentials.
  // This uses the system's default root certificates.
  grpcCredentials = grpc.credentials.createSsl();
  console.log("gRPC client configured for PRODUCTION (using SSL).");
} else {
  // In development or any other environment, use an insecure connection.
  grpcCredentials = grpc.credentials.createInsecure();
  console.log("gRPC client configured for DEVELOPMENT (insecure).");
}

export const serverConfig = {
  // Your gRPC service address
  grpcUrl: process.env.GRPC_SERVER_URL || 'localhost:50051',

  // The API key from the environment
  proxyApiKey: process.env.PROXY_API_KEY,

  // The dynamically chosen gRPC credentials object
  grpcCredentials: grpcCredentials,

  jwt_secret: process.env.JWT_SECRET
};
