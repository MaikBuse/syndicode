// package: syndicode_economy_v1
// file: economy/v1/economy.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";
import * as google_protobuf_wrappers_pb from "google-protobuf/google/protobuf/wrappers_pb";

export class QueryBuildingsRequest extends jspb.Message { 

    hasOwningCorporationUuid(): boolean;
    clearOwningCorporationUuid(): void;
    getOwningCorporationUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setOwningCorporationUuid(value?: google_protobuf_wrappers_pb.StringValue): QueryBuildingsRequest;

    hasMinLon(): boolean;
    clearMinLon(): void;
    getMinLon(): google_protobuf_wrappers_pb.DoubleValue | undefined;
    setMinLon(value?: google_protobuf_wrappers_pb.DoubleValue): QueryBuildingsRequest;

    hasMaxLon(): boolean;
    clearMaxLon(): void;
    getMaxLon(): google_protobuf_wrappers_pb.DoubleValue | undefined;
    setMaxLon(value?: google_protobuf_wrappers_pb.DoubleValue): QueryBuildingsRequest;

    hasMinLat(): boolean;
    clearMinLat(): void;
    getMinLat(): google_protobuf_wrappers_pb.DoubleValue | undefined;
    setMinLat(value?: google_protobuf_wrappers_pb.DoubleValue): QueryBuildingsRequest;

    hasMaxLat(): boolean;
    clearMaxLat(): void;
    getMaxLat(): google_protobuf_wrappers_pb.DoubleValue | undefined;
    setMaxLat(value?: google_protobuf_wrappers_pb.DoubleValue): QueryBuildingsRequest;

    hasLimit(): boolean;
    clearLimit(): void;
    getLimit(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setLimit(value?: google_protobuf_wrappers_pb.Int64Value): QueryBuildingsRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryBuildingsRequest.AsObject;
    static toObject(includeInstance: boolean, msg: QueryBuildingsRequest): QueryBuildingsRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryBuildingsRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryBuildingsRequest;
    static deserializeBinaryFromReader(message: QueryBuildingsRequest, reader: jspb.BinaryReader): QueryBuildingsRequest;
}

export namespace QueryBuildingsRequest {
    export type AsObject = {
        owningCorporationUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        minLon?: google_protobuf_wrappers_pb.DoubleValue.AsObject,
        maxLon?: google_protobuf_wrappers_pb.DoubleValue.AsObject,
        minLat?: google_protobuf_wrappers_pb.DoubleValue.AsObject,
        maxLat?: google_protobuf_wrappers_pb.DoubleValue.AsObject,
        limit?: google_protobuf_wrappers_pb.Int64Value.AsObject,
    }
}

export class BuildingDetails extends jspb.Message { 
    getGmlId(): string;
    setGmlId(value: string): BuildingDetails;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BuildingDetails.AsObject;
    static toObject(includeInstance: boolean, msg: BuildingDetails): BuildingDetails.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BuildingDetails, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BuildingDetails;
    static deserializeBinaryFromReader(message: BuildingDetails, reader: jspb.BinaryReader): BuildingDetails;
}

export namespace BuildingDetails {
    export type AsObject = {
        gmlId: string,
    }
}

export class QueryBuildingsResponse extends jspb.Message { 
    getGameTick(): number;
    setGameTick(value: number): QueryBuildingsResponse;
    clearBuildingsList(): void;
    getBuildingsList(): Array<BuildingDetails>;
    setBuildingsList(value: Array<BuildingDetails>): QueryBuildingsResponse;
    addBuildings(value?: BuildingDetails, index?: number): BuildingDetails;
    getTotalCount(): number;
    setTotalCount(value: number): QueryBuildingsResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryBuildingsResponse.AsObject;
    static toObject(includeInstance: boolean, msg: QueryBuildingsResponse): QueryBuildingsResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryBuildingsResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryBuildingsResponse;
    static deserializeBinaryFromReader(message: QueryBuildingsResponse, reader: jspb.BinaryReader): QueryBuildingsResponse;
}

export namespace QueryBuildingsResponse {
    export type AsObject = {
        gameTick: number,
        buildingsList: Array<BuildingDetails.AsObject>,
        totalCount: number,
    }
}

export class QueryBusinessListingsRequest extends jspb.Message { 

