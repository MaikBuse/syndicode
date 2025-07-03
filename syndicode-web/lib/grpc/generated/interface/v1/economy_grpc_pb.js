// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var economy_v1_economy_pb = require('../../economy/v1/economy_pb.js');

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


// EconomyService provides methods for querying economy-related data.
var EconomyServiceService = exports.EconomyServiceService = {
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
};

exports.EconomyServiceClient = grpc.makeGenericClientConstructor(EconomyServiceService, 'EconomyService');
