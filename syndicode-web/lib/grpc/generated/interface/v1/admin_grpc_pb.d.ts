// package: syndicode_interface_v1
// file: interface/v1/admin.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as interface_v1_admin_pb from "../../interface/v1/admin_pb";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";

interface IAdminServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    createUser: IAdminServiceService_ICreateUser;
    getUser: IAdminServiceService_IGetUser;
    deleteUser: IAdminServiceService_IDeleteUser;
}

interface IAdminServiceService_ICreateUser extends grpc.MethodDefinition<interface_v1_admin_pb.CreateUserRequest, interface_v1_admin_pb.CreateUserResponse> {
    path: "/syndicode_interface_v1.AdminService/CreateUser";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_admin_pb.CreateUserRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_admin_pb.CreateUserRequest>;
    responseSerialize: grpc.serialize<interface_v1_admin_pb.CreateUserResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_admin_pb.CreateUserResponse>;
}
interface IAdminServiceService_IGetUser extends grpc.MethodDefinition<interface_v1_admin_pb.GetUserRequest, interface_v1_shared_pb.GetUserResponse> {
    path: "/syndicode_interface_v1.AdminService/GetUser";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_admin_pb.GetUserRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_admin_pb.GetUserRequest>;
    responseSerialize: grpc.serialize<interface_v1_shared_pb.GetUserResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_shared_pb.GetUserResponse>;
}
interface IAdminServiceService_IDeleteUser extends grpc.MethodDefinition<interface_v1_admin_pb.DeleteUserRequest, interface_v1_admin_pb.DeleteUserResponse> {
    path: "/syndicode_interface_v1.AdminService/DeleteUser";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_admin_pb.DeleteUserRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_admin_pb.DeleteUserRequest>;
    responseSerialize: grpc.serialize<interface_v1_admin_pb.DeleteUserResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_admin_pb.DeleteUserResponse>;
}

export const AdminServiceService: IAdminServiceService;

export interface IAdminServiceServer extends grpc.UntypedServiceImplementation {
    createUser: grpc.handleUnaryCall<interface_v1_admin_pb.CreateUserRequest, interface_v1_admin_pb.CreateUserResponse>;
    getUser: grpc.handleUnaryCall<interface_v1_admin_pb.GetUserRequest, interface_v1_shared_pb.GetUserResponse>;
    deleteUser: grpc.handleUnaryCall<interface_v1_admin_pb.DeleteUserRequest, interface_v1_admin_pb.DeleteUserResponse>;
}

export interface IAdminServiceClient {
    createUser(request: interface_v1_admin_pb.CreateUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.CreateUserResponse) => void): grpc.ClientUnaryCall;
    createUser(request: interface_v1_admin_pb.CreateUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.CreateUserResponse) => void): grpc.ClientUnaryCall;
    createUser(request: interface_v1_admin_pb.CreateUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.CreateUserResponse) => void): grpc.ClientUnaryCall;
    getUser(request: interface_v1_admin_pb.GetUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    getUser(request: interface_v1_admin_pb.GetUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    getUser(request: interface_v1_admin_pb.GetUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    deleteUser(request: interface_v1_admin_pb.DeleteUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.DeleteUserResponse) => void): grpc.ClientUnaryCall;
    deleteUser(request: interface_v1_admin_pb.DeleteUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.DeleteUserResponse) => void): grpc.ClientUnaryCall;
    deleteUser(request: interface_v1_admin_pb.DeleteUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.DeleteUserResponse) => void): grpc.ClientUnaryCall;
}

export class AdminServiceClient extends grpc.Client implements IAdminServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public createUser(request: interface_v1_admin_pb.CreateUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.CreateUserResponse) => void): grpc.ClientUnaryCall;
    public createUser(request: interface_v1_admin_pb.CreateUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.CreateUserResponse) => void): grpc.ClientUnaryCall;
    public createUser(request: interface_v1_admin_pb.CreateUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.CreateUserResponse) => void): grpc.ClientUnaryCall;
    public getUser(request: interface_v1_admin_pb.GetUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    public getUser(request: interface_v1_admin_pb.GetUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    public getUser(request: interface_v1_admin_pb.GetUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    public deleteUser(request: interface_v1_admin_pb.DeleteUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.DeleteUserResponse) => void): grpc.ClientUnaryCall;
    public deleteUser(request: interface_v1_admin_pb.DeleteUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.DeleteUserResponse) => void): grpc.ClientUnaryCall;
    public deleteUser(request: interface_v1_admin_pb.DeleteUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_admin_pb.DeleteUserResponse) => void): grpc.ClientUnaryCall;
}
