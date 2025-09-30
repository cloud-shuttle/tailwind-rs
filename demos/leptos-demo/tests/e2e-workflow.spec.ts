import { test, expect } from '@playwright/test';

test.describe('End-to-End Workflow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 15000 });

    // Wait for WASM to load and content to render
    await page.waitForTimeout(3000);
  });

  test('should load demo successfully', async ({ page }) => {
    // Check that the main app loaded
    await expect(page.locator('[data-testid="app"]')).toBeVisible();

    // Check hero section is visible
    await expect(page.locator('h1:has-text("TAILWIND")')).toBeVisible();

    // Check that CSS was generated (app has background gradient)
    const appDiv = page.locator('[data-testid="app"]');
    await expect(appDiv).toHaveClass(/bg-gradient-to-br/);
  });

  test('should display component gallery', async ({ page }) => {
    // Check component gallery section exists
    await expect(page.locator('h2:has-text("🎨 Component Gallery")')).toBeVisible();

    // Check some component cards are visible
    await expect(page.locator('h3:has-text("Gradient Magic")')).toBeVisible();
    await expect(page.locator('h3:has-text("Interactive Elements")')).toBeVisible();
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('h1:has-text("TAILWIND")')).toBeVisible();

    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('h2:has-text("🚀 Advanced Features")')).toBeVisible();

    // Reset to default viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });
});
