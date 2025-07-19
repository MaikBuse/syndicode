'use client';

import { useEffect, useState } from 'react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogPortal,
} from '@/components/ui/dialog';
import { CorporationInfoContent } from './corporation-info-content';
import { useUserDataStore } from '@/stores/use_user_data_store';
import { Skeleton } from '@/components/ui/skeleton';
import { toast } from 'sonner';

interface CorporationDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function CorporationDialog({ isOpen, onClose }: CorporationDialogProps) {
  const { data, fetchCorporation } = useUserDataStore();
  const [isLoading, setIsLoading] = useState(false);
  const [hasError, setHasError] = useState(false);

  // Reload corporation data when dialog opens
  useEffect(() => {
    if (isOpen) {
      setIsLoading(true);
      setHasError(false);
      fetchCorporation()
        .then(() => {
          setHasError(false);
        })
        .catch((error) => {
          console.error('Failed to fetch corporation data:', error);
          setHasError(true);
          toast.error('Failed to connect to server. Please check your connection and try again.');
        })
        .finally(() => setIsLoading(false));
    }
  }, [isOpen, fetchCorporation]);

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogPortal>
        <DialogContent className="bg-card/80 border border-border shadow-lg sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Corporation Details</DialogTitle>
          </DialogHeader>
          <div className="mt-4">
            {isLoading || hasError ? (
              <div className="space-y-6">
                {/* Corporation Name Skeleton */}
                <div className="flex items-center gap-3">
                  <Skeleton className="h-12 w-12 rounded-lg" />
                  <Skeleton className="h-6 w-48" />
                </div>

                {/* Corporation Overview Skeleton */}
                <div className="space-y-4">
                  <div className="grid grid-cols-1 gap-4">
                    <div className="p-4 rounded-lg bg-muted/20 border border-border">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <Skeleton className="h-4 w-4" />
                          <Skeleton className="h-4 w-16" />
                        </div>
                        <Skeleton className="h-6 w-24" />
                      </div>
                    </div>
                  </div>
                </div>

                {/* Corporation UUID Skeleton */}
                <div className="pt-4 border-t border-border">
                  <div className="text-center">
                    <Skeleton className="h-3 w-72 mx-auto" />
                  </div>
                </div>
              </div>
            ) : (
              data?.corporation && <CorporationInfoContent corporation={data.corporation} />
            )}
          </div>
        </DialogContent>
      </DialogPortal>
    </Dialog>
  );
}