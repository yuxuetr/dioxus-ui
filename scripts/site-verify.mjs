#!/usr/bin/env node
import { readFileSync, readdirSync } from "node:fs";
import { expect } from "@playwright/test";
import { buildDocsCatalog } from "./docs-catalog-builder.mjs";
import {
  accessibilityViolations,
  launchBrowser,
  lowContrastText,
  serveDioxusWeb,
  setDarkTheme,
} from "./browser-check-support.mjs";

// Visits every component site route (RFC 0052) and fails on a console error,
// a route that renders the not found page, low text contrast or an axe-core
// violation (RFC 0054) in either theme, or a sideways scroll at 375px. Every
// theme preset (RFC 0057) gets the same contrast and axe-core checks on the
// pages in `presetPages`, chosen through the header theme menu.
const server = serveDioxusWeb({ packageName: "dioxus-ui-site", port: 45241 });
const viewport = { width: 1280, height: 900 };
const routes = [
  { path: "/", page: "home" },
  { path: "/docs/getting-started", page: "getting-started" },
  { path: "/docs/theming", page: "theming" },
  ...buildDocsCatalog().catalog.map((item) => ({ path: `/components/${item.slug}`, page: "component", slug: item.slug })),
];
const missingRoutes = ["/no-such-page", "/components/no-such-component"];
const presets = readdirSync(new URL("../crates/dioxus-shadcn-cli/themes/", import.meta.url))
  .filter((file) => file.endsWith(".css"))
  .map((file) => file.replace(/\.css$/, ""))
  .sort();
// Pages with the most token pairs: the variants, status colors, and muted text.
const presetPages = ["/", "/components/button", "/components/alert", "/components/badge", "/components/tabs"];
// The examples each page should show, from the site's example list.
const examplesSource = readFileSync(new URL("../site/src/examples/mod.rs", import.meta.url), "utf8");
const examples = [...examplesSource.matchAll(/^\s*(\w+) => "([a-z-]+)", "([^"]+)";$/gm)].map((match) => ({
  module: match[1],
  slug: match[2],
  title: match[3],
}));
const unknownExamples = examples.filter((example) => !routes.some((route) => route.slug === example.slug));
const componentsWithoutExamples = routes
  .filter((route) => route.slug && !examples.some((example) => example.slug === route.slug))
  .map((route) => route.slug);

async function visit(page, path) {
  await page.goto(`${server.url}${path}`, { waitUntil: "domcontentloaded", timeout: 30000 });
  const article = page.locator("main [data-site-page]");
  await article.waitFor({ state: "attached", timeout: 60000 });
  return article;
}

async function expectReadable(page, label, { audit = false } = {}) {
  for (const dark of [false, true]) {
    await setDarkTheme(page, dark);
    const theme = dark ? "dark" : "light";
    const failures = await page.evaluate(lowContrastText);
    if (failures.length > 0) {
      throw new Error(`${label} (${theme} theme): low contrast text: ${failures.slice(0, 5).join("; ")}`);
    }
    const violations = audit ? await accessibilityViolations(page) : [];
    if (violations.length > 0) {
      throw new Error(`${label} (${theme} theme): accessibility violations: ${violations.join("; ")}`);
    }
  }
  await setDarkTheme(page, false);
}

// Waits for the color transitions a theme change starts.
async function settle(page) {
  await page.evaluate(async () => {
    const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));
    await frame();
    const running = document.getAnimations().filter((animation) => animation instanceof CSSTransition);
    await Promise.all(running.map((animation) => animation.finished.catch(() => {})));
    await frame();
  });
}

async function expectNoSidewaysScroll(page, label) {
  await page.setViewportSize({ width: 375, height: 800 });
  const widths = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  await page.setViewportSize(viewport);
  if (widths[0] > widths[1]) {
    throw new Error(`${label} at 375px: page is ${widths[0]}px wide in a ${widths[1]}px viewport`);
  }
}

