// package: syndicode_interface_v1
// file: interface/v1/auth.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as interface_v1_auth_pb from "../../interface/v1/auth_pb";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";

interface IAuthServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    register: IAuthServiceService_IRegister;
    verifyUser: IAuthServiceService_IVerifyUser;
    resendVerificationEmail: IAuthServiceService_IResendVerificationEmail;
    login: IAuthServiceService_ILogin;
    getCurrentUser: IAuthServiceService_IGetCurrentUser;
}

interface IAuthServiceService_IRegister extends grpc.MethodDefinition<interface_v1_auth_pb.RegisterRequest, interface_v1_auth_pb.RegisterResponse> {
    path: "/syndicode_interface_v1.AuthService/Register";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_auth_pb.RegisterRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_auth_pb.RegisterRequest>;
    responseSerialize: grpc.serialize<interface_v1_auth_pb.RegisterResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_auth_pb.RegisterResponse>;
}
interface IAuthServiceService_IVerifyUser extends grpc.MethodDefinition<interface_v1_auth_pb.VerifyUserRequest, interface_v1_auth_pb.VerifyUserResponse> {
    path: "/syndicode_interface_v1.AuthService/VerifyUser";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_auth_pb.VerifyUserRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_auth_pb.VerifyUserRequest>;
    responseSerialize: grpc.serialize<interface_v1_auth_pb.VerifyUserResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_auth_pb.VerifyUserResponse>;
}
interface IAuthServiceService_IResendVerificationEmail extends grpc.MethodDefinition<interface_v1_auth_pb.ResendVerificationEmailRequest, interface_v1_auth_pb.ResendVerificationEmailResponse> {
    path: "/syndicode_interface_v1.AuthService/ResendVerificationEmail";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_auth_pb.ResendVerificationEmailRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_auth_pb.ResendVerificationEmailRequest>;
    responseSerialize: grpc.serialize<interface_v1_auth_pb.ResendVerificationEmailResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_auth_pb.ResendVerificationEmailResponse>;
}
interface IAuthServiceService_ILogin extends grpc.MethodDefinition<interface_v1_auth_pb.LoginRequest, interface_v1_auth_pb.LoginResponse> {
    path: "/syndicode_interface_v1.AuthService/Login";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_auth_pb.LoginRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_auth_pb.LoginRequest>;
    responseSerialize: grpc.serialize<interface_v1_auth_pb.LoginResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_auth_pb.LoginResponse>;
}
interface IAuthServiceService_IGetCurrentUser extends grpc.MethodDefinition<interface_v1_auth_pb.GetCurrentUserRequest, interface_v1_shared_pb.GetUserResponse> {
    path: "/syndicode_interface_v1.AuthService/GetCurrentUser";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<interface_v1_auth_pb.GetCurrentUserRequest>;
    requestDeserialize: grpc.deserialize<interface_v1_auth_pb.GetCurrentUserRequest>;
    responseSerialize: grpc.serialize<interface_v1_shared_pb.GetUserResponse>;
    responseDeserialize: grpc.deserialize<interface_v1_shared_pb.GetUserResponse>;
}

export const AuthServiceService: IAuthServiceService;

export interface IAuthServiceServer extends grpc.UntypedServiceImplementation {
    register: grpc.handleUnaryCall<interface_v1_auth_pb.RegisterRequest, interface_v1_auth_pb.RegisterResponse>;
    verifyUser: grpc.handleUnaryCall<interface_v1_auth_pb.VerifyUserRequest, interface_v1_auth_pb.VerifyUserResponse>;
    resendVerificationEmail: grpc.handleUnaryCall<interface_v1_auth_pb.ResendVerificationEmailRequest, interface_v1_auth_pb.ResendVerificationEmailResponse>;
    login: grpc.handleUnaryCall<interface_v1_auth_pb.LoginRequest, interface_v1_auth_pb.LoginResponse>;
    getCurrentUser: grpc.handleUnaryCall<interface_v1_auth_pb.GetCurrentUserRequest, interface_v1_shared_pb.GetUserResponse>;
}

