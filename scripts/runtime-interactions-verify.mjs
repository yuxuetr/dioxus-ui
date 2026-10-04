#!/usr/bin/env node
import { request } from "node:http";
import { once } from "node:events";
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { chromium, expect } from "@playwright/test";

const repoRoot = new URL("..", import.meta.url).pathname;
const host = "127.0.0.1";
const port = 45239;
const previewUrl = `http://${host}:${port}`;
const installHint = "npx playwright install chromium";
const executablePath = process.env.DIOXUS_UI_BROWSER_EXECUTABLE;
const viewport = { width: 1280, height: 900 };

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

async function expectFocused(page, selector, label) {
  const focused = await page.evaluate((targetSelector) => {
    return document.activeElement?.matches(targetSelector) ?? false;
  }, selector);

  if (!focused) {
    throw new Error(`${label}: expected ${selector} to be focused`);
  }
}

async function runBrowserAssertions() {
  let browser;

  try {
    browser = await chromium.launch(browserLaunchOptions());
  } catch (error) {
    if (!executablePath && isMissingBrowserError(error)) {
      throw new Error(
        `Playwright Chromium is not installed. Run \`${installHint}\` before node scripts/runtime-interactions-verify.mjs.`,
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

    const root = page.locator('[data-interaction-root="runtime"]');
    await expect(root).toHaveCount(1);

    const disclosure = page.locator('[data-interaction-target="disclosure"]');
    const disclosureTrigger = page.locator('[data-interaction-control="disclosure-trigger"]');
    const disclosureContent = page.locator('[data-interaction-state="disclosure-content"]');
    await expect(disclosure).toHaveAttribute("data-state", "closed");
    await expect(disclosureTrigger).toHaveAttribute("aria-expanded", "false");
    await expect(disclosureContent).toBeHidden();
    await disclosureTrigger.click();
    await expect(disclosure).toHaveAttribute("data-state", "open");
    await expect(disclosureTrigger).toHaveAttribute("aria-expanded", "true");
    await expect(disclosureContent).toBeVisible();
    await disclosureTrigger.click();
    await expect(disclosure).toHaveAttribute("data-state", "closed");
    await expect(disclosureContent).toBeHidden();

    const overlay = page.locator('[data-interaction-target="overlay"]');
    const overlayTrigger = page.locator('[data-interaction-control="overlay-trigger"]');
    const overlayContent = page.locator('[data-interaction-state="overlay-content"]');
    const overlayClose = page.locator('[data-interaction-control="overlay-close"]');
    await expect(overlay).toHaveAttribute("data-state", "closed");
    await expect(overlayTrigger).toHaveAttribute("aria-expanded", "false");
    await expect(overlayContent).toBeHidden();
    await overlayTrigger.click();
    await expect(overlay).toHaveAttribute("data-state", "open");
    await expect(overlayTrigger).toHaveAttribute("aria-expanded", "true");
    await expect(overlayContent).toBeVisible();
    await expect(overlayContent).toHaveAttribute("role", "dialog");
    await overlayClose.click();
    await expect(overlay).toHaveAttribute("data-state", "closed");
    await expect(overlayContent).toBeHidden();

    const selection = page.locator('[data-interaction-target="selection"]');
    const selectedAlpha = page.locator(
      '[data-interaction-target="selection"] [data-interaction-option="alpha"]',
    );
    const selectedBeta = page.locator(
      '[data-interaction-target="selection"] [data-interaction-option="beta"]',
    );
    await expect(selection).toHaveAttribute("data-state", "alpha");
    await expect(selectedAlpha).toHaveAttribute("aria-checked", "true");
    await expect(selectedBeta).toHaveAttribute("aria-checked", "false");
    await selectedBeta.focus();
    await expectFocused(
      page,
      '[data-interaction-target="selection"] [data-interaction-option="beta"]',
      "selection keyboard activation",
    );
    await page.keyboard.press("Enter");
    await expect(selection).toHaveAttribute("data-state", "beta");
    await expect(selectedAlpha).toHaveAttribute("aria-checked", "false");
    await expect(selectedBeta).toHaveAttribute("aria-checked", "true");

    const keyboardListbox = page.locator('[data-interaction-control="keyboard-listbox"]');
    const openFileOption = page.locator(
      '[data-interaction-target="keyboard"] [data-interaction-option="open-file"]',
    );
    const saveFileOption = page.locator(
      '[data-interaction-target="keyboard"] [data-interaction-option="save-file"]',
    );
    await expect(keyboardListbox).toHaveAttribute(
      "aria-activedescendant",
      "interaction-command-open-file",
    );
    await expect(openFileOption).toHaveAttribute("aria-selected", "true");
    await expect(saveFileOption).toHaveAttribute("aria-selected", "false");
    await saveFileOption.focus();
    await expectFocused(
      page,
      '[data-interaction-target="keyboard"] [data-interaction-option="save-file"]',
      "keyboard-visible state activation",
    );
    await page.keyboard.press("Enter");
    await expect(keyboardListbox).toHaveAttribute(
      "aria-activedescendant",
      "interaction-command-save-file",
    );
    await expect(openFileOption).toHaveAttribute("aria-selected", "false");
    await expect(saveFileOption).toHaveAttribute("aria-selected", "true");

    const scrollStatus = page.locator('[data-interaction-target="scroll-status"]');
    const scrollStatusText = page.locator('[data-interaction-state="scroll-status"]');
    const scrollJump = page.locator('[data-interaction-control="scroll-jump"]');
    await expect(scrollStatus).toHaveAttribute("data-state", "held");
    await expect(scrollStatusText).toContainText("held");
    await scrollJump.click();
    await expect(scrollStatus).toHaveAttribute("data-state", "jumped");
    await expect(scrollStatusText).toContainText("jumped");

    const popover = page.locator('[data-interaction-target="popover"]');
    const popoverTrigger = page.locator('[data-interaction-control="popover-trigger"]');
    const popoverContent = popover.locator('[role="dialog"]');
    const boxes = async () => ({
      trigger: await popoverTrigger.boundingBox(),
      content: await popoverContent.boundingBox(),
    });
    const expectInViewport = (box, label) => {
      if (box.x < 0 || box.y < 0 || box.x + box.width > viewport.width || box.y + box.height > viewport.height) {
        throw new Error(`${label}: popover content ${JSON.stringify(box)} is outside the viewport`);
      }
    };
    await expect(popoverContent).toBeHidden();
    await popoverTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await popoverTrigger.click();
    await expect(popoverContent).toHaveAttribute("data-side", "bottom");
    let placed = await boxes();
    if (placed.content.y < placed.trigger.y + placed.trigger.height) {
      throw new Error(`popover should sit below its trigger: ${JSON.stringify(placed)}`);
    }
    expectInViewport(placed.content, "bottom placement");
    await page.keyboard.press("Escape");
    await expect(popoverContent).toBeHidden();
    await popoverTrigger.click();
    await expect(popoverContent).toBeVisible();
    await page.getByRole("heading", { name: "Disclosure interaction" }).click();
    await expect(popoverContent).toBeHidden();
    await popoverTrigger.evaluate((element) => element.scrollIntoView({ block: "end" }));
    await popoverTrigger.click();
    await expect(popoverContent).toHaveAttribute("data-side", "top");
    placed = await boxes();
    if (placed.content.y + placed.content.height > placed.trigger.y) {
      throw new Error(`popover should flip above its trigger: ${JSON.stringify(placed)}`);
    }
    expectInViewport(placed.content, "flipped placement");
    await popoverTrigger.click();
    await expect(popoverContent).toBeHidden();

    const alertDialog = page.locator('[data-interaction-target="alert-dialog"]');
    const alertDialogTrigger = page.locator('[data-interaction-control="alert-dialog-trigger"]');
    const alertDialogContent = alertDialog.locator('[role="alertdialog"]');
    const alertDialogCancel = alertDialogContent.getByRole("button", { name: "Cancel" });
    const alertDialogAction = alertDialogContent.getByRole("button", { name: "Delete" });
    await expect(alertDialogContent).toBeHidden();
    await alertDialogTrigger.click();
    await expect(alertDialogCancel).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(alertDialogAction).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(alertDialogCancel).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(alertDialogContent).toBeHidden();
    await expect(alertDialogTrigger).toBeFocused();
    await expect(alertDialog).toHaveAttribute("data-result", "pending");
    await alertDialogTrigger.click();
    await expect(alertDialogCancel).toBeFocused();
    await alertDialogAction.click();
    await expect(alertDialogContent).toBeHidden();
    await expect(alertDialog).toHaveAttribute("data-result", "confirmed");
    await expect(alertDialogTrigger).toBeFocused();

    const dialog = page.locator('[data-interaction-target="dialog"]');
    const dialogTrigger = page.locator('[data-interaction-control="dialog-trigger"]');
    const dialogContent = page.locator('[data-interaction-target="dialog"] [role="dialog"]');
    const dialogInput = page.locator('[data-interaction-control="dialog-input"]');
    const dialogClose = dialogContent.getByRole("button", { name: "Cancel" });
    // The preview serves uncompiled Tailwind input, so the overlay has no
    // `fixed inset-0` box to hit; dispatch the click on the element instead.
    const dialogOverlay = page.locator('[data-interaction-target="dialog"] > [data-state]:not([role])');
    await expect(dialog).toHaveAttribute("data-state", "closed");
    await expect(dialogContent).toBeHidden();
    await dialogTrigger.click();
    await expect(dialogContent).toBeVisible();
    await expect(dialogInput).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(dialogClose).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(dialogInput).toBeFocused();
    await page.keyboard.press("Shift+Tab");
    await expect(dialogClose).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(dialog).toHaveAttribute("data-state", "closed");
    await expect(dialogContent).toBeHidden();
    await expect(dialogTrigger).toBeFocused();
    await dialogTrigger.click();
    await expect(dialogInput).toBeFocused();
    await dialogOverlay.dispatchEvent("click");
    await expect(dialogContent).toBeHidden();
    await expect(dialogTrigger).toBeFocused();
    await dialogTrigger.click();
    await expect(dialogContent).toBeVisible();
    await dialogClose.click();
    await expect(dialogContent).toBeHidden();
    await expect(dialogTrigger).toBeFocused();
  } finally {
    await browser.close();
  }
}

try {
  validateBrowserConfig();
  startServer();
  await waitForPreview();
  await runBrowserAssertions();
  console.log("runtime interaction verification passed (8 fixtures)");
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await stopServer();
}
