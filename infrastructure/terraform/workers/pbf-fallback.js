(() => {
  // pbf-fallback.ts
  addEventListener("fetch", (event) => {
    event.respondWith(handleRequest(event.request));
  });
  async function handleRequest(request) {
    const url = new URL(request.url);
    const key = url.pathname.slice(1);
    try {
      const object = await ASSETS_BUCKET.get(key);
      if (object) {
        const headers = new Headers();
        object.writeHttpMetadata(headers);
        headers.set("etag", object.etag);
        return new Response(object.body, {
          headers
        });
      }
      if (url.pathname.endsWith(".pbf")) {
        const defaultPbf = await ASSETS_BUCKET.get("default-empty.pbf");
        if (defaultPbf) {
          const headers = new Headers();
          headers.set("Content-Type", "application/x-protobuf");
          headers.set("Cache-Control", "public, max-age=3600");
          return new Response(defaultPbf.body, {
            headers,
            status: 200
          });
        }
        const emptyPbf = new Uint8Array(0);
        return new Response(emptyPbf, {
          headers: {
            "Content-Type": "application/x-protobuf",
            "Cache-Control": "public, max-age=3600"
          },
          status: 200
        });
      }
      return new Response("File not found", {
        status: 404,
        headers: {
          "Content-Type": "text/plain"
        }
      });
    } catch (error) {
      console.error("Error in pbf-fallback worker:", error);
      return new Response("Internal server error", {
        status: 500,
        headers: {
          "Content-Type": "text/plain"
        }
      });
    }
  }
})();
