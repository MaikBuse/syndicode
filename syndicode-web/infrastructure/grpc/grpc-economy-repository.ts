import type { EconomyRepository } from '@/domain/economy/economy-repository';
import type { QueryBuildingsFilters, QueryBuildingsResult } from '@/domain/economy/economy.types';
import { getEconomyServiceClient } from '@/lib/grpc/economy-client';
import * as grpc from '@grpc/grpc-js';

import { QueryBuildingsRequest, QueryBuildingsResponse } from '@/lib/grpc/generated/economy/v1/economy_pb';
import { BuildingDetails } from '@/lib/grpc/generated/economy/v1/economy_pb';
import { CallContext } from './types';


export class GrpcEconomyRepository implements EconomyRepository {
  private client = getEconomyServiceClient();

  async queryBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult> {
    const grpcRequest = new QueryBuildingsRequest();

    if (filters.owningCorporationUuid) {
      grpcRequest.setOwningCorporationUuid(filters.owningCorporationUuid);
    }
    if (filters.minLon != null) {
      grpcRequest.setMinLon(filters.minLon);
    }
    if (filters.maxLon != null) {
      grpcRequest.setMaxLon(filters.maxLon);
    }
    if (filters.minLat != null) {
      grpcRequest.setMinLat(filters.minLat);
    }
    if (filters.maxLat != null) {
      grpcRequest.setMaxLat(filters.maxLat);
    }
    if (filters.limit) {
      grpcRequest.setLimit(filters.limit);
    }

    const metadata = new grpc.Metadata();

    const customContext: CallContext = { ipAddress, jwt };

    const callOptions: grpc.CallOptions & { customContext: CallContext } = {
      customContext: customContext,
    };

    const response: QueryBuildingsResponse = await new Promise((resolve, reject) => {
      this.client.queryBuildings(grpcRequest, metadata, callOptions, (error, response) => {
        if (error) {
          reject(error);
        } else if (response) {
          resolve(response);
        } else {
          reject(new Error("No response or error received from gRPC call."));
        }
      });
    });

    return {
      gameTick: response.getGameTick(),
      buildings: response.getBuildingsList().map((b: BuildingDetails) => ({
        gmlId: b.getGmlId(),
      })),
      totalCount: response.getTotalCount(),
    };
  }
}
