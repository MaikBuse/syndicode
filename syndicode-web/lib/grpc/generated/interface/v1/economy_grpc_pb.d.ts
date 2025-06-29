// package: syndicode_interface_v1
// file: interface/v1/economy.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as interface_v1_economy_pb from "../../interface/v1/economy_pb";
import * as economy_v1_economy_pb from "../../economy/v1/economy_pb";

interface IEconomyServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    queryBuildingOwnerships: IEconomyServiceService_IQueryBuildingOwnerships;
}

interface IEconomyServiceService_IQueryBuildingOwnerships extends grpc.MethodDefinition<economy_v1_economy_pb.QueryBuildingOwnershipsRequest, economy_v1_economy_pb.BuildingOwnershipsResponse> {
    path: "/syndicode_interface_v1.EconomyService/QueryBuildingOwnerships";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<economy_v1_economy_pb.QueryBuildingOwnershipsRequest>;
    requestDeserialize: grpc.deserialize<economy_v1_economy_pb.QueryBuildingOwnershipsRequest>;
    responseSerialize: grpc.serialize<economy_v1_economy_pb.BuildingOwnershipsResponse>;
    responseDeserialize: grpc.deserialize<economy_v1_economy_pb.BuildingOwnershipsResponse>;
}

export const EconomyServiceService: IEconomyServiceService;

export interface IEconomyServiceServer extends grpc.UntypedServiceImplementation {
    queryBuildingOwnerships: grpc.handleUnaryCall<economy_v1_economy_pb.QueryBuildingOwnershipsRequest, economy_v1_economy_pb.BuildingOwnershipsResponse>;
}

export interface IEconomyServiceClient {
    queryBuildingOwnerships(request: economy_v1_economy_pb.QueryBuildingOwnershipsRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.BuildingOwnershipsResponse) => void): grpc.ClientUnaryCall;
    queryBuildingOwnerships(request: economy_v1_economy_pb.QueryBuildingOwnershipsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.BuildingOwnershipsResponse) => void): grpc.ClientUnaryCall;
    queryBuildingOwnerships(request: economy_v1_economy_pb.QueryBuildingOwnershipsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.BuildingOwnershipsResponse) => void): grpc.ClientUnaryCall;
}

export class EconomyServiceClient extends grpc.Client implements IEconomyServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public queryBuildingOwnerships(request: economy_v1_economy_pb.QueryBuildingOwnershipsRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.BuildingOwnershipsResponse) => void): grpc.ClientUnaryCall;
    public queryBuildingOwnerships(request: economy_v1_economy_pb.QueryBuildingOwnershipsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.BuildingOwnershipsResponse) => void): grpc.ClientUnaryCall;
    public queryBuildingOwnerships(request: economy_v1_economy_pb.QueryBuildingOwnershipsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.BuildingOwnershipsResponse) => void): grpc.ClientUnaryCall;
}
