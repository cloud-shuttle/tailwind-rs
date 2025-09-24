const { test, expect } = require('@playwright/test');

test.describe('Enhanced Features Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8082/enhanced-features-demo.html');
    // Wait for the page to fully load
    await page.waitForLoadState('networkidle');
  });

  test('should load the enhanced features demo page', async ({ page }) => {
    // Check page title
    await expect(page).toHaveTitle(/Tailwind-RS Enhanced Features Demo/);
    
    // Check main header
    await expect(page.locator('h1')).toContainText('🎨 Tailwind-RS Enhanced Features Demo');
    
    // Check navigation tabs are present
    const navTabs = page.locator('.nav-tab');
    await expect(navTabs).toHaveCount(10);
    
    // Check all expected tabs are present
    const expectedTabs = [
      '✨ Text Shadow',
      '🎭 Mask', 
      '🌈 Backdrop',
      '📦 Container',
      '🔲 Grid',
      '🔄 Logical',
      '🔌 Plugins',
      '✅ Validation',
      '🪆 Nesting',
      '⚡ Performance'
    ];
    
    for (const tabText of expectedTabs) {
      await expect(page.locator('.nav-tab').filter({ hasText: tabText })).toBeVisible();
    }
  });

  test('should have working navigation tabs', async ({ page }) => {
    // Test tab switching
    const tabs = [
      { text: '🎭 Mask', id: 'tab-mask' },
      { text: '🌈 Backdrop', id: 'tab-backdrop' },
      { text: '📦 Container', id: 'tab-container' },
      { text: '🔲 Grid', id: 'tab-grid' },
      { text: '🔄 Logical', id: 'tab-logical' },
      { text: '🔌 Plugins', id: 'tab-plugins' },
      { text: '✅ Validation', id: 'tab-validation' },
      { text: '🪆 Nesting', id: 'tab-nesting' },
      { text: '⚡ Performance', id: 'tab-performance' }
    ];

    for (const tab of tabs) {
      // Click the tab
      await page.locator('.nav-tab').filter({ hasText: tab.text }).click();
      
      // Check that the tab content is visible
      await expect(page.locator(`#${tab.id}`)).toBeVisible();
      
      // Check that other tabs are hidden
      const otherTabs = tabs.filter(t => t.id !== tab.id);
      for (const otherTab of otherTabs) {
        await expect(page.locator(`#${otherTab.id}`)).toHaveClass(/hidden/);
      }
    }
  });

  test('should display text shadow features correctly', async ({ page }) => {
    // Navigate to text shadow tab
    await page.locator('.nav-tab').filter({ hasText: '✨ Text Shadow' }).click();
    
    // Check text shadow section is visible
    await expect(page.locator('#tab-text-shadow')).toBeVisible();
    
    // Check text shadow examples are present
    await expect(page.locator('h3').filter({ hasText: '✨ Text Shadow Utilities' })).toBeVisible();
    
    // Check text shadow sizes section
    await expect(page.locator('h4').filter({ hasText: 'Text Shadow Sizes' })).toBeVisible();
    
    // Check custom shadows section
    await expect(page.locator('h4').filter({ hasText: 'Custom Shadows' })).toBeVisible();
    
    // Check interactive demo section
    await expect(page.locator('h4').filter({ hasText: 'Interactive Demo' })).toBeVisible();
    
    // Check input field is present
    const textShadowInput = page.locator('#text-shadow-input');
    await expect(textShadowInput).toBeVisible();
    
    // Test interactive input
    await textShadowInput.fill('text-shadow-lg');
    await expect(textShadowInput).toHaveValue('text-shadow-lg');
  });

  test('should display mask utilities correctly', async ({ page }) => {
    // Navigate to mask tab
    await page.locator('.nav-tab').filter({ hasText: '🎭 Mask' }).click();
    
    // Check mask section is visible
    await expect(page.locator('#tab-mask')).toBeVisible();
    
    // Check mask utilities section
    await expect(page.locator('h3').filter({ hasText: '🎭 Mask Utilities' })).toBeVisible();
    
    // Check mask types section
    await expect(page.locator('h4').filter({ hasText: 'Mask Types' })).toBeVisible();
    
    // Check mask modes section
    await expect(page.locator('h4').filter({ hasText: 'Mask Modes' })).toBeVisible();
    
    // Check mask clips section
    await expect(page.locator('h4').filter({ hasText: 'Mask Clips' })).toBeVisible();
  });

  test('should display backdrop filters correctly', async ({ page }) => {
    // Navigate to backdrop tab
    await page.locator('.nav-tab').filter({ hasText: '🌈 Backdrop' }).click();
    
    // Check backdrop section is visible
    await expect(page.locator('#tab-backdrop')).toBeVisible();
    
    // Check backdrop filters section
    await expect(page.locator('h3').filter({ hasText: '🌈 Enhanced Backdrop Filters' })).toBeVisible();
    
    // Check backdrop blur section
    await expect(page.locator('h4').filter({ hasText: 'Backdrop Blur' })).toBeVisible();
    
    // Check backdrop brightness section
    await expect(page.locator('h4').filter({ hasText: 'Backdrop Brightness' })).toBeVisible();
    
    // Check backdrop contrast section
    await expect(page.locator('h4').filter({ hasText: 'Backdrop Contrast' })).toBeVisible();
  });

  test('should display container queries correctly', async ({ page }) => {
    // Navigate to container tab
    await page.locator('.nav-tab').filter({ hasText: '📦 Container' }).click();
    
    // Check container section is visible
    await expect(page.locator('#tab-container')).toBeVisible();
    
    // Check container queries section
    await expect(page.locator('h3').filter({ hasText: '📦 Container Queries' })).toBeVisible();
    
    // Check container sizes section
    await expect(page.locator('h4').filter({ hasText: 'Container Sizes' })).toBeVisible();
    
    // Check aspect ratio queries section
    await expect(page.locator('h4').filter({ hasText: 'Aspect Ratio Queries' })).toBeVisible();
  });

  test('should display CSS grid subgrid correctly', async ({ page }) => {
    // Navigate to grid tab
    await page.locator('.nav-tab').filter({ hasText: '🔲 Grid' }).click();
    
    // Check grid section is visible
    await expect(page.locator('#tab-grid')).toBeVisible();
    
    // Check CSS grid subgrid section
    await expect(page.locator('h3').filter({ hasText: '🔲 CSS Grid Subgrid' })).toBeVisible();
    
    // Check subgrid columns section
    await expect(page.locator('h4').filter({ hasText: 'Subgrid Columns' })).toBeVisible();
    
    // Check subgrid rows section
    await expect(page.locator('h4').filter({ hasText: 'Subgrid Rows' })).toBeVisible();
  });

  test('should display logical properties correctly', async ({ page }) => {
    // Navigate to logical tab
    await page.locator('.nav-tab').filter({ hasText: '🔄 Logical' }).click();
    
    // Check logical section is visible
    await expect(page.locator('#tab-logical')).toBeVisible();
    
    // Check logical properties section
    await expect(page.locator('h3').filter({ hasText: '🔄 Logical Properties' })).toBeVisible();
    
    // Check margin section
    await expect(page.locator('h4').filter({ hasText: 'Margin' })).toBeVisible();
    
    // Check padding section
    await expect(page.locator('h4').filter({ hasText: 'Padding' })).toBeVisible();
    
    // Check border section
    await expect(page.locator('h4').filter({ hasText: 'Border' })).toBeVisible();
  });

  test('should display advanced plugin system correctly', async ({ page }) => {
    // Navigate to plugins tab
    await page.locator('.nav-tab').filter({ hasText: '🔌 Plugins' }).click();
    
    // Check plugins section is visible
    await expect(page.locator('#tab-plugins')).toBeVisible();
    
    // Check advanced plugin system section
    await expect(page.locator('h3').filter({ hasText: '🔌 Advanced Plugin System' })).toBeVisible();
    
    // Check plugin types section
    await expect(page.locator('h4').filter({ hasText: 'Plugin Types' })).toBeVisible();
    
    // Check plugin priority section
    await expect(page.locator('h4').filter({ hasText: 'Plugin Priority' })).toBeVisible();
    
    // Check plugin lifecycle section
    await expect(page.locator('h4').filter({ hasText: 'Plugin Lifecycle' })).toBeVisible();
  });

  test('should display enhanced validation correctly', async ({ page }) => {
    // Navigate to validation tab
    await page.locator('.nav-tab').filter({ hasText: '✅ Validation' }).click();
    
    // Check validation section is visible
    await expect(page.locator('#tab-validation')).toBeVisible();
    
    // Check enhanced validation section
    await expect(page.locator('h3').filter({ hasText: '✅ Enhanced Validation' })).toBeVisible();
    
    // Check validation rules section
    await expect(page.locator('h4').filter({ hasText: 'Validation Rules' })).toBeVisible();
    
    // Check validation severity section
    await expect(page.locator('h4').filter({ hasText: 'Validation Severity' })).toBeVisible();
    
    // Check validation modes section
    await expect(page.locator('h4').filter({ hasText: 'Validation Modes' })).toBeVisible();
  });

  test('should display CSS nesting correctly', async ({ page }) => {
    // Navigate to nesting tab
    await page.locator('.nav-tab').filter({ hasText: '🪆 Nesting' }).click();
    
    // Check nesting section is visible
    await expect(page.locator('#tab-nesting')).toBeVisible();
    
    // Check CSS nesting section
    await expect(page.locator('h3').filter({ hasText: '🪆 CSS Nesting' })).toBeVisible();
    
    // Check nesting selectors section
    await expect(page.locator('h4').filter({ hasText: 'Nesting Selectors' })).toBeVisible();
    
    // Check media queries section
    await expect(page.locator('h4').filter({ hasText: 'Media Queries' })).toBeVisible();
  });

  test('should display performance optimization correctly', async ({ page }) => {
    // Navigate to performance tab
    await page.locator('.nav-tab').filter({ hasText: '⚡ Performance' }).click();
    
    // Check performance section is visible
    await expect(page.locator('#tab-performance')).toBeVisible();
    
    // Check performance optimization section
    await expect(page.locator('h3').filter({ hasText: '⚡ Performance Optimization' })).toBeVisible();
    
    // Check tree shaking section
    await expect(page.locator('h4').filter({ hasText: 'Tree Shaking' })).toBeVisible();
    
    // Check optimization section
    await expect(page.locator('h4').filter({ hasText: 'Optimization' })).toBeVisible();
    
    // Check metrics section
    await expect(page.locator('h4').filter({ hasText: 'Metrics' })).toBeVisible();
  });

  test('should have working dark mode toggle', async ({ page }) => {
    // Check dark mode toggle button is present
    const toggleButton = page.locator('#toggle-theme');
    await expect(toggleButton).toBeVisible();
    
    // Check initial state (should be dark mode button)
    await expect(toggleButton).toContainText('🌙 Dark Mode');
    
    // Click to toggle to dark mode
    await toggleButton.click();
    
    // Check that dark mode is applied
    await expect(page.locator('html')).toHaveClass(/dark/);
    
    // Check button text changes
    await expect(toggleButton).toContainText('☀️ Light Mode');
    
    // Click to toggle back to light mode
    await toggleButton.click();
    
    // Check that dark mode is removed
    await expect(page.locator('html')).not.toHaveClass(/dark/);
    
    // Check button text changes back
    await expect(toggleButton).toContainText('🌙 Dark Mode');
  });

  test('should have responsive design', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Check that navigation tabs are still visible and functional
    await expect(page.locator('.nav-tab').first()).toBeVisible();
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    
    // Check that content is still properly laid out
    await expect(page.locator('main')).toBeVisible();
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    
    // Check that all elements are visible and properly spaced
    await expect(page.locator('main.container')).toBeVisible();
  });

  test('should have proper accessibility features', async ({ page }) => {
    // Check that all interactive elements have proper labels
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const text = await button.textContent();
      expect(text).toBeTruthy(); // All buttons should have text content
    }
    
    // Check that form inputs have proper labels
    const inputs = page.locator('input');
    const inputCount = await inputs.count();
    
    for (let i = 0; i < inputCount; i++) {
      const input = inputs.nth(i);
      const placeholder = await input.getAttribute('placeholder');
      expect(placeholder).toBeTruthy(); // All inputs should have placeholders
    }
  });

  test('should display footer correctly', async ({ page }) => {
    // Check footer is present
    await expect(page.locator('footer')).toBeVisible();
    
    // Check footer content
    await expect(page.locator('footer')).toContainText('Built with ❤️ using Tailwind-RS and WebAssembly');
    await expect(page.locator('footer')).toContainText('Rust-native Tailwind CSS implementation with type safety and performance');
  });

  test('should handle tab switching smoothly', async ({ page }) => {
    // Test rapid tab switching
    const tabs = [
      '✨ Text Shadow',
      '🎭 Mask',
      '🌈 Backdrop',
      '📦 Container',
      '🔲 Grid',
      '🔄 Logical',
      '🔌 Plugins',
      '✅ Validation',
      '🪆 Nesting',
      '⚡ Performance'
    ];

    for (const tabText of tabs) {
      await page.locator('.nav-tab').filter({ hasText: tabText }).click();
      
      // Wait for tab content to be visible
      await page.waitForTimeout(100);
      
      // Check that the tab is active
      const activeTab = page.locator('.nav-tab.active');
      await expect(activeTab).toContainText(tabText);
    }
  });

  test('should have proper styling and visual elements', async ({ page }) => {
    // Check that gradient text is applied
    await expect(page.locator('h1')).toHaveClass(/gradient-text/);
    
    // Check that feature cards have proper styling
    const featureCards = page.locator('.feature-card');
    if (await featureCards.count() > 0) {
      await expect(featureCards.first()).toBeVisible();
    }
    
    // Check that demo containers have proper styling
    await expect(page.locator('.demo-container')).toBeVisible();
  });
});
