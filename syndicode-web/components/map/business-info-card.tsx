import type { BusinessDetails } from '@/domain/economy/economy.types';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Building, MapPin, DollarSign } from 'lucide-react';

interface BusinessInfoCardProps {
  business: BusinessDetails;
}

export function BusinessInfoCard({ business }: BusinessInfoCardProps) {
  return (
    <Card className="w-80 shadow-lg border-border bg-background/95 backdrop-blur-sm">
      <CardHeader className="pb-1">
        <div className="flex items-center gap-2">
          <Building className="h-5 w-5 text-primary" />
          <CardTitle className="text-lg font-semibold truncate">
            {business.businessName}
          </CardTitle>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex items-center gap-2 text-sm">
          <MapPin className="h-4 w-4 text-muted-foreground" />
          <span className="text-muted-foreground">Location:</span>
          <span className="font-medium">
            {business.headquarterLatitude.toFixed(4)}, {business.headquarterLongitude.toFixed(4)}
          </span>
        </div>

        <div className="flex items-center gap-2 text-sm">
          <DollarSign className="h-4 w-4 text-muted-foreground" />
          <span className="text-muted-foreground">Expenses:</span>
          <span className="font-medium text-green-600">
            ${business.operationalExpenses?.toLocaleString() || '0'}
          </span>
        </div>

        <div className="pt-2 border-t space-y-1">
          <div className="text-xs text-muted-foreground">
            Business UUID: {business.businessUuid}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
