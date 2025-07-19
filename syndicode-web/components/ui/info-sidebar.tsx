import { Button } from '@/components/ui/button';
import { X } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useIsMobile } from '@/hooks/use-mobile';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from '@/components/ui/sheet';

interface InfoSidebarProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
}

export function InfoSidebar({ isOpen, onClose, title, children }: InfoSidebarProps) {
  const isMobile = useIsMobile();
  
  if (isMobile) {
    return (
      <Sheet open={isOpen} onOpenChange={onClose}>
        <SheetContent side="right" className="w-full max-w-sm p-0">
          <SheetHeader className="sr-only">
            <SheetTitle>{title}</SheetTitle>
            <SheetDescription>Business details sidebar</SheetDescription>
          </SheetHeader>
          <div className="flex h-full w-full flex-col">
            {/* Header */}
            <div className="p-4 border-b border-border">
              <h2 className="text-lg font-semibold">{title}</h2>
            </div>

            {/* Content */}
            <div className="p-3 flex-1 overflow-y-auto">
              {children}
            </div>
          </div>
        </SheetContent>
      </Sheet>
    );
  }

  // Desktop version
  return (
    <>
      {/* Backdrop for desktop */}
      {isOpen && (
        <div 
          className="fixed inset-0 bg-black/20 backdrop-blur-sm z-30 lg:hidden"
          onClick={onClose}
        />
      )}
      
      {/* Desktop Sidebar */}
      <div className={cn(
        "fixed top-0 right-0 h-full bg-background border-l border-border shadow-xl z-40 transform transition-transform duration-300 ease-in-out flex flex-col w-80",
        isOpen ? "translate-x-0" : "translate-x-full"
      )}>
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-border">
          <h2 className="text-lg font-semibold">{title}</h2>
          <Button
            variant="ghost"
            size="sm"
            onClick={onClose}
            className="p-0 h-8 w-8"
          >
            <X className="h-4 w-4" />
          </Button>
        </div>

        {/* Content */}
        <div className="p-3 flex-1 overflow-y-auto">
          {children}
        </div>
      </div>
    </>
  );
}