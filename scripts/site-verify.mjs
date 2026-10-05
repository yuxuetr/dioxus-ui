#!/usr/bin/env node
import { expect } from "@playwright/test";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";
import { launchBrowser, lowContrastText, serveDioxusWeb, setDarkTheme } from "./browser-check-support.mjs";

// Visits every component site route (RFC 0052) and fails on a console error,
// a route that renders the not found page, low text contrast in either
// theme, or a sideways scroll at 375px.
const server = serveDioxusWeb({ packageName: "dioxus-ui-site", port: 45241 });
const viewport = { width: 1280, height: 900 };
const routes = [
  { path: "/", page: "home" },
  { path: "/docs/getting-started", page: "getting-started" },
  { path: "/docs/theming", page: "theming" },
  ...buildDocsCatalog().catalog.map((item) => ({ path: `/components/${item.slug}`, page: "component", slug: item.slug })),
];
const missingRoutes = ["/no-such-page", "/components/no-such-component"];

async function visit(page, path) {
  await page.goto(`${server.url}${path}`, { waitUntil: "domcontentloaded", timeout: 30000 });
  const article = page.locator("main [data-site-page]");
  await article.waitFor({ state: "attached", timeout: 60000 });
  return article;
}

async function expectReadable(page, label) {
  for (const dark of [false, true]) {
    await setDarkTheme(page, dark);
    const failures = await page.evaluate(lowContrastText);
    if (failures.length > 0) {
      throw new Error(`${label} (${dark ? "dark" : "light"} theme): low contrast text: ${failures.slice(0, 5).join("; ")}`);
    }
  }
  await setDarkTheme(page, false);
}

async function expectNoSidewaysScroll(page, label) {
  await page.setViewportSize({ width: 375, height: 800 });
  const widths = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  await page.setViewportSize(viewport);
  if (widths[0] > widths[1]) {
    throw new Error(`${label} at 375px: page is ${widths[0]}px wide in a ${widths[1]}px viewport`);
  }
}

async function run() {
  const browser = await launchBrowser("scripts/site-verify.mjs");
  try {
    const page = await (await browser.newContext({ viewport })).newPage();
    const errors = [];
    page.on("console", (message) => {
      if (message.type() === "error") {
        errors.push(message.text());
      }
    });
    page.on("pageerror", (error) => errors.push(String(error)));

    for (const route of routes) {
      const article = await visit(page, route.path);
      const rendered = await article.getAttribute("data-site-page");
      if (rendered !== route.page) {
        throw new Error(`${route.path}: expected the ${route.page} page, got ${rendered}`);
      }
      if (route.slug && (await article.getAttribute("data-component")) !== route.slug) {
        throw new Error(`${route.path}: the page does not show ${route.slug}`);
      }
      await expectReadable(page, route.path);
      await expectNoSidewaysScroll(page, route.path);
      if (errors.length > 0) {
        throw new Error(`${route.path}: console errors: ${errors.join("; ")}`);
      }
    }

    for (const path of missingRoutes) {
      const article = await visit(page, path);
      await expect(article).toHaveAttribute("data-site-page", "not-found");
    }

    // The header toggle turns the whole site dark and back.
    await visit(page, "/");
    const toggle = page.locator("#site-theme-toggle");
    const root = page.locator("[data-site-root]");
    for (const dark of [true, false]) {
      await toggle.click();
      await expect(toggle).toHaveAttribute("aria-pressed", String(dark));
      await expect(root).toHaveCSS("color-scheme", dark ? "dark" : "normal");
    }

    // Below md the catalog opens from the header menu.
    await page.setViewportSize({ width: 375, height: 800 });
    await page.getByRole("button", { name: "Open the component menu" }).click();
    const menu = page.getByRole("dialog");
    await expect(menu).toBeVisible();
    await menu.getByRole("link", { name: "Dialog", exact: true }).click();
    await expect(menu).toBeHidden();
    await expect(page.locator("main [data-site-page]")).toHaveAttribute("data-component", "dialog");
    if (errors.length > 0) {
      throw new Error(`console errors: ${errors.join("; ")}`);
    }
  } finally {
    await browser.close();
  }
}

try {
  await server.ready();
  await run();
  console.log(`site verification passed (${routes.length} routes)`);
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
