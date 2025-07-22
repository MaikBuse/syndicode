# Infrastructure management with Terraform

help:
  @echo "[infra]"
  @just --list infra --list-heading ""

# Build TypeScript Worker
[group('worker')]
build-worker:
  cd infrastructure/terraform/workers && npx esbuild pbf-fallback.ts --bundle --format=iife --outfile=pbf-fallback.js --platform=neutral
