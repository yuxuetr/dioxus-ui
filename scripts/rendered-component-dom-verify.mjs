#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { serveDioxusWeb } from "./browser-check-support.mjs";
import { chromium } from "@playwright/test";

const repoRoot = new URL("..", import.meta.url).pathname;
const port = 45238;
const installHint = "npx playwright install chromium";
const executablePath = process.env.DIOXUS_UI_BROWSER_EXECUTABLE;
const viewport = { width: 1280, height: 900 };
const manifest = JSON.parse(
  readFileSync(join(repoRoot, "docs/components/rendered-coverage.json"), "utf8"),
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

function isMissingBrowserError(error) {
  return String(error?.message ?? error).includes("Executable doesn't exist");
}

function validateManifestRecords() {
  if (manifest.schema_version !== 1 || !Array.isArray(manifest.records)) {
    throw new Error("rendered coverage manifest must have schema_version 1 and records array");
  }
}

async function runBrowserAssertions() {
  let browser;

  try {
    browser = await chromium.launch(browserLaunchOptions());
  } catch (error) {
    if (!executablePath && isMissingBrowserError(error)) {
      throw new Error(
        `Playwright Chromium is not installed. Run \`${installHint}\` before npm run verify:rendered-component-dom.`,
      );
    }

    throw error;
  }

  try {
    const context = await browser.newContext({ viewport });
    const page = await context.newPage();

    await page.goto(previewUrl, { waitUntil: "domcontentloaded", timeout: 30000 });
    await page.waitForSelector('[data-preview-root="web"]', {
      state: "attached",
      timeout: 30000,
    });

    const result = await page.evaluate((records) => {
      const failures = [];
      const summary = [];

      for (const record of records) {
        const selector = `[data-component-preview="${record.test_id}"]`;
        const nodes = Array.from(document.querySelectorAll(selector));

        if (nodes.length !== 1) {
          failures.push(`${record.component}: expected one target for ${record.test_id}, found ${nodes.length}`);
          continue;
        }

        const node = nodes[0];
        const style = window.getComputedStyle(node);
        const rect = node.getBoundingClientRect();
        const visible =
          node.isConnected &&
          style.display !== "none" &&
          style.visibility !== "hidden" &&
          Number.parseFloat(style.opacity || "1") > 0 &&
          rect.width > 0 &&
          rect.height > 0;
        const text = (node.innerText || node.textContent || "").trim();
        const component = node.getAttribute("data-component");
        const panel = node.getAttribute("data-component-panel");
        const coverage = node.getAttribute("data-component-coverage");

        if (!node.isConnected) failures.push(`${record.component}: target is detached`);
        if (!visible) failures.push(`${record.component}: target is hidden or has empty layout box`);
        if (text.length === 0) failures.push(`${record.component}: target has no visible text`);
        if (component !== record.component) {
          failures.push(`${record.component}: data-component is ${component}`);
        }
        if (panel !== record.panel) {
          failures.push(`${record.component}: data-component-panel is ${panel}`);
        }
        if (coverage !== record.coverage_level) {
          failures.push(`${record.component}: data-component-coverage is ${coverage}`);
        }

        summary.push({
          component: record.component,
          test_id: record.test_id,
          width: Math.round(rect.width),
          height: Math.round(rect.height),
          text_length: text.length,
        });
      }

      return {
        title: document.title,
        rootCount: document.querySelectorAll('[data-preview-root="web"]').length,
        bodyTextLength: document.body.innerText.length,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight,
          dpr: window.devicePixelRatio,
        },
        checked: summary.length,
        failures,
      };
    }, manifest.records);

    const failures = [...result.failures];

    if (result.title !== "dioxus-shadcn preview") failures.push("page title");
    if (result.rootCount !== 1) failures.push("web preview root");
    if (result.bodyTextLength <= 0) failures.push("nonblank body text");
    if (result.checked !== manifest.records.length) {
      failures.push(`checked ${result.checked} of ${manifest.records.length} records`);
    }

    if (failures.length > 0) {
      throw new Error(
        `rendered component DOM verification failed:\n- ${failures.join("\n- ")}\n${JSON.stringify(result, null, 2)}`,
      );
    }
  } finally {
    await browser.close();
  }
}

try {
  validateManifestRecords();
  validateBrowserConfig();
  await server.ready();
  await runBrowserAssertions();
  console.log(`rendered component DOM verification passed (${manifest.records.length} components)`);
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