// Every listed example renders a preview, and its Code tab shows the source
// that defines the rendered `Demo`.
async function expectExamples(page, route) {
  const expected = examples.filter((example) => example.slug === route.slug).map((example) => example.title);
  const sections = page.locator("main [data-site-example]");
  const rendered = await sections.evaluateAll((elements) => elements.map((element) => element.dataset.siteExample));
  if (JSON.stringify(rendered) !== JSON.stringify(expected)) {
    throw new Error(`${route.path}: expected examples ${JSON.stringify(expected)}, got ${JSON.stringify(rendered)}`);
  }
  for (const title of expected) {
    const section = page.locator(`main [data-site-example="${title}"]`);
    // Scope to the card's own tabs; an example may render tabs of its own.
    const tabs = section.getByRole("tablist", { name: `${title} example`, exact: true });
    const preview = section.locator("[data-site-example-preview]");
    await expect(preview).toBeVisible();
    const drawn = await preview.evaluate((element) =>
      [...element.querySelectorAll("*")].some((child) => child.getBoundingClientRect().width > 0),
    );
    if (!drawn) {
      throw new Error(`${route.path}: the ${title} example renders nothing`);
    }
    await tabs.getByRole("tab", { name: "Code" }).click();
    const source = section.locator("[data-site-example-source]");
    await expect(source).toBeVisible();
    if (!(await source.textContent())?.includes("fn Demo()")) {
      throw new Error(`${route.path}: the ${title} example has no source`);
    }
    await expectReadable(page, `${route.path} ${title} source`);
    await tabs.getByRole("tab", { name: "Preview" }).click();
    await expect(preview).toBeVisible();
  }
}

async function run() {
  if (unknownExamples.length > 0) {
    throw new Error(`examples for components outside the catalog: ${unknownExamples.map((example) => example.module).join(", ")}`);
  }
  if (componentsWithoutExamples.length > 0) {
    throw new Error(`catalog components without examples: ${componentsWithoutExamples.join(", ")}`);
  }
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
      if (route.slug) {
        await expectExamples(page, route);
      }
      await expectReadable(page, route.path, { audit: true });
      await expectNoSidewaysScroll(page, route.path);
      if (errors.length > 0) {
        throw new Error(`${route.path}: console errors: ${errors.join("; ")}`);
      }
    }

    for (const path of missingRoutes) {
      const article = await visit(page, path);
      await expect(article).toHaveAttribute("data-site-page", "not-found");
    }

    // The theme menu applies a scheme or preset to the document root through
    // the ThemeController (RFC 0071), which remembers it across visits and
    // follows the system scheme while the theme is "system".
    await visit(page, "/");
    const themeMenu = page.getByRole("combobox", { name: "Theme", exact: true });
    const root = page.locator("html");
    await expect(themeMenu).toHaveValue("system");
    for (const [value, scheme] of [["dark", "dark"], ["light", "normal"]]) {
      await themeMenu.selectOption(value);
      await expect(root).toHaveCSS("color-scheme", scheme);
    }
    await themeMenu.selectOption("dark");
    // The page template applies the stored theme before the app loads.
    await page.goto(`${server.url}/`, { waitUntil: "commit" });
    await expect(root).toHaveClass(/\bdark\b/);
    await visit(page, "/");
    await expect(themeMenu).toHaveValue("dark");
    await themeMenu.selectOption("system");
    for (const colorScheme of ["dark", "light"]) {
      await page.emulateMedia({ colorScheme });
      await expect(root).toHaveCSS("color-scheme", colorScheme === "dark" ? "dark" : "normal");
    }

    // Each preset sets data-theme on the root, brings its own color scheme,
    // and stays readable and passes axe-core.
    for (const path of presetPages) {
      await visit(page, path);
      for (const preset of presets) {
        await themeMenu.selectOption(preset);
        await expect(root).toHaveAttribute("data-theme", preset);
        await expect(root).not.toHaveClass(/\bdark\b/);
        await settle(page);
        const label = `${path} (${preset} preset)`;
        const failures = await page.evaluate(lowContrastText);
        if (failures.length > 0) {
          throw new Error(`${label}: low contrast text: ${failures.slice(0, 5).join("; ")}`);
        }
        const violations = await accessibilityViolations(page);
        if (violations.length > 0) {
          throw new Error(`${label}: accessibility violations: ${violations.join("; ")}`);
        }
      }
      await themeMenu.selectOption("system");
      await expect(root).not.toHaveAttribute("data-theme");
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
  console.log(
    `site verification passed (${routes.length} routes, ${examples.length} examples, ${presets.length} presets on ${presetPages.length} pages)`,
  );
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
