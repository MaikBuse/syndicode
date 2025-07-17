import type { EconomyRepository } from '@/domain/economy/economy-repository';
import type { 
  Corporation, 
  QueryBuildingsFilters, 
  QueryBuildingsResult,
  QueryBusinessesFilters,
  QueryBusinessesResult
} from '@/domain/economy/economy.types';
import { BusinessSortBy, SortDirection } from '@/domain/economy/economy.types';
import { getEconomyServiceClient } from '@/lib/grpc/economy-client';
import * as grpc from '@grpc/grpc-js';
import { 
  GetCorporationRequest, 
  QueryBuildingsRequest, 
  QueryBuildingsResponse,
  QueryBusinessesRequest,
  QueryBusinessesResponse,
  BusinessSortBy as ProtoBusinessSortBy,
} from '@/lib/grpc/generated/economy/v1/economy_pb';
import { BuildingDetails, BusinessDetails } from '@/lib/grpc/generated/economy/v1/economy_pb';
import { SortDirection as ProtoSortDirection } from '@/lib/grpc/generated/interface/v1/shared_pb';
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
    if (filters.owningBusinessUuid) {
      const uuidValue = new google_protobuf_wrappers_pb.StringValue();
      uuidValue.setValue(filters.owningBusinessUuid);
      grpcRequest.setOwningBusinessUuid(uuidValue);
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
        longitude: b.getLongitude(),
        latitude: b.getLatitude(),
      })),
      totalCount: response.getTotalCount(),
    };
  }

  async queryBusinesses(filters: QueryBusinessesFilters, ipAddress: string, jwt: string): Promise<QueryBusinessesResult> {
    const grpcRequest = new QueryBusinessesRequest();

    if (filters.owningCorporationUuid) {
      const uuidValue = new google_protobuf_wrappers_pb.StringValue();
      uuidValue.setValue(filters.owningCorporationUuid);
      grpcRequest.setOwningCorporationUuid(uuidValue);
    }
    if (filters.marketUuid) {
      const marketUuidValue = new google_protobuf_wrappers_pb.StringValue();
      marketUuidValue.setValue(filters.marketUuid);
      grpcRequest.setMarketUuid(marketUuidValue);
    }
    if (filters.minOperationalExpenses != null) {
      const minOpExpValue = new google_protobuf_wrappers_pb.Int64Value();
      minOpExpValue.setValue(filters.minOperationalExpenses);
      grpcRequest.setMinOperationalExpenses(minOpExpValue);
    }
    if (filters.maxOperationalExpenses != null) {
      const maxOpExpValue = new google_protobuf_wrappers_pb.Int64Value();
      maxOpExpValue.setValue(filters.maxOperationalExpenses);
      grpcRequest.setMaxOperationalExpenses(maxOpExpValue);
    }
    if (filters.sortBy != null) {
      grpcRequest.setSortBy(this.mapBusinessSortBy(filters.sortBy));
    }
    if (filters.sortDirection != null) {
      grpcRequest.setSortDirection(this.mapSortDirection(filters.sortDirection));
    }
    if (filters.limit != null) {
      const limitValue = new google_protobuf_wrappers_pb.Int64Value();
      limitValue.setValue(filters.limit);
      grpcRequest.setLimit(limitValue);
    }
    if (filters.offset != null) {
      const offsetValue = new google_protobuf_wrappers_pb.Int64Value();
      offsetValue.setValue(filters.offset);
      grpcRequest.setOffset(offsetValue);
    }

    const metadata = new grpc.Metadata();
    const customContext: CallContext = { ipAddress, jwt };
    const callOptions: grpc.CallOptions & { customContext: CallContext } = {
      customContext: customContext,
    };

    const response: QueryBusinessesResponse = await new Promise((resolve, reject) => {
      this.client.queryBusinesses(grpcRequest, metadata, callOptions, (error, response) => {
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
      businesses: response.getBusinessesList().map((b: BusinessDetails) => ({
        businessUuid: b.getBusinessUuid(),
        businessName: b.getBusinessName(),
        owningCorporationUuid: b.getOwningCorporationUuid()?.getValue() || null,
        marketUuid: b.getMarketUuid(),
        operationalExpenses: b.getOperationalExpenses(),
        headquarterBuildingUuid: b.getHeadquarterBuildingUuid(),
        headquarterBuildingGmlId: b.getHeadquarterBuildingGmlId(),
        headquarterLongitude: b.getHeadquarterLongitude(),
        headquarterLatitude: b.getHeadquarterLatitude(),
      })),
      totalCount: response.getTotalCount(),
    };
  }

  private mapBusinessSortBy(sortBy: BusinessSortBy): ProtoBusinessSortBy {
    switch (sortBy) {
      case BusinessSortBy.BUSINESS_NAME:
        return ProtoBusinessSortBy.BUSINESS_NAME;
      case BusinessSortBy.BUSINESS_OPERATION_EXPENSES:
        return ProtoBusinessSortBy.BUSINESS_OPERATION_EXPENSES;
      case BusinessSortBy.BUSINESS_MARKET_VOLUME:
        return ProtoBusinessSortBy.BUSINESS_MARKET_VOLUME;
      default:
        return ProtoBusinessSortBy.BUSINESS_SORT_BY_UNSPECIFIED;
    }
  }

  private mapSortDirection(sortDirection: SortDirection): ProtoSortDirection {
    switch (sortDirection) {
      case SortDirection.ASCENDING:
        return ProtoSortDirection.ASCENDING;
      case SortDirection.DESCENDING:
        return ProtoSortDirection.DESCENDING;
      case SortDirection.UNSPECIFIED:
      default:
        return ProtoSortDirection.SORT_DIRECTION_UNSPECIFIED;
    }
  }
}
