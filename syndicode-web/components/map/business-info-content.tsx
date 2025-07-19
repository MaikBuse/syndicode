'use client';

import type { BusinessDetails, BusinessListingDetails, BuildingDetails } from '@/domain/economy/economy.types';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import { ActionButton } from '@/components/ui/action-button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { Building, MapPin, DollarSign, ShoppingCart, Home, ChevronDown } from 'lucide-react';
import { acquireListedBusinessAction } from '@/app/actions/economy.actions';
import { useState, useEffect } from 'react';
import { toast } from 'sonner';

interface BusinessInfoContentProps {
  business: BusinessDetails | BusinessListingDetails | null;
  buildingsLoading: boolean;
  buildings: BuildingDetails[];
}

export function BusinessInfoContent({ business, buildingsLoading, buildings }: BusinessInfoContentProps) {
  const [isAcquiring, setIsAcquiring] = useState(false);
  const [isAcquired, setIsAcquired] = useState(false);

  // Reset acquisition state when business changes
  useEffect(() => {
    setIsAcquired(false);
    setIsAcquiring(false);
  }, [business?.businessUuid]);

  if (!business) {
    return (
      <div className="flex items-center justify-center h-32 text-muted-foreground">
        <div className="text-center">
          <Building className="h-8 w-8 mx-auto mb-2 opacity-50" />
          <p className="text-sm">No business selected</p>
        </div>
      </div>
    );
  }

  // Check if it's a listed business
  const isListedBusiness = 'listingUuid' in business;

  const handleAcquireBusiness = async () => {
    if (!isListedBusiness || isAcquired) return;

    setIsAcquiring(true);

    try {
      const result = await acquireListedBusinessAction({
        businessListingUuid: (business as BusinessListingDetails).listingUuid,
      });

      if (result.success) {
        setIsAcquired(true);
        toast.success('Business acquisition queued! Operation will complete in 1 game tick. Check your owned businesses to track progress.');
      } else {
        toast.error(`Acquisition failed: ${result.message}`);
      }
    } catch (error) {
      toast.error(`Acquisition error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsAcquiring(false);
    }
  };

  const formatCurrency = (amount: number) => {
    // Format as Japanese Digital Yen
    return new Intl.NumberFormat('ja-JP', {
      style: 'currency',
      currency: 'JPY',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount).replace('￥', 'Ð￥');
  };

  return (
    <Card className="shadow-lg border-0 bg-background">
      {/* Header Section - Business Name */}
      <CardHeader className="pb-1">
        <div className="flex items-center gap-3">
          <div className="p-3 rounded-lg bg-primary/10">
            <Building className="h-6 w-6 text-primary" />
          </div>
          <CardTitle className="text-lg font-semibold">
            {business.businessName}
          </CardTitle>
        </div>
      </CardHeader>

      {/* Intel Section - Business Information */}
      <CardContent className="pt-0 space-y-4">
        <div>
          <h3 className="text-sm font-semibold text-foreground mb-3">Corporate Intelligence</h3>

          {/* Asking Price - Only for listed businesses */}
          {isListedBusiness && (
            <div className="flex items-center gap-2 text-sm p-3 rounded-lg border border-cyan-500 mb-4">
              <ShoppingCart className="h-4 w-4 text-cyan-600" />
              <span className="text-foreground font-medium">Asking Price:</span>
              <span className="font-bold text-cyan-600">
                {formatCurrency((business as BusinessListingDetails).askingPrice)}
              </span>
            </div>
          )}
        </div>

        {/* Location */}
        <div className="space-y-3">
          <div className="flex items-center gap-2 text-sm">
            <MapPin className="h-4 w-4 text-muted-foreground" />
            <span className="text-muted-foreground">Location:</span>
          </div>
          <div className="text-sm font-medium ml-6 space-y-2">
            <div>Lat: {business.headquarterLatitude.toFixed(4)}</div>
            <div>Lng: {business.headquarterLongitude.toFixed(4)}</div>
          </div>
        </div>

        {/* Operational Expenses */}
        <div className="flex items-center gap-2 text-sm">
          <DollarSign className="h-4 w-4 text-muted-foreground" />
          <span className="text-muted-foreground">Expenses:</span>
          <span className="font-medium text-red-600">
            {formatCurrency(business.operationalExpenses)}
          </span>
        </div>

        {/* Associated Buildings */}
        <div className="border-t border-border pt-4">
          <h3 className="text-sm font-semibold text-foreground mb-3">Associated Buildings</h3>

          {buildingsLoading ? (
            // Skeleton loading state
            <div className="text-xs text-muted-foreground p-2 rounded border border-dashed">
              <Skeleton className="h-3 w-32" />
            </div>
          ) : buildings.length === 0 ? (
            // No buildings state
            <div className="text-center py-3 text-muted-foreground text-xs">
              No properties found
            </div>
          ) : (
            // Collapsible buildings list
            <Collapsible>
              <CollapsibleTrigger className="flex items-center justify-between text-xs text-muted-foreground hover:text-foreground transition-colors mb-2 w-full">
                <span>{buildings.length} {buildings.length === 1 ? 'property' : 'properties'}</span>
                <ChevronDown className="h-3 w-3 transition-transform duration-200 data-[state=open]:rotate-180" />
              </CollapsibleTrigger>

              <CollapsibleContent className="space-y-2">
                {buildings.map((building) => (
                  <div key={building.gmlId} className="flex items-center gap-2 p-2 rounded border border-muted text-xs">
                    <Home className="h-4 w-4 text-muted-foreground" />
                    <div className="flex-1">
                      <div className="font-medium">Building {building.gmlId?.slice(-8) || 'Unknown'}</div>
                      <div className="text-muted-foreground">
                        {building.latitude?.toFixed(4) || 'N/A'}, {building.longitude?.toFixed(4) || 'N/A'}
                      </div>
                    </div>
                  </div>
                ))}
              </CollapsibleContent>
            </Collapsible>
          )}
        </div>

        {/* Actions Section - Only for listed businesses */}
        {isListedBusiness && (
          <div className="border-t border-border pt-4">
            <h3 className="text-sm font-semibold text-foreground mb-3">Strategic Operations</h3>
            <div className="space-y-3">
              <ActionButton
                onClick={handleAcquireBusiness}
                disabled={isAcquiring || isAcquired}
                tickCost={1}
                isLoading={isAcquiring}
                isCompleted={isAcquired}
              >
                {isAcquiring
                  ? 'Acquiring...'
                  : isAcquired
                    ? 'Acquisition queued'
                    : 'Acquire Business'
                }
              </ActionButton>
            </div>
          </div>
        )}

        {/* Technical Details Section */}
        <div className="border-t border-border pt-4">
          <h3 className="text-sm font-semibold text-foreground mb-3">Network Identifiers</h3>
          <div className="space-y-3">
            {isListedBusiness && (
              <div className="text-xs text-muted-foreground">
                <span className="font-medium">Listing UUID:</span>
                <div className="mt-1 font-mono text-xs break-all">
                  {(business as BusinessListingDetails).listingUuid}
                </div>
              </div>
            )}

            <div className="text-xs text-muted-foreground">
              <span className="font-medium">Business UUID:</span>
              <div className="mt-1 font-mono text-xs break-all">
                {business.businessUuid}
              </div>
            </div>

            <div className="text-xs text-muted-foreground">
              <span className="font-medium">Market UUID:</span>
              <div className="mt-1 font-mono text-xs break-all">
                {business.marketUuid}
              </div>
            </div>

            <div className="text-xs text-muted-foreground">
              <span className="font-medium">Headquarter Building:</span>
              <div className="mt-1 font-mono text-xs break-all">
                {business.headquarterBuildingGmlId}
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
