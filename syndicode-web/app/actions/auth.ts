'use server';

import { cookies } from 'next/headers';
import { jwtVerify } from 'jose';
import { serverConfig } from '@/config/server';
import { User } from '@/domain/auth/auth.types';
import { TokenInvalidError } from '@/lib/errors/auth-errors';

function getJwtSecretKey() {
  const secret = serverConfig.jwt_secret;
  return new TextEncoder().encode(secret);
}

export async function getCurrentUser(): Promise<User | null> {
  const cookieStore = await cookies();
  const jwt = cookieStore.get('auth_token')?.value;

  if (!jwt) {
    return null;
  }

  try {
    const { payload } = await jwtVerify(jwt, getJwtSecretKey());

    return {
      uuid: payload.sub as string,
      name: payload.user_name as string,
      email: payload.user_email as string,
      role: payload.user_role as string
    };
  } catch (error) {
    // Any token verification error - clear the cookie and handle client-side logout
    console.error('JWT Verification Error:', error);
    const cookieStore = await cookies();
    cookieStore.delete('auth_token');
    
    throw new TokenInvalidError('JWT token verification failed');
  }
}

export async function logout(): Promise<void> {
  const cookieStore = await cookies();
  cookieStore.delete('auth_token');
}
