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
