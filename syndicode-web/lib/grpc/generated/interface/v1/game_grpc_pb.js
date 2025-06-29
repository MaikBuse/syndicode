// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var interface_v1_game_pb = require('../../interface/v1/game_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var economy_v1_economy_pb = require('../../economy/v1/economy_pb.js');
var warfare_v1_warfare_pb = require('../../warfare/v1/warfare_pb.js');
var interface_v1_shared_pb = require('../../interface/v1/shared_pb.js');

function serialize_syndicode_interface_v1_GameUpdate(arg) {
  if (!(arg instanceof interface_v1_game_pb.GameUpdate)) {
    throw new Error('Expected argument of type syndicode_interface_v1.GameUpdate');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_GameUpdate(buffer_arg) {
  return interface_v1_game_pb.GameUpdate.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_PlayerAction(arg) {
  if (!(arg instanceof interface_v1_game_pb.PlayerAction)) {
    throw new Error('Expected argument of type syndicode_interface_v1.PlayerAction');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_PlayerAction(buffer_arg) {
  return interface_v1_game_pb.PlayerAction.deserializeBinary(new Uint8Array(buffer_arg));
}


// Main entry point for player actions and game updates.
var GameServiceService = exports.GameServiceService = {
  // Bidirectional stream where players send actions and receive updates.
playStream: {
    path: '/syndicode_interface_v1.GameService/PlayStream',
    requestStream: true,
    responseStream: true,
    requestType: interface_v1_game_pb.PlayerAction,
    responseType: interface_v1_game_pb.GameUpdate,
    requestSerialize: serialize_syndicode_interface_v1_PlayerAction,
    requestDeserialize: deserialize_syndicode_interface_v1_PlayerAction,
    responseSerialize: serialize_syndicode_interface_v1_GameUpdate,
    responseDeserialize: deserialize_syndicode_interface_v1_GameUpdate,
  },
};

exports.GameServiceClient = grpc.makeGenericClientConstructor(GameServiceService, 'GameService');
