/**
 * Defines a container for dynamic, per-request data that can be
 * passed to the gRPC interceptors via the call options.
 */
export interface CallContext {
  ipAddress?: string;
  jwt?: string;
}
