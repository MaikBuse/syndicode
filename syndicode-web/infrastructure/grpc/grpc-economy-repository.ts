import type { EconomyRepository } from '@/domain/economy/economy-repository';
import type { Corporation, QueryBuildingsFilters, QueryBuildingsResult } from '@/domain/economy/economy.types';
import { getEconomyServiceClient } from '@/lib/grpc/economy-client';
import * as grpc from '@grpc/grpc-js';
import { GetCorporationRequest, QueryBuildingsRequest, QueryBuildingsResponse } from '@/lib/grpc/generated/economy/v1/economy_pb';
import { BuildingDetails } from '@/lib/grpc/generated/economy/v1/economy_pb';
import { CallContext } from './types';
import { UnknownAuthError } from '@/domain/auth/auth.error';
import * as google_protobuf_wrappers_pb from "google-protobuf/google/protobuf/wrappers_pb";

export class GrpcEconomyRepository implements EconomyRepository {
  private client = getEconomyServiceClient();

  getCorporation(ipAddress: string, jwt: string): Promise<Corporation> {
    return new Promise((resolve, reject) => {
      const request = new GetCorporationRequest();

      const metadata = new grpc.Metadata();

      const customContext: CallContext = { ipAddress, jwt };

      const callOptions: grpc.CallOptions & { customContext: CallContext } = {
        customContext: customContext,
      };

      this.client.getCurrentCorporation(
        request,
        metadata,
        callOptions,
        (error, response) => {
          if (error) {
            console.log(error);
            reject(error);
            return;
          }

          if (response) {
            resolve({
              uuid: response.getUuid(),
              name: response.getName(),
              cash_balance: response.getBalance(),
            });
          } else {
            // This case is unlikely but good to handle
            reject(new UnknownAuthError("Received an empty response from the server."));
          }
        }
      );
    });
  }


  async queryBuildings(filters: QueryBuildingsFilters, ipAddress: string, jwt: string): Promise<QueryBuildingsResult> {
    const grpcRequest = new QueryBuildingsRequest();

    if (filters.owningCorporationUuid) {
      const uuidValue = new google_protobuf_wrappers_pb.StringValue();
      uuidValue.setValue(filters.owningCorporationUuid);
      grpcRequest.setOwningCorporationUuid(uuidValue);
    }
    if (filters.minLon != null) {
      const minLonValue = new google_protobuf_wrappers_pb.DoubleValue();
      minLonValue.setValue(filters.minLon);
      grpcRequest.setMinLon(minLonValue);
    }
    if (filters.maxLon != null) {
      const maxLonValue = new google_protobuf_wrappers_pb.DoubleValue();
      maxLonValue.setValue(filters.maxLon);
      grpcRequest.setMaxLon(maxLonValue);
    }
    if (filters.minLat != null) {
      const minLatValue = new google_protobuf_wrappers_pb.DoubleValue();
      minLatValue.setValue(filters.minLat);
      grpcRequest.setMinLat(minLatValue);
    }
    if (filters.maxLat != null) {
      const maxLatValue = new google_protobuf_wrappers_pb.DoubleValue();
      maxLatValue.setValue(filters.maxLat);
      grpcRequest.setMaxLat(maxLatValue);
    }
    if (filters.limit) {
      const limitValue = new google_protobuf_wrappers_pb.DoubleValue();
      limitValue.setValue(filters.limit);
      grpcRequest.setLimit(limitValue);
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
