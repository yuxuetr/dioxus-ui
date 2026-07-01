#!/usr/bin/env node
import { request } from "node:http";
import { once } from "node:events";
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { chromium, devices } from "@playwright/test";

const repoRoot = new URL("..", import.meta.url).pathname;
const host = "127.0.0.1";
const port = 45237;
const previewUrl = `http://${host}:${port}`;
const installHint = "npx playwright install chromium";
const executablePath = process.env.DIOXUS_UI_BROWSER_EXECUTABLE;
const shouldCaptureScreenshot = ["1", "true", "yes"].includes(
  String(process.env.DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT ?? "").toLowerCase(),
);

let server;
let serverOutput = "";

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

function startServer() {
  server = spawn(
    "dx",
    [
      "serve",
      "--web",
      "--package",
      "dioxus-ui-web-demo",
      "--bin",
      "preview",
      "--port",
      String(port),
      "--addr",
      host,
      "--open",
      "false",
      "--hot-reload",
      "false",
      "--watch",
      "false",
      "--interactive",
      "false",
    ],
    {
      cwd: repoRoot,
      stdio: ["ignore", "pipe", "pipe"],
    },
  );

  server.stdout.on("data", (chunk) => {
    serverOutput += chunk.toString();
  });

  server.stderr.on("data", (chunk) => {
    serverOutput += chunk.toString();
  });
}

async function stopServer() {
  if (!server) {
    return;
  }

  if (server.exitCode !== null || server.signalCode !== null) {
    return;
  }

  server.kill("SIGINT");

  const timeout = setTimeout(() => {
    if (server.exitCode === null && server.signalCode === null) {
      server.kill("SIGTERM");
    }
  }, 3000);

  try {
    await once(server, "exit");
  } finally {
    clearTimeout(timeout);
  }
}

function requestPreview() {
  return new Promise((resolve) => {
    const req = request(previewUrl, { method: "GET", timeout: 1000 }, (res) => {
      res.resume();
      resolve(res.statusCode === 200);
    });

    req.on("timeout", () => {
      req.destroy();
      resolve(false);
    });

    req.on("error", () => {
      resolve(false);
    });

    req.end();
  });
}

async function waitForPreview() {
  const startedAt = Date.now();
  const timeoutMs = 120000;

  while (Date.now() - startedAt < timeoutMs) {
    if (server.exitCode !== null || server.signalCode !== null) {
      throw new Error(`dx serve exited before preview became ready.\n${serverOutput}`);
    }

    if (await requestPreview()) {
      return;
    }

    await new Promise((resolve) => setTimeout(resolve, 1000));
  }

  throw new Error(`Timed out waiting for ${previewUrl}.\n${serverOutput}`);
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
      viewport: { width: 390, height: 844 },
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

    if (result.title !== "dioxus-ui preview") failures.push("page title");
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
      console.log(`mobile browser screenshot saved: ${path}`);
    }
  } finally {
    await browser.close();
  }
}

try {
  validateBrowserConfig();
  startServer();
  await waitForPreview();
  await runBrowserAssertions();
  console.log("mobile browser smoke passed");
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await stopServer();
}
