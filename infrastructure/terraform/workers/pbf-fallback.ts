declare const ASSETS_BUCKET: R2Bucket;

addEventListener('fetch', (event: FetchEvent) => {
  event.respondWith(handleRequest(event.request));
});

function getCorsHeaders(request: Request): Headers {
  const headers = new Headers();
  const origin = request.headers.get('Origin');
  
  // Allowed origins
  const allowedOrigins = [
    'https://syndicode.dev',
    'https://www.syndicode.dev',
    'http://localhost:3000'
  ];
  
  if (origin && allowedOrigins.includes(origin)) {
    headers.set('Access-Control-Allow-Origin', origin);
    headers.set('Access-Control-Allow-Methods', 'GET, HEAD, OPTIONS');
    headers.set('Access-Control-Allow-Headers', '*');
    headers.set('Access-Control-Max-Age', '86400');
  }
  
  return headers;
}

async function handleRequest(request: Request): Promise<Response> {
  const url = new URL(request.url);
  const key = url.pathname.slice(1); // Remove leading slash

  // Handle CORS preflight requests
  if (request.method === 'OPTIONS') {
    return new Response(null, {
      headers: getCorsHeaders(request),
      status: 204,
    });
  }

  try {
    // Try to get the requested file from R2
    const object = await ASSETS_BUCKET.get(key);
    
    if (object) {
      // File exists, return it with appropriate headers
      const headers = new Headers();
      object.writeHttpMetadata(headers);
      headers.set('etag', object.etag);
      
      // Add CORS headers
      const corsHeaders = getCorsHeaders(request);
      corsHeaders.forEach((value, key) => {
        headers.set(key, value);
      });
      
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
        
        // Add CORS headers
        const corsHeaders = getCorsHeaders(request);
        corsHeaders.forEach((value, key) => {
          headers.set(key, value);
        });
        
        return new Response(defaultPbf.body, {
          headers,
          status: 200,
        });
      }
      
      // If no default PBF exists, create minimal empty PBF response
      const headers = new Headers({
        'Content-Type': 'application/x-protobuf',
        'Cache-Control': 'public, max-age=3600',
      });
      
      // Add CORS headers
      const corsHeaders = getCorsHeaders(request);
      corsHeaders.forEach((value, key) => {
        headers.set(key, value);
      });
      
      const emptyPbf = new Uint8Array(0);
      return new Response(emptyPbf, {
        headers,
        status: 200,
      });
    }

    // Not a PBF file, return 404
    const headers = new Headers({
      'Content-Type': 'text/plain',
    });
    
    // Add CORS headers
    const corsHeaders = getCorsHeaders(request);
    corsHeaders.forEach((value, key) => {
      headers.set(key, value);
    });
    
    return new Response('File not found', {
      status: 404,
      headers,
    });

  } catch (error) {
    console.error('Error in pbf-fallback worker:', error);
    
    const headers = new Headers({
      'Content-Type': 'text/plain',
    });
    
    // Add CORS headers
    const corsHeaders = getCorsHeaders(request);
    corsHeaders.forEach((value, key) => {
      headers.set(key, value);
    });
    
    return new Response('Internal server error', {
      status: 500,
      headers,
    });
  }
}