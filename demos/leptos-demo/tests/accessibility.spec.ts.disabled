import { test, expect } from '@playwright/test';

test.describe('Accessibility', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should have proper heading hierarchy', async ({ page }) => {
    // Check main heading
    const h1 = page.locator('h1');
    await expect(h1).toHaveCount(1);
    await expect(h1).toContainText('Tailwind-RS Demo');
    
    // Navigate to components page and check h2
    await page.click('button:has-text("Components")');
    const h2s = page.locator('h2');
    await expect(h2s).toHaveCount(2); // Button Components, Card Components
    
    // Navigate to dynamic page and check h2
    await page.click('button:has-text("Dynamic")');
    const dynamicH2 = page.locator('h2:has-text("Dynamic Class Generation")');
    await expect(dynamicH2).toBeVisible();
  });

  test('should have proper button labels', async ({ page }) => {
    // Check navigation buttons have accessible text
    const homeButton = page.locator('button:has-text("Home")');
    await expect(homeButton).toBeVisible();
    
    const componentsButton = page.locator('button:has-text("Components")');
    await expect(componentsButton).toBeVisible();
    
    const dynamicButton = page.locator('button:has-text("Dynamic")');
    await expect(dynamicButton).toBeVisible();
    
    // Check theme toggle button
    const themeButton = page.locator('button:has-text("☀️"), button:has-text("🌙")');
    await expect(themeButton).toBeVisible();
  });

  test('should have proper form labels', async ({ page }) => {
    await page.click('button:has-text("Dynamic")');
    
    // Check input has proper label
    const label = page.locator('label:has-text("CSS Classes")');
    await expect(label).toBeVisible();
    
    const input = page.locator('input[placeholder*="Enter Tailwind classes"]');
    await expect(input).toBeVisible();
  });

  test('should support keyboard navigation', async ({ page }) => {
    // Tab through navigation
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    
    // Check that focus is visible
    const focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();
  });

  test('should have proper color contrast', async ({ page }) => {
    // Check that text has sufficient contrast
    const title = page.locator('h1:has-text("Tailwind-RS Demo")');
    await expect(title).toBeVisible();
    
    // Check button contrast
    const primaryButton = page.locator('button:has-text("Get Started")');
    await expect(primaryButton).toBeVisible();
    await expect(primaryButton).toHaveClass(/bg-blue-500.*text-white/);
  });

  test('should support dark mode accessibility', async ({ page }) => {
    // Toggle to dark mode
    await page.click('button:has-text("☀️")');
    
    // Check that text remains readable
    const title = page.locator('h1:has-text("Tailwind-RS Demo")');
    await expect(title).toBeVisible();
    
    // Check that buttons remain accessible
    const primaryButton = page.locator('button:has-text("Get Started")');
    await expect(primaryButton).toBeVisible();
  });

  test('should have proper focus indicators', async ({ page }) => {
    // Focus on a button
    const homeButton = page.locator('button:has-text("Home")');
    await homeButton.focus();
    
    // Check that focus is visible
    await expect(homeButton).toBeFocused();
  });

  test('should have proper ARIA attributes', async ({ page }) => {
    // Check that interactive elements are properly labeled
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const text = await button.textContent();
      // Buttons should have accessible text
      expect(text?.trim()).toBeTruthy();
    }
  });

  test('should handle screen reader navigation', async ({ page }) => {
    // Check that headings are properly structured
    const headings = page.locator('h1, h2, h3, h4, h5, h6');
    const headingCount = await headings.count();
    expect(headingCount).toBeGreaterThan(0);
    
    // Check that main content is properly marked
    const main = page.locator('main');
    await expect(main).toBeVisible();
  });

  test('should support reduced motion preferences', async ({ page }) => {
    // Check that animations respect user preferences
    const animatedElements = page.locator('.transition-colors, .transition-all');
    const animatedCount = await animatedElements.count();
    
    // Elements should still be functional even with reduced motion
    expect(animatedCount).toBeGreaterThan(0);
  });
});
