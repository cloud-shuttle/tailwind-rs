import { test, expect } from '@playwright/test';

test.describe('Home Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should display hero section', async ({ page }) => {
    // Check main title
    const title = page.locator('h1:has-text("Tailwind-RS Demo - Actually Using Core Models!")');
    await expect(title).toBeVisible();
    await expect(title).toHaveClass(/text-4xl/);

    // Check that the demo shows the dynamic class preview
    const previewArea = page.locator('div.w-full.h-32.rounded-lg.border-2.border-dashed:has-text("Preview Area")');
    await expect(previewArea).toBeVisible();
  });

  test('should have dynamic class input', async ({ page }) => {
    // Check input field exists
    const inputField = page.locator('input[placeholder="Enter Tailwind classes"]');
    await expect(inputField).toBeVisible();

    // Check input has proper styling
    await expect(inputField).toHaveClass(/border.*border-gray-300/);

    // Check label exists
    const label = page.locator('label:has-text("Enter Tailwind classes:")');
    await expect(label).toBeVisible();
  });

  test('should display demo sections', async ({ page }) => {
    // Check for the dynamic class preview section
    const previewSection = page.locator('h2:has-text("Dynamic Class Preview")');
    await expect(previewSection).toBeVisible();

    // Check for the generated CSS section
    const cssSection = page.locator('h2:has-text("Generated CSS (from Tailwind-RS Core)")');
    await expect(cssSection).toBeVisible();

    // Check for the current classes section
    const classesSection = page.locator('h2:has-text("Current Classes")');
    await expect(classesSection).toBeVisible();

    // Check that classes are actually generated and displayed
    const cssPre = page.locator('pre');
    await expect(cssPre).toBeVisible();
    const cssContent = await cssPre.textContent();
    expect(cssContent).toContain('bg-blue-500');
  });

  test('should have responsive layout', async ({ page }) => {
    // Check that the main container has responsive classes
    const container = page.locator('.container.mx-auto.px-4.py-8');
    await expect(container).toBeVisible();

    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(container).toBeVisible();

    // Test desktop viewport
    await page.setViewportSize({ width: 1024, height: 768 });
    await expect(container).toBeVisible();
  });

  test('should demonstrate dynamic class functionality', async ({ page }) => {
    // Get initial preview area classes - use more specific selector
    const previewArea = page.locator('div.w-full.h-32.rounded-lg.border-2.border-dashed');
    const initialClass = await previewArea.getAttribute('class');

    // Type new classes into the input
    const inputField = page.locator('input[placeholder="Enter Tailwind classes"]');
    await inputField.fill('bg-red-500 text-white p-6 rounded-xl');

    // Check that the preview area updates (this shows the demo is interactive)
    await expect(previewArea).toHaveClass(/bg-red-500/);
    await expect(previewArea).toHaveClass(/text-white/);

    // Check that current classes section shows the new classes
    const currentClasses = page.locator('code');
    await expect(currentClasses).toContainText('bg-red-500 text-white p-6 rounded-xl');
  });
});
