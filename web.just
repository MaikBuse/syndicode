help:
  @echo "[web]"
  @just --list web --list-heading ""


# Run the web app
[group('npm')]
run:
  npm run dev --prefix syndicode-web

# Run linting and other checks
[group('npm')]
check:
  npm run pre-deploy --prefix syndicode-web

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
