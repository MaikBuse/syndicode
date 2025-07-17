import { useMapLoadingStore } from '@/stores/use-map-loading-store';

export function MapLoadingIndicator() {
  const isMapLoading = useMapLoadingStore((state) => state.isMapLoading);

  if (!isMapLoading) return null;

  return (
    <div className="absolute top-4 right-4 z-20 flex items-center gap-2 bg-black/80 backdrop-blur-sm border border-cyan-500/20 rounded-lg px-3 py-2">
      <div className="w-4 h-4 border-2 border-cyan-500/30 rounded-full animate-spin border-t-cyan-400"></div>
      <span className="text-cyan-400 text-sm font-mono">Loading data...</span>
    </div>
  );
}
