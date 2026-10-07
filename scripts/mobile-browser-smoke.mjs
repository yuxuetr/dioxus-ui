#!/usr/bin/env node
import { existsSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { serveDioxusWeb } from "./browser-check-support.mjs";
import { chromium, devices } from "@playwright/test";

const repoRoot = new URL("..", import.meta.url).pathname;
const port = 45237;
const installHint = "npx playwright install chromium";
const executablePath = process.env.DIOXUS_UI_BROWSER_EXECUTABLE;
const mobileViewport = { width: 390, height: 844 };
const shouldCaptureScreenshot = ["1", "true", "yes"].includes(
  String(process.env.DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT ?? "").toLowerCase(),
);

const server = serveDioxusWeb({ packageName: "dioxus-ui-web-demo", bin: "preview", port });
const previewUrl = server.url;

function validateBrowserConfig() {
  if (!executablePath) {
    return;
  }

  if (!existsSync(executablePath)) {
    throw new Error(
      `DIOXUS_UI_BROWSER_EXECUTABLE does not exist: ${executablePath}`,
    );
  }
}

function browserLaunchOptions() {
  if (!executablePath) {
    return { headless: true };
  }

  return { headless: true, executablePath };
}

function screenshotPath() {
  const stamp = new Date().toISOString().replaceAll(":", "-").replaceAll(".", "-");
  return join(repoRoot, `dioxus-ui-mobile-browser-preview-${stamp}.png`);
}

function readPngMetadata(path) {
  const bytes = readFileSync(path);
  const stat = statSync(path);
  const signature = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

  if (stat.size <= 0) {
    throw new Error(`screenshot artifact is empty: ${path}`);
  }

  if (bytes.length < 24) {
    throw new Error(`screenshot artifact is too small to be a PNG: ${path}`);
  }

  for (let index = 0; index < signature.length; index += 1) {
    if (bytes[index] !== signature[index]) {
      throw new Error(`screenshot artifact is not a PNG: ${path}`);
    }
  }

  if (bytes.toString("ascii", 12, 16) !== "IHDR") {
    throw new Error(`screenshot artifact is missing PNG IHDR metadata: ${path}`);
  }

  const width = bytes.readUInt32BE(16);
  const height = bytes.readUInt32BE(20);

  if (width < mobileViewport.width || height < mobileViewport.height) {
    throw new Error(
      `screenshot artifact is below mobile viewport size: ${width}x${height}, expected at least ${mobileViewport.width}x${mobileViewport.height}`,
    );
  }

  return { width, height, bytes: stat.size };
}

function isMissingBrowserError(error) {
  return String(error?.message ?? error).includes("Executable doesn't exist");
}

async function runBrowserAssertions() {
  let browser;

  try {
    browser = await chromium.launch(browserLaunchOptions());
  } catch (error) {
    if (!executablePath && isMissingBrowserError(error)) {
      throw new Error(
        `Playwright Chromium is not installed. Run \`${installHint}\` before npm run verify:mobile-browser.`,
      );
    }

    throw error;
  }

  try {
    const context = await browser.newContext({
      ...devices["iPhone 12"],
      viewport: mobileViewport,
    });
    const page = await context.newPage();

    await page.goto(previewUrl, { waitUntil: "networkidle", timeout: 30000 });

    const result = await page.evaluate(() => {
      const count = (selector) => document.querySelectorAll(selector).length;
      const chart = document.querySelector('[data-preview-panel="chart"] svg');
      const chartRect = chart?.getBoundingClientRect();

      return {
        title: document.title,
        bodyTextLength: document.body.innerText.length,
        root: count('[data-preview-root="web"]'),
        mobileProfile: count('[data-preview-panel="mobile-profile"]'),
        touchTargets: count('[data-mobile-profile="touch-targets"]'),
        hoverAlternative: count('[data-mobile-profile="hover-alternative"]'),
        safeAreaOwned: count('[data-mobile-profile="safe-area-owned"]'),
        reducedMotion: count('[data-mobile-profile="reduced-motion"]'),
        visibleStatus: count('[data-mobile-profile="visible-status"]'),
        formInputs: count('[data-preview-panel="form"] input'),
        message: count('[data-preview-panel="message"]'),
        chartSvg: count('[data-preview-panel="chart"] svg'),
        chartRows: count('[data-preview-panel="chart"] tbody tr'),
        overlayDialog: count('[data-preview-panel="overlay-open"] [role="dialog"]'),
        chartBox: chartRect
          ? { width: Math.round(chartRect.width), height: Math.round(chartRect.height) }
          : null,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight,
          dpr: window.devicePixelRatio,
        },
      };
    });

    const failures = [];

    if (result.title !== "dioxus-shadcn preview") failures.push("page title");
    if (result.bodyTextLength <= 0) failures.push("nonblank body text");
    if (result.root !== 1) failures.push("web preview root");
    if (result.mobileProfile !== 1) failures.push("mobile-profile panel");
    if (result.touchTargets !== 1) failures.push("touch-targets marker");
    if (result.hoverAlternative !== 1) failures.push("hover-alternative marker");
    if (result.safeAreaOwned !== 1) failures.push("safe-area-owned marker");
    if (result.reducedMotion !== 1) failures.push("reduced-motion marker");
    if (result.visibleStatus !== 1) failures.push("visible-status marker");
    if (result.formInputs < 1) failures.push("form input");
    if (result.message !== 1) failures.push("message panel");
    if (result.chartSvg !== 1) failures.push("chart svg");
    if (result.chartRows < 1) failures.push("chart fallback rows");
    if (result.overlayDialog !== 1) failures.push("overlay dialog");
    if (!result.chartBox || result.chartBox.width <= 0 || result.chartBox.height <= 0) {
      failures.push("non-empty chart bounding box");
    }

    if (failures.length > 0) {
      throw new Error(
        `mobile browser smoke failed: ${failures.join(", ")}\n${JSON.stringify(result, null, 2)}`,
      );
    }

    if (shouldCaptureScreenshot) {
      const path = screenshotPath();
      await page.screenshot({ path, fullPage: true });
      const metadata = readPngMetadata(path);
      console.log(`mobile browser screenshot saved: ${path}`);
      console.log(
        `mobile browser screenshot metadata: ${metadata.width}x${metadata.height}, ${metadata.bytes} bytes`,
      );
    }
  } finally {
    await browser.close();
  }
}

try {
  validateBrowserConfig();
  await server.ready();
  await runBrowserAssertions();
  console.log("mobile browser smoke passed");
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
