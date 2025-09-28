import { chromium } from 'playwright';

async function checkStyling() {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  console.log('🔍 Checking Tailwind-RS demo styling...');

  try {
    // Navigate to the demo
    await page.goto('http://localhost:8080');
    await page.waitForTimeout(2000); // Wait for WASM to load

    console.log('📄 Page loaded, checking for content...');

    // Check if content is loaded
    const content = await page.textContent('body');
    console.log('Page content length:', content.length);

    // Check for specific elements and their colors
    const titleElement = await page.locator('h1').first();
    if (await titleElement.count() > 0) {
      const titleText = await titleElement.textContent();
      console.log('📝 Title found:', titleText);

      const titleColor = await titleElement.evaluate(el => {
        return window.getComputedStyle(el).color;
      });
      console.log('🎨 Title color:', titleColor);
    }

    // Check for purple/cyan text
    const purpleElements = await page.locator('[class*="text-purple"]').all();
    console.log('🟣 Purple elements found:', purpleElements.length);

    for (let i = 0; i < Math.min(purpleElements.length, 3); i++) {
      const color = await purpleElements[i].evaluate(el => {
        return window.getComputedStyle(el).color;
      });
      console.log(`🟣 Purple element ${i + 1} color:`, color);
    }

    const cyanElements = await page.locator('[class*="text-cyan"]').all();
    console.log('🔵 Cyan elements found:', cyanElements.length);

    for (let i = 0; i < Math.min(cyanElements.length, 3); i++) {
      const color = await cyanElements[i].evaluate(el => {
        return window.getComputedStyle(el).color;
      });
      console.log(`🔵 Cyan element ${i + 1} color:`, color);
    }

    // Check background gradients
    const gradientElements = await page.locator('[class*="bg-gradient"]').all();
    console.log('🌈 Gradient elements found:', gradientElements.length);

    for (let i = 0; i < Math.min(gradientElements.length, 2); i++) {
      const background = await gradientElements[i].evaluate(el => {
        return window.getComputedStyle(el).background;
      });
      console.log(`🌈 Gradient element ${i + 1} background:`, background);
    }

    // Check if CSS is loaded
    const cssLinks = await page.locator('link[rel="stylesheet"]').all();
    console.log('📄 CSS links found:', cssLinks.length);

    for (let link of cssLinks) {
      const href = await link.getAttribute('href');
      console.log('📄 CSS href:', href);
    }

    // Check for CSS content
    const cssResponse = await page.request.get('http://localhost:8080/assets/generated.css');
    console.log('📄 CSS response status:', cssResponse.status());
    console.log('📄 CSS content length:', cssResponse.text().length);

    // Take a screenshot
    await page.screenshot({ path: 'demo-screenshot.png', fullPage: true });
    console.log('📸 Screenshot saved as demo-screenshot.png');

  } catch (error) {
    console.error('❌ Error:', error.message);
  } finally {
    await browser.close();
  }
}

checkStyling();
