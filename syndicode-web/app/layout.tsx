import type { Metadata } from 'next';
import { Fira_Code } from 'next/font/google';
import 'maplibre-gl/dist/maplibre-gl.css';
import './globals.css';

import { AuthDialog } from '@/components/auth/auth-dialog';
import { Toaster } from '@/components/ui/sonner';
import AuthStoreInitializer from '../components/initializers/auth-store-initializer';
import { getCurrentUser } from './actions/auth';
import { SessionDataInitializer } from '@/components/initializers/session-data-initializer';

export const metadata: Metadata = {
  title: 'Syndicode',
  viewport: {
    width: 'device-width',
    initialScale: 1,
    maximumScale: 5,
    userScalable: true,
  },
};

const firaCode = Fira_Code({
  subsets: ['latin'],
  display: 'swap',
});

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  // Fetch the user data on the server
  const user = await getCurrentUser();

  return (
    <html lang="en" className={`${firaCode.className}`}>
      <body className={`antialiased`}>
        <SessionDataInitializer />
        <AuthStoreInitializer user={user} />
        {children}
        <AuthDialog />
        <Toaster position="bottom-center" />
      </body>
    </html>
  );
}
