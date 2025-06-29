// package: syndicode_interface_v1
// file: interface/v1/shared.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class ActionInitResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): ActionInitResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ActionInitResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ActionInitResponse): ActionInitResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ActionInitResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ActionInitResponse;
    static deserializeBinaryFromReader(message: ActionInitResponse, reader: jspb.BinaryReader): ActionInitResponse;
}

export namespace ActionInitResponse {
    export type AsObject = {
        requestUuid: string,
    }
}

export class GetUserResponse extends jspb.Message { 
    getUserUuid(): string;
    setUserUuid(value: string): GetUserResponse;
    getUserName(): string;
    setUserName(value: string): GetUserResponse;
    getEmail(): string;
    setEmail(value: string): GetUserResponse;
    getUserRole(): UserRole;
    setUserRole(value: UserRole): GetUserResponse;
    getStatus(): string;
    setStatus(value: string): GetUserResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetUserResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetUserResponse): GetUserResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetUserResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetUserResponse;
    static deserializeBinaryFromReader(message: GetUserResponse, reader: jspb.BinaryReader): GetUserResponse;
}

export namespace GetUserResponse {
    export type AsObject = {
        userUuid: string,
        userName: string,
        email: string,
        userRole: UserRole,
        status: string,
    }
}

export enum SortDirection {
    SORT_DIRECTION_UNSPECIFIED = 0,
    ASCENDING = 1,
    DESCENDING = 2,
}

export enum UserRole {
    USER_ROLE_UNSPECIFIED = 0,
    ADMIN = 1,
    PLAYER = 2,
}
