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
// Blocks (RFC 0073), one route each.
const blockNames = readdirSync(new URL("../crates/dioxus-shadcn-cli/blocks/", import.meta.url))
  .filter((file) => file.endsWith(".json"))
  .map((file) => file.replace(/\.json$/, ""))
  .sort();
const routes = [
  { path: "/", page: "home" },
  { path: "/docs/getting-started", page: "getting-started" },
  { path: "/docs/theming", page: "theming" },
  ...buildDocsCatalog().catalog.map((item) => ({ path: `/components/${item.slug}`, page: "component", slug: item.slug })),
  { path: "/blocks", page: "blocks" },
  ...blockNames.map((name) => ({ path: `/blocks/${name}`, page: "block", block: name })),
];
const missingRoutes = ["/no-such-page", "/components/no-such-component"];
const presets = readdirSync(new URL("../crates/dioxus-shadcn-cli/themes/", import.meta.url))
  .filter((file) => file.endsWith(".css"))
  .map((file) => file.replace(/\.css$/, ""))
  .sort();
// Pages with the most token pairs: the variants, status colors, and muted text.
const presetPages = ["/", "/components/button", "/components/alert", "/components/badge", "/components/tabs", "/blocks/dashboard"];
// The examples each page should show, from the site's example list.
const examplesSource = readFileSync(new URL("../site/src/examples/mod.rs", import.meta.url), "utf8");
const examples = [...examplesSource.matchAll(/^\s*(\w+) => (\w+), "([a-z-]+)", "([^"]+)";$/gm)].map((match) => ({
  module: match[1],
  demo: match[2],
  slug: match[3],
  title: match[4],
}));
const unknownExamples = examples.filter((example) => !routes.some((route) => route.slug === example.slug));
const componentsWithoutExamples = routes
  .filter((route) => route.slug && !examples.some((example) => example.slug === route.slug))
  .map((route) => route.slug);

async function visit(page, path) {
  await page.goto(`${server.url}${path}`, { waitUntil: "domcontentloaded", timeout: 30000 });
  const article = page.locator("main [data-site-page]");
  await article.waitFor({ state: "attached", timeout: 60000 });
  // From `site/Dioxus.toml`, not `document::Title`, which runs through eval.
  await expect(page).toHaveTitle("dioxus-shadcn");
  return article;
}

// A block is a whole app screen with its own main and sidebar landmarks; on
// the site it sits inside the page's main, which only the preview causes.
const blockLandmarkRules = [
  "landmark-main-is-top-level",
  "landmark-no-duplicate-main",
  "landmark-unique",
  "landmark-complementary-is-top-level",
];