    hasMinAskingPrice(): boolean;
    clearMinAskingPrice(): void;
    getMinAskingPrice(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setMinAskingPrice(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessListingsRequest;

    hasMaxAskingPrice(): boolean;
    clearMaxAskingPrice(): void;
    getMaxAskingPrice(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setMaxAskingPrice(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessListingsRequest;

    hasSellerCorporationUuid(): boolean;
    clearSellerCorporationUuid(): void;
    getSellerCorporationUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setSellerCorporationUuid(value?: google_protobuf_wrappers_pb.StringValue): QueryBusinessListingsRequest;

    hasMarketUuid(): boolean;
    clearMarketUuid(): void;
    getMarketUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setMarketUuid(value?: google_protobuf_wrappers_pb.StringValue): QueryBusinessListingsRequest;

    hasMinOperationalExpenses(): boolean;
    clearMinOperationalExpenses(): void;
    getMinOperationalExpenses(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setMinOperationalExpenses(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessListingsRequest;

    hasMaxOperationalExpenses(): boolean;
    clearMaxOperationalExpenses(): void;
    getMaxOperationalExpenses(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setMaxOperationalExpenses(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessListingsRequest;
    getSortBy(): BusinessListingSortBy;
    setSortBy(value: BusinessListingSortBy): QueryBusinessListingsRequest;
    getSortDirection(): interface_v1_shared_pb.SortDirection;
    setSortDirection(value: interface_v1_shared_pb.SortDirection): QueryBusinessListingsRequest;

    hasLimit(): boolean;
    clearLimit(): void;
    getLimit(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setLimit(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessListingsRequest;

    hasOffset(): boolean;
    clearOffset(): void;
    getOffset(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setOffset(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessListingsRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryBusinessListingsRequest.AsObject;
    static toObject(includeInstance: boolean, msg: QueryBusinessListingsRequest): QueryBusinessListingsRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryBusinessListingsRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryBusinessListingsRequest;
    static deserializeBinaryFromReader(message: QueryBusinessListingsRequest, reader: jspb.BinaryReader): QueryBusinessListingsRequest;
}

export namespace QueryBusinessListingsRequest {
    export type AsObject = {
        minAskingPrice?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        maxAskingPrice?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        sellerCorporationUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        marketUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        minOperationalExpenses?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        maxOperationalExpenses?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        sortBy: BusinessListingSortBy,
        sortDirection: interface_v1_shared_pb.SortDirection,
        limit?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        offset?: google_protobuf_wrappers_pb.Int64Value.AsObject,
    }
}

export class QueryBusinessesRequest extends jspb.Message { 

    hasOwningCorporationUuid(): boolean;
    clearOwningCorporationUuid(): void;
    getOwningCorporationUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setOwningCorporationUuid(value?: google_protobuf_wrappers_pb.StringValue): QueryBusinessesRequest;

    hasMarketUuid(): boolean;
    clearMarketUuid(): void;
    getMarketUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setMarketUuid(value?: google_protobuf_wrappers_pb.StringValue): QueryBusinessesRequest;

    hasMinOperationalExpenses(): boolean;
    clearMinOperationalExpenses(): void;
    getMinOperationalExpenses(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setMinOperationalExpenses(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessesRequest;

    hasMaxOperationalExpenses(): boolean;
    clearMaxOperationalExpenses(): void;
    getMaxOperationalExpenses(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setMaxOperationalExpenses(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessesRequest;
    getSortBy(): BusinessSortBy;
    setSortBy(value: BusinessSortBy): QueryBusinessesRequest;
    getSortDirection(): interface_v1_shared_pb.SortDirection;
    setSortDirection(value: interface_v1_shared_pb.SortDirection): QueryBusinessesRequest;

    hasLimit(): boolean;
    clearLimit(): void;
    getLimit(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setLimit(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessesRequest;

    hasOffset(): boolean;
    clearOffset(): void;
    getOffset(): google_protobuf_wrappers_pb.Int64Value | undefined;
    setOffset(value?: google_protobuf_wrappers_pb.Int64Value): QueryBusinessesRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryBusinessesRequest.AsObject;
    static toObject(includeInstance: boolean, msg: QueryBusinessesRequest): QueryBusinessesRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryBusinessesRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryBusinessesRequest;
    static deserializeBinaryFromReader(message: QueryBusinessesRequest, reader: jspb.BinaryReader): QueryBusinessesRequest;
}

export namespace QueryBusinessesRequest {
    export type AsObject = {
        owningCorporationUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        marketUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        minOperationalExpenses?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        maxOperationalExpenses?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        sortBy: BusinessSortBy,
        sortDirection: interface_v1_shared_pb.SortDirection,
        limit?: google_protobuf_wrappers_pb.Int64Value.AsObject,
        offset?: google_protobuf_wrappers_pb.Int64Value.AsObject,
    }
}

export class BusinessDetails extends jspb.Message { 
    getBusinessUuid(): string;
    setBusinessUuid(value: string): BusinessDetails;
    getBusinessName(): string;
    setBusinessName(value: string): BusinessDetails;

    hasOwningCorporationUuid(): boolean;
    clearOwningCorporationUuid(): void;
    getOwningCorporationUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setOwningCorporationUuid(value?: google_protobuf_wrappers_pb.StringValue): BusinessDetails;
    getMarketUuid(): string;
    setMarketUuid(value: string): BusinessDetails;
    getOperationalExpenses(): number;
    setOperationalExpenses(value: number): BusinessDetails;
    getHeadquarterBuildingUuid(): string;
    setHeadquarterBuildingUuid(value: string): BusinessDetails;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BusinessDetails.AsObject;
    static toObject(includeInstance: boolean, msg: BusinessDetails): BusinessDetails.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BusinessDetails, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BusinessDetails;
    static deserializeBinaryFromReader(message: BusinessDetails, reader: jspb.BinaryReader): BusinessDetails;
}

export namespace BusinessDetails {
    export type AsObject = {
        businessUuid: string,
        businessName: string,
        owningCorporationUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        marketUuid: string,
        operationalExpenses: number,
        headquarterBuildingUuid: string,
    }
}

export class QueryBusinessesResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): QueryBusinessesResponse;
    clearBusinessesList(): void;
    getBusinessesList(): Array<BusinessDetails>;
    setBusinessesList(value: Array<BusinessDetails>): QueryBusinessesResponse;
    addBusinesses(value?: BusinessDetails, index?: number): BusinessDetails;
    getTotalCount(): number;
    setTotalCount(value: number): QueryBusinessesResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryBusinessesResponse.AsObject;
    static toObject(includeInstance: boolean, msg: QueryBusinessesResponse): QueryBusinessesResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryBusinessesResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryBusinessesResponse;
    static deserializeBinaryFromReader(message: QueryBusinessesResponse, reader: jspb.BinaryReader): QueryBusinessesResponse;
}

export namespace QueryBusinessesResponse {
    export type AsObject = {
        requestUuid: string,
        businessesList: Array<BusinessDetails.AsObject>,
        totalCount: number,
    }
}

export class BusinessListingDetails extends jspb.Message { 
    getListingUuid(): string;
    setListingUuid(value: string): BusinessListingDetails;
    getBusinessUuid(): string;
    setBusinessUuid(value: string): BusinessListingDetails;
    getBusinessName(): string;
    setBusinessName(value: string): BusinessListingDetails;

    hasSellerCorporationUuid(): boolean;
    clearSellerCorporationUuid(): void;
    getSellerCorporationUuid(): google_protobuf_wrappers_pb.StringValue | undefined;
    setSellerCorporationUuid(value?: google_protobuf_wrappers_pb.StringValue): BusinessListingDetails;
    getMarketUuid(): string;
    setMarketUuid(value: string): BusinessListingDetails;
    getAskingPrice(): number;
    setAskingPrice(value: number): BusinessListingDetails;
    getOperationalExpenses(): number;
    setOperationalExpenses(value: number): BusinessListingDetails;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): BusinessListingDetails.AsObject;
    static toObject(includeInstance: boolean, msg: BusinessListingDetails): BusinessListingDetails.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: BusinessListingDetails, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): BusinessListingDetails;
    static deserializeBinaryFromReader(message: BusinessListingDetails, reader: jspb.BinaryReader): BusinessListingDetails;
}

export namespace BusinessListingDetails {
    export type AsObject = {
        listingUuid: string,
        businessUuid: string,
        businessName: string,
        sellerCorporationUuid?: google_protobuf_wrappers_pb.StringValue.AsObject,
        marketUuid: string,
        askingPrice: number,
        operationalExpenses: number,
    }
}

export class QueryBusinessListingsResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): QueryBusinessListingsResponse;
    clearListingsList(): void;
    getListingsList(): Array<BusinessListingDetails>;
    setListingsList(value: Array<BusinessListingDetails>): QueryBusinessListingsResponse;
    addListings(value?: BusinessListingDetails, index?: number): BusinessListingDetails;
    getTotalCount(): number;
    setTotalCount(value: number): QueryBusinessListingsResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryBusinessListingsResponse.AsObject;
    static toObject(includeInstance: boolean, msg: QueryBusinessListingsResponse): QueryBusinessListingsResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryBusinessListingsResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryBusinessListingsResponse;
    static deserializeBinaryFromReader(message: QueryBusinessListingsResponse, reader: jspb.BinaryReader): QueryBusinessListingsResponse;
}

export namespace QueryBusinessListingsResponse {
    export type AsObject = {
        requestUuid: string,
        listingsList: Array<BusinessListingDetails.AsObject>,
        totalCount: number,
    }
}

export class GetCorporationRequest extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetCorporationRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetCorporationRequest): GetCorporationRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetCorporationRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetCorporationRequest;
    static deserializeBinaryFromReader(message: GetCorporationRequest, reader: jspb.BinaryReader): GetCorporationRequest;
}

export namespace GetCorporationRequest {
    export type AsObject = {
    }
}

export class GetCorporationResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): GetCorporationResponse;

    hasCorporation(): boolean;
    clearCorporation(): void;
    getCorporation(): Corporation | undefined;
    setCorporation(value?: Corporation): GetCorporationResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetCorporationResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetCorporationResponse): GetCorporationResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetCorporationResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetCorporationResponse;
    static deserializeBinaryFromReader(message: GetCorporationResponse, reader: jspb.BinaryReader): GetCorporationResponse;
}

export namespace GetCorporationResponse {
    export type AsObject = {
        requestUuid: string,
        corporation?: Corporation.AsObject,
    }
}

export class CreateCorporationResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): CreateCorporationResponse;

    hasCorporation(): boolean;
    clearCorporation(): void;
    getCorporation(): Corporation | undefined;
    setCorporation(value?: Corporation): CreateCorporationResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): CreateCorporationResponse.AsObject;
    static toObject(includeInstance: boolean, msg: CreateCorporationResponse): CreateCorporationResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: CreateCorporationResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): CreateCorporationResponse;
    static deserializeBinaryFromReader(message: CreateCorporationResponse, reader: jspb.BinaryReader): CreateCorporationResponse;
}

export namespace CreateCorporationResponse {
    export type AsObject = {
        requestUuid: string,
        corporation?: Corporation.AsObject,
    }
}

export class DeleteCorporationResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): DeleteCorporationResponse;
    getUserUuid(): string;
    setUserUuid(value: string): DeleteCorporationResponse;
    getCorporationUuid(): string;
    setCorporationUuid(value: string): DeleteCorporationResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeleteCorporationResponse.AsObject;
    static toObject(includeInstance: boolean, msg: DeleteCorporationResponse): DeleteCorporationResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeleteCorporationResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeleteCorporationResponse;
    static deserializeBinaryFromReader(message: DeleteCorporationResponse, reader: jspb.BinaryReader): DeleteCorporationResponse;
}

export namespace DeleteCorporationResponse {
    export type AsObject = {
        requestUuid: string,
        userUuid: string,
        corporationUuid: string,
    }
}

export class AcquireListedBusinessRequest extends jspb.Message { 
    getBusinessListingUuid(): string;
    setBusinessListingUuid(value: string): AcquireListedBusinessRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AcquireListedBusinessRequest.AsObject;
    static toObject(includeInstance: boolean, msg: AcquireListedBusinessRequest): AcquireListedBusinessRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AcquireListedBusinessRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AcquireListedBusinessRequest;
    static deserializeBinaryFromReader(message: AcquireListedBusinessRequest, reader: jspb.BinaryReader): AcquireListedBusinessRequest;
}

export namespace AcquireListedBusinessRequest {
    export type AsObject = {
        businessListingUuid: string,
    }
}

export class AcquireListedBusinessResponse extends jspb.Message { 
    getRequestUuid(): string;
    setRequestUuid(value: string): AcquireListedBusinessResponse;

