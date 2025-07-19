"use client"

import { Menu } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useSidebar } from "@/components/ui/sidebar"
import { useIsMobile } from "@/hooks/use-mobile"
import { cn } from "@/lib/utils"

interface MobileSidebarTriggerProps {
  className?: string
}

export function MobileSidebarTrigger({ className }: MobileSidebarTriggerProps) {
  const { toggleSidebar, openMobile } = useSidebar()
  const isMobile = useIsMobile()

  // Only show on mobile when sidebar is not open
  if (!isMobile || openMobile) {
    return null
  }

  return (
    <Button
      variant="outline"
      size="sm"
      onClick={toggleSidebar}
      className={cn(
        "fixed top-4 left-4 z-50 bg-card/80 backdrop-blur-sm border-border hover:bg-card/60 active:bg-card/40 touch-manipulation rounded-lg transition-all",
        "h-10 w-10 p-0",
        className
      )}
      aria-label="Open navigation menu"
    >
      <Menu className="h-5 w-5" />
    </Button>
  )
}