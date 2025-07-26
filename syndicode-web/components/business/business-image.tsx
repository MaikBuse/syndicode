'use client';

import React from 'react';
import { cn } from '@/lib/utils';
import { getBusinessImageSrcSet, getDefaultBusinessImageUrl } from '@/lib/utils/image-utils';

interface BusinessImageProps {
  marketNumber: number;
  imageNumber: number;
  businessName: string;
  className?: string;
  sizes?: string;
  priority?: boolean;
}

/**
 * Responsive business image component that displays business images with appropriate
 * responsive behavior and fallback handling.
 */
export function BusinessImage({
  marketNumber,
  imageNumber,
  businessName,
  className,
  sizes = '(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw',
  priority = false
}: BusinessImageProps) {
  const [imageError, setImageError] = React.useState(false);
  const [imageLoaded, setImageLoaded] = React.useState(false);

  const srcSet = getBusinessImageSrcSet(marketNumber, imageNumber);
  const defaultSrc = getDefaultBusinessImageUrl(marketNumber, imageNumber);

  const handleImageError = () => {
    setImageError(true);
  };

  const handleImageLoad = () => {
    setImageLoaded(true);
  };

  // If image failed to load, show a placeholder
  if (imageError) {
    return (
      <div
        className={cn(
          'flex items-center justify-center bg-muted text-muted-foreground rounded-lg',
          'border border-border',
          className
        )}
      >
        <div className="text-center p-4">
          <div className="text-sm font-medium">{businessName}</div>
          <div className="text-xs opacity-70 mt-1">Image unavailable</div>
        </div>
      </div>
    );
  }

  return (
    <div className={cn('relative overflow-hidden rounded-lg', className)}>
      {/* Loading skeleton */}
      {!imageLoaded && (
        <div className="absolute inset-0 bg-muted animate-pulse" />
      )}
      
      {/* Actual image */}
      <img
        src={defaultSrc}
        srcSet={srcSet}
        sizes={sizes}
        alt={`${businessName} business image`}
        onError={handleImageError}
        onLoad={handleImageLoad}
        className={cn(
          'w-full h-full object-cover transition-opacity duration-300',
          imageLoaded ? 'opacity-100' : 'opacity-0'
        )}
        loading={priority ? 'eager' : 'lazy'}
        decoding="async"
      />
      
      {/* Optional overlay for better text readability */}
      <div className="absolute inset-0 bg-black/20 opacity-0 hover:opacity-100 transition-opacity duration-200" />
    </div>
  );
}