import { test, expect } from '@playwright/test';

test.describe('Home Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should display hero section', async ({ page }) => {
    // Check main title
    const title = page.locator('h1:has-text("Tailwind-RS Demo")');
    await expect(title).toBeVisible();
    await expect(title).toHaveClass(/text-5xl/);
    
    // Check subtitle
    const subtitle = page.locator('p:has-text("Experience the power of Rust-native Tailwind CSS")');
    await expect(subtitle).toBeVisible();
    await expect(subtitle).toHaveClass(/text-xl/);
  });

  test('should display call-to-action buttons', async ({ page }) => {
    const ctaButtons = page.locator('button:has-text("Get Started"), button:has-text("View Documentation")');
    await expect(ctaButtons).toHaveCount(2);
    
    // Check button styles
    const getStartedButton = page.locator('button:has-text("Get Started")');
    await expect(getStartedButton).toHaveClass(/bg-blue-500/);
    
    const docButton = page.locator('button:has-text("View Documentation")');
    await expect(docButton).toHaveClass(/border-2.*border-blue-600/);
  });

  test('should display feature cards', async ({ page }) => {
    const featureCards = page.locator('.bg-white.dark\\:bg-gray-800.rounded-lg.shadow-md');
    await expect(featureCards).toHaveCount(3);
    
    // Check Performance card
    const performanceCard = featureCards.nth(0);
    await expect(performanceCard).toContainText('🚀 Performance');
    await expect(performanceCard).toContainText('Optimized for speed with tree-shaking');
    
    // Check Type Safety card
    const typeSafetyCard = featureCards.nth(1);
    await expect(typeSafetyCard).toContainText('🛡️ Type Safety');
    await expect(typeSafetyCard).toContainText('Compile-time validation ensures');
    
    // Check Dynamic Styling card
    const dynamicCard = featureCards.nth(2);
    await expect(dynamicCard).toContainText('🎨 Dynamic Styling');
    await expect(dynamicCard).toContainText('Generate classes at runtime');
  });

  test('should have responsive layout', async ({ page }) => {
    // Check grid layout
    const featureGrid = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-3');
    await expect(featureGrid).toBeVisible();
    
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(featureGrid).toBeVisible();
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(featureGrid).toBeVisible();
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(featureGrid).toBeVisible();
  });

  test('should support dark mode', async ({ page }) => {
    // Toggle to dark mode
    await page.click('button:has-text("☀️")');
    
    // Check that cards have dark mode classes
    const featureCards = page.locator('.bg-white.dark\\:bg-gray-800');
    await expect(featureCards).toHaveCount(3);
    
    // Check that text colors adapt
    const subtitle = page.locator('p:has-text("Experience the power")');
    await expect(subtitle).toHaveClass(/dark:text-gray-300/);
  });
});
