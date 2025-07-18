'use server';

import { z } from 'zod';
import economyService from '@/application/economy-service';
import type { Corporation, QueryBuildingsResult, QueryBusinessesResult, QueryBusinessListingsResult, AcquireBusinessResult } from '@/domain/economy/economy.types';
import { cookies, headers } from 'next/headers';
import { getClientIp } from './utils';

type ActionResponse<T> =
  | { success: true; data: T }
  | { success: false; message: string; errors?: z.ZodIssue[] };

// Zod schema for validating input from the client.
const queryBuildingsSchema = z.object({
  owningCorporationUuid: z.string().uuid().optional().nullable(),
  owningBusinessUuid: z.string().uuid().optional().nullable(),
  minLon: z.coerce.number().optional().nullable(),
  maxLon: z.coerce.number().optional().nullable(),
  minLat: z.coerce.number().optional().nullable(),
  maxLat: z.coerce.number().optional().nullable(),
  limit: z.coerce.number().int().positive().max(100, "Limit cannot exceed 100.").optional().nullable(),
});

const queryBusinessesSchema = z.object({
  owningCorporationUuid: z.string().uuid().optional().nullable(),
  marketUuid: z.string().uuid().optional().nullable(),
  minOperationalExpenses: z.coerce.number().int().optional().nullable(),
  maxOperationalExpenses: z.coerce.number().int().optional().nullable(),
  sortBy: z.number().int().min(0).max(3).optional().nullable(),
  sortDirection: z.number().int().min(0).max(2).optional().nullable(),
  limit: z.coerce.number().int().positive().max(100, "Limit cannot exceed 100.").optional().nullable(),
  offset: z.coerce.number().int().min(0).optional().nullable(),
});

const queryBusinessListingsSchema = z.object({
  minAskingPrice: z.coerce.number().int().optional().nullable(),
  maxAskingPrice: z.coerce.number().int().optional().nullable(),
  sellerCorporationUuid: z.string().uuid().optional().nullable(),
  marketUuid: z.string().uuid().optional().nullable(),
  minOperationalExpenses: z.coerce.number().int().optional().nullable(),
  maxOperationalExpenses: z.coerce.number().int().optional().nullable(),
  sortBy: z.number().int().min(0).max(4).optional().nullable(),
  sortDirection: z.number().int().min(0).max(2).optional().nullable(),
  limit: z.coerce.number().int().positive().max(100, "Limit cannot exceed 100.").optional().nullable(),
  offset: z.coerce.number().int().min(0).optional().nullable(),
});

const acquireListedBusinessSchema = z.object({
  businessListingUuid: z.string().uuid(),
});

export async function getCurrentCorporationAction(): Promise<ActionResponse<Corporation>> {
  const ipAddress = getClientIp(await headers());
  const cookieStore = await cookies();
  const jwt = cookieStore.get('auth_token')?.value;

  if (!jwt) {
    return { success: false, message: "Failed to retrieve jwt." };
  }


  try {
    const corporation = await economyService.getCurrentCorporation(ipAddress, jwt);

    return { success: true, data: corporation };
  } catch (error) {
    const message = (typeof error === 'object' && error && 'message' in error)
      ? String((error as { message: unknown }).message)
      : 'Registration failed.';
    return { success: false, message: message };
  }
}

/**
 * Server Action to query buildings.
 */
export async function queryBuildingsAction(
  values: z.infer<typeof queryBuildingsSchema>,
): Promise<ActionResponse<QueryBuildingsResult>> {
  // 1. Validate the input from the client.
  const validatedFields = queryBuildingsSchema.safeParse(values);
  if (!validatedFields.success) {
    return {
      success: false,
      message: "Invalid input provided.",
      errors: validatedFields.error.issues,
    };
  }

  const ipAddress = getClientIp(await headers());

  try {
    const cookieStore = await cookies();
    const jwt = cookieStore.get('auth_token')?.value;

    if (!jwt) {
      return { success: false, message: "Failed to retrieve jwt." };
    }

    // 2. Call the application service with the validated, clean data.
    const result = await economyService.getBuildings(validatedFields.data, ipAddress, jwt);

    // 3. Return a successful response with the data.
    return { success: true, data: result };
  } catch (error) {
    // 4. Catch any errors (e.g., from the gRPC call) and return a friendly message.
    console.error("queryBuildingsAction failed:", error);
    return { success: false, message: "An unexpected error occurred while fetching buildings." };
  }
}

