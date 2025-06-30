import type { AuthRepository } from '@/domain/auth/auth-repository';
import type { UserCredentials, UserRegistration, VerificationInfo } from '@/domain/auth/auth.types';
import * as grpc from '@grpc/grpc-js';

const PROXY_API_KEY = process.env.PROXY_API_KEY || '';

// Import the client and the generated message classes
import { getAuthServiceClient, LoginRequest, RegisterRequest, VerifyUserRequest, ResendVerificationEmailRequest } from '@/lib/grpc/auth-client';

export class GrpcAuthRepository implements AuthRepository {
  private client = getAuthServiceClient();

  async login(credentials: UserCredentials, ipAddress: string): Promise<{ jwt: string }> {
    return new Promise((resolve, reject) => {
      const request = new LoginRequest();
      request.setUserName(credentials.userName);
      request.setUserPassword(credentials.userPassword);

      const metadata = new grpc.Metadata();
      metadata.set('proxy-ip-address', ipAddress);
      metadata.set('proxy-api-key', PROXY_API_KEY);

      this.client.login(request, metadata, (error, response) => {
        if (error) {
          return reject(error);
        }

        resolve({ jwt: response.getJwt() });
      });
    });
  }

  async register(data: UserRegistration): Promise<{ userUuid: string }> {
    return new Promise((resolve, reject) => {
      const request = new RegisterRequest();
      request.setUserName(data.userName);
      request.setUserPassword(data.userPassword);
      request.setEmail(data.email);
      request.setCorporationName(data.corporationName);

      this.client.register(request, (error, response) => {
        if (error) {
          return reject(error);
        }
        resolve({ userUuid: response.getUserUuid() });
      });
    });
  }

  async verifyUser(data: VerificationInfo): Promise<{ userUuid: string }> {
    return new Promise((resolve, reject) => {
      const request = new VerifyUserRequest();
      request.setUserName(data.userName);
      request.setCode(data.code);

      this.client.verifyUser(request, (error, response) => {
        if (error) {
          return reject(error);
        }
        resolve({ userUuid: response.getUserUuid() });
      });
    });
  }

  async resendVerificationEmail(userName: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = new ResendVerificationEmailRequest();
      request.setUserName(userName);

      this.client.resendVerificationEmail(request, (error, response) => {
        if (error) {
          return reject(error);
        }
        resolve();
      });
    });
  }
}
