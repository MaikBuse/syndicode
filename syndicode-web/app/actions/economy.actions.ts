'use server';

import { z } from 'zod';
import economyService from '@/application/economy-service';
import type { Corporation, QueryBuildingsResult } from '@/domain/economy/economy.types';
import { cookies, headers } from 'next/headers';
import { getClientIp } from './utils';

type ActionResponse<T> =
  | { success: true; data: T }
  | { success: false; message: string; errors?: z.ZodIssue[] };

// Zod schema for validating input from the client.
const queryBuildingsSchema = z.object({
  owningCorporationUuid: z.string().uuid().optional().nullable(),
  minLon: z.coerce.number().optional().nullable(),
  maxLon: z.coerce.number().optional().nullable(),
  minLat: z.coerce.number().optional().nullable(),
  maxLat: z.coerce.number().optional().nullable(),
  limit: z.coerce.number().int().positive().max(100, "Limit cannot exceed 100.").optional().nullable(),
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
  } catch (error: any) {

    console.error("getCurrentCorporationAction failed:", error);
    return { success: false, message: error.message };
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
