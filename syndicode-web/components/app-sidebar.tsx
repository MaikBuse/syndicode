"use client"

import * as React from "react"
import {
  Building2,
  Map,
  Settings2,
  BarChart3,
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

// Syndicode navigation data
const syndicodeNavigation = [
  {
    title: "Game View",
    url: "/",
    icon: Map,
    isActive: true,
  },
  {
    title: "Economy",
    url: "#",
    icon: Building2,
    items: [
      {
        title: "My Businesses",
        url: "#",
      },
      {
        title: "Market",
        url: "#",
      },
      {
        title: "Statistics",
        url: "#",
      },
    ],
  },
  {
    title: "Analytics",
    url: "#",
    icon: BarChart3,
    items: [
      {
        title: "Performance",
        url: "#",
      },
      {
        title: "Reports",
        url: "#",
      },
    ],
  },
  {
    title: "Settings",
    url: "#",
    icon: Settings2,
    items: [
      {
        title: "Profile",
        url: "#",
      },
      {
        title: "Preferences",
        url: "#",
      },
    ],
  },
]

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const { user, isAuthenticated } = useAuthStore()
  const { toggleSidebar } = useSidebar()

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
          <div className="px-3 py-4 border-t border-b border-sidebar-border mb-4">
            <SidebarMenu>
              <SidebarMenuItem>
                <AuthButton />
              </SidebarMenuItem>
            </SidebarMenu>
          </div>
        )}
        <NavMain items={syndicodeNavigation} />
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
    </Sidebar>
  )
}
