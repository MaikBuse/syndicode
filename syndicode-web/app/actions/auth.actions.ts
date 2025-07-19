'use server';

import { headers } from 'next/headers';
import authService from '@/application/auth-service';
import { z } from 'zod';
import { getClientIp } from './utils';
import { User } from '@/domain/auth/auth.types';
import { UserInactiveError, VerificationCodeExpired, VerificationCodeFalse } from '@/domain/auth/auth.error';

// Zod schemas for validation
const loginSchema = z.object({
  userName: z.string().min(3, "Username must be at least 3 characters."),
  userPassword: z.string().min(6, "Password must be at least 6 characters."),
});

const registerSchema = z.object({
  userName: z.string().min(3),
  email: z.string().email(),
  userPassword: z.string().min(6),
  corporationName: z.string().min(1),
});

const verifySchema = z.object({
  userName: z.string().min(3),
  code: z.string().length(10, "Verification code must be 10 characters."),
});

type ActionResponse = {
  success: boolean;
  message: string;
}

type LoginActionResponse = {
  success: boolean;
  isInactive: boolean;
  message: string;
  user: User | null;
}

export async function loginAction(values: z.infer<typeof loginSchema>): Promise<LoginActionResponse> {
  const validatedFields = loginSchema.safeParse(values);
  if (!validatedFields.success) {
    return { success: false, isInactive: false, user: null, message: "Invalid input." };
  }

  const ipAddress = getClientIp(await headers());

  try {
    const user = await authService.login(validatedFields.data, ipAddress);
    return { success: true, isInactive: false, user: user, message: "Login successful!" };
  } catch (error) {
    if (error instanceof UserInactiveError) {
      return { success: false, isInactive: true, user: null, message: "Login failed. Please verify your account." };
    }

    console.error(error);
    return { success: false, isInactive: false, user: null, message: "Login failed. Please check your credentials." };
  }
}

export async function registerAction(values: z.infer<typeof registerSchema>): Promise<ActionResponse> {
  const validatedFields = registerSchema.safeParse(values);
  if (!validatedFields.success) {
    console.error('Validation failed:', validatedFields.error);
    return { success: false, message: "Invalid input." };
  }

  const ipAddress = getClientIp(await headers());

  try {
    await authService.register(validatedFields.data, ipAddress);
    return { success: true, message: "Registration successful! Please check your email for a verification code." };
  } catch (error) {
    console.error('Registration error:', error);
    const message = (typeof error === 'object' && error && 'message' in error)
      ? String((error as { message: unknown }).message)
      : 'Registration failed.';
    return { success: false, message };
  }
}

export async function verifyUserAction(values: z.infer<typeof verifySchema>): Promise<ActionResponse> {
  const validatedFields = verifySchema.safeParse(values);
  if (!validatedFields.success) {
    return { success: false, message: "Invalid input." };
  }

  const ipAddress = getClientIp(await headers());

  try {
    await authService.verifyUser(validatedFields.data, ipAddress);
    return { success: true, message: "Verification successful! You can now log in." };
  } catch (error) {
    if (error instanceof VerificationCodeExpired) {
      return { success: false, message: "Your verification code has expired. Please request a new one." };
    }
    if (error instanceof VerificationCodeFalse) {
      return { success: false, message: "The verification code is incorrect. Please check and try again." };
    }
    return { success: false, message: "Verification failed. Please check the code and try again." };
  }
}

export async function resendCodeAction(userName: string): Promise<ActionResponse> {
  if (!userName) {
    return { success: false, message: "Username is required." };
  }

  const ipAddress = getClientIp(await headers());

  try {
    await authService.resendVerificationEmail(userName, ipAddress);
    return { success: true, message: "A new verification code has been sent." };
  } catch {
    return { success: false, message: "Failed to resend code." };
  }
}
