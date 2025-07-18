import { useControl } from 'react-map-gl/maplibre';
import { MapboxOverlay } from '@deck.gl/mapbox';
import type { DeckProps } from '@deck.gl/core';

export function DeckGLOverlay(props: DeckProps) {
  const overlay = useControl<MapboxOverlay>(() => new MapboxOverlay({
    ...props,
    interleaved: true
  }));
  
  overlay.setProps({
    ...props,
    interleaved: true
  });
  
  return null;
}