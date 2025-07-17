import { useMapLoadingStore } from '@/stores/use-map-loading-store';
import { cn } from '@/lib/utils';

export function MapLoadingIndicator() {
  const isMapLoading = useMapLoadingStore((state) => state.isMapLoading);
  
  if (!isMapLoading) return null;
  
  return (
    <div className="absolute top-4 right-4 z-20 flex items-center gap-2 bg-black/80 backdrop-blur-sm border border-cyan-500/20 rounded-lg px-3 py-2">
      {/* Cyberpunk-style loading spinner */}
      <div className="relative">
        <div className="w-4 h-4 border-2 border-cyan-500/30 rounded-full animate-spin border-t-cyan-400"></div>
        <div className="absolute inset-0 w-4 h-4 border border-cyan-400/20 rounded-full animate-pulse"></div>
      </div>
      
      {/* Loading text */}
      <span className="text-cyan-400 text-sm font-mono">
        Loading map data...
      </span>
      
      {/* Animated dots */}
      <div className="flex space-x-1">
        <div className="w-1 h-1 bg-cyan-400 rounded-full animate-bounce [animation-delay:-0.3s]"></div>
        <div className="w-1 h-1 bg-cyan-400 rounded-full animate-bounce [animation-delay:-0.15s]"></div>
        <div className="w-1 h-1 bg-cyan-400 rounded-full animate-bounce"></div>
      </div>
    </div>
  );
}