import { test, expect } from '@playwright/test';

test.describe('Authentication Flow', () => {
  test('should display login form when not authenticated', async ({ page }) => {
    await page.goto('/');
    
    // Should redirect to home page with auth prompt
    await expect(page).toHaveURL('/');
    
    // Should show login button or auth dialog trigger
    const authButton = page.getByRole('button', { name: /sign in|login/i });
    await expect(authButton).toBeVisible();
  });

  test('should open auth dialog when clicking login', async ({ page }) => {
    await page.goto('/');
    
    // Click login button
    const authButton = page.getByRole('button', { name: /sign in|login/i });
    await authButton.click();
    
    // Should open auth dialog
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    
    // Should have login form fields
    await expect(page.getByLabel(/username|user name/i)).toBeVisible();
    await expect(page.getByLabel(/password/i)).toBeVisible();
  });

  test('should show validation errors for invalid login', async ({ page }) => {
    await page.goto('/');
    
    // Open auth dialog
    const authButton = page.getByRole('button', { name: /sign in|login/i });
    await authButton.click();
    
    // Try to submit empty form
    const loginButton = page.getByRole('button', { name: /sign in|login/i }).last();
    await loginButton.click();
    
    // Should show validation errors
    await expect(page.getByText(/required|enter/i)).toBeVisible();
  });

  test('should handle login failure gracefully', async ({ page }) => {
    await page.goto('/');
    
    // Open auth dialog
    const authButton = page.getByRole('button', { name: /sign in|login/i });
    await authButton.click();
    
    // Fill invalid credentials
    await page.getByLabel(/username|user name/i).fill('invalid_user');
    await page.getByLabel(/password/i).fill('wrong_password');
    
    // Submit form
    const loginButton = page.getByRole('button', { name: /sign in|login/i }).last();
    await loginButton.click();
    
    // Should show error message (but not crash)
    // Note: This will fail with actual invalid creds, but tests error handling
    await expect(page.getByText(/error|invalid|failed/i)).toBeVisible({ timeout: 5000 });
  });

  test('should switch between login and register forms', async ({ page }) => {
    await page.goto('/');
    
    // Open auth dialog
    const authButton = page.getByRole('button', { name: /sign in|login/i });
    await authButton.click();
    
    // Should be on login by default
    await expect(page.getByLabel(/username|user name/i)).toBeVisible();
    
    // Switch to register
    const registerTab = page.getByRole('tab', { name: /register|sign up/i });
    await registerTab.click();
    
    // Should show additional register fields
    await expect(page.getByLabel(/email/i)).toBeVisible();
    await expect(page.getByLabel(/corporation|company/i)).toBeVisible();
    
    // Switch back to login
    const loginTab = page.getByRole('tab', { name: /login|sign in/i });
    await loginTab.click();
    
    // Should be back to login form
    await expect(page.getByLabel(/username|user name/i)).toBeVisible();
  });
});