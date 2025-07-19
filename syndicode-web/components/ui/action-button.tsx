'use client';

import { Clock } from 'lucide-react';
import { cn } from '@/lib/utils';

interface ActionButtonProps {
  /** The main text content of the button */
  children: React.ReactNode;
  /** Number of game ticks this action requires */
  tickCost: number;
  /** Whether the action is currently processing */
  isLoading?: boolean;
  /** Whether the action has been completed */
  isCompleted?: boolean;
  /** Whether the button is disabled */
  disabled?: boolean;
  /** Click handler */
  onClick?: () => void;
  /** Additional CSS classes */
  className?: string;
  /** Custom color scheme for the cost container */
  costVariant?: 'default' | 'success' | 'warning' | 'destructive';
  /** Custom color scheme for the action container */
  actionVariant?: 'default' | 'success' | 'warning' | 'destructive';
}

const costVariants = {
  default: 'bg-muted text-muted-foreground',
  success: 'bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300',
  warning: 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900 dark:text-yellow-300',
  destructive: 'bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-300',
};

const actionVariants = {
  default: 'bg-primary text-primary-foreground hover:bg-primary/90',
  success: 'bg-green-600 text-white hover:bg-green-700',
  warning: 'bg-yellow-600 text-white hover:bg-yellow-700',
  destructive: 'bg-red-600 text-white hover:bg-red-700',
};

export function ActionButton({
  children,
  tickCost,
  isLoading = false,
  isCompleted = false,
  disabled = false,
  onClick,
  className,
  costVariant = 'default',
  actionVariant = 'default',
}: ActionButtonProps) {
  const isDisabled = disabled || isLoading;
  const finalActionVariant = isCompleted ? 'success' : actionVariant;

  return (
    <div className={cn('w-full flex items-center gap-2', className)}>
      {/* Left side: Tick Cost */}
      <div className={cn(
        'flex items-center justify-center gap-1 px-3 h-9 rounded text-xs font-medium flex-shrink-0',
        costVariants[costVariant]
      )}>
        <Clock className="h-3 w-3" />
        <span>{tickCost}</span>
      </div>

      {/* Right side: Action Button */}
      <button
        onClick={onClick}
        disabled={isDisabled}
        className={cn(
          'flex-1 h-9 px-3 rounded text-sm font-medium transition-colors',
          'disabled:opacity-50 disabled:cursor-not-allowed',
          'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
          actionVariants[finalActionVariant]
        )}
      >
        {children}
      </button>
    </div>
  );
}