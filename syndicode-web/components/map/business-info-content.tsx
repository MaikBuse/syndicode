import type { BusinessDetails } from '@/domain/economy/economy.types';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Building, MapPin, DollarSign } from 'lucide-react';

interface BusinessInfoContentProps {
  business: BusinessDetails | null;
}

export function BusinessInfoContent({ business }: BusinessInfoContentProps) {
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
          <span className="font-medium text-green-600">
            ${business.operationalExpenses?.toLocaleString() || '0'}
          </span>
        </div>

        <div className="pt-2 border-t space-y-2">
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