"use client"

import * as React from "react"
import {
  Building,
} from "lucide-react"
import Image from "next/image"

import { NavMain } from "@/components/nav-main"
import { NavUser } from "@/components/nav-user"
import { AuthButton } from "@/components/auth/auth-button"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  useSidebar,
} from "@/components/ui/sidebar"
import { useAuthStore } from "@/stores/use-auth-store"
import { CorporationDialog } from "@/components/corporation/corporation-dialog"

// Syndicode navigation data - only show corporation when authenticated
const getSyndicodeNavigation = (isAuthenticated: boolean) => {
  if (!isAuthenticated) {
    return [];
  }
  
  return [
    {
      title: "Corporation",
      url: "#",
      icon: Building,
      onClick: "corporation",
    },
  ];
};

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const { user, isAuthenticated } = useAuthStore()
  const { toggleSidebar } = useSidebar()
  const [isCorporationDialogOpen, setIsCorporationDialogOpen] = React.useState(false)

  const handleNavItemClick = (action: string) => {
    if (action === 'corporation') {
      setIsCorporationDialogOpen(true)
    }
  }

  return (
    <Sidebar collapsible="icon" className="[&>*]:bg-background bg-background" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              size="lg"
              onClick={toggleSidebar}
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground hover:bg-sidebar-accent/50 cursor-pointer"
            >
              <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-black text-white">
                <Image
                  src="/icon.svg"
                  alt="Syndicode Logo"
                  width={24}
                  height={24}
                  className="size-6"
                />
              </div>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-semibold">Syndicode</span>
                <span className="truncate text-xs">Cyberpunk Strategy</span>
              </div>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        {!isAuthenticated && (
          <div className="px-3 py-4 border-t border-b border-sidebar-border">
            <SidebarMenu>
              <SidebarMenuItem>
                <AuthButton />
              </SidebarMenuItem>
            </SidebarMenu>
          </div>
        )}
        <NavMain items={getSyndicodeNavigation(isAuthenticated)} onItemClick={handleNavItemClick} />
      </SidebarContent>
      <SidebarFooter>
        {isAuthenticated && user && (
          <NavUser user={{
            name: user.name,
            email: user.email,
            avatar: ""
          }} />
        )}
      </SidebarFooter>
      <SidebarRail />
      <CorporationDialog
        isOpen={isCorporationDialogOpen}
        onClose={() => setIsCorporationDialogOpen(false)}
      />
    </Sidebar>
  )
}
