import { test, expect } from '@playwright/test';

test.describe('Components Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
    await page.click('button:has-text("Components")');
  });

  test('should display button components section', async ({ page }) => {
    const section = page.locator('h2:has-text("Button Components")');
    await expect(section).toBeVisible();
    
    // Check button grid
    const buttonGrid = page.locator('.grid.grid-cols-2.md\\:grid-cols-3.lg\\:grid-cols-5');
    await expect(buttonGrid).toBeVisible();
    
    // Check primary buttons
    const primaryButton = page.locator('button:has-text("Primary")');
    await expect(primaryButton).toBeVisible();
    await expect(primaryButton).toHaveClass(/bg-blue-600/);
    
    const secondaryButton = page.locator('button:has-text("Secondary")');
    await expect(secondaryButton).toBeVisible();
    await expect(secondaryButton).toHaveClass(/bg-gray-200/);
    
    const successButton = page.locator('button:has-text("Success")');
    await expect(successButton).toBeVisible();
    await expect(successButton).toHaveClass(/bg-green-600/);
    
    const dangerButton = page.locator('button:has-text("Danger")');
    await expect(dangerButton).toBeVisible();
    await expect(dangerButton).toHaveClass(/bg-red-600/);
    
    const outlineButton = page.locator('button:has-text("Outline")');
    await expect(outlineButton).toBeVisible();
    await expect(outlineButton).toHaveClass(/border-2.*border-blue-600/);
  });

  test('should display button size variants', async ({ page }) => {
    const sizeButtons = page.locator('button:has-text("Small"), button:has-text("Medium"), button:has-text("Large")');
    await expect(sizeButtons).toHaveCount(3);
    
    // Check size classes
    const smallButton = page.locator('button:has-text("Small")');
    await expect(smallButton).toHaveClass(/text-sm/);
    
    const largeButton = page.locator('button:has-text("Large")');
    await expect(largeButton).toHaveClass(/text-lg/);
  });

  test('should display card components section', async ({ page }) => {
    const section = page.locator('h2:has-text("Card Components")');
    await expect(section).toBeVisible();
    
    // Check card grid
    const cardGrid = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-3');
    await expect(cardGrid).toBeVisible();
    
    // Check individual cards
    const cards = page.locator('.bg-white.dark\\:bg-gray-800.rounded-lg.shadow-md');
    await expect(cards).toHaveCount(3);
    
    // Check Basic Card
    const basicCard = cards.nth(0);
    await expect(basicCard).toContainText('Basic Card');
    await expect(basicCard).toContainText('This is a basic card component');
    
    // Check Feature Card
    const featureCard = cards.nth(1);
    await expect(featureCard).toContainText('Feature Card');
    await expect(featureCard).toContainText('Cards can contain any content');
    
    // Check card without title
    const noTitleCard = cards.nth(2);
    await expect(noTitleCard).toContainText('Card without Title Prop');
  });

  test('should have interactive elements', async ({ page }) => {
    // Check that buttons are clickable
    const learnMoreButton = page.locator('button:has-text("Learn More")');
    await expect(learnMoreButton).toBeVisible();
    await learnMoreButton.click();
    // Button should still be visible after click (no navigation)
    await expect(learnMoreButton).toBeVisible();
  });

  test('should support dark mode', async ({ page }) => {
    // Toggle to dark mode
    await page.click('button:has-text("☀️")');
    
    // Check that cards adapt to dark mode
    const cards = page.locator('.bg-white.dark\\:bg-gray-800');
    await expect(cards).toHaveCount(3);
    
    // Check that button text adapts
    const secondaryButton = page.locator('button:has-text("Secondary")');
    await expect(secondaryButton).toHaveClass(/dark:text-gray-100/);
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile layout
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Button grid should stack on mobile
    const buttonGrid = page.locator('.grid.grid-cols-2.md\\:grid-cols-3.lg\\:grid-cols-5');
    await expect(buttonGrid).toBeVisible();
    
    // Card grid should stack on mobile
    const cardGrid = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-3');
    await expect(cardGrid).toBeVisible();
    
    // Test desktop layout
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(buttonGrid).toBeVisible();
    await expect(cardGrid).toBeVisible();
  });
});
