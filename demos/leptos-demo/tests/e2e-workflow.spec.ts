import { test, expect } from '@playwright/test';

test.describe('End-to-End Workflow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should complete full user journey', async ({ page }) => {
    // 1. User lands on home page
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
    await expect(page.locator('p:has-text("Experience the power of Rust-native Tailwind CSS")')).toBeVisible();
    
    // 2. User explores feature cards
    const featureCards = page.locator('.bg-white.dark\\:bg-gray-800.rounded-lg.shadow-md');
    await expect(featureCards).toHaveCount(3);
    
    // 3. User clicks "Get Started" button
    const getStartedButton = page.locator('button:has-text("Get Started")');
    await expect(getStartedButton).toBeVisible();
    await getStartedButton.click();
    
    // 4. User navigates to Components page
    await page.click('button:has-text("Components")');
    await expect(page.locator('h2:has-text("Button Components")')).toBeVisible();
    
    // 5. User explores different button styles
    const primaryButton = page.locator('button:has-text("Primary")');
    await expect(primaryButton).toBeVisible();
    await expect(primaryButton).toHaveClass(/bg-blue-600/);
    
    const successButton = page.locator('button:has-text("Success")');
    await expect(successButton).toBeVisible();
    await expect(successButton).toHaveClass(/bg-green-600/);
    
    // 6. User explores card components
    await expect(page.locator('h2:has-text("Card Components")')).toBeVisible();
    const cards = page.locator('.bg-white.dark\\:bg-gray-800.rounded-lg.shadow-md');
    await expect(cards).toHaveCount(3);
    
    // 7. User navigates to Dynamic page
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    // 8. User experiments with dynamic styling
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Test preset styles
    await page.click('button:has-text("Gradient")');
    await expect(inputField).toHaveValue('bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl');
    await expect(previewArea).toHaveClass(/bg-gradient-to-r/);
    
    // Test custom classes
    await inputField.clear();
    await inputField.fill('bg-red-500 text-white p-8 rounded-full shadow-lg');
    await expect(previewArea).toHaveClass(/bg-red-500/);
    await expect(previewArea).toHaveClass(/p-8/);
    await expect(previewArea).toHaveClass(/rounded-full/);
    
    // 9. User toggles theme
    await page.click('button:has-text("☀️")');
    await expect(page.locator('button:has-text("🌙")')).toBeVisible();
    
    // 10. User returns to home page
    await page.click('button:has-text("Home")');
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
  });

  test('should handle theme switching throughout journey', async ({ page }) => {
    // Start in light mode
    await expect(page.locator('button:has-text("☀️")')).toBeVisible();
    
    // Navigate to components page
    await page.click('button:has-text("Components")');
    await expect(page.locator('h2:has-text("Button Components")')).toBeVisible();
    
    // Switch to dark mode
    await page.click('button:has-text("☀️")');
    await expect(page.locator('button:has-text("🌙")')).toBeVisible();
    
    // Navigate to dynamic page in dark mode
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    // Switch back to light mode
    await page.click('button:has-text("🌙")');
    await expect(page.locator('button:has-text("☀️")')).toBeVisible();
    
    // Return to home page
    await page.click('button:has-text("Home")');
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
  });

  test('should maintain state across navigation', async ({ page }) => {
    // Navigate to dynamic page
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    // Set custom classes
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    await inputField.clear();
    await inputField.fill('bg-green-500 text-white p-4 rounded-lg');
    
    // Navigate away and back
    await page.click('button:has-text("Components")');
    await page.click('button:has-text("Dynamic")');
    
    // Check that state is maintained
    await expect(inputField).toHaveValue('bg-green-500 text-white p-4 rounded-lg');
    
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    await expect(previewArea).toHaveClass(/bg-green-500/);
  });

  test('should handle rapid navigation', async ({ page }) => {
    // Rapidly navigate between pages
    const pages = ['Home', 'Components', 'Dynamic'];
    
    for (let i = 0; i < 3; i++) {
      for (const pageName of pages) {
        await page.click(`button:has-text("${pageName}")`);
        await page.waitForTimeout(100); // Small delay
      }
    }
    
    // App should still be functional
    await page.click('button:has-text("Home")');
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
  });

  test('should handle concurrent user interactions', async ({ page }) => {
    // Start multiple interactions simultaneously
    const interactions = [
      page.click('button:has-text("Components")'),
      page.click('button:has-text("☀️")'),
    ];
    
    await Promise.all(interactions);
    
    // App should handle concurrent interactions gracefully
    await expect(page.locator('h2:has-text("Button Components")')).toBeVisible();
    await expect(page.locator('button:has-text("🌙")')).toBeVisible();
  });

  test('should handle form interactions', async ({ page }) => {
    // Navigate to dynamic page
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Test various input scenarios
    const testCases = [
      'bg-blue-500 text-white p-4 rounded-lg',
      'bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl',
      'bg-gray-100 dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 p-4 rounded',
      'bg-yellow-400 text-black p-4 rounded-full shadow-lg',
    ];
    
    for (const testCase of testCases) {
      await inputField.clear();
      await inputField.fill(testCase);
      await expect(inputField).toHaveValue(testCase);
      await expect(previewArea).toBeVisible();
    }
  });

  test('should handle error scenarios gracefully', async ({ page }) => {
    // Navigate to dynamic page
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Test invalid input
    await inputField.clear();
    await inputField.fill('invalid-class another-invalid-class');
    
    // App should handle invalid input gracefully
    await expect(inputField).toHaveValue('invalid-class another-invalid-class');
    await expect(previewArea).toBeVisible();
    
    // Test empty input
    await inputField.clear();
    await expect(inputField).toHaveValue('');
    await expect(previewArea).toBeVisible();
    
    // Test very long input
    const longInput = 'bg-red-500 text-white p-4 rounded-lg shadow-lg border-2 border-blue-500 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500 transition-colors duration-200 ease-in-out';
    await inputField.fill(longInput);
    await expect(inputField).toHaveValue(longInput);
    await expect(previewArea).toBeVisible();
  });

  test('should handle responsive design changes', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
    
    // Navigate to components page on mobile
    await page.click('button:has-text("Components")');
    await expect(page.locator('h2:has-text("Button Components")')).toBeVisible();
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('h2:has-text("Button Components")')).toBeVisible();
    
    // Navigate to dynamic page on tablet
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    // Return to home page
    await page.click('button:has-text("Home")');
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
  });
});
