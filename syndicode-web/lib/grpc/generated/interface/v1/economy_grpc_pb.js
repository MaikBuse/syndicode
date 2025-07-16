// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var economy_v1_economy_pb = require('../../economy/v1/economy_pb.js');

function serialize_syndicode_economy_v1_Corporation(arg) {
  if (!(arg instanceof economy_v1_economy_pb.Corporation)) {
    throw new Error('Expected argument of type syndicode_economy_v1.Corporation');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_Corporation(buffer_arg) {
  return economy_v1_economy_pb.Corporation.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_economy_v1_GetCorporationRequest(arg) {
  if (!(arg instanceof economy_v1_economy_pb.GetCorporationRequest)) {
    throw new Error('Expected argument of type syndicode_economy_v1.GetCorporationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_GetCorporationRequest(buffer_arg) {
  return economy_v1_economy_pb.GetCorporationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_economy_v1_QueryBuildingsRequest(arg) {
  if (!(arg instanceof economy_v1_economy_pb.QueryBuildingsRequest)) {
    throw new Error('Expected argument of type syndicode_economy_v1.QueryBuildingsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_QueryBuildingsRequest(buffer_arg) {
  return economy_v1_economy_pb.QueryBuildingsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_economy_v1_QueryBuildingsResponse(arg) {
  if (!(arg instanceof economy_v1_economy_pb.QueryBuildingsResponse)) {
    throw new Error('Expected argument of type syndicode_economy_v1.QueryBuildingsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_QueryBuildingsResponse(buffer_arg) {
  return economy_v1_economy_pb.QueryBuildingsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_economy_v1_QueryBusinessesRequest(arg) {
  if (!(arg instanceof economy_v1_economy_pb.QueryBusinessesRequest)) {
    throw new Error('Expected argument of type syndicode_economy_v1.QueryBusinessesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_QueryBusinessesRequest(buffer_arg) {
  return economy_v1_economy_pb.QueryBusinessesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_economy_v1_QueryBusinessesResponse(arg) {
  if (!(arg instanceof economy_v1_economy_pb.QueryBusinessesResponse)) {
    throw new Error('Expected argument of type syndicode_economy_v1.QueryBusinessesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_QueryBusinessesResponse(buffer_arg) {
  return economy_v1_economy_pb.QueryBusinessesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// EconomyService provides methods for querying economy-related data.
var EconomyServiceService = exports.EconomyServiceService = {
  // Request to fetch corporation data.
getCurrentCorporation: {
    path: '/syndicode_interface_v1.EconomyService/GetCurrentCorporation',
    requestStream: false,
    responseStream: false,
    requestType: economy_v1_economy_pb.GetCorporationRequest,
    responseType: economy_v1_economy_pb.Corporation,
    requestSerialize: serialize_syndicode_economy_v1_GetCorporationRequest,
    requestDeserialize: deserialize_syndicode_economy_v1_GetCorporationRequest,
    responseSerialize: serialize_syndicode_economy_v1_Corporation,
    responseDeserialize: deserialize_syndicode_economy_v1_Corporation,
  },
  // Queries buildings with optional filters and pagination.
queryBuildings: {
    path: '/syndicode_interface_v1.EconomyService/QueryBuildings',
    requestStream: false,
    responseStream: false,
    requestType: economy_v1_economy_pb.QueryBuildingsRequest,
    responseType: economy_v1_economy_pb.QueryBuildingsResponse,
    requestSerialize: serialize_syndicode_economy_v1_QueryBuildingsRequest,
    requestDeserialize: deserialize_syndicode_economy_v1_QueryBuildingsRequest,
    responseSerialize: serialize_syndicode_economy_v1_QueryBuildingsResponse,
    responseDeserialize: deserialize_syndicode_economy_v1_QueryBuildingsResponse,
  },
  // Queries businesses with optional filters and pagination.
queryBusinesses: {
    path: '/syndicode_interface_v1.EconomyService/QueryBusinesses',
    requestStream: false,
    responseStream: false,
    requestType: economy_v1_economy_pb.QueryBusinessesRequest,
    responseType: economy_v1_economy_pb.QueryBusinessesResponse,
    requestSerialize: serialize_syndicode_economy_v1_QueryBusinessesRequest,
    requestDeserialize: deserialize_syndicode_economy_v1_QueryBusinessesRequest,
    responseSerialize: serialize_syndicode_economy_v1_QueryBusinessesResponse,
    responseDeserialize: deserialize_syndicode_economy_v1_QueryBusinessesResponse,
  },
};

exports.EconomyServiceClient = grpc.makeGenericClientConstructor(EconomyServiceService, 'EconomyService');
