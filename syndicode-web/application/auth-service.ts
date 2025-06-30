import type { AuthRepository } from '@/domain/auth/auth-repository';
import { GrpcAuthRepository } from '@/infrastructure/grpc/grpc-auth-repository';
import type { UserCredentials, UserRegistration, VerificationInfo } from '@/domain/auth/auth.types';
import { cookies } from 'next/headers';

class AuthService {
  constructor(private authRepository: AuthRepository) { }

  async register(data: UserRegistration) {
    return this.authRepository.register(data);
  }

  async verifyUser(data: VerificationInfo) {
    return this.authRepository.verifyUser(data);
  }

  async resendVerificationEmail(userName: string) {
    return this.authRepository.resendVerificationEmail(userName);
  }

  async login(credentials: UserCredentials, ipAddress: string) {
    const { jwt } = await this.authRepository.login(credentials, ipAddress);

    // After successful login, set the auth cookie
    const cookieStore = await cookies();
    cookieStore.set('auth_token', jwt, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'strict',
      path: '/',
    });

    return { success: true };
  }
}

// Dependency Injection: Create a single instance with the concrete implementation
const authService = new AuthService(new GrpcAuthRepository());
export default authService;
