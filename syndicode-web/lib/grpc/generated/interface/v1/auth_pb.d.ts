// package: syndicode_interface_v1
// file: interface/v1/auth.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";

export class RegisterRequest extends jspb.Message { 
    getUserName(): string;
    setUserName(value: string): RegisterRequest;
    getUserPassword(): string;
    setUserPassword(value: string): RegisterRequest;
    getEmail(): string;
    setEmail(value: string): RegisterRequest;
    getCorporationName(): string;
    setCorporationName(value: string): RegisterRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterRequest): RegisterRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterRequest;
    static deserializeBinaryFromReader(message: RegisterRequest, reader: jspb.BinaryReader): RegisterRequest;
}

export namespace RegisterRequest {
    export type AsObject = {
        userName: string,
        userPassword: string,
        email: string,
        corporationName: string,
    }
}

export class RegisterResponse extends jspb.Message { 
    getUserUuid(): string;
    setUserUuid(value: string): RegisterResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterResponse.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterResponse): RegisterResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterResponse;
    static deserializeBinaryFromReader(message: RegisterResponse, reader: jspb.BinaryReader): RegisterResponse;
}

export namespace RegisterResponse {
    export type AsObject = {
        userUuid: string,
    }
}

export class VerifyUserRequest extends jspb.Message { 
    getUserName(): string;
    setUserName(value: string): VerifyUserRequest;
    getCode(): string;
    setCode(value: string): VerifyUserRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): VerifyUserRequest.AsObject;
    static toObject(includeInstance: boolean, msg: VerifyUserRequest): VerifyUserRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: VerifyUserRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): VerifyUserRequest;
    static deserializeBinaryFromReader(message: VerifyUserRequest, reader: jspb.BinaryReader): VerifyUserRequest;
}

export namespace VerifyUserRequest {
    export type AsObject = {
        userName: string,
        code: string,
    }
}

export class VerifyUserResponse extends jspb.Message { 
    getUserUuid(): string;
    setUserUuid(value: string): VerifyUserResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): VerifyUserResponse.AsObject;
    static toObject(includeInstance: boolean, msg: VerifyUserResponse): VerifyUserResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: VerifyUserResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): VerifyUserResponse;
    static deserializeBinaryFromReader(message: VerifyUserResponse, reader: jspb.BinaryReader): VerifyUserResponse;
}

export namespace VerifyUserResponse {
    export type AsObject = {
        userUuid: string,
    }
}

export class ResendVerificationEmailRequest extends jspb.Message { 
    getUserName(): string;
    setUserName(value: string): ResendVerificationEmailRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ResendVerificationEmailRequest.AsObject;
    static toObject(includeInstance: boolean, msg: ResendVerificationEmailRequest): ResendVerificationEmailRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ResendVerificationEmailRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ResendVerificationEmailRequest;
    static deserializeBinaryFromReader(message: ResendVerificationEmailRequest, reader: jspb.BinaryReader): ResendVerificationEmailRequest;
}

export namespace ResendVerificationEmailRequest {
    export type AsObject = {
        userName: string,
    }
}

export class ResendVerificationEmailResponse extends jspb.Message { 
    getUserName(): string;
    setUserName(value: string): ResendVerificationEmailResponse;
    getEmail(): string;
    setEmail(value: string): ResendVerificationEmailResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ResendVerificationEmailResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ResendVerificationEmailResponse): ResendVerificationEmailResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ResendVerificationEmailResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ResendVerificationEmailResponse;
    static deserializeBinaryFromReader(message: ResendVerificationEmailResponse, reader: jspb.BinaryReader): ResendVerificationEmailResponse;
}

export namespace ResendVerificationEmailResponse {
    export type AsObject = {
        userName: string,
        email: string,
    }
}

export class LoginRequest extends jspb.Message { 
    getUserName(): string;
    setUserName(value: string): LoginRequest;
    getUserPassword(): string;
    setUserPassword(value: string): LoginRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LoginRequest.AsObject;
    static toObject(includeInstance: boolean, msg: LoginRequest): LoginRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: LoginRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LoginRequest;
    static deserializeBinaryFromReader(message: LoginRequest, reader: jspb.BinaryReader): LoginRequest;
}

export namespace LoginRequest {
    export type AsObject = {
        userName: string,
        userPassword: string,
    }
}

export class LoginResponse extends jspb.Message { 
    getJwt(): string;
    setJwt(value: string): LoginResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LoginResponse.AsObject;
    static toObject(includeInstance: boolean, msg: LoginResponse): LoginResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: LoginResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LoginResponse;
    static deserializeBinaryFromReader(message: LoginResponse, reader: jspb.BinaryReader): LoginResponse;
}

export namespace LoginResponse {
    export type AsObject = {
        jwt: string,
    }
}

export class GetCurrentUserRequest extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetCurrentUserRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetCurrentUserRequest): GetCurrentUserRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetCurrentUserRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetCurrentUserRequest;
    static deserializeBinaryFromReader(message: GetCurrentUserRequest, reader: jspb.BinaryReader): GetCurrentUserRequest;
}

export namespace GetCurrentUserRequest {
    export type AsObject = {
    }
}
