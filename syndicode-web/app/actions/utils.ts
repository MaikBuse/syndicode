// Extract the IP address
// 'x-forwarded-for' is the standard header for proxies (like Vercel)

import { ServiceError } from "@grpc/grpc-js";

// Fallback to 'x-real-ip' or a default value
export function getClientIp(requestHeaders: Headers): string {
  return requestHeaders.get('x-forwarded-for') || requestHeaders.get('x-real-ip') || '127.0.0.1';
}


// A type guard function to check if an error is a gRPC ServiceError
export function isGrpcError(error: unknown): error is ServiceError {
  return (
    typeof error === 'object' &&
    error !== null &&
    'code' in error &&
    'message' in error
  );
}
