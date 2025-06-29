import type { Metadata } from 'next';
import { Fira_Code } from 'next/font/google';
import 'maplibre-gl/dist/maplibre-gl.css';
import './globals.css';

import { AuthDialog } from '@/components/auth/auth-dialog';
import { Toaster } from '@/components/ui/sonner';

export const metadata: Metadata = {
  title: 'Syndicode',
};

const firaCode = Fira_Code({
  subsets: ['latin'],
  display: 'swap',
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${firaCode.className}`}>
      <body className={`antialiased`}>
        {children}
        <AuthDialog />
        <Toaster />
      </body>
    </html>
  );
}
