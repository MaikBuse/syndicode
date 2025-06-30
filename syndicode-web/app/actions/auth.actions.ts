'use server';

import { headers } from 'next/headers'; // <--- Import this
import authService from '@/application/auth-service';
import { z } from 'zod';

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
  code: z.string().length(6, "Verification code must be 6 characters."),
});

// A helper for structured responses
type ActionResponse = {
  success: boolean;
  message: string;
}

export async function loginAction(values: z.infer<typeof loginSchema>): Promise<ActionResponse> {
  const validatedFields = loginSchema.safeParse(values);
  if (!validatedFields.success) {
    return { success: false, message: "Invalid input." };
  }

  const requestHeaders = await headers();

  // Extract the IP address
  // 'x-forwarded-for' is the standard header for proxies (like Vercel)
  // Fallback to 'x-real-ip' or a default value
  const ipAddress = requestHeaders.get('x-forwarded-for') || requestHeaders.get('x-real-ip') || '127.0.0.1';

  try {
    await authService.login(validatedFields.data, ipAddress);
    return { success: true, message: "Login successful!" };
  } catch (error) {
    console.error(error); // Log the real error
    return { success: false, message: "Login failed. Please check your credentials." };
  }
}

export async function registerAction(values: z.infer<typeof registerSchema>): Promise<ActionResponse> {
  const validatedFields = registerSchema.safeParse(values);
  if (!validatedFields.success) {
    return { success: false, message: "Invalid input." };
  }

  try {
    await authService.register(validatedFields.data);
    return { success: true, message: "Registration successful! Please check your email for a verification code." };
  } catch (error) {
    return { success: false, message: "Registration failed. This user may already exist." };
  }
}

export async function verifyUserAction(values: z.infer<typeof verifySchema>): Promise<ActionResponse> {
  const validatedFields = verifySchema.safeParse(values);
  if (!validatedFields.success) {
    return { success: false, message: "Invalid input." };
  }

  try {
    await authService.verifyUser(validatedFields.data);
    return { success: true, message: "Verification successful! You can now log in." };
  } catch (error) {
    return { success: false, message: "Verification failed. Please check the code and try again." };
  }
}

export async function resendCodeAction(userName: string): Promise<ActionResponse> {
  if (!userName) {
    return { success: false, message: "Username is required." };
  }
  try {
    await authService.resendVerificationEmail(userName);
    return { success: true, message: "A new verification code has been sent." };
  } catch (error) {
    return { success: false, message: "Failed to resend code." };
  }
}
