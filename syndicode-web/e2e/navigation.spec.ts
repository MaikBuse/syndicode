import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('should render homepage correctly', async ({ page }) => {
    await page.goto('/');
    
    // Should show the main page title or logo
    await expect(page.getByText(/syndicode/i)).toBeVisible();
    
    // Should show auth button when not logged in
    const authButton = page.getByRole('button', { name: /sign in|login/i });
    await expect(authButton).toBeVisible();
  });

  test('should be responsive on mobile viewport', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    // Should still show main content
    await expect(page.getByText(/syndicode/i)).toBeVisible();
    
    // Mobile-specific elements should be visible
    const mobileMenu = page.getByRole('button', { name: /menu|☰/i });
    if (await mobileMenu.isVisible()) {
      await mobileMenu.click();
      // Menu should expand
      await expect(page.getByRole('navigation')).toBeVisible();
    }
  });

  test('should handle dashboard navigation', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Dashboard might redirect to login if not authenticated
    // OR show dashboard content if public
    // Either behavior is valid - test both paths
    
    const currentUrl = page.url();
    if (currentUrl.includes('/dashboard')) {
      // Dashboard is accessible
      await expect(page.getByText(/dashboard|map|overview/i)).toBeVisible();
    } else {
      // Redirected to auth
      await expect(page.getByRole('button', { name: /sign in|login/i })).toBeVisible();
    }
  });

  test('should load without JavaScript errors', async ({ page }) => {
    // Capture console errors
    const errors: string[] = [];
    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    await page.goto('/');
    
    // Wait for page to fully load
    await page.waitForLoadState('networkidle');
    
    // Check for critical errors (ignore minor ones)
    const criticalErrors = errors.filter(error => 
      !error.includes('favicon') && 
      !error.includes('404') &&
      !error.includes('net::ERR_INTERNET_DISCONNECTED')
    );
    
    expect(criticalErrors).toHaveLength(0);
  });

  test('should have proper page metadata', async ({ page }) => {
    await page.goto('/');
    
    // Should have title
    await expect(page).toHaveTitle(/syndicode/i);
    
    // Should have favicon
    const favicon = page.locator('link[rel="icon"]');
    await expect(favicon).toHaveCount(1);
  });
});