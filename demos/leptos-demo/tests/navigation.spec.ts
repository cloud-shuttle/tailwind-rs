import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the app to load
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should display the header with logo and navigation', async ({ page }) => {
    // Check header is visible
    const header = page.locator('header');
    await expect(header).toBeVisible();
    
    // Check logo
    const logo = header.locator('h1');
    await expect(logo).toContainText('🎨 Tailwind-RS Demo');
    
    // Check navigation buttons
    const navButtons = header.locator('button');
    await expect(navButtons).toHaveCount(4); // Home, Components, Dynamic, Theme toggle
    
    // Check specific navigation items
    await expect(header.locator('button:has-text("Home")')).toBeVisible();
    await expect(header.locator('button:has-text("Components")')).toBeVisible();
    await expect(header.locator('button:has-text("Dynamic")')).toBeVisible();
  });

  test('should navigate between pages', async ({ page }) => {
    // Start on Home page
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
    
    // Navigate to Components page
    await page.click('button:has-text("Components")');
    await expect(page.locator('h2:has-text("Button Components")')).toBeVisible();
    
    // Navigate to Dynamic page
    await page.click('button:has-text("Dynamic")');
    await expect(page.locator('h2:has-text("Dynamic Class Generation")')).toBeVisible();
    
    // Navigate back to Home
    await page.click('button:has-text("Home")');
    await expect(page.locator('h1:has-text("Tailwind-RS Demo")')).toBeVisible();
  });

  test('should highlight active navigation item', async ({ page }) => {
    // Home should be active by default
    const homeButton = page.locator('button:has-text("Home")');
    await expect(homeButton).toHaveClass(/bg-blue-100/);
    
    // Navigate to Components
    await page.click('button:has-text("Components")');
    const componentsButton = page.locator('button:has-text("Components")');
    await expect(componentsButton).toHaveClass(/bg-blue-100/);
    await expect(homeButton).not.toHaveClass(/bg-blue-100/);
  });

  test('should toggle theme', async ({ page }) => {
    const themeButton = page.locator('button:has-text("☀️")');
    await expect(themeButton).toBeVisible();
    
    // Click theme toggle
    await themeButton.click();
    
    // Check that theme icon changed
    await expect(page.locator('button:has-text("🌙")')).toBeVisible();
    
    // Check that body classes changed for dark mode
    const body = page.locator('body');
    await expect(body).toHaveClass(/dark/);
    
    // Toggle back
    await page.click('button:has-text("🌙")');
    await expect(page.locator('button:has-text("☀️")')).toBeVisible();
  });

  test('should display footer', async ({ page }) => {
    const footer = page.locator('footer');
    await expect(footer).toBeVisible();
    await expect(footer).toContainText('Built with ❤️ using Tailwind-RS and Leptos');
    await expect(footer).toContainText('Rust-native Tailwind CSS implementation');
  });
});
