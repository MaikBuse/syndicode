// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var interface_v1_auth_pb = require('../../interface/v1/auth_pb.js');
var interface_v1_shared_pb = require('../../interface/v1/shared_pb.js');

function serialize_syndicode_interface_v1_GetCurrentUserRequest(arg) {
  if (!(arg instanceof interface_v1_auth_pb.GetCurrentUserRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.GetCurrentUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_GetCurrentUserRequest(buffer_arg) {
  return interface_v1_auth_pb.GetCurrentUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
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

function serialize_syndicode_interface_v1_LoginRequest(arg) {
  if (!(arg instanceof interface_v1_auth_pb.LoginRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.LoginRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_LoginRequest(buffer_arg) {
  return interface_v1_auth_pb.LoginRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_LoginResponse(arg) {
  if (!(arg instanceof interface_v1_auth_pb.LoginResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.LoginResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_LoginResponse(buffer_arg) {
  return interface_v1_auth_pb.LoginResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_RegisterRequest(arg) {
  if (!(arg instanceof interface_v1_auth_pb.RegisterRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.RegisterRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_RegisterRequest(buffer_arg) {
  return interface_v1_auth_pb.RegisterRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_RegisterResponse(arg) {
  if (!(arg instanceof interface_v1_auth_pb.RegisterResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.RegisterResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_RegisterResponse(buffer_arg) {
  return interface_v1_auth_pb.RegisterResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_ResendVerificationEmailRequest(arg) {
  if (!(arg instanceof interface_v1_auth_pb.ResendVerificationEmailRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.ResendVerificationEmailRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_ResendVerificationEmailRequest(buffer_arg) {
  return interface_v1_auth_pb.ResendVerificationEmailRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_ResendVerificationEmailResponse(arg) {
  if (!(arg instanceof interface_v1_auth_pb.ResendVerificationEmailResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.ResendVerificationEmailResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_ResendVerificationEmailResponse(buffer_arg) {
  return interface_v1_auth_pb.ResendVerificationEmailResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_VerifyUserRequest(arg) {
  if (!(arg instanceof interface_v1_auth_pb.VerifyUserRequest)) {
    throw new Error('Expected argument of type syndicode_interface_v1.VerifyUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_VerifyUserRequest(buffer_arg) {
  return interface_v1_auth_pb.VerifyUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_syndicode_interface_v1_VerifyUserResponse(arg) {
  if (!(arg instanceof interface_v1_auth_pb.VerifyUserResponse)) {
    throw new Error('Expected argument of type syndicode_interface_v1.VerifyUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_syndicode_interface_v1_VerifyUserResponse(buffer_arg) {
  return interface_v1_auth_pb.VerifyUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// Handles user authentication and registration.
var AuthServiceService = exports.AuthServiceService = {
  // Registers a new user and their corporation.
register: {
    path: '/syndicode_interface_v1.AuthService/Register',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_auth_pb.RegisterRequest,
    responseType: interface_v1_auth_pb.RegisterResponse,
    requestSerialize: serialize_syndicode_interface_v1_RegisterRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_RegisterRequest,
    responseSerialize: serialize_syndicode_interface_v1_RegisterResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_RegisterResponse,
  },
  verifyUser: {
    path: '/syndicode_interface_v1.AuthService/VerifyUser',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_auth_pb.VerifyUserRequest,
    responseType: interface_v1_auth_pb.VerifyUserResponse,
    requestSerialize: serialize_syndicode_interface_v1_VerifyUserRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_VerifyUserRequest,
    responseSerialize: serialize_syndicode_interface_v1_VerifyUserResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_VerifyUserResponse,
  },
  resendVerificationEmail: {
    path: '/syndicode_interface_v1.AuthService/ResendVerificationEmail',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_auth_pb.ResendVerificationEmailRequest,
    responseType: interface_v1_auth_pb.ResendVerificationEmailResponse,
    requestSerialize: serialize_syndicode_interface_v1_ResendVerificationEmailRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_ResendVerificationEmailRequest,
    responseSerialize: serialize_syndicode_interface_v1_ResendVerificationEmailResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_ResendVerificationEmailResponse,
  },
  // Authenticates a user and returns a JWT token.
login: {
    path: '/syndicode_interface_v1.AuthService/Login',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_auth_pb.LoginRequest,
    responseType: interface_v1_auth_pb.LoginResponse,
    requestSerialize: serialize_syndicode_interface_v1_LoginRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_LoginRequest,
    responseSerialize: serialize_syndicode_interface_v1_LoginResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_LoginResponse,
  },
  // Retrieves the currently authenticated user's information.
getCurrentUser: {
    path: '/syndicode_interface_v1.AuthService/GetCurrentUser',
    requestStream: false,
    responseStream: false,
    requestType: interface_v1_auth_pb.GetCurrentUserRequest,
    responseType: interface_v1_shared_pb.GetUserResponse,
    requestSerialize: serialize_syndicode_interface_v1_GetCurrentUserRequest,
    requestDeserialize: deserialize_syndicode_interface_v1_GetCurrentUserRequest,
    responseSerialize: serialize_syndicode_interface_v1_GetUserResponse,
    responseDeserialize: deserialize_syndicode_interface_v1_GetUserResponse,
  },
};

exports.AuthServiceClient = grpc.makeGenericClientConstructor(AuthServiceService, 'AuthService');
