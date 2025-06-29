// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var economy_v1_economy_pb = require('../../economy/v1/economy_pb.js');

function serialize_syndicode_economy_v1_BuildingOwnershipsResponse(arg) {
  if (!(arg instanceof economy_v1_economy_pb.BuildingOwnershipsResponse)) {
    throw new Error('Expected argument of type syndicode_economy_v1.BuildingOwnershipsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_BuildingOwnershipsResponse(buffer_arg) {
  return economy_v1_economy_pb.BuildingOwnershipsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_economy_v1_QueryBuildingOwnershipsRequest(arg) {
  if (!(arg instanceof economy_v1_economy_pb.QueryBuildingOwnershipsRequest)) {
    throw new Error('Expected argument of type syndicode_economy_v1.QueryBuildingOwnershipsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_economy_v1_QueryBuildingOwnershipsRequest(buffer_arg) {
  return economy_v1_economy_pb.QueryBuildingOwnershipsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}


// EconomyService provides methods for querying economy-related data.
var EconomyServiceService = exports.EconomyServiceService = {
  // Queries building ownerships with optional filters and pagination.
queryBuildingOwnerships: {
    path: '/syndicode_interface_v1.EconomyService/QueryBuildingOwnerships',
    requestStream: false,
    responseStream: false,
    requestType: economy_v1_economy_pb.QueryBuildingOwnershipsRequest,
    responseType: economy_v1_economy_pb.BuildingOwnershipsResponse,
    requestSerialize: serialize_syndicode_economy_v1_QueryBuildingOwnershipsRequest,
    requestDeserialize: deserialize_syndicode_economy_v1_QueryBuildingOwnershipsRequest,
    responseSerialize: serialize_syndicode_economy_v1_BuildingOwnershipsResponse,
    responseDeserialize: deserialize_syndicode_economy_v1_BuildingOwnershipsResponse,
  },
};

exports.EconomyServiceClient = grpc.makeGenericClientConstructor(EconomyServiceService, 'EconomyService');
