import { test, expect } from '@playwright/test';

test.describe('Visual Regression', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should match home page screenshot', async ({ page }) => {
    // Take screenshot of home page
    await expect(page).toHaveScreenshot('home-page.png');
  });

  test('should match components page screenshot', async ({ page }) => {
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    
    // Take screenshot of components page
    await expect(page).toHaveScreenshot('components-page.png');
  });

  test('should match dynamic page screenshot', async ({ page }) => {
    await page.click('button:has-text("Dynamic")');
    await page.waitForSelector('h2:has-text("Dynamic Class Generation")');
    
    // Take screenshot of dynamic page
    await expect(page).toHaveScreenshot('dynamic-page.png');
  });

  test('should match dark mode screenshots', async ({ page }) => {
    // Toggle to dark mode
    await page.click('button:has-text("☀️")');
    
    // Take screenshot of home page in dark mode
    await expect(page).toHaveScreenshot('home-page-dark.png');
    
    // Navigate to components page in dark mode
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    await expect(page).toHaveScreenshot('components-page-dark.png');
    
    // Navigate to dynamic page in dark mode
    await page.click('button:has-text("Dynamic")');
    await page.waitForSelector('h2:has-text("Dynamic Class Generation")');
    await expect(page).toHaveScreenshot('dynamic-page-dark.png');
  });

  test('should match mobile screenshots', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Take screenshot of home page on mobile
    await expect(page).toHaveScreenshot('home-page-mobile.png');
    
    // Navigate to components page on mobile
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    await expect(page).toHaveScreenshot('components-page-mobile.png');
    
    // Navigate to dynamic page on mobile
    await page.click('button:has-text("Dynamic")');
    await page.waitForSelector('h2:has-text("Dynamic Class Generation")');
    await expect(page).toHaveScreenshot('dynamic-page-mobile.png');
  });

  test('should match tablet screenshots', async ({ page }) => {
    // Set tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    
    // Take screenshot of home page on tablet
    await expect(page).toHaveScreenshot('home-page-tablet.png');
    
    // Navigate to components page on tablet
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    await expect(page).toHaveScreenshot('components-page-tablet.png');
  });

  test('should match desktop screenshots', async ({ page }) => {
    // Set desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    
    // Take screenshot of home page on desktop
    await expect(page).toHaveScreenshot('home-page-desktop.png');
    
    // Navigate to components page on desktop
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    await expect(page).toHaveScreenshot('components-page-desktop.png');
  });

  test('should match dynamic styling screenshots', async ({ page }) => {
    await page.click('button:has-text("Dynamic")');
    await page.waitForSelector('h2:has-text("Dynamic Class Generation")');
    
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Test different preset styles
    const presets = [
      { button: 'Blue Card', screenshot: 'dynamic-blue-card.png' },
      { button: 'Gradient', screenshot: 'dynamic-gradient.png' },
      { button: 'Bordered', screenshot: 'dynamic-bordered.png' },
      { button: 'Yellow Circle', screenshot: 'dynamic-yellow-circle.png' },
    ];
    
    for (const preset of presets) {
      await page.click(`button:has-text("${preset.button}")`);
      await expect(previewArea).toHaveScreenshot(preset.screenshot);
    }
  });

  test('should match header component screenshot', async ({ page }) => {
    // Take screenshot of just the header
    const header = page.locator('header');
    await expect(header).toHaveScreenshot('header-component.png');
  });

  test('should match footer component screenshot', async ({ page }) => {
    // Scroll to footer
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    
    // Take screenshot of just the footer
    const footer = page.locator('footer');
    await expect(footer).toHaveScreenshot('footer-component.png');
  });

  test('should match feature cards screenshot', async ({ page }) => {
    // Take screenshot of feature cards section
    const featureGrid = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-3');
    await expect(featureGrid).toHaveScreenshot('feature-cards.png');
  });

  test('should match button components screenshot', async ({ page }) => {
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    
    // Take screenshot of button components section
    const buttonSection = page.locator('h2:has-text("Button Components")').locator('..');
    await expect(buttonSection).toHaveScreenshot('button-components.png');
  });

  test('should match card components screenshot', async ({ page }) => {
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Card Components")');
    
    // Take screenshot of card components section
    const cardSection = page.locator('h2:has-text("Card Components")').locator('..');
    await expect(cardSection).toHaveScreenshot('card-components.png');
  });
});
