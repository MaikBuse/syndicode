declare const ASSETS_BUCKET: R2Bucket;

addEventListener('fetch', (event: FetchEvent) => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request: Request): Promise<Response> {
  const url = new URL(request.url);
  const key = url.pathname.slice(1); // Remove leading slash

  try {
    // Try to get the requested file from R2
    const object = await ASSETS_BUCKET.get(key);
    
    if (object) {
      // File exists, return it with appropriate headers
      const headers = new Headers();
      object.writeHttpMetadata(headers);
      headers.set('etag', object.etag);
      
      return new Response(object.body, {
        headers,
      });
    }

    // File doesn't exist, check if it's a PBF request
    if (url.pathname.endsWith('.pbf')) {
      // Try to get the default empty PBF file
      const defaultPbf = await ASSETS_BUCKET.get('default-empty.pbf');
      
      if (defaultPbf) {
        const headers = new Headers();
        headers.set('Content-Type', 'application/x-protobuf');
        headers.set('Cache-Control', 'public, max-age=3600');
        
        return new Response(defaultPbf.body, {
          headers,
          status: 200,
        });
      }
      
      // If no default PBF exists, create minimal empty PBF response
      const emptyPbf = new Uint8Array(0);
      return new Response(emptyPbf, {
        headers: {
          'Content-Type': 'application/x-protobuf',
          'Cache-Control': 'public, max-age=3600',
        },
        status: 200,
      });
    }

    // Not a PBF file, return 404
    return new Response('File not found', {
      status: 404,
      headers: {
        'Content-Type': 'text/plain',
      },
    });

  } catch (error) {
    console.error('Error in pbf-fallback worker:', error);
    return new Response('Internal server error', {
      status: 500,
      headers: {
        'Content-Type': 'text/plain',
      },
    });
  }
}