import { chromium } from 'playwright';

async function checkSSRDemo() {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  console.log('🔍 Checking Tailwind-RS SSR Demo styling...');

  try {
    // Navigate to the SSR demo
    await page.goto('http://localhost:3001');
    await page.waitForTimeout(1000); // Wait for page to load

    console.log('📄 SSR Page loaded, checking for content...');

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

      const titleBackground = await titleElement.evaluate(el => {
        return window.getComputedStyle(el).backgroundImage;
      });
      console.log('🌈 Title background (gradient):', titleBackground);
    }

    // Check for body background gradient
    const bodyElement = await page.locator('body');
    const bodyBackground = await bodyElement.evaluate(el => {
      return window.getComputedStyle(el).backgroundImage;
    });
    console.log('🌊 Body background gradient:', bodyBackground);

    // Check for CSS in <style> tag
    const styleTag = await page.locator('style');
    if (await styleTag.count() > 0) {
      const cssContent = await styleTag.textContent();
      console.log('📄 Inline CSS found, length:', cssContent.length);

      // Check for key CSS rules
      const hasMinHeight = cssContent.includes('min-height: 100vh');
      const hasGradient = cssContent.includes('linear-gradient');
      const hasText6xl = cssContent.includes('font-size: var(--text-6xl');
      const hasBgClipText = cssContent.includes('background-clip: text');

      console.log('✅ CSS contains min-height:', hasMinHeight);
      console.log('✅ CSS contains gradients:', hasGradient);
      console.log('✅ CSS contains text-6xl:', hasText6xl);
      console.log('✅ CSS contains bg-clip-text:', hasBgClipText);
    } else {
      console.log('❌ No inline <style> tag found');
    }

    // Check for external CSS links (should be none for SSR)
    const cssLinks = await page.locator('link[rel="stylesheet"]').all();
    console.log('📄 External CSS links found:', cssLinks.length);

    // Check for any purple/cyan text
    const coloredTextElements = await page.locator('[class*="text-"]').all();
    console.log('🎨 Colored text elements found:', coloredTextElements.length);

    for (let i = 0; i < Math.min(coloredTextElements.length, 5); i++) {
      const classes = await coloredTextElements[i].getAttribute('class');
      const color = await coloredTextElements[i].evaluate(el => {
        return window.getComputedStyle(el).color;
      });
      console.log(`🎨 Element ${i + 1} classes: ${classes}`);
      console.log(`🎨 Element ${i + 1} color: ${color}`);
    }

    // Check for gradient backgrounds
    const gradientElements = await page.locator('[class*="bg-gradient"]').all();
    console.log('🌈 Gradient background elements found:', gradientElements.length);

    for (let i = 0; i < Math.min(gradientElements.length, 3); i++) {
      const background = await gradientElements[i].evaluate(el => {
        return window.getComputedStyle(el).backgroundImage;
      });
      console.log(`🌈 Gradient element ${i + 1} background:`, background);
    }

    // Check for the success message
    const successElement = await page.locator('text="CSS Generated Server-Side: This page was rendered with Tailwind-RS!"');
    if (await successElement.count() > 0) {
      console.log('✅ Success message found');
      const successColor = await successElement.evaluate(el => {
        return window.getComputedStyle(el).color;
      });
      console.log('✅ Success message color:', successColor);
    } else {
      console.log('❌ Success message not found');
    }

    // Take a screenshot
    await page.screenshot({ path: 'ssr-demo-screenshot.png', fullPage: true });
    console.log('📸 Screenshot saved as ssr-demo-screenshot.png');

  } catch (error) {
    console.error('❌ Error:', error.message);
  } finally {
    await browser.close();
  }
}

checkSSRDemo();
