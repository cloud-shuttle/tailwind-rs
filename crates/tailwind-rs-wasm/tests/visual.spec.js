// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Tailwind-RS WASM Demo - Visual Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo.html');
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
  });

  test('should match visual snapshot of the main demo page', async ({ page }) => {
    // Take a full page screenshot
    await expect(page).toHaveScreenshot('demo-main-page.png');
  });

  test('should match visual snapshot of the preview area', async ({ page }) => {
    // Take a screenshot of just the preview area
    const preview = page.locator('#preview');
    await expect(preview).toHaveScreenshot('demo-preview-default.png');
  });

  test('should match visual snapshot with gradient preset', async ({ page }) => {
    // Apply gradient preset
    await page.locator('button[data-classes="bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl"]').click();
    
    // Take screenshot of preview with gradient
    const preview = page.locator('#preview');
    await expect(preview).toHaveScreenshot('demo-preview-gradient.png');
  });

  test('should match visual snapshot with yellow circle preset', async ({ page }) => {
    // Apply yellow circle preset
    await page.locator('button[data-classes="bg-yellow-400 text-black p-4 rounded-full shadow-lg"]').click();
    
    // Take screenshot of preview with yellow circle
    const preview = page.locator('#preview');
    await expect(preview).toHaveScreenshot('demo-preview-yellow-circle.png');
  });

  test('should match visual snapshot of feature cards', async ({ page }) => {
    // Take screenshot of the feature cards section
    const featureCards = page.locator('.grid > div').filter({ hasText: /Performance|Type Safety|Dynamic Styling/ });
    await expect(featureCards).toHaveScreenshot('demo-feature-cards.png');
  });

  test('should match visual snapshot on mobile viewport', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Take a full page screenshot on mobile
    await expect(page).toHaveScreenshot('demo-mobile-view.png');
  });

  test('should match visual snapshot of the header section', async ({ page }) => {
    // Take screenshot of the header section
    const header = page.locator('header');
    await expect(header).toHaveScreenshot('demo-header.png');
  });

  test('should match visual snapshot of the WASM info section', async ({ page }) => {
    // Take screenshot of the WASM info section
    const wasmInfo = page.locator('.bg-gray-50.dark\\:bg-gray-800');
    await expect(wasmInfo).toHaveScreenshot('demo-wasm-info.png');
  });
});
