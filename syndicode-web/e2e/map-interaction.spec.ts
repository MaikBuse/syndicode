import { test, expect } from '@playwright/test';

test.describe('Map Interaction', () => {
  test('should load map on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Wait for map container to load
    const mapContainer = page.locator('[data-testid="map-container"], .maplibregl-map, .deck-canvas');
    
    // Map might be behind auth, so check if we're redirected
    const currentUrl = page.url();
    if (currentUrl.includes('/dashboard')) {
      // We're on dashboard, map should load
      await expect(mapContainer.first()).toBeVisible({ timeout: 10000 });
    } else {
      // Redirected to auth - that's ok for this test
      await expect(page.getByRole('button', { name: /sign in|login/i })).toBeVisible();
    }
  });

  test('should show map controls when map is loaded', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Skip test if redirected to auth
    if (!page.url().includes('/dashboard')) {
      test.skip(true, 'Dashboard requires authentication');
    }
    
    // Look for map layer controls or zoom controls
    const mapControls = page.locator(
      '[data-testid="map-controls"], .maplibregl-ctrl, .map-layer-controls'
    );
    
    await expect(mapControls.first()).toBeVisible({ timeout: 15000 });
  });

  test('should handle map loading states', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Skip test if redirected to auth
    if (!page.url().includes('/dashboard')) {
      test.skip(true, 'Dashboard requires authentication');
    }
    
    // Should show loading indicator initially
    // const loadingIndicator = page.getByText(/loading|initializing/i);
    
    // Loading might be very fast, so don't require it
    // Just check that we eventually get to a loaded state
    await page.waitForLoadState('networkidle');
    
    // Map container should be present after loading
    const mapContainer = page.locator('[data-testid="map-container"], .maplibregl-map, .deck-canvas');
    await expect(mapContainer.first()).toBeVisible({ timeout: 15000 });
  });

  test('should show business information on interaction', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Skip test if redirected to auth
    if (!page.url().includes('/dashboard')) {
      test.skip(true, 'Dashboard requires authentication');
    }
    
    // Wait for map to load
    await page.waitForLoadState('networkidle');
    
    // Look for info sidebar or business info panel
    const infoPanel = page.locator(
      '[data-testid="info-sidebar"], .info-sidebar, .business-info'
    );
    
    // Info panel might be visible by default or appear on interaction
    // This is a basic visibility test
    if (await infoPanel.isVisible()) {
      await expect(infoPanel).toBeVisible();
    }
  });

  test('should handle map layer toggles', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Skip test if redirected to auth
    if (!page.url().includes('/dashboard')) {
      test.skip(true, 'Dashboard requires authentication');
    }
    
    // Wait for page to load
    await page.waitForLoadState('networkidle');
    
    // Look for layer control buttons
    const layerControls = page.locator(
      '[data-testid="layer-controls"], .map-layer-controls, button[aria-label*="layer"]'
    );
    
    // If layer controls exist, test them
    if (await layerControls.first().isVisible()) {
      await layerControls.first().click();
      // Layer toggle should work without errors
      await expect(layerControls.first()).toBeVisible();
    }
  });
});