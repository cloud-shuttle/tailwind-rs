import { test, expect } from '@playwright/test';

test.describe('Dynamic Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
    await page.click('button:has-text("Dynamic")');
  });

  test('should display dynamic class generation interface', async ({ page }) => {
    const title = page.locator('h2:has-text("Dynamic Class Generation")');
    await expect(title).toBeVisible();
    
    // Check preview area
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    await expect(previewArea).toBeVisible();
    await expect(previewArea).toContainText('Dynamic Styling Preview');
    
    // Check input field
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    await expect(inputField).toBeVisible();
    await expect(inputField).toHaveValue('bg-blue-500 text-white p-4 rounded-lg');
  });

  test('should update preview when classes change', async ({ page }) => {
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Clear and enter new classes
    await inputField.clear();
    await inputField.fill('bg-red-500 text-white p-8 rounded-full');
    
    // Check that preview updates
    await expect(previewArea).toHaveClass(/bg-red-500/);
    await expect(previewArea).toHaveClass(/p-8/);
    await expect(previewArea).toHaveClass(/rounded-full/);
  });

  test('should display preset style buttons', async ({ page }) => {
    const presetButtons = page.locator('button:has-text("Blue Card"), button:has-text("Gradient"), button:has-text("Bordered"), button:has-text("Yellow Circle")');
    await expect(presetButtons).toHaveCount(4);
    
    // Check button grid
    const buttonGrid = page.locator('.grid.grid-cols-2.md\\:grid-cols-4');
    await expect(buttonGrid).toBeVisible();
  });

  test('should apply preset styles when clicked', async ({ page }) => {
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Click Gradient preset
    await page.click('button:has-text("Gradient")');
    
    // Check that input and preview update
    await expect(inputField).toHaveValue('bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl');
    await expect(previewArea).toHaveClass(/bg-gradient-to-r/);
    await expect(previewArea).toHaveClass(/from-purple-500/);
    await expect(previewArea).toHaveClass(/to-pink-500/);
  });

  test('should display generated classes', async ({ page }) => {
    const generatedClassesSection = page.locator('h4:has-text("Generated Classes:")');
    await expect(generatedClassesSection).toBeVisible();
    
    const codeElement = page.locator('code');
    await expect(codeElement).toBeVisible();
    await expect(codeElement).toContainText('bg-blue-500 text-white p-4 rounded-lg');
  });

  test('should update generated classes display', async ({ page }) => {
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const codeElement = page.locator('code');
    
    // Change classes
    await inputField.clear();
    await inputField.fill('bg-green-500 text-black p-2 rounded-none');
    
    // Check that generated classes display updates
    await expect(codeElement).toContainText('bg-green-500 text-black p-2 rounded-none');
  });

  test('should handle invalid classes gracefully', async ({ page }) => {
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    
    // Enter invalid classes
    await inputField.clear();
    await inputField.fill('invalid-class another-invalid-class');
    
    // Preview should still be visible (classes are applied as-is)
    await expect(previewArea).toBeVisible();
    await expect(previewArea).toHaveClass(/invalid-class/);
  });

  test('should support dark mode', async ({ page }) => {
    // Toggle to dark mode
    await page.click('button:has-text("☀️")');
    
    // Check that input field adapts
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    await expect(inputField).toHaveClass(/dark:bg-gray-700/);
    await expect(inputField).toHaveClass(/dark:text-gray-100/);
    
    // Check that generated classes section adapts
    const generatedSection = page.locator('.bg-gray-50.dark\\:bg-gray-800');
    await expect(generatedSection).toBeVisible();
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile layout
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Preset buttons should stack on mobile
    const buttonGrid = page.locator('.grid.grid-cols-2.md\\:grid-cols-4');
    await expect(buttonGrid).toBeVisible();
    
    // Preview area should be visible
    const previewArea = page.locator('.w-full.h-32.rounded-lg.border-2.border-dashed');
    await expect(previewArea).toBeVisible();
    
    // Test desktop layout
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(buttonGrid).toBeVisible();
    await expect(previewArea).toBeVisible();
  });

  test('should maintain state across navigation', async ({ page }) => {
    const inputField = page.locator('input[placeholder*="Enter Tailwind classes"]');
    
    // Change classes
    await inputField.clear();
    await inputField.fill('bg-purple-500 text-white p-6 rounded-xl');
    
    // Navigate away and back
    await page.click('button:has-text("Home")');
    await page.click('button:has-text("Dynamic")');
    
    // Check that state is maintained
    await expect(inputField).toHaveValue('bg-purple-500 text-white p-6 rounded-xl');
  });
});
