// package: syndicode_warfare_v1
// file: warfare/v1/warfare.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class SpawnUnitRequest extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): SpawnUnitRequest.AsObject;
    static toObject(includeInstance: boolean, msg: SpawnUnitRequest): SpawnUnitRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: SpawnUnitRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): SpawnUnitRequest;
    static deserializeBinaryFromReader(message: SpawnUnitRequest, reader: jspb.BinaryReader): SpawnUnitRequest;
}

export namespace SpawnUnitRequest {
    export type AsObject = {
    }
}

export class SpawnUnitResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): SpawnUnitResponse;

    hasUnit(): boolean;
    clearUnit(): void;
    getUnit(): Unit | undefined;
    setUnit(value?: Unit): SpawnUnitResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): SpawnUnitResponse.AsObject;
    static toObject(includeInstance: boolean, msg: SpawnUnitResponse): SpawnUnitResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: SpawnUnitResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): SpawnUnitResponse;
    static deserializeBinaryFromReader(message: SpawnUnitResponse, reader: jspb.BinaryReader): SpawnUnitResponse;
}

export namespace SpawnUnitResponse {
    export type AsObject = {
        requestUuid: string,
        unit?: Unit.AsObject,
    }
}

export class ListUnitsRequest extends jspb.Message { 
    getCorporationUuid(): string;
    setCorporationUuid(value: string): ListUnitsRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ListUnitsRequest.AsObject;
    static toObject(includeInstance: boolean, msg: ListUnitsRequest): ListUnitsRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ListUnitsRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ListUnitsRequest;
    static deserializeBinaryFromReader(message: ListUnitsRequest, reader: jspb.BinaryReader): ListUnitsRequest;
}

export namespace ListUnitsRequest {
    export type AsObject = {
        corporationUuid: string,
    }
}

export class ListUnitsResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): ListUnitsResponse;
    clearUnitsList(): void;
    getUnitsList(): Array<Unit>;
    setUnitsList(value: Array<Unit>): ListUnitsResponse;
    addUnits(value?: Unit, index?: number): Unit;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ListUnitsResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ListUnitsResponse): ListUnitsResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ListUnitsResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ListUnitsResponse;
    static deserializeBinaryFromReader(message: ListUnitsResponse, reader: jspb.BinaryReader): ListUnitsResponse;
}

export namespace ListUnitsResponse {
    export type AsObject = {
        requestUuid: string,
        unitsList: Array<Unit.AsObject>,
    }
}

export class Unit extends jspb.Message { 
    getUuid(): string;
    setUuid(value: string): Unit;
    getCorporationUuid(): string;
    setCorporationUuid(value: string): Unit;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Unit.AsObject;
    static toObject(includeInstance: boolean, msg: Unit): Unit.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Unit, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Unit;
    static deserializeBinaryFromReader(message: Unit, reader: jspb.BinaryReader): Unit;
}

export namespace Unit {
    export type AsObject = {
        uuid: string,
        corporationUuid: string,
    }
}