/**
 * Server Action to query businesses.
 */
export async function queryBusinessesAction(
  values: z.infer<typeof queryBusinessesSchema>,
): Promise<ActionResponse<QueryBusinessesResult>> {
  // 1. Validate the input from the client.
  const validatedFields = queryBusinessesSchema.safeParse(values);
  if (!validatedFields.success) {
    return {
      success: false,
      message: "Invalid input provided.",
      errors: validatedFields.error.issues,
    };
  }

  const ipAddress = getClientIp(await headers());

  try {
    const cookieStore = await cookies();
    const jwt = cookieStore.get('auth_token')?.value;

    if (!jwt) {
      return { success: false, message: "Failed to retrieve jwt." };
    }

    // 2. Call the application service with the validated, clean data.
    const result = await economyService.getBusinesses(validatedFields.data, ipAddress, jwt);

    // 3. Return a successful response with the data.
    return { success: true, data: result };
  } catch (error) {
    // 4. Catch any errors (e.g., from the gRPC call) and return a friendly message.
    console.error("queryBusinessesAction failed:", error);
    return { success: false, message: "An unexpected error occurred while fetching businesses." };
  }
}

/**
 * Server Action to query business listings.
 */
export async function queryBusinessListings(
  values: z.infer<typeof queryBusinessListingsSchema>,
): Promise<QueryBusinessListingsResult> {
  // 1. Validate the input from the client.
  const validatedFields = queryBusinessListingsSchema.safeParse(values);
  if (!validatedFields.success) {
    throw new Error("Invalid input provided.");
  }

  const ipAddress = getClientIp(await headers());

  try {
    const cookieStore = await cookies();
    const jwt = cookieStore.get('auth_token')?.value;

    if (!jwt) {
      throw new Error("Failed to retrieve jwt.");
    }

    // 2. Call the application service with the validated, clean data.
    const result = await economyService.getBusinessListings(validatedFields.data, ipAddress, jwt);

    // 3. Return the result directly.
    return result;
  } catch (error) {
    // 4. Catch any errors (e.g., from the gRPC call) and re-throw.
    console.error("queryBusinessListings failed:", error);
    throw error;
  }
}

/**
 * Server Action to acquire a listed business.
 */
export async function acquireListedBusinessAction(
  values: z.infer<typeof acquireListedBusinessSchema>,
): Promise<ActionResponse<AcquireBusinessResult>> {
  // 1. Validate the input from the client.
  const validatedFields = acquireListedBusinessSchema.safeParse(values);
  if (!validatedFields.success) {
    return {
      success: false,
      message: "Invalid input provided.",
      errors: validatedFields.error.issues,
    };
  }

  const ipAddress = getClientIp(await headers());

  try {
    const cookieStore = await cookies();
    const jwt = cookieStore.get('auth_token')?.value;

    if (!jwt) {
      return { success: false, message: "Failed to retrieve jwt." };
    }

    // 2. Call the application service with the validated data.
    const result = await economyService.acquireListedBusiness(
      validatedFields.data.businessListingUuid,
      ipAddress,
      jwt
    );

    // 3. Return a successful response with the data.
    return { success: true, data: result };
  } catch (error) {
    // 4. Catch any errors (e.g., from the gRPC call) and return a friendly message.
    console.error("acquireListedBusinessAction failed:", error);
    return { success: false, message: "An unexpected error occurred while acquiring the business." };
  }
}
