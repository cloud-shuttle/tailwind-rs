import { test, expect } from '@playwright/test';

test.describe('Component Gallery', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 15000 });

    // Wait for WASM to load and content to render
    await page.waitForTimeout(3000);

    // Scroll to bottom to ensure all content loads
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    await page.waitForTimeout(2000); // Wait for any lazy loading
    await page.evaluate(() => window.scrollTo(0, 0)); // Scroll back to top
    await page.waitForTimeout(1000);
  });

  test('should display component gallery section', async ({ page }) => {
    const section = page.locator('h2:has-text("🎨 Component Gallery")');
    await expect(section).toBeVisible();

    // Check component grid
    const componentGrid = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-3');
    await expect(componentGrid).toBeVisible();
  });

  test('should display gradient magic card', async ({ page }) => {
    const gradientCard = page.locator('h3:has-text("Gradient Magic")');
    await expect(gradientCard).toBeVisible();

    // Check card content
    await expect(page.locator('text=Beautiful gradient backgrounds')).toBeVisible();
    await expect(page.locator('text=backdrop-blur')).toBeVisible();
    await expect(page.locator('text=gradients')).toBeVisible();
  });

  test('should display interactive elements card', async ({ page }) => {
    const interactiveCard = page.locator('h3:has-text("Interactive Elements")');
    await expect(interactiveCard).toBeVisible();

    // Check card content
    await expect(page.locator('text=Hover effects, transforms')).toBeVisible();

    // Check interactive button
    const clickMeButton = page.locator('button:has-text("Click Me!")');
    await expect(clickMeButton).toBeVisible();
    await clickMeButton.click();
    // Button should still be visible after click
    await expect(clickMeButton).toBeVisible();
  });

  test('should display typography scale card', async ({ page }) => {
    const typographyCard = page.locator('h4:has-text("Typography Scale")');
    await expect(typographyCard).toBeVisible();

    // Check typography examples
    await expect(page.locator('p:has-text("Extra Small")')).toBeVisible();
    await expect(page.locator('p:has-text("Small Text")')).toBeVisible();
    await expect(page.locator('p:has-text("Base Size")')).toBeVisible();
    await expect(page.locator('p:has-text("Large Text")')).toBeVisible();
    await expect(page.locator('p:has-text("Extra Large")')).toBeVisible();
    await expect(page.locator('p:has-text("Display Size")')).toBeVisible();
  });

  test('should display color harmony card', async ({ page }) => {
    // Wait for the component gallery to be visible first
    await expect(page.locator('h2:has-text("🎨 Component Gallery")')).toBeVisible();

    const colorCard = page.locator('h4:has-text("Color Harmony")');
    await expect(colorCard).toBeVisible();

    // Check color description
    await expect(page.locator('text=Full spectrum of Tailwind colors')).toBeVisible();

    // Check color squares exist (8 gradient squares in Color Harmony card)
    const colorSquares = page.locator('div.grid.grid-cols-4.gap-3 div.aspect-square');
    await expect(colorSquares).toHaveCount(8);
  });

  test('should display animation engine card', async ({ page }) => {
    // Wait for the component gallery to be visible first
    await expect(page.locator('h2:has-text("🎨 Component Gallery")')).toBeVisible();

    const animationCard = page.locator('h3:has-text("Animation Engine")');
    await expect(animationCard).toBeVisible();

    // Check bouncing dots
    const bouncingDots = page.locator('div.animate-bounce');
    await expect(bouncingDots).toHaveCount(3);
  });

  test('should display layout system card', async ({ page }) => {
    // Wait for the component gallery to be visible first
    await expect(page.locator('h2:has-text("🎨 Component Gallery")')).toBeVisible();

    const layoutCard = page.locator('h4:has-text("📐 Layout System")');
    await expect(layoutCard).toBeVisible();

    // Check layout description (unique to this card)
    await expect(page.locator('text=CSS Grid & Flexbox')).toBeVisible();
  });

  test('should be responsive', async ({ page }) => {
    // Wait for content to load
    await expect(page.locator('h2:has-text("🎨 Component Gallery")')).toBeVisible();

    // Test mobile layout
    await page.setViewportSize({ width: 375, height: 667 });

    // Component grid should stack on mobile
    const componentGrid = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-3');
    await expect(componentGrid).toBeVisible();

    // Test desktop layout
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(componentGrid).toBeVisible();

    // Reset to default viewport
    await page.setViewportSize({ width: 1280, height: 720 });
  });
});
