// package: syndicode_interface_v1
// file: interface/v1/admin.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";

export class CreateUserRequest extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): CreateUserRequest;
    getUserName(): string;
    setUserName(value: string): CreateUserRequest;
    getUserPassword(): string;
    setUserPassword(value: string): CreateUserRequest;
    getUserEmail(): string;
    setUserEmail(value: string): CreateUserRequest;
    getUserRole(): interface_v1_shared_pb.UserRole;
    setUserRole(value: interface_v1_shared_pb.UserRole): CreateUserRequest;
    getCorporationName(): string;
    setCorporationName(value: string): CreateUserRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CreateUserRequest.AsObject;
    static toObject(includeInstance: boolean, msg: CreateUserRequest): CreateUserRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CreateUserRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CreateUserRequest;
    static deserializeBinaryFromReader(message: CreateUserRequest, reader: jspb.BinaryReader): CreateUserRequest;
}

export namespace CreateUserRequest {
    export type AsObject = {
        requestUuid: string,
        userName: string,
        userPassword: string,
        userEmail: string,
        userRole: interface_v1_shared_pb.UserRole,
        corporationName: string,
    }
}

export class CreateUserResponse extends jspb.Message { 
    getUserUuid(): string;
    setUserUuid(value: string): CreateUserResponse;
    getUserName(): string;
    setUserName(value: string): CreateUserResponse;
    getUserRole(): interface_v1_shared_pb.UserRole;
    setUserRole(value: interface_v1_shared_pb.UserRole): CreateUserResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CreateUserResponse.AsObject;
    static toObject(includeInstance: boolean, msg: CreateUserResponse): CreateUserResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CreateUserResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CreateUserResponse;
    static deserializeBinaryFromReader(message: CreateUserResponse, reader: jspb.BinaryReader): CreateUserResponse;
}

export namespace CreateUserResponse {
    export type AsObject = {
        userUuid: string,
        userName: string,
        userRole: interface_v1_shared_pb.UserRole,
    }
}

export class GetUserRequest extends jspb.Message { 
    getUserUuid(): string;
    setUserUuid(value: string): GetUserRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetUserRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetUserRequest): GetUserRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetUserRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetUserRequest;
    static deserializeBinaryFromReader(message: GetUserRequest, reader: jspb.BinaryReader): GetUserRequest;
}

export namespace GetUserRequest {
    export type AsObject = {
        userUuid: string,
    }
}

export class DeleteUserRequest extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): DeleteUserRequest;
    getUserUuid(): string;
    setUserUuid(value: string): DeleteUserRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeleteUserRequest.AsObject;
    static toObject(includeInstance: boolean, msg: DeleteUserRequest): DeleteUserRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeleteUserRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeleteUserRequest;
    static deserializeBinaryFromReader(message: DeleteUserRequest, reader: jspb.BinaryReader): DeleteUserRequest;
}

export namespace DeleteUserRequest {
    export type AsObject = {
        requestUuid: string,
        userUuid: string,
    }
}

export class DeleteUserResponse extends jspb.Message { 
    getUserUuid(): string;
    setUserUuid(value: string): DeleteUserResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeleteUserResponse.AsObject;
    static toObject(includeInstance: boolean, msg: DeleteUserResponse): DeleteUserResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeleteUserResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeleteUserResponse;
    static deserializeBinaryFromReader(message: DeleteUserResponse, reader: jspb.BinaryReader): DeleteUserResponse;
}

export namespace DeleteUserResponse {
    export type AsObject = {
        userUuid: string,
    }
}
