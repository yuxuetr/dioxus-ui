#!/usr/bin/env node
import { existsSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { serveDioxusWeb } from "./browser-check-support.mjs";
import { chromium, devices } from "@playwright/test";

const repoRoot = new URL("..", import.meta.url).pathname;
const port = 45240;
const installHint = "npx playwright install chromium";
const executablePath = process.env.DIOXUS_UI_BROWSER_EXECUTABLE;
const desktopViewport = { width: 1280, height: 900 };
const mobileViewport = { width: 390, height: 844 };
const shouldCaptureScreenshot = ["1", "true", "yes"].includes(
  String(process.env.DIOXUS_UI_WEB_SCREENSHOT ?? "").toLowerCase(),
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

function screenshotPath(name) {
  const stamp = new Date().toISOString().replaceAll(":", "-").replaceAll(".", "-");
  return join(repoRoot, `dioxus-ui-web-preview-${name}-${stamp}.png`);
}

function readPngMetadata(path, viewport) {
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

  if (width < viewport.width || height < viewport.height) {
    throw new Error(
      `screenshot artifact is below viewport size: ${width}x${height}, expected at least ${viewport.width}x${viewport.height}`,
    );
  }

  return { width, height, bytes: stat.size };
}

function isMissingBrowserError(error) {
  return String(error?.message ?? error).includes("Executable doesn't exist");
}

async function verifyViewport(browser, profile) {
  const contextOptions =
    profile.name === "mobile"
      ? { ...devices["iPhone 12"], viewport: profile.viewport }
      : { viewport: profile.viewport };
  const context = await browser.newContext(contextOptions);
  const page = await context.newPage();

  try {
    await page.goto(previewUrl, { waitUntil: "networkidle", timeout: 30000 });

    const result = await page.evaluate(() => {
      const count = (selector) => document.querySelectorAll(selector).length;
      const chart = document.querySelector('[data-preview-panel="chart"] svg');
      const chartRect = chart?.getBoundingClientRect();
      const conversation = document.querySelector('[data-preview-panel="message"] [role="region"]');

      return {
        title: document.title,
        bodyTextLength: document.body.innerText.length,
        root: count('[data-preview-root="web"]'),
        form: count('[data-preview-panel="form"]'),
        formInputs: count('[data-preview-panel="form"] input'),
        message: count('[data-preview-panel="message"]'),
        chart: count('[data-preview-panel="chart"]'),
        chartSvg: count('[data-preview-panel="chart"] svg'),
        chartRows: count('[data-preview-panel="chart"] tbody tr'),
        overlayDialog: count('[data-preview-panel="overlay-open"] [role="dialog"]'),
        interactions: count('[data-preview-panel="interactions"]'),
        interactionRoot: count('[data-preview-panel="interactions"][data-interaction-root="runtime"]'),
        conversationOverflow: conversation ? getComputedStyle(conversation).overflowY : null,
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
    if (result.form !== 1) failures.push("form panel");
    if (result.formInputs < 1) failures.push("form input");
    if (result.message !== 1) failures.push("message panel");
    if (result.chart !== 1) failures.push("chart panel");
    if (result.chartSvg !== 1) failures.push("chart svg");
    if (result.chartRows < 1) failures.push("chart fallback rows");
    if (result.overlayDialog !== 1) failures.push("overlay dialog");
    if (result.interactions !== 1) failures.push("interactions panel");
    if (result.interactionRoot !== 1) failures.push("runtime interaction root");
    // The conversation passes `overflow-auto` over the scroller's own
    // `overflow-hidden`, which wins by stylesheet order unless the user class
    // replaces it (RFC 0076).
    if (result.conversationOverflow !== "auto") failures.push(`conversation overflow ${result.conversationOverflow}`);
    if (result.viewport.width !== profile.viewport.width) {
      failures.push(`viewport width ${result.viewport.width}`);
    }
    if (result.viewport.height !== profile.viewport.height) {
      failures.push(`viewport height ${result.viewport.height}`);
    }
    if (!result.chartBox || result.chartBox.width <= 0 || result.chartBox.height <= 0) {
      failures.push("non-empty chart bounding box");
    }

    if (failures.length > 0) {
      throw new Error(
        `web screenshot smoke failed for ${profile.name}: ${failures.join(", ")}\n${JSON.stringify(result, null, 2)}`,
      );
    }

    if (shouldCaptureScreenshot) {
      const path = screenshotPath(profile.name);
      await page.screenshot({ path, fullPage: true });
      const metadata = readPngMetadata(path, profile.viewport);
      console.log(`web screenshot saved: ${path}`);
      console.log(
        `web screenshot metadata: ${profile.name} ${metadata.width}x${metadata.height}, ${metadata.bytes} bytes`,
      );
    }
  } finally {
    await context.close();
  }
}

async function runBrowserAssertions() {
  let browser;

  try {
    browser = await chromium.launch(browserLaunchOptions());
  } catch (error) {
    if (!executablePath && isMissingBrowserError(error)) {
      throw new Error(
        `Playwright Chromium is not installed. Run \`${installHint}\` before node scripts/web-screenshot-smoke.mjs.`,
      );
    }

    throw error;
  }

  try {
    await verifyViewport(browser, { name: "desktop", viewport: desktopViewport });
    await verifyViewport(browser, { name: "mobile", viewport: mobileViewport });
  } finally {
    await browser.close();
  }
}

try {
  validateBrowserConfig();
  await server.ready();
  await runBrowserAssertions();
  console.log("web screenshot smoke passed (2 viewports)");
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
