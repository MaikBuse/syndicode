import type { Metadata } from 'next';
import { Fira_Code } from 'next/font/google';
import 'maplibre-gl/dist/maplibre-gl.css';
import './globals.css';

import { AuthDialog } from '@/components/auth/auth-dialog';
import { Toaster } from '@/components/ui/sonner';
import AuthStoreInitializer from './AuthStoreInitializer';
import { getCurrentUser } from './actions/auth';

export const metadata: Metadata = {
  title: 'Syndicode',
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
        <AuthStoreInitializer user={user} />
        {children}
        <AuthDialog />
        <Toaster />
      </body>
    </html>
  );
}
