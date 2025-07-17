import * as grpc from '@grpc/grpc-js';
import { serverConfig } from '@/config/server';
import { CallContext } from './types';

/**
 * A unified gRPC interceptor that handles both static and dynamic metadata.
 */
export const contextMetadataInterceptor: grpc.Interceptor = (options, nextCall) => {
  const customContext = (options as grpc.CallOptions & { customContext?: CallContext }).customContext;

  return new grpc.InterceptingCall(nextCall(options), {
    start: function(metadata: grpc.Metadata, listener, next) {
      metadata.set('proxy-api-key', serverConfig.proxyApiKey);

      if (customContext?.ipAddress) {
        metadata.set('proxy-ip-address', customContext.ipAddress);
      }

      if (customContext?.jwt) {
        metadata.set('authorization', `Bearer ${customContext.jwt}`);
      }

      next(metadata, listener);
    },
  });
};
