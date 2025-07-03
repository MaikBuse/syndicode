import type { User, UserCredentials, UserRegistration, VerificationInfo } from './auth.types';

export interface AuthRepository {
  register(data: UserRegistration, ipAddress: string): Promise<{ userUuid: string }>;
  verifyUser(data: VerificationInfo, ipAddress: string): Promise<{ userUuid: string }>;
  resendVerificationEmail(userName: string, ipAddress: string): Promise<void>;
  login(credentials: UserCredentials, ipAddress: string): Promise<{ jwt: string }>;
  getCurrentUser(ipAddress: string, jwt: string): Promise<User>;
}