async function expectReadable(page, label, { audit = false, disabledRules = [] } = {}) {
  for (const dark of [false, true]) {
    await setDarkTheme(page, dark);
    const theme = dark ? "dark" : "light";
    const failures = await page.evaluate(lowContrastText);
    if (failures.length > 0) {
      throw new Error(`${label} (${theme} theme): low contrast text: ${failures.slice(0, 5).join("; ")}`);
    }
    const violations = audit ? await accessibilityViolations(page, { disabledRules }) : [];
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
// that defines the rendered demo component.
async function expectExamples(page, route) {
  const listed = examples.filter((example) => example.slug === route.slug);
  const expected = listed.map((example) => example.title);
  const sections = page.locator("main [data-site-example]");
  const rendered = await sections.evaluateAll((elements) => elements.map((element) => element.dataset.siteExample));
  if (JSON.stringify(rendered) !== JSON.stringify(expected)) {
    throw new Error(`${route.path}: expected examples ${JSON.stringify(expected)}, got ${JSON.stringify(rendered)}`);
  }
  for (const { title, demo } of listed) {
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
    if (!(await source.textContent())?.includes(`fn ${demo}()`)) {
      throw new Error(`${route.path}: the ${title} example has no source`);
    }
    await expectReadable(page, `${route.path} ${title} source`);
    await tabs.getByRole("tab", { name: "Preview" }).click();
    await expect(preview).toBeVisible();
  }
}

// A block page shows the block in a preview frame, its copied source on the
// Code tab, and its docs page.
async function expectBlock(page, route) {
  const preview = page.locator("main [data-site-block-preview]");
  await expect(preview).toBeVisible();
  const drawn = await preview.evaluate((element) =>
    [...element.querySelectorAll("*")].some((child) => child.getBoundingClientRect().width > 0),
  );
  if (!drawn) {
    throw new Error(`${route.path}: the block renders nothing`);
  }
  const component = route.block.split("-").map((part) => part[0].toUpperCase() + part.slice(1)).join("");
  const tabs = page.getByRole("tablist", { name: /block$/ });
  await tabs.getByRole("tab", { name: "Code" }).click();
  await expect(page.locator("main [data-site-block-source]")).toContainText(`pub fn ${component}Block(`);
  await tabs.getByRole("tab", { name: "Preview" }).click();
  await expect(page.locator("main [data-site-reference]").getByRole("heading", { name: "Behavior" })).toHaveCount(1);
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
    const page = await (
      await browser.newContext({ viewport, permissions: ["clipboard-read", "clipboard-write"] })
    ).newPage();
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
        // The docs page renders on the site, from its API Surface on.
        const reference = article.locator("[data-site-reference]");
        for (const heading of ["API Surface", "Accessibility Notes"]) {
          await expect(reference.getByRole("heading", { name: heading, exact: true })).toHaveCount(1);
        }
      }
      if (route.block) {
        await expectBlock(page, route);
      }
      await expectReadable(page, route.path, {
        audit: true,
        disabledRules: route.block ? blockLandmarkRules : [],
      });
      await expectNoSidewaysScroll(page, route.path);
      if (errors.length > 0) {
        throw new Error(`${route.path}: console errors: ${errors.join("; ")}`);
      }
    }

    for (const path of missingRoutes) {
      const article = await visit(page, path);
      await expect(article).toHaveAttribute("data-site-page", "not-found");
    }

    // Code blocks and example sources copy to the clipboard.
    await visit(page, "/components/button");
    const clipboard = () => page.evaluate(() => navigator.clipboard.readText());
    await page.getByRole("button", { name: "Copy code", exact: true }).first().click();
    await expect.poll(clipboard).toBe("dxui add button");
    const example = page.locator("[data-site-example]").first();
    await example.getByRole("tab", { name: "Code", exact: true }).click();
    const copySource = example.getByRole("button", { name: /^Copy .+ source$/ });
    await copySource.click();
    await expect(copySource).toHaveText("Copied");
    await expect(example.getByRole("status")).toHaveText("Copied to the clipboard");
    const shown = await example.locator("[data-site-example-source]").textContent();
    await expect.poll(clipboard).toBe(shown);
    await expect(copySource).toHaveText("Copy", { timeout: 4000 });
    // Blocks work as screens: the login block checks its fields, the
    // settings block tracks unsaved changes, and the dashboard sorts orders.
    await visit(page, "/blocks/login");
    const login = page.locator("main [data-site-block-preview]");
    await login.getByRole("button", { name: "Sign in", exact: true }).click();
    const email = login.getByRole("textbox", { name: "Email", exact: true });
    await expect(email).toHaveAttribute("aria-invalid", "true");
    await expect(email).toHaveAccessibleDescription("Enter your email address.");
    await email.fill("ada@acme.example");
    await expect(email).toHaveAttribute("aria-invalid", "false");
    await visit(page, "/blocks/settings");
    const settings = page.locator("main [data-site-block-preview]");
    const save = settings.getByRole("button", { name: "Save", exact: true });
    await expect(save).toBeDisabled();
    await settings.getByRole("textbox", { name: "Name", exact: true }).fill("");
    await expect(settings.getByRole("status")).toHaveText("Unsaved changes");
    await save.click();
    await expect(settings.getByRole("textbox", { name: "Name", exact: true })).toHaveAttribute("aria-invalid", "true");
    await settings.getByRole("textbox", { name: "Name", exact: true }).fill("Grace Hopper");
    await save.click();
    await expect(settings.getByRole("status")).toHaveText("All changes saved");
    // The signup block rates the password as it is typed and refuses a weak one.
    await visit(page, "/blocks/signup");
    const signup = page.locator("main [data-site-block-preview]");
    const newPassword = signup.getByLabel("Password", { exact: true });
    const strength = signup.getByRole("progressbar", { name: "Password strength" });
    await newPassword.fill("abc");
    await expect(strength).toHaveAttribute("aria-valuetext", "Weak");
    await signup.getByRole("button", { name: "Create account", exact: true }).click();
    await expect(newPassword).toHaveAttribute("aria-invalid", "true");
    await expect(newPassword).toHaveAccessibleDescription(/Use 8 or more characters/);
    await newPassword.fill("Correct-Horse-9");
    await expect(strength).toHaveAttribute("aria-valuetext", "Strong");
    await expect(newPassword).toHaveAttribute("aria-invalid", "false");
    // The chat block appends a sent message and takes files dropped on the composer.
    await visit(page, "/blocks/chat");
    const chat = page.locator("main [data-site-block-preview]");
    const transcript = chat.getByRole("log", { name: "Messages" });
    const composerText = chat.getByRole("textbox", { name: "Message", exact: true });
    await composerText.fill("Shipping it today");
    await composerText.press("Enter");
    // Sent last, so it reads after the conversation's last message.
    await expect.poll(async () => {
      const text = await transcript.textContent();
      return text.indexOf("Shipping it today") > text.indexOf("Here are the logs");
    }).toBe(true);
    await expect(composerText).toHaveValue("");
    await chat.getByRole("form", { name: "Message composer" }).evaluate((form) => {
      const data = new DataTransfer();
      data.items.add(new File(["hello"], "notes.txt", { type: "text/plain" }));
      form.dispatchEvent(new DragEvent("dragover", { bubbles: true, cancelable: true, dataTransfer: data }));
      form.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer: data }));
    });
    const toSend = chat.getByRole("group", { name: "Files to send" });
    await expect(toSend).toContainText("notes.txt");
    await chat.getByRole("button", { name: "Send", exact: true }).click();
    await expect(toSend).toHaveCount(0);
    await expect(transcript).toContainText("notes.txt");
    // The files block: the tree and the breadcrumbs pick the folder, a drop
    // adds a file to it, and the context menu deletes the row it opened on.
    await visit(page, "/blocks/files");
    const filesBlock = page.locator("main [data-site-block-preview]");
    const folderTree = filesBlock.getByRole("tree", { name: "Folders" });
    const fileRows = () => filesBlock.locator("tbody tr");
    const crumbs = filesBlock.getByRole("navigation", { name: "breadcrumb" });
    await expect(fileRows()).toHaveCount(2);
    await folderTree.getByRole("treeitem", { name: "Design" }).locator(":scope > [data-dxui-tree-row]").click();
    await expect(fileRows()).toHaveCount(2);
    await expect(fileRows().first()).toContainText("Logo.svg");
    await expect(crumbs).toContainText("Design");
    await filesBlock.getByRole("region", { name: "Upload area" }).evaluate((area) => {
      const data = new DataTransfer();
      data.items.add(new File(["<svg/>"], "Icon.svg", { type: "image/svg+xml" }));
      area.dispatchEvent(new DragEvent("dragover", { bubbles: true, cancelable: true, dataTransfer: data }));
      area.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer: data }));
    });
    await expect(fileRows()).toHaveCount(3);
    await expect(fileRows().last()).toContainText("Icon.svg");
    await filesBlock.locator("tbody").getByText("Logo.svg", { exact: true }).click({ button: "right" });
    await page.getByRole("menuitem", { name: "Delete Logo.svg" }).click();
    await expect(fileRows()).toHaveCount(2);
    await expect(filesBlock.locator("tbody")).not.toContainText("Logo.svg");
    await crumbs.getByRole("button", { name: "My files" }).click();
    await expect(fileRows().first()).toContainText("Roadmap.pdf");
    await expect(folderTree.getByRole("treeitem", { name: "My files" })).toHaveAttribute("aria-selected", "true");
    // The schedule block adds an event to its day and checks the times.
    await visit(page, "/blocks/schedule");
    const schedule = page.locator("main [data-site-block-preview]");
    const thursday = schedule.getByRole("region", { name: "Thursday, October 15" });
    await expect(thursday.getByRole("listitem")).toHaveCount(2);
    await schedule.getByRole("button", { name: "New event", exact: true }).click();
    const newEvent = page.getByRole("dialog", { name: "New event" });
    await newEvent.getByRole("textbox", { name: "Title" }).fill("Design review");
    await newEvent.getByLabel("End").fill("09:00");
    await newEvent.getByRole("button", { name: "Save" }).click();
    await expect(newEvent.getByLabel("End")).toHaveAttribute("aria-invalid", "true");
    await newEvent.getByLabel("End").fill("15:00");
    await newEvent.getByLabel("Start").fill("14:00");
    await newEvent.getByRole("button", { name: "Save" }).click();
    await expect(newEvent).toBeHidden();
    await expect(thursday.getByRole("listitem")).toHaveCount(3);
    await expect(thursday.getByRole("listitem").last()).toContainText("Design review");
    await schedule.getByRole("button", { name: "Next week", exact: true }).click();
    await expect(schedule.getByRole("region", { name: "Thursday, October 22" })).toBeVisible();
    // The inbox block opens a message in its pane and searches the list.
    await visit(page, "/blocks/inbox");
    const inbox = page.locator("main [data-site-block-preview]");
    const messages = inbox.getByRole("list", { name: "Messages" }).getByRole("listitem");
    const pane = inbox.getByRole("region", { name: "Reading pane" });
    await expect(pane.getByRole("heading", { level: 2 })).toHaveText("Quarterly numbers");
    await messages.filter({ hasText: "Design review notes" }).getByRole("button").click();
    await expect(pane.getByRole("heading", { level: 2 })).toHaveText("Design review notes");
    await expect(messages).toHaveCount(4);
    await inbox.getByRole("searchbox", { name: "Search mail" }).fill("invoice");
    await expect(messages).toHaveCount(1);
    await expect(messages.first()).toContainText("Billing");
    // The landing block's early-access form works from the keyboard alone.
    await visit(page, "/blocks/landing");
    const landing = page.locator("main [data-site-block-preview]");
    const notify = landing.getByRole("button", { name: "Notify me", exact: true });
    const landingEmail = landing.getByRole("textbox", { name: "Email", exact: true });
    await landingEmail.focus();
    await page.keyboard.press("Tab");
    await expect(notify).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(landingEmail).toHaveAttribute("aria-invalid", "true");
    await expect(landingEmail).toHaveAccessibleDescription("Enter your email address.");
    await landingEmail.focus();
    await page.keyboard.type("ada@acme.example");
    await page.keyboard.press("Enter");
    await expect(landing.getByRole("status")).toHaveText("Thanks. We will write to ada@acme.example.");
    // The pricing block's billing switch changes every plan's price.
    await visit(page, "/blocks/pricing");
    const pricing = page.locator("main [data-site-block-preview]");
    const pro = pricing.getByRole("region", { name: "Pro", exact: true });
    await expect(pro).toContainText("$12/month");
    await pricing.getByRole("group", { name: "Billing period" }).getByRole("button", { name: "Yearly" }).click();
    await expect(pro).toContainText("$120/year");
    await expect(pricing.getByRole("region", { name: "Team", exact: true })).toContainText("$290/year");
    await visit(page, "/blocks/dashboard");
    const dashboard = page.locator("main [data-site-block-preview]");
    // The chart's fallback table also has rows; the orders table has a Customer column.
    const orderRows = () => dashboard.locator("table", { hasText: "Customer" }).locator("tbody tr");
    const firstOrder = () => orderRows().first();
    await expect(firstOrder()).toContainText("#3210");
    await dashboard.getByRole("button", { name: "Amount", exact: true }).click();
    await expect(firstOrder()).toContainText("#3209");
    await dashboard.getByRole("searchbox", { name: "Search customers" }).fill("sofia");
    await expect(orderRows()).toHaveCount(1);

    // A link to another component's docs opens that page on the site.
    await visit(page, "/components/date-picker");
    await page.locator("[data-site-reference]").getByRole("link", { name: /Calendar/ }).first().click();
    await expect(page.locator("main [data-site-page]")).toHaveAttribute("data-component", "calendar");
    // Version snippets follow the crate version.
    await visit(page, "/docs/getting-started");
    await expect(page.locator("main")).not.toContainText("0.1.0");

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
        const violations = await accessibilityViolations(page, {
          disabledRules: path.startsWith("/blocks/") ? blockLandmarkRules : [],
        });
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
