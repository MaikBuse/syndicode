help:
  @echo "[grpcurl]"
  @just --list grpcurl --list-heading ""

# Run a simple command such as 'list'
[group('reflection')]
cmd command:
  grpcurl -plaintext -H "${IP_ADDRESS_HEADER}: 127.0.0.1" 127.0.0.1:50051 {{command}}

[group('reflection')]
list:
  grpcurl -plaintext -H "${IP_ADDRESS_HEADER}: 127.0.0.1" -d '{}' 127.0.0.1:50051 list
