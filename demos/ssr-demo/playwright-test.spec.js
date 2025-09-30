const { test, expect } = require('@playwright/test');

  test.describe('Tailwind-RS SSR Demo', () => {
    test.beforeEach(async ({ page }) => {
      // Wait for the server to be ready
      await page.goto('http://localhost:3000', {
        waitUntil: 'domcontentloaded',
        timeout: 10000
      });
    });

  test('should load the demo page', async ({ page }) => {
    await expect(page).toHaveTitle(/🚀 Tailwind-RS Objects Demo/);
    await expect(page.locator('h1')).toContainText('Tailwind-RS Objects Demo');
  });

  test('should display the status indicator', async ({ page }) => {
    const statusIndicator = page.locator('text=✅ Tailwind-RS Objects Active');
    await expect(statusIndicator).toBeVisible();
  });

  test('should show typography showcase section', async ({ page }) => {
    // Wait for the typography showcase section
    await page.waitForSelector('text=🔤 Typography Showcase');

    // Check that the typography section is present
    await expect(page.locator('text=🔤 Typography Showcase')).toBeVisible();

    // Check that we have typography examples (simplified check)
    const typographySection = page.locator('[class*="grid-cols-1"]').first();
    await expect(typographySection).toBeVisible();

    // Check that we have some typography-related content
    await expect(page.locator('text=Font Black')).toBeVisible();
    await expect(page.locator('text=Font Bold')).toBeVisible();
    await expect(page.locator('text=Font Mono')).toBeVisible();
  });

  test('should display pangram text with different fonts', async ({ page }) => {
    // Check that the pangram appears multiple times (once for each font)
    const pangrams = page.locator('text=Sphinx of black quartz, judge my vow');
    await expect(pangrams).toHaveCount(4); // Should appear in each of the 4 typography cards
  });

  test('should verify font classes are applied correctly', async ({ page }) => {
    // Check that CSS is loaded
    const cssResponse = await page.request.get('http://localhost:3000/styles.css');
    expect(cssResponse.ok()).toBeTruthy();

    const cssContent = await cssResponse.text();

    // Verify font classes are in the generated CSS
    expect(cssContent).toContain('font-black');
    expect(cssContent).toContain('font-bold');
    expect(cssContent).toContain('font-semibold');
    expect(cssContent).toContain('font-medium');
    expect(cssContent).toContain('font-mono');
    expect(cssContent).toContain('font-sans');
    expect(cssContent).toContain('font-serif');

    // Verify font-weight values
    expect(cssContent).toContain('font-weight: 900'); // font-black
    expect(cssContent).toContain('font-weight: 700'); // font-bold
    expect(cssContent).toContain('font-weight: 600'); // font-semibold
    expect(cssContent).toContain('font-weight: 500'); // font-medium

    // Verify font-family values
    expect(cssContent).toContain('font-family: ui-monospace');
    expect(cssContent).toContain('font-family: ui-sans-serif');
    expect(cssContent).toContain('font-family: ui-serif');
  });

  test('should show interactive counter functionality', async ({ page }) => {
    // Check initial counter value
    const counter = page.locator('#count');
    await expect(counter).toHaveText('0');

    // Test increment
    await page.locator('text=⬆️ Increment').click();
    await expect(counter).toHaveText('1');

    // Test decrement
    await page.locator('text=⬇️ Decrement').click();
    await expect(counter).toHaveText('0');

    // Test reset
    await page.locator('text=⬆️ Increment').click();
    await page.locator('text=⬆️ Increment').click();
    await expect(counter).toHaveText('2');

    await page.locator('text=🔄 Reset').click();
    await expect(counter).toHaveText('0');
  });

  test('should display Tailwind-RS Objects features', async ({ page }) => {
    await page.waitForSelector('text=🚀 Tailwind-RS Objects Features');

    // Check that feature descriptions are displayed in the features list
    const featuresList = page.locator('.space-y-2.text-blue-300');
    await expect(featuresList.locator('text=CssGenerator::new()')).toBeVisible();
    await expect(featuresList.locator('text=generator.add_class()')).toBeVisible();
    await expect(featuresList.locator('text=generator.generate_css()')).toBeVisible();
    await expect(featuresList.locator('text=ClassBuilder::new()')).toBeVisible();
  });

  test('should have working CSS gradients', async ({ page }) => {
    // Check that gradient classes are applied
    const gradientElements = page.locator('[class*="bg-gradient-to"]');
    await expect(gradientElements.first()).toBeVisible();
  });

  test('should capture visual screenshot', async ({ page }) => {
    // Take a full page screenshot to verify visual appearance
    await page.screenshot({
      path: 'test-results/demo-visual-test.png',
      fullPage: true
    });
  });

  test('should verify CSS parsing coverage', async ({ page }) => {
    // The server logs show coverage stats, but we can verify key classes are working
    // by checking computed styles

    // Check that a font-black element has the correct computed font-weight
    const fontBlackElement = page.locator('text=Ultra Bold');
    const fontWeight = await fontBlackElement.evaluate(el =>
      window.getComputedStyle(el).fontWeight
    );
    expect(fontWeight).toBe('900');

    // Check that font-mono elements have monospace font family
    const fontMonoElement = page.locator('text=CODE FONT');
    const fontFamily = await fontMonoElement.evaluate(el =>
      window.getComputedStyle(el).fontFamily
    );
    expect(fontFamily.toLowerCase()).toContain('mono');
  });

  test('should test responsive design', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('text=🔤 Typography Showcase')).toBeVisible();

    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('text=🔤 Typography Showcase')).toBeVisible();

    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('text=🔤 Typography Showcase')).toBeVisible();
  });
});
