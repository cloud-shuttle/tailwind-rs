import { test, expect } from '@playwright/test';

test.describe('Home Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="app"]', { timeout: 10000 });
  });

  test('should display hero section', async ({ page }) => {
    // Check main title - matches actual demo content
    const title = page.locator('h1:has-text("TAILWIND")');
    await expect(title).toBeVisible();

    // Check subtitle
    const subtitle = page.locator('p').filter({ hasText: 'The most advanced WebAssembly-powered CSS generation engine' });
    await expect(subtitle).toBeVisible();

    // Check action buttons
    const getStartedButton = page.locator('button:has-text("Get Started")');
    const viewSourceButton = page.locator('button:has-text("View Source")');
    await expect(getStartedButton).toBeVisible();
    await expect(viewSourceButton).toBeVisible();

    // Check stats (more specific locators to avoid strict mode violations)
    await expect(page.locator('div.text-3xl.font-bold.text-cyan-400').filter({ hasText: '100%' })).toBeVisible();
    await expect(page.locator('div.text-3xl.font-bold.text-purple-400').filter({ hasText: '<1ms' })).toBeVisible();
    await expect(page.locator('div.text-3xl.font-bold.text-pink-400').filter({ hasText: '0KB' })).toBeVisible();
    await expect(page.locator('div.text-3xl.font-bold.text-orange-400').filter({ hasText: '∞' })).toBeVisible();
  });

  test('should display component gallery', async ({ page }) => {
    // Wait for hero to load first
    await expect(page.locator('h1:has-text("TAILWIND")')).toBeVisible();

    // Check component gallery title
    const galleryTitle = page.locator('h2:has-text("🎨 Component Gallery")');
    await expect(galleryTitle).toBeVisible();

    // Check component cards
    await expect(page.locator('h3:has-text("Gradient Magic")')).toBeVisible();
    await expect(page.locator('h3:has-text("Interactive Elements")')).toBeVisible();
    await expect(page.locator('h4:has-text("Typography Scale")')).toBeVisible();
    await expect(page.locator('h4:has-text("Color Harmony")')).toBeVisible();
    await expect(page.locator('h3:has-text("Animation Engine")')).toBeVisible();
    await expect(page.locator('h4:has-text("📐 Layout System")')).toBeVisible();
  });

  test('should display advanced features section', async ({ page }) => {
    // Check advanced features title
    const featuresTitle = page.locator('h2:has-text("🚀 Advanced Features")');
    await expect(featuresTitle).toBeVisible();

    // Check feature sections
    await expect(page.locator('h3:has-text("✨ Visual Effects")')).toBeVisible();
    await expect(page.locator('h3:has-text("🎭 Hover Interactions")')).toBeVisible();
    await expect(page.locator('h3:has-text("📱 Responsive Design")')).toBeVisible();
    await expect(page.locator('h3:has-text("⚡ Performance")')).toBeVisible();
  });

  test('should display typography showcase', async ({ page }) => {
    // Check typography card exists
    const typographyCard = page.locator('h4:has-text("Typography Scale")');
    await expect(typographyCard).toBeVisible();

    // Check different text sizes
    await expect(page.locator('p:has-text("Extra Small")')).toBeVisible();
    await expect(page.locator('p:has-text("Small Text")')).toBeVisible();
    await expect(page.locator('p:has-text("Base Size")')).toBeVisible();
    await expect(page.locator('p:has-text("Large Text")')).toBeVisible();
    await expect(page.locator('p:has-text("Extra Large")')).toBeVisible();
    await expect(page.locator('p:has-text("Display Size")')).toBeVisible();
  });

  test('should display color palette', async ({ page }) => {
    // Check color harmony title
    const colorTitle = page.locator('h4:has-text("Color Harmony")');
    await expect(colorTitle).toBeVisible();

    // Check color description
    await expect(page.locator('text=Full spectrum of Tailwind colors with opacity variants')).toBeVisible();

    // Check color squares exist (8 gradient squares in Color Harmony card)
    const colorSquares = page.locator('div.grid.grid-cols-4.gap-3 div.aspect-square');
    await expect(colorSquares).toHaveCount(8);
  });

  test('should debug page content', async ({ page }) => {
    // Wait for page to load
    await page.waitForTimeout(3000);

    // Check the main app div (which has the classes)
    const appDivInfo = await page.evaluate(() => {
      const app = document.querySelector('[data-testid="app"]');
      if (!app) return { exists: false };

      const computed = window.getComputedStyle(app);
      return {
        exists: true,
        classes: app.className,
        minHeight: computed.minHeight,
        backgroundImage: computed.backgroundImage,
        color: computed.color
      };
    });
    console.log('App div info:', appDivInfo);

    // Check if the gradient variables are set correctly
    const gradientVars = await page.evaluate(() => {
      const app = document.querySelector('[data-testid="app"]');
      if (!app) return {};

      const computed = window.getComputedStyle(app);
      return {
        from: computed.getPropertyValue('--tw-gradient-from'),
        via: computed.getPropertyValue('--tw-gradient-via'),
        to: computed.getPropertyValue('--tw-gradient-to'),
        stops: computed.getPropertyValue('--tw-gradient-stops')
      };
    });
    console.log('Gradient CSS variables:', gradientVars);
  });
});
