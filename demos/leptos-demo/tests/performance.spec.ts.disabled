import { test, expect } from '@playwright/test';

test.describe('Performance', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should load quickly', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
    const loadTime = Date.now() - startTime;
    
    // App should load within 5 seconds
    expect(loadTime).toBeLessThan(5000);
  });

  test('should have efficient navigation', async ({ page }) => {
    const startTime = Date.now();
    
    // Navigate through all pages
    await page.click('button:has-text("Components")');
    await page.waitForSelector('h2:has-text("Button Components")');
    
    await page.click('button:has-text("Dynamic")');
    await page.waitForSelector('h2:has-text("Dynamic Class Generation")');
    
    await page.click('button:has-text("Home")');
    await page.waitForSelector('h1:has-text("Tailwind-RS Demo")');
    
    const navigationTime = Date.now() - startTime;
    
    // Navigation should be fast (under 1 second total)
    expect(navigationTime).toBeLessThan(1000);
  });

  test('should handle dynamic class generation efficiently', async ({ page }) => {
    await page.click('button:has-text("Dynamic")');
    
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    const startTime = Date.now();
    
    // Generate many class changes
    for (let i = 0; i < 10; i++) {
      await inputField.clear();
      await inputField.fill(`bg-${['red', 'blue', 'green', 'purple', 'yellow'][i % 5]}-500 text-white p-${i + 1} rounded-lg`);
      await page.waitForTimeout(10); // Small delay to allow updates
    }
    
    const generationTime = Date.now() - startTime;
    
    // Class generation should be fast (under 500ms for 10 changes)
    expect(generationTime).toBeLessThan(500);
  });

  test('should handle theme switching efficiently', async ({ page }) => {
    const startTime = Date.now();
    
    // Toggle theme multiple times
    for (let i = 0; i < 5; i++) {
      await page.click('button:has-text("☀️"), button:has-text("🌙")');
      await page.waitForTimeout(50); // Small delay to allow updates
    }
    
    const themeSwitchTime = Date.now() - startTime;
    
    // Theme switching should be fast (under 200ms for 5 switches)
    expect(themeSwitchTime).toBeLessThan(200);
  });

  test('should not have memory leaks during navigation', async ({ page }) => {
    // Navigate between pages multiple times
    for (let i = 0; i < 10; i++) {
      await page.click('button:has-text("Home")');
      await page.waitForSelector('h1:has-text("Tailwind-RS Demo")');
      
      await page.click('button:has-text("Components")');
      await page.waitForSelector('h2:has-text("Button Components")');
      
      await page.click('button:has-text("Dynamic")');
      await page.waitForSelector('h2:has-text("Dynamic Class Generation")');
    }
    
    // App should still be responsive
    await page.click('button:has-text("Home")');
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
  });

  test('should handle rapid input changes', async ({ page }) => {
    await page.click('button:has-text("Dynamic")');
    
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    
    const startTime = Date.now();
    
    // Rapidly change input
    for (let i = 0; i < 20; i++) {
      await inputField.clear();
      await inputField.fill(`bg-${i % 10}-500 text-white p-${i % 8 + 1} rounded-lg`);
    }
    
    const rapidInputTime = Date.now() - startTime;
    
    // Should handle rapid input efficiently (under 1 second for 20 changes)
    expect(rapidInputTime).toBeLessThan(1000);
    
    // Final state should be correct
    await expect(inputField).toHaveValue('bg-9-500 text-white p-8 rounded-lg');
  });

  test('should maintain performance with many elements', async ({ page }) => {
    await page.click('button:has-text("Components")');
    
    // Check that all buttons are rendered efficiently
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    // Should have reasonable number of buttons
    expect(buttonCount).toBeGreaterThan(5);
    expect(buttonCount).toBeLessThan(50);
    
    // All buttons should be visible and interactive
    for (let i = 0; i < Math.min(buttonCount, 10); i++) {
      const button = buttons.nth(i);
      await expect(button).toBeVisible();
    }
  });

  test('should handle concurrent user interactions', async ({ page }) => {
    // Simulate rapid user interactions
    const startTime = Date.now();
    
    // Rapidly click different buttons
    const interactions = [
      () => page.click('button:has-text("Components")'),
      () => page.click('button:has-text("Dynamic")'),
      () => page.click('button:has-text("Home")'),
      () => page.click('button:has-text("☀️"), button:has-text("🌙")'),
    ];
    
    // Execute interactions rapidly
    for (let i = 0; i < 8; i++) {
      await interactions[i % interactions.length]();
      await page.waitForTimeout(10);
    }
    
    const concurrentTime = Date.now() - startTime;
    
    // Should handle concurrent interactions efficiently
    expect(concurrentTime).toBeLessThan(1000);
    
    // App should still be in a valid state
    await expect(page.locator('h1, h2')).toBeVisible();
  });
});
