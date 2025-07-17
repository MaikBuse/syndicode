import type { AuthRepository } from '@/domain/auth/auth-repository';
import type { User, UserCredentials, UserRegistration, VerificationInfo } from '@/domain/auth/auth.types';
import * as grpc from '@grpc/grpc-js';

import { CallContext } from './types';
import { InvalidCredentialsError, UnauthenticatedError, UniqueConstraint, UnknownAuthError, UserInactiveError, VerificationCodeFalse } from '@/domain/auth/auth.error';
import { getAuthServiceClient } from '@/lib/grpc/auth-client';
import { GetCurrentUserRequest, LoginRequest, RegisterRequest, ResendVerificationEmailRequest, VerifyUserRequest } from '@/lib/grpc/generated/interface/v1/auth_pb';

export class GrpcAuthRepository implements AuthRepository {
  private client = getAuthServiceClient();

  async register(data: UserRegistration, ipAddress: string): Promise<{ userUuid: string }> {
    return new Promise((resolve, reject) => {
      const request = new RegisterRequest();
      request.setUserName(data.userName);
      request.setUserPassword(data.userPassword);
      request.setEmail(data.email);
      request.setCorporationName(data.corporationName);

      const metadata = new grpc.Metadata();
      const customContext: CallContext = { ipAddress };
      const callOptions: grpc.CallOptions & { customContext: CallContext } = {
        customContext: customContext,
      };

      // The error handling now lives inside the callback
      this.client.register(request, metadata, callOptions, (error, response) => {
        if (error) {
          console.log('gRPC registration error:', {
            code: error.code,
            message: error.message,
            details: error.details,
            metadata: error.metadata
          });
          
          switch (error.code) {
            case grpc.status.ALREADY_EXISTS:
              return reject(new UniqueConstraint(error.details));

            case grpc.status.FAILED_PRECONDITION:
              return reject(new UserInactiveError(error.details));

            default:
              console.log('Unknown error code:', error.code, 'Expected codes:', {
                ALREADY_EXISTS: grpc.status.ALREADY_EXISTS,
                FAILED_PRECONDITION: grpc.status.FAILED_PRECONDITION
              });
              return reject(new UnknownAuthError("An unexpected error occurred during registration."));
          }
        }

        if (response) {
          resolve({ userUuid: response.getUserUuid() });
        } else {
          // This case is unlikely but good to handle
          reject(new UnknownAuthError("Received an empty response from the server."));
        }
      });
    });
  }

  async getCurrentUser(ipAddress: string, jwt: string): Promise<User> {
    return new Promise((resolve, reject) => {
      const request = new GetCurrentUserRequest();

      const metadata = new grpc.Metadata();

      const ctx: CallContext = { ipAddress, jwt };

      const callOptions: grpc.CallOptions & { customContext: CallContext } = {
        customContext: ctx,
      };

      this.client.getCurrentUser(request, metadata, callOptions, (error, response) => {
        if (error) {
          switch (error.code) {
            case grpc.status.FAILED_PRECONDITION:
              reject(new UserInactiveError());
              break;

            case grpc.status.UNAUTHENTICATED:
              reject(new UnauthenticatedError());
              break;

            default:
              reject(new UnknownAuthError());
              break;
          }
        }

        if (response) {
          resolve(
            {
              uuid: response.getUserUuid(),
              name: response.getUserName(),
              email: response.getEmail(),
              role: response.getUserRole().toString()
            }
          );
        } else {
          // This case is unlikely but good to handle
          reject(new UnknownAuthError("Received an empty response from the server."));
        }
      });
    });
  }

  async login(credentials: UserCredentials, ipAddress: string): Promise<{ jwt: string }> {
    return new Promise((resolve, reject) => {
      const request = new LoginRequest();
      request.setUserName(credentials.userName);
      request.setUserPassword(credentials.userPassword);

      const metadata = new grpc.Metadata();

      const customContext: CallContext = { ipAddress };

      const callOptions: grpc.CallOptions & { customContext: CallContext } = {
        customContext: customContext,
      };

      this.client.login(request, metadata, callOptions, (error, response) => {
        if (error) {
          switch (error.code) {
            case grpc.status.FAILED_PRECONDITION:
              reject(new UserInactiveError());
              return;

            case grpc.status.INVALID_ARGUMENT:
              reject(new InvalidCredentialsError());
              return;

            default:
              console.error('Unexpected gRPC error code:', error.code, 'Message:', error.message);
              reject(new UnknownAuthError(`Unexpected error code: ${error.code}, Message: ${error.message}`));
              return;
          }
        }

        if (response) {
          resolve({ jwt: response.getJwt() });
        } else {
          // This case is unlikely but good to handle
          reject(new UnknownAuthError("Received an empty response from the server."));
        }
      });
    });
  }


  async verifyUser(data: VerificationInfo, ipAddress: string): Promise<{ userUuid: string }> {
    return new Promise((resolve, reject) => {
      const request = new VerifyUserRequest();
      request.setUserName(data.userName);
      request.setCode(data.code);

      const metadata = new grpc.Metadata();

      const customContext: CallContext = { ipAddress };

      const callOptions: grpc.CallOptions & { customContext: CallContext } = {
        customContext: customContext,
      };

      this.client.verifyUser(request, metadata, callOptions, (error, response) => {
        if (error) {
          switch (error.code) {
            case grpc.status.DEADLINE_EXCEEDED:
              reject(new UserInactiveError());
              break;

            case grpc.status.INVALID_ARGUMENT:
              reject(new VerificationCodeFalse());
              break;

            default:
              reject(new UnknownAuthError());
              break;
          }
        }

        if (response) {
          resolve({ userUuid: response.getUserUuid() });
        } else {
          // This case is unlikely but good to handle
          reject(new UnknownAuthError("Received an empty response from the server."));
        }
      });
    });
  }

  async resendVerificationEmail(userName: string, ipAddress: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = new ResendVerificationEmailRequest();
      request.setUserName(userName);

      const metadata = new grpc.Metadata();

      const customContext: CallContext = { ipAddress };

      const callOptions: grpc.CallOptions & { customContext: CallContext } = {
        customContext: customContext,
      };

      this.client.resendVerificationEmail(request, metadata, callOptions, (error, response) => {
        if (error) {
          switch (error.code) {
            case grpc.status.FAILED_PRECONDITION:
              reject(new UserInactiveError());
              break;
            case grpc.status.UNAUTHENTICATED:
              reject(new InvalidCredentialsError());
              break;
            default:
              reject(new UnknownAuthError());
              break;
          }
        }

        if (response) {
          resolve();
        } else {
          // This case is unlikely but good to handle
          reject(new UnknownAuthError("Received an empty response from the server."));
        }
      });
    });
  }
}


