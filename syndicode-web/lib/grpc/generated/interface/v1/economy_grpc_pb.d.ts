// package: syndicode_interface_v1
// file: interface/v1/economy.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as interface_v1_economy_pb from "../../interface/v1/economy_pb";
import * as economy_v1_economy_pb from "../../economy/v1/economy_pb";

interface IEconomyServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    queryBuildings: IEconomyServiceService_IQueryBuildings;
}

interface IEconomyServiceService_IQueryBuildings extends grpc.MethodDefinition<economy_v1_economy_pb.QueryBuildingsRequest, economy_v1_economy_pb.QueryBuildingsResponse> {
    path: "/syndicode_interface_v1.EconomyService/QueryBuildings";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<economy_v1_economy_pb.QueryBuildingsRequest>;
    requestDeserialize: grpc.deserialize<economy_v1_economy_pb.QueryBuildingsRequest>;
    responseSerialize: grpc.serialize<economy_v1_economy_pb.QueryBuildingsResponse>;
    responseDeserialize: grpc.deserialize<economy_v1_economy_pb.QueryBuildingsResponse>;
}

export const EconomyServiceService: IEconomyServiceService;

export interface IEconomyServiceServer extends grpc.UntypedServiceImplementation {
    queryBuildings: grpc.handleUnaryCall<economy_v1_economy_pb.QueryBuildingsRequest, economy_v1_economy_pb.QueryBuildingsResponse>;
}

export interface IEconomyServiceClient {
    queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
}

export class EconomyServiceClient extends grpc.Client implements IEconomyServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    public queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    public queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
}