export interface IAuthServiceClient {
    register(request: interface_v1_auth_pb.RegisterRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    register(request: interface_v1_auth_pb.RegisterRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    register(request: interface_v1_auth_pb.RegisterRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    verifyUser(request: interface_v1_auth_pb.VerifyUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.VerifyUserResponse) => void): grpc.ClientUnaryCall;
    verifyUser(request: interface_v1_auth_pb.VerifyUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.VerifyUserResponse) => void): grpc.ClientUnaryCall;
    verifyUser(request: interface_v1_auth_pb.VerifyUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.VerifyUserResponse) => void): grpc.ClientUnaryCall;
    resendVerificationEmail(request: interface_v1_auth_pb.ResendVerificationEmailRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.ResendVerificationEmailResponse) => void): grpc.ClientUnaryCall;
    resendVerificationEmail(request: interface_v1_auth_pb.ResendVerificationEmailRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.ResendVerificationEmailResponse) => void): grpc.ClientUnaryCall;
    resendVerificationEmail(request: interface_v1_auth_pb.ResendVerificationEmailRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.ResendVerificationEmailResponse) => void): grpc.ClientUnaryCall;
    login(request: interface_v1_auth_pb.LoginRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.LoginResponse) => void): grpc.ClientUnaryCall;
    login(request: interface_v1_auth_pb.LoginRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.LoginResponse) => void): grpc.ClientUnaryCall;
    login(request: interface_v1_auth_pb.LoginRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.LoginResponse) => void): grpc.ClientUnaryCall;
    getCurrentUser(request: interface_v1_auth_pb.GetCurrentUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    getCurrentUser(request: interface_v1_auth_pb.GetCurrentUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    getCurrentUser(request: interface_v1_auth_pb.GetCurrentUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
}

export class AuthServiceClient extends grpc.Client implements IAuthServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public register(request: interface_v1_auth_pb.RegisterRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    public register(request: interface_v1_auth_pb.RegisterRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    public register(request: interface_v1_auth_pb.RegisterRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    public verifyUser(request: interface_v1_auth_pb.VerifyUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.VerifyUserResponse) => void): grpc.ClientUnaryCall;
    public verifyUser(request: interface_v1_auth_pb.VerifyUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.VerifyUserResponse) => void): grpc.ClientUnaryCall;
    public verifyUser(request: interface_v1_auth_pb.VerifyUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.VerifyUserResponse) => void): grpc.ClientUnaryCall;
    public resendVerificationEmail(request: interface_v1_auth_pb.ResendVerificationEmailRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.ResendVerificationEmailResponse) => void): grpc.ClientUnaryCall;
    public resendVerificationEmail(request: interface_v1_auth_pb.ResendVerificationEmailRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.ResendVerificationEmailResponse) => void): grpc.ClientUnaryCall;
    public resendVerificationEmail(request: interface_v1_auth_pb.ResendVerificationEmailRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.ResendVerificationEmailResponse) => void): grpc.ClientUnaryCall;
    public login(request: interface_v1_auth_pb.LoginRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.LoginResponse) => void): grpc.ClientUnaryCall;
    public login(request: interface_v1_auth_pb.LoginRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.LoginResponse) => void): grpc.ClientUnaryCall;
    public login(request: interface_v1_auth_pb.LoginRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_auth_pb.LoginResponse) => void): grpc.ClientUnaryCall;
    public getCurrentUser(request: interface_v1_auth_pb.GetCurrentUserRequest, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    public getCurrentUser(request: interface_v1_auth_pb.GetCurrentUserRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
    public getCurrentUser(request: interface_v1_auth_pb.GetCurrentUserRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: interface_v1_shared_pb.GetUserResponse) => void): grpc.ClientUnaryCall;
}
