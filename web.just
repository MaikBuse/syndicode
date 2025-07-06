help:
  @echo "[web]"
  @just --list web --list-heading ""


# Run the web app
[group('npm')]
run:
  npm run dev --prefix syndicode-web
