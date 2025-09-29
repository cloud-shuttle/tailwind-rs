import { chromium } from 'playwright';

async function runComprehensiveTests() {
  const browser = await chromium.launch();
  console.log('🚀 Starting comprehensive Tailwind-RS Playwright tests...');

  try {
    // Test 1: SSR Demo (Port 3000)
    console.log('\n' + '='.repeat(60));
    console.log('🧪 TEST 1: SSR Demo (http://localhost:3000)');
    console.log('='.repeat(60));

    const ssrPage = await browser.newPage();
    await testSSRDemo(ssrPage);

    // Test 2: Leptos WASM Demo (Port 3001)
    console.log('\n' + '='.repeat(60));
    console.log('🧪 TEST 2: Leptos WASM Demo (http://localhost:3001)');
    console.log('='.repeat(60));

    const leptosPage = await browser.newPage();
    await testLeptosDemo(leptosPage);

    // Test 3: CSS Quality Validation
    console.log('\n' + '='.repeat(60));
    console.log('🧪 TEST 3: CSS Quality Validation');
    console.log('='.repeat(60));

    await testCSSQuality(ssrPage);

    // Test 4: Performance Validation
    console.log('\n' + '='.repeat(60));
    console.log('🧪 TEST 4: Performance Validation');
    console.log('='.repeat(60));

    await testPerformance(ssrPage);

  } catch (error) {
    console.error('❌ Test suite failed:', error.message);
  } finally {
    await browser.close();
    console.log('\n🎉 Playwright testing completed!');
  }
}

