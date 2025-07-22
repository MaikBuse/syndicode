(() => {
  // pbf-fallback.ts
  addEventListener("fetch", (event) => {
    event.respondWith(handleRequest(event.request));
  });
  function getCorsHeaders(request) {
    const headers = new Headers();
    const origin = request.headers.get("Origin");
    const allowedOrigins = [
      "https://syndicode.dev",
      "https://www.syndicode.dev",
      "http://localhost:3000"
    ];
    if (origin && allowedOrigins.includes(origin)) {
      headers.set("Access-Control-Allow-Origin", origin);
      headers.set("Access-Control-Allow-Methods", "GET, HEAD, OPTIONS");
      headers.set("Access-Control-Allow-Headers", "*");
      headers.set("Access-Control-Max-Age", "86400");
    }
    return headers;
  }
  async function handleRequest(request) {
    const url = new URL(request.url);
    const key = url.pathname.slice(1);
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: getCorsHeaders(request),
        status: 204
      });
    }
    try {
      const object = await ASSETS_BUCKET.get(key);
      if (object) {
        const headers2 = new Headers();
        object.writeHttpMetadata(headers2);
        headers2.set("etag", object.etag);
        const corsHeaders2 = getCorsHeaders(request);
        corsHeaders2.forEach((value, key2) => {
          headers2.set(key2, value);
        });
        return new Response(object.body, {
          headers: headers2
        });
      }
      if (url.pathname.endsWith(".pbf")) {
        const emptyTile = await ASSETS_BUCKET.get("map/buildings/empty-tile.pbf");
        if (emptyTile) {
          const headers3 = new Headers();
          headers3.set("Content-Type", "application/x-protobuf");
          headers3.set("Cache-Control", "public, max-age=3600");
          const corsHeaders3 = getCorsHeaders(request);
          corsHeaders3.forEach((value, key2) => {
            headers3.set(key2, value);
          });
          return new Response(emptyTile.body, {
            headers: headers3,
            status: 200
          });
        }
        const headers2 = new Headers({
          "Content-Type": "text/plain"
        });
        const corsHeaders2 = getCorsHeaders(request);
        corsHeaders2.forEach((value, key2) => {
          headers2.set(key2, value);
        });
        return new Response("Empty tile not found", {
          headers: headers2,
          status: 404
        });
      }
      const headers = new Headers({
        "Content-Type": "text/plain"
      });
      const corsHeaders = getCorsHeaders(request);
      corsHeaders.forEach((value, key2) => {
        headers.set(key2, value);
      });
      return new Response("File not found", {
        status: 404,
        headers
      });
    } catch (error) {
      console.error("Error in pbf-fallback worker:", error);
      const headers = new Headers({
        "Content-Type": "text/plain"
      });
      const corsHeaders = getCorsHeaders(request);
      corsHeaders.forEach((value, key2) => {
        headers.set(key2, value);
      });
      return new Response("Internal server error", {
        status: 500,
        headers
      });
    }
  }
})();