    hasBusiness(): boolean;
    clearBusiness(): void;
    getBusiness(): Business | undefined;
    setBusiness(value?: Business): AcquireListedBusinessResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AcquireListedBusinessResponse.AsObject;
    static toObject(includeInstance: boolean, msg: AcquireListedBusinessResponse): AcquireListedBusinessResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AcquireListedBusinessResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AcquireListedBusinessResponse;
    static deserializeBinaryFromReader(message: AcquireListedBusinessResponse, reader: jspb.BinaryReader): AcquireListedBusinessResponse;
}

export namespace AcquireListedBusinessResponse {
    export type AsObject = {
        requestUuid: string,
        business?: Business.AsObject,
    }
}

export class Corporation extends jspb.Message { 
    getUuid(): string;
    setUuid(value: string): Corporation;
    getUserUuid(): string;
    setUserUuid(value: string): Corporation;
    getName(): string;
    setName(value: string): Corporation;
    getBalance(): number;
    setBalance(value: number): Corporation;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Corporation.AsObject;
    static toObject(includeInstance: boolean, msg: Corporation): Corporation.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Corporation, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Corporation;
    static deserializeBinaryFromReader(message: Corporation, reader: jspb.BinaryReader): Corporation;
}

export namespace Corporation {
    export type AsObject = {
        uuid: string,
        userUuid: string,
        name: string,
        balance: number,
    }
}

export class Business extends jspb.Message { 
    getUuid(): string;
    setUuid(value: string): Business;
    getMarketUuid(): string;
    setMarketUuid(value: string): Business;
    getOwningCorporationUuid(): string;
    setOwningCorporationUuid(value: string): Business;
    getName(): string;
    setName(value: string): Business;
    getOperationalExpenses(): number;
    setOperationalExpenses(value: number): Business;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Business.AsObject;
    static toObject(includeInstance: boolean, msg: Business): Business.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Business, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Business;
    static deserializeBinaryFromReader(message: Business, reader: jspb.BinaryReader): Business;
}

export namespace Business {
    export type AsObject = {
        uuid: string,
        marketUuid: string,
        owningCorporationUuid: string,
        name: string,
        operationalExpenses: number,
    }
}

export enum BusinessListingSortBy {
    SORT_BY_UNSPECIFIED = 0,
    PRICE = 1,
    NAME = 2,
    OPERATION_EXPENSES = 3,
    MARKET_VOLUME = 4,
}

export enum BusinessSortBy {
    BUSINESS_SORT_BY_UNSPECIFIED = 0,
    BUSINESS_NAME = 1,
    BUSINESS_OPERATION_EXPENSES = 2,
    BUSINESS_MARKET_VOLUME = 3,
}
