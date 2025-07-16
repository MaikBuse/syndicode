// package: syndicode_interface_v1
// file: interface/v1/economy.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as interface_v1_economy_pb from "../../interface/v1/economy_pb";
import * as economy_v1_economy_pb from "../../economy/v1/economy_pb";

interface IEconomyServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    getCurrentCorporation: IEconomyServiceService_IGetCurrentCorporation;
    queryBuildings: IEconomyServiceService_IQueryBuildings;
    queryBusinesses: IEconomyServiceService_IQueryBusinesses;
}

interface IEconomyServiceService_IGetCurrentCorporation extends grpc.MethodDefinition<economy_v1_economy_pb.GetCorporationRequest, economy_v1_economy_pb.Corporation> {
    path: "/syndicode_interface_v1.EconomyService/GetCurrentCorporation";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<economy_v1_economy_pb.GetCorporationRequest>;
    requestDeserialize: grpc.deserialize<economy_v1_economy_pb.GetCorporationRequest>;
    responseSerialize: grpc.serialize<economy_v1_economy_pb.Corporation>;
    responseDeserialize: grpc.deserialize<economy_v1_economy_pb.Corporation>;
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
interface IEconomyServiceService_IQueryBusinesses extends grpc.MethodDefinition<economy_v1_economy_pb.QueryBusinessesRequest, economy_v1_economy_pb.QueryBusinessesResponse> {
    path: "/syndicode_interface_v1.EconomyService/QueryBusinesses";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<economy_v1_economy_pb.QueryBusinessesRequest>;
    requestDeserialize: grpc.deserialize<economy_v1_economy_pb.QueryBusinessesRequest>;
    responseSerialize: grpc.serialize<economy_v1_economy_pb.QueryBusinessesResponse>;
    responseDeserialize: grpc.deserialize<economy_v1_economy_pb.QueryBusinessesResponse>;
}

export const EconomyServiceService: IEconomyServiceService;

export interface IEconomyServiceServer extends grpc.UntypedServiceImplementation {
    getCurrentCorporation: grpc.handleUnaryCall<economy_v1_economy_pb.GetCorporationRequest, economy_v1_economy_pb.Corporation>;
    queryBuildings: grpc.handleUnaryCall<economy_v1_economy_pb.QueryBuildingsRequest, economy_v1_economy_pb.QueryBuildingsResponse>;
    queryBusinesses: grpc.handleUnaryCall<economy_v1_economy_pb.QueryBusinessesRequest, economy_v1_economy_pb.QueryBusinessesResponse>;
}

export interface IEconomyServiceClient {
    getCurrentCorporation(request: economy_v1_economy_pb.GetCorporationRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.Corporation) => void): grpc.ClientUnaryCall;
    getCurrentCorporation(request: economy_v1_economy_pb.GetCorporationRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.Corporation) => void): grpc.ClientUnaryCall;
    getCurrentCorporation(request: economy_v1_economy_pb.GetCorporationRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.Corporation) => void): grpc.ClientUnaryCall;
    queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    queryBusinesses(request: economy_v1_economy_pb.QueryBusinessesRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBusinessesResponse) => void): grpc.ClientUnaryCall;
    queryBusinesses(request: economy_v1_economy_pb.QueryBusinessesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBusinessesResponse) => void): grpc.ClientUnaryCall;
    queryBusinesses(request: economy_v1_economy_pb.QueryBusinessesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBusinessesResponse) => void): grpc.ClientUnaryCall;
}

export class EconomyServiceClient extends grpc.Client implements IEconomyServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public getCurrentCorporation(request: economy_v1_economy_pb.GetCorporationRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.Corporation) => void): grpc.ClientUnaryCall;
    public getCurrentCorporation(request: economy_v1_economy_pb.GetCorporationRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.Corporation) => void): grpc.ClientUnaryCall;
    public getCurrentCorporation(request: economy_v1_economy_pb.GetCorporationRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.Corporation) => void): grpc.ClientUnaryCall;
    public queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    public queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    public queryBuildings(request: economy_v1_economy_pb.QueryBuildingsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBuildingsResponse) => void): grpc.ClientUnaryCall;
    public queryBusinesses(request: economy_v1_economy_pb.QueryBusinessesRequest, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBusinessesResponse) => void): grpc.ClientUnaryCall;
    public queryBusinesses(request: economy_v1_economy_pb.QueryBusinessesRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBusinessesResponse) => void): grpc.ClientUnaryCall;
    public queryBusinesses(request: economy_v1_economy_pb.QueryBusinessesRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: economy_v1_economy_pb.QueryBusinessesResponse) => void): grpc.ClientUnaryCall;
}
