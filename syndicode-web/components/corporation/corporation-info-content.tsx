'use client';

import { Building2, Coins } from 'lucide-react';
import type { Corporation } from '@/domain/economy/economy.types';

interface CorporationInfoContentProps {
  corporation: Corporation;
}

export function CorporationInfoContent({ corporation }: CorporationInfoContentProps) {
  const formatCurrency = (amount: number | undefined | null) => {
    if (amount === undefined || amount === null || isNaN(amount)) {
      return 'N/A';
    }
    
    // Format as Japanese Yen but replace ￥ with Ð￥ for cyberpunk digital yen
    const formatted = new Intl.NumberFormat('ja-JP', {
      style: 'currency',
      currency: 'JPY',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
    
    // Replace ￥ (full-width yen) with Ð￥ for digital yen cyberpunk aesthetic
    return formatted.replace('￥', 'Ð￥');
  };

  return (
    <div className="space-y-6">
      {/* Corporation Name */}
      <div className="flex items-center gap-3">
        <div className="p-3 rounded-lg bg-primary/10">
          <Building2 className="h-6 w-6 text-primary" />
        </div>
        <h2 className="text-xl font-bold text-foreground">{corporation.name}</h2>
      </div>

      {/* Corporation Overview */}
      <div className="space-y-4">
        
        <div className="grid grid-cols-1 gap-4">
          <div className="p-4 rounded-lg bg-muted/20 border border-border">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Coins className="h-4 w-4 text-chart-1" />
                <span className="text-sm font-medium">Balance</span>
              </div>
              <span className="text-lg font-bold text-chart-1">
                {formatCurrency(corporation.balance)}
              </span>
            </div>
          </div>
          
        </div>
      </div>

      {/* Corporation UUID at bottom */}
      <div className="pt-4 border-t border-border">
        <div className="text-center">
          <span className="text-xs font-mono text-muted-foreground">
            {corporation.uuid || 'N/A'}
          </span>
        </div>
      </div>
    </div>
  );
}