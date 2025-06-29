import type { UserCredentials, UserRegistration, VerificationInfo } from './auth.types';

export interface AuthRepository {
  register(data: UserRegistration): Promise<{ userUuid: string }>;
  verifyUser(data: VerificationInfo): Promise<{ userUuid: string }>;
  resendVerificationEmail(userName: string): Promise<void>;
  login(credentials: UserCredentials): Promise<{ jwt: string }>;
}
