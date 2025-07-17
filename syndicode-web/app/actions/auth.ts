'use server';

import { cookies } from 'next/headers';
import { jwtVerify } from 'jose';
import { serverConfig } from '@/config/server';
import { User } from '@/domain/auth/auth.types';

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
      name: payload.userName as string,
      email: payload.userEmail as string,
      role: payload.userRole as string
    };
  } catch (error) {
    // Token is invalid or expired
    console.error('JWT Verification Error:', error);
    return null;
  }
}

export async function logout(): Promise<void> {
  const cookieStore = await cookies();
  cookieStore.delete('auth_token');
}
