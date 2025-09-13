// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Tailwind-RS Comprehensive WASM Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/comprehensive-demo.html');
    await expect(page.locator('#demo-content')).toBeVisible({ timeout: 10000 });
  });

  test('should load the comprehensive demo page successfully', async ({ page }) => {
    // Check that the main demo content is visible
    await expect(page.locator('#demo-content')).toBeVisible();
    
    // Check that the main heading is present
    await expect(page.locator('h1')).toContainText('Tailwind-RS Comprehensive Demo');
    
    // Check that WASM status is shown
    await expect(page.locator('#wasm-stats')).toBeVisible();
  });

  test('should have working navigation tabs', async ({ page }) => {
    // Check that all navigation tabs are present
    await expect(page.locator('.nav-tab[data-tab="class-builder"]')).toBeVisible();
    await expect(page.locator('.nav-tab[data-tab="components"]')).toBeVisible();
    await expect(page.locator('.nav-tab[data-tab="utilities"]')).toBeVisible();
    await expect(page.locator('.nav-tab[data-tab="responsive"]')).toBeVisible();
    await expect(page.locator('.nav-tab[data-tab="performance"]')).toBeVisible();
    
    // Test tab switching
    await page.click('.nav-tab[data-tab="components"]');
    await expect(page.locator('#tab-components')).toBeVisible();
    await expect(page.locator('#tab-class-builder')).toBeHidden();
    
    await page.click('.nav-tab[data-tab="utilities"]');
    await expect(page.locator('#tab-utilities')).toBeVisible();
    await expect(page.locator('#tab-components')).toBeHidden();
  });

  test('should have working class builder functionality', async ({ page }) => {
    // Check that class input is present and functional
    const classInput = page.locator('#class-input');
    await expect(classInput).toBeVisible();
    
    // Test class input
    await classInput.fill('bg-red-500 text-white p-8 rounded-full');
    await expect(page.locator('#generated-classes')).toContainText('bg-red-500 text-white p-8 rounded-full');
    
    // Check that preview updates
    const preview = page.locator('#preview');
    await expect(preview).toHaveClass(/bg-red-500/);
  });

  test('should have working preset buttons', async ({ page }) => {
    // Test preset buttons
    await page.click('.preset-btn[data-classes*="gradient"]');
    
    // Check that input is updated
    const classInput = page.locator('#class-input');
    await expect(classInput).toHaveValue(/gradient/);
    
    // Check that preview updates
    const preview = page.locator('#preview');
    await expect(preview).toHaveClass(/gradient/);
  });

  test('should display components showcase', async ({ page }) => {
    // Navigate to components tab
    await page.click('.nav-tab[data-tab="components"]');
    
    // Check that components are displayed
    await expect(page.locator('#tab-components')).toBeVisible();
    await expect(page.locator('text=Buttons')).toBeVisible();
    await expect(page.locator('text=Cards')).toBeVisible();
    await expect(page.locator('text=Form Elements')).toBeVisible();
  });

  test('should display utilities showcase', async ({ page }) => {
    // Navigate to utilities tab
    await page.click('.nav-tab[data-tab="utilities"]');
    
    // Check that utilities are displayed
    await expect(page.locator('#tab-utilities')).toBeVisible();
    await expect(page.locator('h4:has-text("Spacing")')).toBeVisible();
    await expect(page.locator('h4:has-text("Colors")')).toBeVisible();
    await expect(page.locator('h4:has-text("Typography")')).toBeVisible();
  });

  test('should display responsive design examples', async ({ page }) => {
    // Navigate to responsive tab
    await page.click('.nav-tab[data-tab="responsive"]');
    
    // Check that responsive examples are displayed
    await expect(page.locator('#tab-responsive')).toBeVisible();
    await expect(page.locator('text=Responsive Design')).toBeVisible();
    
    // Check for grid examples
    const gridContainer = page.locator('.grid.grid-cols-1.sm\\:grid-cols-2.md\\:grid-cols-3.lg\\:grid-cols-4');
    await expect(gridContainer).toBeVisible();
  });

  test('should display performance metrics', async ({ page }) => {
    // Navigate to performance tab
    await page.click('.nav-tab[data-tab="performance"]');
    
    // Check that performance metrics are displayed
    await expect(page.locator('#tab-performance')).toBeVisible();
    await expect(page.locator('text=Performance Metrics')).toBeVisible();
    await expect(page.locator('text=WASM Performance')).toBeVisible();
    await expect(page.locator('text=Benchmarks')).toBeVisible();
  });

  test('should handle dark mode correctly', async ({ page }) => {
    // Check that dark mode toggle is present
    const toggleButton = page.locator('#toggle-theme');
    await expect(toggleButton).toBeVisible();
    
    // Check initial state (light mode)
    const body = page.locator('body');
    await expect(body).toHaveClass(/demo-container/);
    
    // Toggle to dark mode
    await toggleButton.click();
    
    // Check that dark mode is applied
    await expect(body).toHaveClass(/dark/);
    
    // Check that icon changes
    const icon = toggleButton.locator('span');
    await expect(icon).toHaveText('🌙');
    
    // Toggle back to light mode
    await toggleButton.click();
    
    // Check that light mode is restored
    await expect(body).not.toHaveClass(/dark/);
    await expect(icon).toHaveText('☀️');
  });

  test('should update WASM statistics in real-time', async ({ page }) => {
    // Check initial WASM stats
    const wasmStats = page.locator('#wasm-stats');
    await expect(wasmStats).toBeVisible();
    
    // Update class input
    const classInput = page.locator('#class-input');
    await classInput.fill('bg-blue-500 text-white p-4 rounded-lg hover:bg-blue-600 transition-colors');
    
    // Check that stats update
    await expect(wasmStats).toContainText(/Classes: \d+/);
    await expect(wasmStats).toContainText(/Builder: Active/);
  });

  test('should be responsive on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Check that demo content is still visible
    await expect(page.locator('#demo-content')).toBeVisible();
    
    // Check that navigation is responsive
    const navTabs = page.locator('.nav-tab');
    await expect(navTabs.first()).toBeVisible();
    
    // Check that content adapts to mobile
    const mainContent = page.locator('main');
    await expect(mainContent).toBeVisible();
  });

  test('should handle errors gracefully', async ({ page }) => {
    // Check that error handling is in place
    const errorDiv = page.locator('#error');
    await expect(errorDiv).toHaveClass(/hidden/);
    
    // The demo should load without errors
    await expect(page.locator('#demo-content')).toBeVisible();
  });

  test('should have proper accessibility attributes', async ({ page }) => {
    // Check for proper heading hierarchy
    await expect(page.locator('h1')).toBeVisible();
    
    // Check for proper button accessibility
    const buttons = page.locator('button');
    await expect(buttons.first()).toBeVisible();
    
    // Check for proper form accessibility
    const inputs = page.locator('input, textarea');
    await expect(inputs.first()).toBeVisible();
  });

  test('should update generated classes display', async ({ page }) => {
    const classInput = page.locator('#class-input');
    const generatedClasses = page.locator('#generated-classes');
    
    // Test different class combinations
    const testClasses = [
      'bg-blue-500 text-white p-4',
      'bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl',
      'bg-gray-100 border-2 border-gray-300 p-4 rounded'
    ];
    
    for (const classes of testClasses) {
      await classInput.fill(classes);
      await expect(generatedClasses).toContainText(classes);
    }
  });
});
