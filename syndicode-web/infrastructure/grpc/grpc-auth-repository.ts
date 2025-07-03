import type { AuthRepository } from '@/domain/auth/auth-repository';
import type { User, UserCredentials, UserRegistration, VerificationInfo } from '@/domain/auth/auth.types';
import * as grpc from '@grpc/grpc-js';

// Import the client and the generated message classes
import { getAuthServiceClient, LoginRequest, RegisterRequest, VerifyUserRequest, ResendVerificationEmailRequest, GetCurrentUserRequest } from '@/lib/grpc/auth-client';
import { CallContext } from './types';

export class GrpcAuthRepository implements AuthRepository {
  private client = getAuthServiceClient();

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
          return reject(error);
        }

        resolve(
          {
            uuid: response.getUserUuid(),
            name: response.getEmail(),
            email: response.getEmail(),
            role: response.getUserRole().toString()
          }
        );
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
          return reject(error);
        }

        resolve({ jwt: response.getJwt() });
      });
    });
  }

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

      this.client.register(request, metadata, callOptions, (error, response) => {
        if (error) {
          return reject(error);
        }
        resolve({ userUuid: response.getUserUuid() });
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
          return reject(error);
        }
        resolve({ userUuid: response.getUserUuid() });
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
          return reject(error);
        }
        resolve();
      });
    });
  }
}
