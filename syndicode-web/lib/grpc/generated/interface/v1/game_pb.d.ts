// package: syndicode_interface_v1
// file: interface/v1/game.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as economy_v1_economy_pb from "../../economy/v1/economy_pb";
import * as warfare_v1_warfare_pb from "../../warfare/v1/warfare_pb";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";

export class PlayerAction extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): PlayerAction;

    hasGetCorporation(): boolean;
    clearGetCorporation(): void;
    getGetCorporation(): economy_v1_economy_pb.GetCorporationRequest | undefined;
    setGetCorporation(value?: economy_v1_economy_pb.GetCorporationRequest): PlayerAction;

    hasSpawnUnit(): boolean;
    clearSpawnUnit(): void;
    getSpawnUnit(): warfare_v1_warfare_pb.SpawnUnitRequest | undefined;
    setSpawnUnit(value?: warfare_v1_warfare_pb.SpawnUnitRequest): PlayerAction;

    hasListUnit(): boolean;
    clearListUnit(): void;
    getListUnit(): warfare_v1_warfare_pb.ListUnitsRequest | undefined;
    setListUnit(value?: warfare_v1_warfare_pb.ListUnitsRequest): PlayerAction;

    hasAcquireListedBusiness(): boolean;
    clearAcquireListedBusiness(): void;
    getAcquireListedBusiness(): economy_v1_economy_pb.AcquireListedBusinessRequest | undefined;
    setAcquireListedBusiness(value?: economy_v1_economy_pb.AcquireListedBusinessRequest): PlayerAction;

    hasQueryBusinessListings(): boolean;
    clearQueryBusinessListings(): void;
    getQueryBusinessListings(): economy_v1_economy_pb.QueryBusinessListingsRequest | undefined;
    setQueryBusinessListings(value?: economy_v1_economy_pb.QueryBusinessListingsRequest): PlayerAction;

    getActionCase(): PlayerAction.ActionCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): PlayerAction.AsObject;
    static toObject(includeInstance: boolean, msg: PlayerAction): PlayerAction.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: PlayerAction, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): PlayerAction;
    static deserializeBinaryFromReader(message: PlayerAction, reader: jspb.BinaryReader): PlayerAction;
}

export namespace PlayerAction {
    export type AsObject = {
        requestUuid: string,
        getCorporation?: economy_v1_economy_pb.GetCorporationRequest.AsObject,
        spawnUnit?: warfare_v1_warfare_pb.SpawnUnitRequest.AsObject,
        listUnit?: warfare_v1_warfare_pb.ListUnitsRequest.AsObject,
        acquireListedBusiness?: economy_v1_economy_pb.AcquireListedBusinessRequest.AsObject,
        queryBusinessListings?: economy_v1_economy_pb.QueryBusinessListingsRequest.AsObject,
    }

    export enum ActionCase {
        ACTION_NOT_SET = 0,
        GET_CORPORATION = 2,
        SPAWN_UNIT = 3,
        LIST_UNIT = 4,
        ACQUIRE_LISTED_BUSINESS = 5,
        QUERY_BUSINESS_LISTINGS = 6,
    }

}

export class GameUpdate extends jspb.Message { 
    getGameTick(): number;
    setGameTick(value: number): GameUpdate;

    hasActionInitResponse(): boolean;
    clearActionInitResponse(): void;
    getActionInitResponse(): interface_v1_shared_pb.ActionInitResponse | undefined;
    setActionInitResponse(value?: interface_v1_shared_pb.ActionInitResponse): GameUpdate;

    hasActionFailedResponse(): boolean;
    clearActionFailedResponse(): void;
    getActionFailedResponse(): ActionFailedResponse | undefined;
    setActionFailedResponse(value?: ActionFailedResponse): GameUpdate;

    hasRateLimitExceeded(): boolean;
    clearRateLimitExceeded(): void;
    getRateLimitExceeded(): RateLimitExceededNotification | undefined;
    setRateLimitExceeded(value?: RateLimitExceededNotification): GameUpdate;

    hasTickNotification(): boolean;
    clearTickNotification(): void;
    getTickNotification(): TickNotification | undefined;
    setTickNotification(value?: TickNotification): GameUpdate;

    hasGetCorporation(): boolean;
    clearGetCorporation(): void;
    getGetCorporation(): economy_v1_economy_pb.GetCorporationResponse | undefined;
    setGetCorporation(value?: economy_v1_economy_pb.GetCorporationResponse): GameUpdate;

    hasListUnits(): boolean;
    clearListUnits(): void;
    getListUnits(): warfare_v1_warfare_pb.ListUnitsResponse | undefined;
    setListUnits(value?: warfare_v1_warfare_pb.ListUnitsResponse): GameUpdate;

    hasSpawnUnit(): boolean;
    clearSpawnUnit(): void;
    getSpawnUnit(): warfare_v1_warfare_pb.SpawnUnitResponse | undefined;
    setSpawnUnit(value?: warfare_v1_warfare_pb.SpawnUnitResponse): GameUpdate;

