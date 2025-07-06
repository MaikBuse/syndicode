import * as grpc from '@grpc/grpc-js';

if (!process.env.WEB_PROXY_API_KEY) {
  throw new Error("FATAL: WEB_PROXY_API_KEY environment variable is not set.");
}

if (!process.env.WEB_JWT_SECRET) {
  throw new Error("FATAL: WEB_JWT_SECRET environment variable is not set.");
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
  grpcServerUrl: process.env.WEB_GRPC_SERVER_URL || 'api.syndicode.dev',

  // The API key from the environment
  proxyApiKey: process.env.WEB_PROXY_API_KEY || 'super-secret-api-key',

  // The dynamically chosen gRPC credentials object
  grpcCredentials: grpcCredentials,

  jwt_secret: process.env.WEB_JWT_SECRET || 'super-secret-jwt'
};
