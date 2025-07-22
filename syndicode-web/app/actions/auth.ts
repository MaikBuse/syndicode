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
      name: payload.user_name as string,
      email: payload.user_email as string,
      role: payload.user_role as string
    };
  } catch (error) {
    // Log the error but don't modify cookies in Server Components
    console.error('JWT Verification Error:', error);
    
    // Return null to indicate no valid user session
    return null;
  }
}

export async function logout(): Promise<void> {
  const cookieStore = await cookies();
  cookieStore.delete('auth_token');
}

export async function clearExpiredAuthToken(): Promise<void> {
  const cookieStore = await cookies();
  cookieStore.delete('auth_token');
}