    hasAcquireListedBusiness(): boolean;
    clearAcquireListedBusiness(): void;
    getAcquireListedBusiness(): economy_v1_economy_pb.AcquireListedBusinessResponse | undefined;
    setAcquireListedBusiness(value?: economy_v1_economy_pb.AcquireListedBusinessResponse): GameUpdate;

    hasQueryBusinessListings(): boolean;
    clearQueryBusinessListings(): void;
    getQueryBusinessListings(): economy_v1_economy_pb.QueryBusinessListingsResponse | undefined;
    setQueryBusinessListings(value?: economy_v1_economy_pb.QueryBusinessListingsResponse): GameUpdate;

    hasCreateCorporation(): boolean;
    clearCreateCorporation(): void;
    getCreateCorporation(): economy_v1_economy_pb.CreateCorporationResponse | undefined;
    setCreateCorporation(value?: economy_v1_economy_pb.CreateCorporationResponse): GameUpdate;

    hasDeleteCorporation(): boolean;
    clearDeleteCorporation(): void;
    getDeleteCorporation(): economy_v1_economy_pb.DeleteCorporationResponse | undefined;
    setDeleteCorporation(value?: economy_v1_economy_pb.DeleteCorporationResponse): GameUpdate;

    getUpdateCase(): GameUpdate.UpdateCase;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GameUpdate.AsObject;
    static toObject(includeInstance: boolean, msg: GameUpdate): GameUpdate.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GameUpdate, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GameUpdate;
    static deserializeBinaryFromReader(message: GameUpdate, reader: jspb.BinaryReader): GameUpdate;
}

export namespace GameUpdate {
    export type AsObject = {
        gameTick: number,
        actionInitResponse?: interface_v1_shared_pb.ActionInitResponse.AsObject,
        actionFailedResponse?: ActionFailedResponse.AsObject,
        rateLimitExceeded?: RateLimitExceededNotification.AsObject,
        tickNotification?: TickNotification.AsObject,
        getCorporation?: economy_v1_economy_pb.GetCorporationResponse.AsObject,
        listUnits?: warfare_v1_warfare_pb.ListUnitsResponse.AsObject,
        spawnUnit?: warfare_v1_warfare_pb.SpawnUnitResponse.AsObject,
        acquireListedBusiness?: economy_v1_economy_pb.AcquireListedBusinessResponse.AsObject,
        queryBusinessListings?: economy_v1_economy_pb.QueryBusinessListingsResponse.AsObject,
        createCorporation?: economy_v1_economy_pb.CreateCorporationResponse.AsObject,
        deleteCorporation?: economy_v1_economy_pb.DeleteCorporationResponse.AsObject,
    }

    export enum UpdateCase {
        UPDATE_NOT_SET = 0,
        ACTION_INIT_RESPONSE = 2,
        ACTION_FAILED_RESPONSE = 3,
        RATE_LIMIT_EXCEEDED = 4,
        TICK_NOTIFICATION = 5,
        GET_CORPORATION = 6,
        LIST_UNITS = 7,
        SPAWN_UNIT = 8,
        ACQUIRE_LISTED_BUSINESS = 9,
        QUERY_BUSINESS_LISTINGS = 10,
        CREATE_CORPORATION = 11,
        DELETE_CORPORATION = 12,
    }

}

export class ActionFailedResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): ActionFailedResponse;
    getReason(): string;
    setReason(value: string): ActionFailedResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ActionFailedResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ActionFailedResponse): ActionFailedResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ActionFailedResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ActionFailedResponse;
    static deserializeBinaryFromReader(message: ActionFailedResponse, reader: jspb.BinaryReader): ActionFailedResponse;
}

export namespace ActionFailedResponse {
    export type AsObject = {
        requestUuid: string,
        reason: string,
    }
}

export class RateLimitExceededNotification extends jspb.Message { 
    getMessage(): string;
    setMessage(value: string): RateLimitExceededNotification;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RateLimitExceededNotification.AsObject;
    static toObject(includeInstance: boolean, msg: RateLimitExceededNotification): RateLimitExceededNotification.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RateLimitExceededNotification, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RateLimitExceededNotification;
    static deserializeBinaryFromReader(message: RateLimitExceededNotification, reader: jspb.BinaryReader): RateLimitExceededNotification;
}

export namespace RateLimitExceededNotification {
    export type AsObject = {
        message: string,
    }
}

export class TickNotification extends jspb.Message { 

    hasEffectiveAt(): boolean;
    clearEffectiveAt(): void;
    getEffectiveAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
    setEffectiveAt(value?: google_protobuf_timestamp_pb.Timestamp): TickNotification;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): TickNotification.AsObject;
    static toObject(includeInstance: boolean, msg: TickNotification): TickNotification.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: TickNotification, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): TickNotification;
    static deserializeBinaryFromReader(message: TickNotification, reader: jspb.BinaryReader): TickNotification;
}

export namespace TickNotification {
    export type AsObject = {
        effectiveAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    }
}
