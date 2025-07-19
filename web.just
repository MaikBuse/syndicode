help:
  @echo "[web]"
  @just --list web --list-heading ""


# Run the web app
[group('npm')]
run:
  npm run dev --prefix syndicode-web

# Run the web app and trigger initial compilation
[group('npm')]
run-compiled:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Starting Next.js dev server with initial compilation..."
  
  # Start server in background
  npm run dev --prefix syndicode-web &
  SERVER_PID=$!
  
  # Function to cleanup on exit
  cleanup() {
    echo "Stopping server..."
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
  }
  trap cleanup EXIT INT TERM
  
  # Wait for server to be ready
  echo "Waiting for server to start..."
  for i in {1..60}; do
    if curl -s -f http://localhost:3000 >/dev/null 2>&1; then
      echo "Server is up! Triggering initial compilation..."
      
      # Trigger compilation for main route
      curl -s http://localhost:3000/ >/dev/null 2>&1 || true
      
      echo "✅ Server ready with initial compilation at http://localhost:3000"
      break
    fi
    if [ $i -eq 60 ]; then
      echo "❌ Server failed to start within 60 seconds"
      exit 1
    fi
    sleep 1
  done
  
  echo "Press Ctrl+C to stop the server"
  wait $SERVER_PID

# Run linting and other checks
[group('npm')]
check:
  npm run pre-deploy --prefix syndicode-web

# Run unit tests
[group('npm')]
test:
  npm run test --prefix syndicode-web -- --run

# Run unit tests with UI
[group('npm')]
test-ui:
  npm run test:ui --prefix syndicode-web

# Run unit tests with coverage
[group('npm')]
test-coverage:
  npm run test:coverage --prefix syndicode-web

# Run E2E tests headlessly
[group('npm')]
test-e2e:
  npm run test:e2e --prefix syndicode-web

# Run E2E tests with UI for debugging
[group('npm')]
test-e2e-ui:
  npm run test:e2e:ui --prefix syndicode-web

# Run TypeScript type checking
[group('npm')]
type-check:
  npm run type-check --prefix syndicode-web

# Run ESLint
[group('npm')]
lint:
  npm run lint --prefix syndicode-web

# Run Prettier format check
[group('npm')]
format-check:
  npm run format:check --prefix syndicode-web

# Run specific test file
[group('npm')]
test-file file:
  npm exec --prefix syndicode-web -- playwright test {{file}}

# Turn geojson into pbf files to be distributed to clients
[group('map')]
tippecanoe:
  tippecanoe \
    --output-to-directory=tokyo-buildings \
    --name="Tokyo Special Wards Buildings" \
    --layer=buildings \
    --minimum-zoom=12 \
    --maximum-zoom=18 \
    --base-zoom=16 \
    --drop-smallest-as-needed \
    --minimum-detail=8 \
    --simplification=4 \
    --buffer=8 \
    --force \
    tokyo_special_wards.geojson

# Upload the pbf files to s3
[group('map')]
s3:
  aws s3 sync tokyo-buildings/ s3://syndicode-web-map-assets/tokyo-buildings/ \
      --content-type "application/x-protobuf" \
      --content-encoding "gzip" \
      --metadata-directive REPLACE \
      --exclude "*" \
      --include "*.pbf"
