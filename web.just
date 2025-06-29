# Define variables for all our paths. Makes the command readable and easy to change.
PROTOC_PLUGIN := './syndicode-web/node_modules/.bin/protoc-gen-ts'
PROTO_SRC_DIR := './protos'
PROTO_GEN_DIR := './syndicode-web/lib/grpc/generated'

help:
  @echo "[web]"
  @just --list web --list-heading ""


# Run the web app
[group('npm')]
run:
  npm run dev --prefix syndicode-web


# Generate gRPC code.
[group('npm')]
grpc: _ensure-dir
    @echo "Generating gRPC client and types..."
    @./syndicode-web/node_modules/.bin/grpc_tools_node_protoc \
        --plugin=protoc-gen-ts={{PROTOC_PLUGIN}} \
        --js_out=import_style=commonjs,binary:{{PROTO_GEN_DIR}} \
        --grpc_out=grpc_js:{{PROTO_GEN_DIR}} \
        --ts_out=grpc_js:{{PROTO_GEN_DIR}} \
        --proto_path={{PROTO_SRC_DIR}} \
        `find {{PROTO_SRC_DIR}} -name '*.proto'`

# A "private" helper recipe to make sure the output directory exists.
# The `grpc` recipe runs this first because it's listed as a dependency.
_ensure-dir:
    @mkdir -p {{PROTO_GEN_DIR}}

# A handy recipe to clean up all generated files.
clean-grpc:
    @echo "Cleaning generated gRPC files..."
    @rm -rf {{PROTO_GEN_DIR}}
