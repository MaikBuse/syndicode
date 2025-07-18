'use client';

import type { BusinessDetails, BusinessListingDetails } from '@/domain/economy/economy.types';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Building, MapPin, DollarSign, ShoppingCart } from 'lucide-react';
import { acquireListedBusinessAction } from '@/app/actions/economy.actions';
import { useState } from 'react';

interface BusinessInfoContentProps {
  business: BusinessDetails | BusinessListingDetails | null;
}

export function BusinessInfoContent({ business }: BusinessInfoContentProps) {
  const [isAcquiring, setIsAcquiring] = useState(false);
  const [acquisitionResult, setAcquisitionResult] = useState<string | null>(null);

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
    if (!isListedBusiness) return;
    
    setIsAcquiring(true);
    setAcquisitionResult(null);
    
    try {
      const result = await acquireListedBusinessAction({
        businessListingUuid: (business as BusinessListingDetails).listingUuid,
      });
      
      if (result.success) {
        setAcquisitionResult(`Business acquisition has been queued and is being processed. Check your owned businesses to see when it completes.`);
      } else {
        setAcquisitionResult(`Failed to queue business acquisition: ${result.message}`);
      }
    } catch (error) {
      setAcquisitionResult(`Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
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
      <CardHeader className="pb-4">
        <div className="flex items-center gap-2">
          <Building className="h-5 w-5 text-primary" />
          <CardTitle className="text-lg font-semibold">
            {business.businessName}
          </CardTitle>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {isListedBusiness && (
          <div className="flex items-center gap-2 text-sm p-3 rounded-lg border border-cyan-500">
            <ShoppingCart className="h-4 w-4 text-cyan-600" />
            <span className="text-foreground font-medium">Asking Price:</span>
            <span className="font-bold text-cyan-600">
              {formatCurrency((business as BusinessListingDetails).askingPrice)}
            </span>
          </div>
        )}

        <div className="space-y-1">
          <div className="flex items-center gap-2 text-sm">
            <MapPin className="h-4 w-4 text-muted-foreground" />
            <span className="text-muted-foreground">Location:</span>
          </div>
          <div className="text-sm font-medium ml-6">
            <div>Lat: {business.headquarterLatitude.toFixed(4)}</div>
            <div>Lng: {business.headquarterLongitude.toFixed(4)}</div>
          </div>
        </div>

        <div className="flex items-center gap-2 text-sm">
          <DollarSign className="h-4 w-4 text-muted-foreground" />
          <span className="text-muted-foreground">Expenses:</span>
          <span className="font-medium text-red-600">
            {formatCurrency(business.operationalExpenses)}
          </span>
        </div>

        {isListedBusiness && (
          <div className="space-y-3">
            <Button 
              onClick={handleAcquireBusiness}
              disabled={isAcquiring}
              className="w-full"
              size="sm"
            >
              {isAcquiring ? 'Queueing Acquisition...' : 'Queue Business Acquisition'}
            </Button>
            
            {acquisitionResult && (
              <div className={`text-xs p-2 rounded ${
                acquisitionResult.includes('queued and is being processed') 
                  ? 'bg-blue-100 text-blue-800 border border-blue-300' 
                  : 'bg-red-100 text-red-800 border border-red-300'
              }`}>
                {acquisitionResult}
              </div>
            )}
          </div>
        )}

        <div className="pt-2 border-t space-y-2">
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
      </CardContent>
    </Card>
  );
}