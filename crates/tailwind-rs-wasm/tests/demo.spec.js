// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Tailwind-RS WASM Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo.html');
  });

  test('should load the demo page successfully', async ({ page }) => {
    // Wait for the demo content to be visible
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    // Check that the main heading is present
    await expect(page.locator('h1')).toContainText('Tailwind-RS WASM Demo');
    
    // Check that the loading indicator is hidden
    await expect(page.locator('#loading')).toBeHidden();
  });

  test('should display WASM information correctly', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    // Check WASM module info
    await expect(page.locator('#wasm-module')).toContainText('tailwind_rs_wasm.js');
    await expect(page.locator('#wasm-builder')).toContainText('WasmClassBuilder');
    
    // Check that generated classes are displayed
    await expect(page.locator('#generated-classes')).toBeVisible();
  });

  test('should have interactive class input', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    const classInput = page.locator('#class-input');
    const preview = page.locator('#preview');
    
    // Check initial state
    await expect(classInput).toHaveValue('bg-blue-500 text-white p-4 rounded-lg');
    
    // Test input change
    await classInput.fill('bg-red-500 text-white p-8 rounded-xl');
    await expect(classInput).toHaveValue('bg-red-500 text-white p-8 rounded-xl');
    
    // Check that preview updates
    await expect(preview).toHaveClass(/bg-red-500/);
  });

  test('should have working preset buttons', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    const classInput = page.locator('#class-input');
    const preview = page.locator('#preview');
    
    // Test Blue Card preset
    await page.locator('button[data-classes="bg-blue-500 text-white p-4 rounded-lg"]').click();
    await expect(classInput).toHaveValue('bg-blue-500 text-white p-4 rounded-lg');
    await expect(preview).toHaveClass(/bg-blue-500/);
    
    // Test Gradient preset
    await page.locator('button[data-classes="bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl"]').click();
    await expect(classInput).toHaveValue('bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl');
    await expect(preview).toHaveClass(/bg-gradient-to-r/);
    
    // Test Bordered preset
    await page.locator('button[data-classes="bg-gray-100 dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 p-4 rounded"]').click();
    await expect(classInput).toHaveValue('bg-gray-100 dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 p-4 rounded');
    await expect(preview).toHaveClass(/border-2/);
    
    // Test Yellow Circle preset
    await page.locator('button[data-classes="bg-yellow-400 text-black p-4 rounded-full shadow-lg"]').click();
    await expect(classInput).toHaveValue('bg-yellow-400 text-black p-4 rounded-full shadow-lg');
    await expect(preview).toHaveClass(/bg-yellow-400/);
  });

  test('should display feature cards correctly', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    // Check that all three feature cards are present
    const featureCards = page.locator('.grid > div').filter({ hasText: /Performance|Type Safety|Dynamic Styling/ });
    await expect(featureCards).toHaveCount(3);
    
    // Check specific feature cards
    await expect(page.locator('text=🚀 Performance')).toBeVisible();
    await expect(page.locator('text=🛡️ Type Safety')).toBeVisible();
    await expect(page.locator('text=🎨 Dynamic Styling')).toBeVisible();
  });

  test('should handle dark mode correctly', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    // Check that the demo content is present
    const demoContent = page.locator('#demo-content');
    await expect(demoContent).toBeVisible();
    
    // Check that dark mode styles are applied
    const preview = page.locator('#preview');
    await expect(preview).toHaveClass(/dark:border-gray-600/);
  });

  test('should be responsive on mobile', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Check that the layout is responsive
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('#preview')).toBeVisible();
    await expect(page.locator('#class-input')).toBeVisible();
    
    // Check that preset buttons are in a grid
    const presetButtons = page.locator('.preset-btn');
    await expect(presetButtons).toHaveCount(4);
  });

  test('should handle errors gracefully', async ({ page }) => {
    // Test error handling by checking if error div exists (but should be hidden)
    await expect(page.locator('#error')).toBeHidden();
    
    // Check that loading indicator is hidden after successful load
    await expect(page.locator('#loading')).toBeHidden({ timeout: 10000 });
  });

  test('should have proper accessibility attributes', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    // Check for proper test IDs
    await expect(page.locator('[data-testid="app"]')).toBeVisible();
    
    // Check for proper labels
    await expect(page.locator('label[for="class-input"]')).toBeVisible();
    
    // Check for proper button accessibility
    const presetButtons = page.locator('.preset-btn');
    await expect(presetButtons.first()).toBeVisible();
  });

  test('should update generated classes display', async ({ page }) => {
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
    
    const classInput = page.locator('#class-input');
    const generatedClasses = page.locator('#generated-classes');
    
    // Test that generated classes update when input changes
    await classInput.fill('bg-green-500 text-white p-2 rounded');
    await expect(generatedClasses).toContainText('bg-green-500 text-white p-2 rounded');
    
    // Test with a different set of classes
    await classInput.fill('bg-purple-600 text-yellow-300 p-8 rounded-full shadow-2xl');
    await expect(generatedClasses).toContainText('bg-purple-600 text-yellow-300 p-8 rounded-full shadow-2xl');
  });
});
