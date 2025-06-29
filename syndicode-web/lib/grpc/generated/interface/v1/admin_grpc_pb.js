// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var interface_v1_admin_pb = require('../../interface/v1/admin_pb.js');
var interface_v1_shared_pb = require('../../interface/v1/shared_pb.js');

function serialize_syndicode_interface_v1_CreateUserRequest(arg) {
  if (!(arg instanceof interface_v1_admin_pb.CreateUserRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.CreateUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_CreateUserRequest(buffer_arg) {
  return interface_v1_admin_pb.CreateUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_CreateUserResponse(arg) {
  if (!(arg instanceof interface_v1_admin_pb.CreateUserResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.CreateUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_CreateUserResponse(buffer_arg) {
  return interface_v1_admin_pb.CreateUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_DeleteUserRequest(arg) {
  if (!(arg instanceof interface_v1_admin_pb.DeleteUserRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.DeleteUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_DeleteUserRequest(buffer_arg) {
  return interface_v1_admin_pb.DeleteUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_DeleteUserResponse(arg) {
  if (!(arg instanceof interface_v1_admin_pb.DeleteUserResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.DeleteUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_DeleteUserResponse(buffer_arg) {
  return interface_v1_admin_pb.DeleteUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_GetUserRequest(arg) {
  if (!(arg instanceof interface_v1_admin_pb.GetUserRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.GetUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_GetUserRequest(buffer_arg) {
  return interface_v1_admin_pb.GetUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_GetUserResponse(arg) {
  if (!(arg instanceof interface_v1_shared_pb.GetUserResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.GetUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_GetUserResponse(buffer_arg) {
  return interface_v1_shared_pb.GetUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// Provides administrative operations
var AdminServiceService = exports.AdminServiceService = {
  // Creates a new user account.
createUser: {
    path: '/syndicode_interface_v1.AdminService/CreateUser',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_admin_pb.CreateUserRequest,
    responseType: interface_v1_admin_pb.CreateUserResponse,
    requestSerialize: serialize_syndicode_interface_v1_CreateUserRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_CreateUserRequest,
    responseSerialize: serialize_syndicode_interface_v1_CreateUserResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_CreateUserResponse,
  },
  // Retrieves a user by UUID.
getUser: {
    path: '/syndicode_interface_v1.AdminService/GetUser',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_admin_pb.GetUserRequest,
    responseType: interface_v1_shared_pb.GetUserResponse,
    requestSerialize: serialize_syndicode_interface_v1_GetUserRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_GetUserRequest,
    responseSerialize: serialize_syndicode_interface_v1_GetUserResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_GetUserResponse,
  },
  // Deletes an existing user by UUID.
deleteUser: {
    path: '/syndicode_interface_v1.AdminService/DeleteUser',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_admin_pb.DeleteUserRequest,
    responseType: interface_v1_admin_pb.DeleteUserResponse,
    requestSerialize: serialize_syndicode_interface_v1_DeleteUserRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_DeleteUserRequest,
    responseSerialize: serialize_syndicode_interface_v1_DeleteUserResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_DeleteUserResponse,
  },
};

exports.AdminServiceClient = grpc.makeGenericClientConstructor(AdminServiceService, 'AdminService');
