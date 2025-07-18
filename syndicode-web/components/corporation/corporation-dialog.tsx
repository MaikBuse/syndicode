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
import { Loader2 } from 'lucide-react';

interface CorporationDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function CorporationDialog({ isOpen, onClose }: CorporationDialogProps) {
  const { data, fetchCorporation } = useUserDataStore();
  const [isLoading, setIsLoading] = useState(false);

  // Reload corporation data when dialog opens
  useEffect(() => {
    if (isOpen) {
      setIsLoading(true);
      fetchCorporation().finally(() => setIsLoading(false));
    }
  }, [isOpen, fetchCorporation]);

  if (!data?.corporation) {
    return null;
  }

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogPortal>
        <DialogContent className="bg-card/80 border border-border shadow-lg sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Corporation Details</DialogTitle>
          </DialogHeader>
          <div className="mt-4">
            {isLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin text-primary" />
                <span className="ml-2 text-sm text-muted-foreground">Loading corporation data...</span>
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