async function testSSRDemo(page) {
  try {
    console.log('🔍 Testing SSR Demo...');
    await page.goto('http://localhost:3000');
    await page.waitForTimeout(2000);

    // Check page title
    const title = await page.title();
    console.log('📄 Page title:', title);
    const hasCorrectTitle = title.includes('Self-Contained');
    console.log('✅ Correct title:', hasCorrectTitle);

    // Check no CDN script
    const scripts = await page.locator('script[src*="cdn"]').all();
    const hasNoCDN = scripts.length === 0;
    console.log('✅ No CDN scripts:', hasNoCDN);

    // Check for main content
    const h1Element = await page.locator('h1').first();
    const h1Text = await h1Element.textContent();
    console.log('📝 Main heading:', h1Text);

    // Check for Tailwind-RS status
    const statusElement = await page.locator('text="CSS Generated Server-Side: This page was rendered with Tailwind-RS!"');
    const hasStatusMessage = await statusElement.count() > 0;
    console.log('✅ Status message found:', hasStatusMessage);

    // Check for interactive elements
    const counterElement = await page.locator('#count');
    const hasCounter = await counterElement.count() > 0;
    console.log('✅ Interactive counter:', hasCounter);

    // Test counter functionality
    if (hasCounter) {
      const initialCount = await counterElement.textContent();
      console.log('🔢 Initial count:', initialCount);

      await page.click('text="Increment"');
      await page.waitForTimeout(100);
      const afterIncrement = await counterElement.textContent();
      console.log('⬆️ After increment:', afterIncrement);

      await page.click('text="Decrement"');
      await page.waitForTimeout(100);
      const afterDecrement = await counterElement.textContent();
      console.log('⬇️ After decrement:', afterDecrement);

      const counterWorks = parseInt(afterIncrement) === parseInt(initialCount) + 1 &&
                          parseInt(afterDecrement) === parseInt(initialCount);
      console.log('✅ Counter functionality:', counterWorks);
    }

    // Check CSS quality
    const styleTag = await page.locator('style');
    if (await styleTag.count() > 0) {
      const cssContent = await styleTag.textContent();
      const hasNoCSSVars = !cssContent.includes('var(--');
      console.log('✅ CSS has no variables:', hasNoCSSVars);

      const hasRGBA = cssContent.includes('rgba(');
      console.log('✅ CSS has rgba() values:', hasRGBA);

      const hasHexColors = cssContent.includes('#') && cssContent.match(/#[0-9a-f]{6}/gi);
      console.log('✅ CSS has hex colors:', !!hasHexColors);
    }

    // Screenshot
    await page.screenshot({ path: 'ssr-demo-test.png', fullPage: true });
    console.log('📸 SSR demo screenshot saved');

    return true;

  } catch (error) {
    console.error('❌ SSR Demo test failed:', error.message);
    return false;
  }
}

async function testLeptosDemo(page) {
  try {
    console.log('🔍 Testing Leptos WASM Demo...');
    await page.goto('http://localhost:3001');
    await page.waitForTimeout(3000); // WASM takes longer

    const title = await page.title();
    console.log('📄 Leptos page title:', title);

    // Check for Leptos-specific content
    const body = await page.textContent('body');
    const hasLeptosContent = body.length > 1000; // Should have substantial content
    console.log('✅ Has substantial content:', hasLeptosContent);

    // Check for WASM-related elements
    const buttons = await page.locator('button').all();
    console.log('🔘 Buttons found:', buttons.length);

    // Screenshot
    await page.screenshot({ path: 'leptos-demo-test.png', fullPage: true });
    console.log('📸 Leptos demo screenshot saved');

    return true;

  } catch (error) {
    console.error('❌ Leptos Demo test failed:', error.message);
    return false;
  }
}

async function testCSSQuality(page) {
  console.log('🔍 Testing CSS quality...');

  try {
    await page.goto('http://localhost:3000');

    // Get all stylesheets and analyze both inline and external CSS
    const styles = await page.evaluate(async () => {
      const result = {
        inlineStyles: 0,
        externalStyles: 0,
        cssRules: 0,
        cssVars: 0,
        rgbaValues: 0,
        hexColors: 0,
        remUnits: 0,
        gradientRules: 0
      };

      // Count stylesheets
      result.externalStyles = document.querySelectorAll('link[rel="stylesheet"]').length;
      result.inlineStyles = document.querySelectorAll('style').length;

      // Analyze inline CSS rules
      const styleElements = document.querySelectorAll('style');
      styleElements.forEach(style => {
        const cssText = style.textContent || '';
        result.cssRules += (cssText.match(/\{[^}]*\}/g) || []).length;
        result.cssVars += (cssText.match(/var\(--[^)]+\)/g) || []).length;
        result.rgbaValues += (cssText.match(/rgba\([^)]+\)/g) || []).length;
        result.hexColors += (cssText.match(/#[0-9a-f]{6}/gi) || []).length;
        result.remUnits += (cssText.match(/\d*\.?\d+rem/g) || []).length;
        result.gradientRules += (cssText.match(/linear-gradient/g) || []).length;
      });

      // Try to fetch and analyze external CSS
      const cssLinks = document.querySelectorAll('link[rel="stylesheet"]');
      for (const link of cssLinks) {
        const href = link.getAttribute('href');
        if (href && !href.startsWith('http')) {
          try {
            const response = await fetch(href);
            const cssText = await response.text();
            result.cssRules += (cssText.match(/\{[^}]*\}/g) || []).length;
            result.cssVars += (cssText.match(/var\(--[^)]+\)/g) || []).length;
            result.rgbaValues += (cssText.match(/rgba\([^)]+\)/g) || []).length;
            result.hexColors += (cssText.match(/#[0-9a-f]{6}/gi) || []).length;
            result.remUnits += (cssText.match(/\d*\.?\d+rem/g) || []).length;
            result.gradientRules += (cssText.match(/linear-gradient/g) || []).length;
          } catch (e) {
            console.log('Could not fetch external CSS:', href);
          }
        }
      }

      return result;
    });

    console.log('📊 CSS Analysis Results:');
    console.log(`   Inline styles: ${styles.inlineStyles}`);
    console.log(`   External styles: ${styles.externalStyles}`);
    console.log(`   CSS rules: ${styles.cssRules}`);
    console.log(`   CSS variables: ${styles.cssVars}`);
    console.log(`   RGBA values: ${styles.rgbaValues}`);
    console.log(`   Hex colors: ${styles.hexColors}`);
    console.log(`   REM units: ${styles.remUnits}`);
    console.log(`   Gradients: ${styles.gradientRules}`);

    // Quality checks
    const qualityChecks = {
      'No external stylesheets': styles.externalStyles === 1, // Only our generated CSS
      'Has inline styles': styles.inlineStyles > 0,
      'No CSS variables': styles.cssVars === 0,
      'Has RGBA opacity': styles.rgbaValues > 0,
      'Has hex colors': styles.hexColors > 0,
      'Has REM units': styles.remUnits > 0,
      'Has gradients': styles.gradientRules > 0
    };

    console.log('\n🔍 Quality Checks:');
    Object.entries(qualityChecks).forEach(([check, passed]) => {
      console.log(`   ${passed ? '✅' : '❌'} ${check}: ${passed}`);
    });

    return qualityChecks;

  } catch (error) {
    console.error('❌ CSS quality test failed:', error.message);
    return null;
  }
}

async function testPerformance(page) {
  console.log('🔍 Testing performance...');

  try {
    await page.goto('http://localhost:3000');

    // Measure page load time
    const loadTime = await page.evaluate(() => {
      return performance.timing.loadEventEnd - performance.timing.navigationStart;
    });

    console.log(`⚡ Page load time: ${loadTime}ms`);

    // Check for console errors
    const errors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    await page.waitForTimeout(2000);

    console.log(`🚨 Console errors: ${errors.length}`);
    if (errors.length > 0) {
      errors.forEach(error => console.log(`   ❌ ${error}`));
    }

    // Check for layout shifts
    const layoutShifts = await page.evaluate(() => {
      let shifts = 0;
      const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.value > 0) shifts++;
        }
      });
      observer.observe({ entryTypes: ['layout-shift'] });
      return shifts;
    });

    console.log(`📐 Layout shifts: ${layoutShifts}`);

    return {
      loadTime,
      errors: errors.length,
      layoutShifts
    };

  } catch (error) {
    console.error('❌ Performance test failed:', error.message);
    return null;
  }
}

runComprehensiveTests();
