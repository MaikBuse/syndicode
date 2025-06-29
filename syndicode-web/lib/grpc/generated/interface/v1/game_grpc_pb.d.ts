// package: syndicode_interface_v1
// file: interface/v1/game.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as interface_v1_game_pb from "../../interface/v1/game_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as economy_v1_economy_pb from "../../economy/v1/economy_pb";
import * as warfare_v1_warfare_pb from "../../warfare/v1/warfare_pb";
import * as interface_v1_shared_pb from "../../interface/v1/shared_pb";

interface IGameServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    playStream: IGameServiceService_IPlayStream;
}

interface IGameServiceService_IPlayStream extends grpc.MethodDefinition<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate> {
    path: "/syndicode_interface_v1.GameService/PlayStream";
    requestStream: true;
    responseStream: true;
    requestSerialize: grpc.serialize<interface_v1_game_pb.PlayerAction>;
    requestDeserialize: grpc.deserialize<interface_v1_game_pb.PlayerAction>;
    responseSerialize: grpc.serialize<interface_v1_game_pb.GameUpdate>;
    responseDeserialize: grpc.deserialize<interface_v1_game_pb.GameUpdate>;
}

export const GameServiceService: IGameServiceService;

export interface IGameServiceServer extends grpc.UntypedServiceImplementation {
    playStream: grpc.handleBidiStreamingCall<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate>;
}

export interface IGameServiceClient {
    playStream(): grpc.ClientDuplexStream<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate>;
    playStream(options: Partial<grpc.CallOptions>): grpc.ClientDuplexStream<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate>;
    playStream(metadata: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientDuplexStream<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate>;
}

export class GameServiceClient extends grpc.Client implements IGameServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public playStream(options?: Partial<grpc.CallOptions>): grpc.ClientDuplexStream<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate>;
    public playStream(metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientDuplexStream<interface_v1_game_pb.PlayerAction, interface_v1_game_pb.GameUpdate>;
}
