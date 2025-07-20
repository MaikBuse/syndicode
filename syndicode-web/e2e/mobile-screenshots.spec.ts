import { test, devices } from '@playwright/test';
import path from 'path';

// Configure for mobile viewport
test.use({ 
  ...devices['iPhone 12'],
  // Override to ensure we get consistent screenshots
  viewport: { width: 390, height: 844 },
  // Ensure baseURL is preserved
  baseURL: 'http://localhost:3000'
});

// Helper to get screenshot path relative to syndicode-web directory
const getScreenshotPath = (filename: string) => path.resolve(__dirname, '..', 'mobile-screenshots', filename);

test.describe('Mobile Screenshots', () => {

  test('capture mobile app screenshots', async ({ page }) => {
    // Navigate to the main page
    await page.goto('/');
    
    // Wait for page to load
    await page.waitForLoadState('networkidle');
    
    // Screenshot 1: Landing page (unauthenticated)
    await page.screenshot({ 
      path: getScreenshotPath('01-landing-unauthenticated.png'),
      fullPage: true 
    });
    
    // Click login button to potentially show auth dialog
    const loginButton = page.getByRole('button', { name: /sign in|login|log in/i });
    if (await loginButton.isVisible()) {
      await loginButton.click();
      await page.waitForTimeout(500);
    }
    
    // Screenshot 2: Auth state/login
    await page.screenshot({ 
      path: getScreenshotPath('02-auth-login.png'),
      fullPage: true 
    });
    
    // Look for auth dialog or login form
    const authDialog = page.locator('[data-testid="auth-dialog"], .auth-dialog, [role="dialog"]');
    
    if (await authDialog.isVisible()) {
      // Screenshot 3: Auth dialog open
      await page.screenshot({ 
        path: getScreenshotPath('03-auth-dialog-open.png'),
        fullPage: true 
      });
    } else if (await loginButton.isVisible()) {
      // Click login button to open auth flow
      await loginButton.click();
      await page.waitForTimeout(500);
      
      // Screenshot 3: Login form
      await page.screenshot({ 
        path: getScreenshotPath('03-login-form.png'),
        fullPage: true 
      });
    }
    
    // Check if there's a register option
    const registerButton = page.getByRole('button', { name: /sign up|register|create account/i });
    if (await registerButton.isVisible()) {
      await registerButton.click();
      await page.waitForTimeout(500);
      
      // Screenshot 4: Register form
      await page.screenshot({ 
        path: getScreenshotPath('04-register-form.png'),
        fullPage: true 
      });
    }
    
    // Try to get to a state where we can see the map interface
    // (This might not work without authentication, but let's try)
    await page.goto('/');
    await page.waitForTimeout(2000);
    
    // Screenshot 5: Main page map interface with hamburger menu
    await page.screenshot({ 
      path: getScreenshotPath('05-main-page-map.png'),
      fullPage: true 
    });
    
    // Look for mobile hamburger menu
    const hamburgerMenu = page.locator('button[aria-label*="navigation menu"], button[aria-label*="menu"]');
    if (await hamburgerMenu.isVisible()) {
      // Screenshot 6: Hamburger menu visible
      await page.screenshot({ 
        path: getScreenshotPath('06-hamburger-menu.png'),
        fullPage: true 
      });
      
      // Click hamburger menu to open sidebar
      await hamburgerMenu.click();
      await page.waitForTimeout(500);
      
      // Screenshot 7: Sidebar open from hamburger menu
      await page.screenshot({ 
        path: getScreenshotPath('07-sidebar-open.png'),
        fullPage: true 
      });
    }
    
    // Look for map controls
    const mapControls = page.locator('.map-layer-controls, [data-testid="map-controls"]');
    if (await mapControls.isVisible()) {
      // Screenshot 8: Map with controls visible
      await page.screenshot({ 
        path: getScreenshotPath('08-map-controls.png'),
        fullPage: true 
      });
    }
    
    // Test landscape orientation
    await page.setViewportSize({ width: 844, height: 390 });
    await page.waitForTimeout(1000);
    
    // Screenshot 9: Landscape orientation
    await page.screenshot({ 
      path: getScreenshotPath('09-landscape-view.png'),
      fullPage: true 
    });
  });
  
  test('capture mobile interaction states', async ({ page }) => {
    // Use mobile viewport
    await page.setViewportSize({ width: 390, height: 844 });
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Try to interact with map elements (if visible)
    const mapContainer = page.locator('.maplibregl-map, .deck-canvas, [data-testid="map-container"]');
    if (await mapContainer.isVisible()) {
      // Tap on map center
      await mapContainer.first().tap();
      await page.waitForTimeout(500);
      
      // Screenshot 10: After map tap
      await page.screenshot({ 
        path: getScreenshotPath('10-map-interaction.png'),
        fullPage: true 
      });
    }
    
    // Look for info sidebar or business details
    const infoSidebar = page.locator('.info-sidebar, [data-testid="info-sidebar"]');
    if (await infoSidebar.isVisible()) {
      // Screenshot 11: Info sidebar open
      await page.screenshot({ 
        path: getScreenshotPath('11-info-sidebar.png'),
        fullPage: true 
      });
    }
  });
});