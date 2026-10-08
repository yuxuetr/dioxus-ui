#!/usr/bin/env node
import { expect } from "@playwright/test";
import { compilePreviewCss, utilityConflicts } from "./preview-tailwind.mjs";
import {
  accessibilityViolations,
  enforceStrictCsp,
  launchBrowser,
  lowContrastText,
  serveDioxusWeb,
  setDarkTheme,
} from "./browser-check-support.mjs";

const viewport = { width: 1280, height: 900 };
// `--csp` runs the same checks under the strict Content Security Policy
// (RFC 0080) and fails on anything the browser refuses.
const strictCsp = process.argv.includes("--csp");
const server = serveDioxusWeb({ packageName: "dioxus-ui-web-demo", bin: "preview", port: 45239 });
const previewUrl = server.url;

async function expectFocused(page, selector, label) {
  const focused = await page.evaluate((targetSelector) => {
    return document.activeElement?.matches(targetSelector) ?? false;
  }, selector);

  if (!focused) {
    throw new Error(`${label}: expected ${selector} to be focused`);
  }
}

// Every rendered class list must leave one utility per property, or the
// stylesheet order rather than the component state decides the style.
async function expectNoUtilityConflicts(page, label) {
  const classLists = await page.evaluate(() =>
    [...document.querySelectorAll("[class]")].map((element) => element.getAttribute("class")),
  );
  const conflicts = await utilityConflicts(classLists);
  if (conflicts.length > 0) {
    throw new Error(`${label}: conflicting Tailwind utilities: ${conflicts.join(", ")}`);
  }
}

// The preview header toggle switches the page to the opt-in dark theme and
// back (RFC 0050).
async function expectThemeToggle(page) {
  const toggle = page.locator("#preview-theme-toggle");
  const main = page.locator('[data-preview-root="web"]');
  const surface = () =>
    main.evaluate((element) => {
      const canvas = document.createElement("canvas");
      const context = canvas.getContext("2d");
      context.fillStyle = getComputedStyle(element).backgroundColor;
      context.fillRect(0, 0, 1, 1);
      return Math.max(...context.getImageData(0, 0, 1, 1).data.slice(0, 3));
    });
  for (const dark of [true, false]) {
    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", String(dark));
    await expect(main).toHaveCSS("color-scheme", dark ? "dark" : "normal");
    const channel = await surface();
    if (dark ? channel > 40 : channel < 240) {
      throw new Error(`theme toggle: the page surface should be ${dark ? "dark" : "light"}, got channel ${channel}`);
    }
  }
}

// The preview shows several instances of one landmark component, such as the
// Toast and Sonner viewports, which an app renders one of.
async function expectNoViolations(page, label) {
  const violations = await accessibilityViolations(page, { disabledRules: ["landmark-unique"] });
  if (violations.length > 0) {
    throw new Error(`${label}: accessibility violations: ${violations.join("; ")}`);
  }
}

// Audits an open overlay, menu, or popup in both themes.
async function expectAccessibleOpen(page, label) {
  for (const theme of ["light", "dark"]) {
    await setDarkTheme(page, theme === "dark");
    await expectNoViolations(page, `${label} (${theme} theme)`);
  }
  await setDarkTheme(page, false);
}

// Text must stay readable in the light theme and under the opt-in `.dark`
// block, which must turn white surfaces dark.
async function expectReadableText(page, label) {
  for (const theme of ["light", "dark"]) {
    // Components use transition-colors, so wait for the theme switch to settle.
    await setDarkTheme(page, theme === "dark");
    if (theme === "dark") {
      const surface = await page.evaluate(() => {
        const probe = document.createElement("div");
        probe.className = "bg-background";
        document.body.append(probe);
        const canvas = document.createElement("canvas");
        const context = canvas.getContext("2d");
        context.fillStyle = getComputedStyle(probe).backgroundColor;
        context.fillRect(0, 0, 1, 1);
        probe.remove();
        return Math.max(...context.getImageData(0, 0, 1, 1).data.slice(0, 3));
      });
      if (surface > 40) {
        throw new Error(`${label}: bg-background under .dark should be a dark surface, got channel ${surface}`);
      }
    }
    const failures = await page.evaluate(lowContrastText);
    if (failures.length > 0) {
      throw new Error(`${label} (${theme} theme): low contrast text: ${failures.join("; ")}`);
    }
    await expectNoViolations(page, `${label} (${theme} theme)`);
  }
  await setDarkTheme(page, false);
}

// At phone width the page must not scroll sideways, and no element may extend
// outside its fixture card unless an ancestor inside the card clips it.
async function expectPhoneWidthLayout(page, label) {
  await page.setViewportSize({ width: 375, height: 800 });
  const failures = await page.evaluate(() => {
    const root = document.documentElement;
    const problems = [];
    if (root.scrollWidth > root.clientWidth) {
      problems.push(`page is ${root.scrollWidth}px wide in a ${root.clientWidth}px viewport`);
    }
    const clipped = (element, card) => {
      for (let node = element.parentElement; node && node !== card; node = node.parentElement) {
        if (getComputedStyle(node).overflowX !== "visible") {
          return true;
        }
      }
      return false;
    };
    for (const card of document.querySelectorAll("main article")) {
      const bounds = card.getBoundingClientRect();
      for (const element of card.querySelectorAll("*")) {
        const rect = element.getBoundingClientRect();
        const outside = rect.left < bounds.left - 1 || rect.right > bounds.right + 1;
        if (rect.width > 0 && rect.height > 0 && outside && getComputedStyle(element).position !== "fixed") {
          if (!clipped(element, card)) {
            const name = card.querySelector("h2")?.textContent ?? card.tagName;
            problems.push(`${element.tagName} "${element.textContent.trim().slice(0, 30)}" outside the ${name} card`);
          }
        }
      }
    }
    return problems;
  });
  await page.setViewportSize(viewport);
  if (failures.length > 0) {
    throw new Error(`${label} at 375px: ${failures.slice(0, 10).join("; ")}`);
  }
}

// The border color a lone utility renders, to compare a state against.
function utilityBorderColor(page, utility) {
  return page.evaluate((className) => {
    const probe = document.createElement("div");
    probe.className = `border ${className}`;
    document.body.append(probe);
    const color = getComputedStyle(probe).borderTopColor;
    probe.remove();
    return color;
  }, utility);
}

// The background color a lone utility renders.
function utilityBackgroundColor(page, utility) {
  return page.evaluate((className) => {
    const probe = document.createElement("div");
    probe.className = className;
    document.body.append(probe);
    const color = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return color;
  }, utility);
}

async function runBrowserAssertions() {
  const browser = await launchBrowser("scripts/runtime-interactions-verify.mjs");
  let csp = null;

  try {
    const context = await browser.newContext({ viewport });
    const page = await context.newPage();
    csp = strictCsp ? await enforceStrictCsp(page) : null;
    const previewCss = await compilePreviewCss();
    // dx serves the stylesheet uncompiled; answer it with compiled Tailwind so
    // class-based layout takes part in every check below.
    await page.route(/\/assets\/preview[^/]*\.css(?:\?.*)?$/, (route) =>
      route.fulfill({ contentType: "text/css", body: previewCss }),
    );

    await page.goto(previewUrl, { waitUntil: "domcontentloaded", timeout: 30000 });
    await page.waitForSelector('[data-preview-root="web"]', {
      state: "attached",
      timeout: 30000,
    });

    const root = page.locator('[data-interaction-root="runtime"]');
    await expect(root).toHaveCount(1);
    await expect(page.locator(".sr-only").first()).toHaveCSS("position", "absolute");
    await expectNoUtilityConflicts(page, "initial render");
    await expectReadableText(page, "initial render");
    await expectThemeToggle(page);

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
    await expectAccessibleOpen(page, "open overlay");
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

    // Visibility comes from the Rust render; the page scripts attach their
    // listeners a frame later, once placement has made the content fixed.
    const expectAnchoredReady = (target) =>
      expect(target.locator("[data-dxui-anchored]").first()).toHaveCSS("position", "fixed");
    const popover = page.locator('[data-interaction-target="popover"]');
    const popoverTrigger = page.locator('[data-interaction-control="popover-trigger"]');
    const popoverContent = popover.locator('[role="dialog"]:not([data-interaction-control])');
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
    // Without a title or description the content points at neither.
    const plainPopover = popover.locator('[data-interaction-control="plain-popover"]');
    await expect(plainPopover).not.toHaveAttribute("aria-labelledby");
    await expect(plainPopover).not.toHaveAttribute("aria-describedby");
    // The content is named by its title and described by its description.
    await expect(popoverContent).toHaveAccessibleName("Dimensions");
    await expect(popoverContent).toHaveAccessibleDescription(
      "Placed below the trigger, or above it near the viewport bottom.",
    );
    let placed = await boxes();
    if (placed.content.y < placed.trigger.y + placed.trigger.height) {
      throw new Error(`popover should sit below its trigger: ${JSON.stringify(placed)}`);
    }
    expectInViewport(placed.content, "bottom placement");
    await expectAccessibleOpen(page, "open popover");
    await page.keyboard.press("Escape");
    await expect(popoverContent).toBeHidden();
    await popoverTrigger.click();
    await expectAnchoredReady(popover);
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

    const select = page.locator('[data-interaction-target="select"]');
    const selectTrigger = page.locator("#interaction-select-trigger");
    await expect(selectTrigger).toHaveAccessibleDescription("Pick one fruit.");
    const selectContent = select.locator('[role="listbox"]');
    const selectHighlighted = selectContent.locator("[data-highlighted]");
    const selectOption = (name) => selectContent.getByRole("option", { name, exact: true });
    const expectSelectHighlight = async (name) => {
      await expect(selectHighlighted).toHaveText(name);
      const id = await selectOption(name).getAttribute("id");
      await expect(selectTrigger).toHaveAttribute("aria-activedescendant", id);
    };
    await selectTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(selectContent).toBeHidden();
    await selectTrigger.click();
    await expectAnchoredReady(select);
    await expectAccessibleOpen(page, "open select");
    await expect(selectContent).toHaveAttribute("data-side", "bottom");
    const selectPlaced = { trigger: await selectTrigger.boundingBox(), content: await selectContent.boundingBox() };
    if (selectPlaced.content.y < selectPlaced.trigger.y + selectPlaced.trigger.height) {
      throw new Error(`select listbox should sit below its trigger: ${JSON.stringify(selectPlaced)}`);
    }
    expectInViewport(selectPlaced.content, "select placement");
    // The list is at least as wide as its trigger.
    expect(selectPlaced.content.width, `select list width ${JSON.stringify(selectPlaced)}`).toBeGreaterThanOrEqual(Math.floor(selectPlaced.trigger.width));
    await expect(selectTrigger).toBeFocused();
    await expectSelectHighlight("Banana");
    await page.keyboard.press("ArrowUp");
    await expectSelectHighlight("Apple");
    await page.keyboard.press("ArrowUp");
    await expectSelectHighlight("Apple");
    await page.keyboard.press("End");
    await expectSelectHighlight("Cherry");
    await page.keyboard.press("Home");
    await expectSelectHighlight("Apple");
    await page.keyboard.type("bl");
    await expectSelectHighlight("Blueberry");
    // Let the typeahead buffer expire so the next letter starts a new search.
    await page.waitForTimeout(600);
    await page.keyboard.press("c");
    await expectSelectHighlight("Cherry");
    await page.waitForTimeout(600);
    await page.keyboard.press("b");
    await expectSelectHighlight("Banana");
    await page.keyboard.press("b");
    await expectSelectHighlight("Blueberry");
    await page.keyboard.press("Enter");
    await expect(selectContent).toBeHidden();
    await expect(select).toHaveAttribute("data-value", "blueberry");
    await expect(selectTrigger).toHaveText("Blueberry");
    await expect(selectTrigger).toBeFocused();
    await expect(selectTrigger).not.toHaveAttribute("aria-activedescendant", /./);
    await page.keyboard.press("ArrowDown");
    await expectAnchoredReady(select);
    await expectSelectHighlight("Blueberry");
    await page.keyboard.press("ArrowDown");
    await expectSelectHighlight("Cherry");
    await page.waitForTimeout(600);
    await page.keyboard.press(" ");
    await expect(selectContent).toBeHidden();
    await expect(select).toHaveAttribute("data-value", "cherry");
    // Space must not also click the trigger and reopen the listbox.
    await page.waitForTimeout(300);
    await expect(selectContent).toBeHidden();
    await selectTrigger.click();
    await expectAnchoredReady(select);
    await selectOption("Apple").hover();
    await expectSelectHighlight("Apple");
    await selectOption("Apple").click();
    await expect(selectContent).toBeHidden();
    await expect(select).toHaveAttribute("data-value", "apple");
    await expect(selectTrigger).toBeFocused();
    await selectTrigger.click();
    await expectAnchoredReady(select);
    await selectOption("Apricot").dispatchEvent("click");
    await page.waitForTimeout(300);
    await expect(selectContent).toBeVisible();
    await expect(select).toHaveAttribute("data-value", "apple");
    await page.keyboard.press("Escape");
    await expect(selectContent).toBeHidden();
    await selectTrigger.click();
    await expectAnchoredReady(select);
    await select.getByRole("heading", { name: "Select interaction" }).click();
    await expect(selectContent).toBeHidden();
    await expect(select).toHaveAttribute("data-value", "apple");

    const command = page.locator('[data-interaction-target="command"]');
    const commandInput = command.getByRole("combobox");
    const commandList = command.getByRole("listbox");
    const commandItem = (name) => command.getByRole("option", { name, exact: true });
    const expectCommandHighlight = async (name) => {
      const id = await commandItem(name).getAttribute("id");
      await expect(commandInput).toHaveAttribute("aria-activedescendant", id);
      await expect(commandItem(name)).toHaveAttribute("data-highlighted", "");
    };
    await commandInput.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(commandList).toHaveAttribute("id", /./);
    await expect(commandInput).toHaveAttribute("aria-controls", (await commandList.getAttribute("id")) ?? "");
    // The status region stays mounted, so its text changes are announced.
    const commandStatus = command.getByRole("status");
    await expect(commandStatus).toHaveAttribute("aria-live", "polite");
    await expect(commandStatus).toHaveAttribute("aria-atomic", "true");
    await expect(commandStatus).toHaveText("");
    await commandStatus.evaluate((element) => {
      element.dxuiMountProbe = true;
    });
    // The first option starts highlighted.
    await expectCommandHighlight("Calendar");
    await commandInput.focus();
    // Arrows skip the disabled item and stop at the ends.
    await page.keyboard.press("ArrowDown");
    await expectCommandHighlight("Search Emoji");
    await page.keyboard.press("ArrowDown");
    await expectCommandHighlight("Profile");
    await page.keyboard.press("End");
    await expectCommandHighlight("Settings");
    await page.keyboard.press("ArrowDown");
    await expectCommandHighlight("Settings");
    await page.keyboard.press("Home");
    await expectCommandHighlight("Calendar");
    await page.keyboard.press("ArrowUp");
    await expectCommandHighlight("Calendar");
    // Typing reaches the input, and a query change goes back to the first
    // match even when the highlighted option still matches.
    await page.keyboard.press("End");
    await expectCommandHighlight("Settings");
    await page.keyboard.type("se ");
    await expect(commandInput).toHaveValue("se ");
    await expect(commandItem("Calendar")).toHaveCount(0);
    await expect(commandStatus).toHaveText("2 results");
    await expectCommandHighlight("Search Emoji");
    await commandInput.fill("set");
    await expect(commandItem("Search Emoji")).toHaveCount(0);
    await expect(commandStatus).toHaveText("1 result");
    await expectCommandHighlight("Settings");
    await page.keyboard.press("Enter");
    await expect(command).toHaveAttribute("data-result", "settings");
    await expect(commandInput).toBeFocused();
    // Widening the query also goes back to the first match.
    await page.keyboard.press("Backspace");
    await expect(commandInput).toHaveValue("se");
    await expectCommandHighlight("Search Emoji");
    // Nothing matches: no highlight, and Enter chooses nothing.
    await commandInput.fill("zzz");
    await expect(command.getByText("No results found.")).toBeVisible();
    await expect(commandStatus).toHaveText("No results");
    await expect(commandInput).not.toHaveAttribute("aria-activedescendant", /./);
    await page.keyboard.press("Enter");
    await expect(command).toHaveAttribute("data-result", "settings");
    // The pointer highlights, and a click chooses while focus stays put.
    await commandInput.fill("");
    await expectCommandHighlight("Calendar");
    await expect(commandStatus).toHaveText("");
    if (!(await commandStatus.evaluate((element) => element.dxuiMountProbe === true))) {
      throw new Error("command status region should stay mounted across query changes");
    }
    await commandItem("Billing").hover();
    await expectCommandHighlight("Billing");
    await commandItem("Profile").click();
    await expect(command).toHaveAttribute("data-result", "profile");
    await expect(commandInput).toBeFocused();
    await commandItem("Calculator").click({ force: true });
    await expect(command).toHaveAttribute("data-result", "profile");

    const combobox = page.locator('[data-interaction-target="combobox"]');
    const comboboxInput = page.locator("#interaction-combobox-input");
    await expect(combobox.getByRole("combobox", { name: "Fruit", exact: true })).toHaveAttribute(
      "id",
      "interaction-combobox-input",
    );
    await expect(comboboxInput).toHaveAccessibleDescription("Pick one fruit.");
    const comboboxContent = combobox.locator('[role="listbox"]');
    const comboboxHighlighted = comboboxContent.locator("[data-highlighted]");
    const comboboxOption = (name) => comboboxContent.getByRole("option", { name, exact: true });
    await comboboxInput.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(comboboxContent).toBeHidden();
    await expect(comboboxInput).toHaveAttribute("aria-expanded", "false");
    // The status region sits outside the hidden popup and stays mounted.
    const comboboxStatus = combobox.getByRole("status");
    await expect(comboboxStatus).toHaveAttribute("aria-live", "polite");
    await expect(comboboxStatus).toHaveText("");
    await comboboxStatus.evaluate((element) => {
      element.dxuiMountProbe = true;
    });
    await comboboxInput.click();
    await page.keyboard.type("b");
    await expectAnchoredReady(combobox);
    await expect(comboboxInput).toHaveAttribute("aria-expanded", "true");
    await expectAccessibleOpen(page, "open combobox");
    const comboboxPlaced = { input: await comboboxInput.boundingBox(), content: await comboboxContent.boundingBox() };
    if (comboboxPlaced.content.y < comboboxPlaced.input.y + comboboxPlaced.input.height) {
      throw new Error(`combobox listbox should sit below its input: ${JSON.stringify(comboboxPlaced)}`);
    }
    // The anchored container, which holds the list inside its padding, is at
    // least as wide as the input.
    const comboboxPanel = await combobox.locator("[data-dxui-anchored]").boundingBox();
    expect(comboboxPanel.width, `combobox panel width ${JSON.stringify({ ...comboboxPlaced, comboboxPanel })}`).toBeGreaterThanOrEqual(
      Math.floor(comboboxPlaced.input.width),
    );
    await expect(comboboxContent.getByRole("option")).toHaveText(["Banana", "Blueberry"]);
    await expect(comboboxStatus).toHaveText("2 results");
    await expect(comboboxHighlighted).toHaveCount(0);
    await page.keyboard.press("Enter");
    await expect(comboboxContent).toBeVisible();
    await page.keyboard.press("ArrowDown");
    await expect(comboboxHighlighted).toHaveText("Banana");
    await page.keyboard.press("ArrowDown");
    await expect(comboboxHighlighted).toHaveText("Blueberry");
    await expect(comboboxInput).toHaveAttribute("aria-activedescendant", await comboboxOption("Blueberry").getAttribute("id"));
    await page.keyboard.press("ArrowDown");
    await expect(comboboxHighlighted).toHaveText("Blueberry");
    await page.keyboard.type("l");
    await expect(comboboxContent.getByRole("option")).toHaveText(["Blueberry"]);
    await expect(comboboxStatus).toHaveText("1 result");
    await expect(comboboxHighlighted).toHaveText("Blueberry");
    await page.keyboard.press("Backspace");
    await page.keyboard.type("a");
    await expect(comboboxContent.getByRole("option")).toHaveText(["Banana"]);
    await expect(comboboxHighlighted).toHaveCount(0);
    await expect(comboboxInput).not.toHaveAttribute("aria-activedescendant", /./);
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(comboboxContent).toBeHidden();
    await expect(combobox).toHaveAttribute("data-value", "banana");
    await expect(comboboxInput).toHaveValue("Banana");
    await expect(comboboxInput).toBeFocused();
    await expect(comboboxStatus).toHaveText("");
    await page.keyboard.press("ArrowDown");
    await expect(comboboxContent).toBeVisible();
    await expectAnchoredReady(combobox);
    await page.keyboard.press("Escape");
    await expect(comboboxContent).toBeHidden();
    await comboboxInput.fill("zzz");
    await expect(comboboxStatus).toHaveText("No results");
    await comboboxInput.fill("");
    await expectAnchoredReady(combobox);
    await expect(comboboxContent.getByRole("option")).toHaveCount(5);
    await expect(comboboxStatus).toHaveText("5 results");
    await comboboxOption("Cherry").click();
    await expect(comboboxContent).toBeHidden();
    await expect(combobox).toHaveAttribute("data-value", "cherry");
    await expect(comboboxInput).toBeFocused();
    await expect(comboboxStatus).toHaveText("");
    if (!(await comboboxStatus.evaluate((element) => element.dxuiMountProbe === true))) {
      throw new Error("combobox status region should stay mounted across query changes");
    }

    const datePicker = page.locator('[data-interaction-target="date-picker"]');
    // The grid is named by its caption, the year and month.
    await expect(datePicker.getByRole("grid", { name: /^\d{4}-\d{2}$/, includeHidden: true })).toHaveCount(1);
    const dateTrigger = page.locator("#interaction-date-trigger");
    const dateContent = datePicker.locator('[role="dialog"]');
    const dateCaption = dateContent.locator("div").filter({ hasText: /^\d{4}-\d{2}$/ });
    const dateDay = (date) => dateContent.locator(`[data-date="${date}"]`);
    const expectDateFocus = async (date, caption) => {
      await expect(dateDay(date)).toBeFocused();
      await expect(dateDay(date)).toHaveAttribute("tabindex", "0");
      if (caption) await expect(dateCaption).toHaveText(caption);
    };
    await dateTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(dateContent).toBeHidden();
    await dateTrigger.click();
    await expectAnchoredReady(datePicker);
    await expectAccessibleOpen(page, "open date picker");
    await expect(dateContent).toHaveAttribute("data-side", "bottom");
    const datePlaced = { trigger: await dateTrigger.boundingBox(), content: await dateContent.boundingBox() };
    if (datePlaced.content.y < datePlaced.trigger.y + datePlaced.trigger.height) {
      throw new Error(`date picker should sit below its trigger: ${JSON.stringify(datePlaced)}`);
    }
    expectInViewport(datePlaced.content, "date picker placement");
    await expectDateFocus("2026-10-15", "2026-10");
    await expect(dateContent.locator('[role="gridcell"][tabindex="0"]')).toHaveCount(1);
    await page.keyboard.press("ArrowRight");
    await expectDateFocus("2026-10-16");
    await page.keyboard.press("ArrowDown");
    await expectDateFocus("2026-10-23");
    await page.keyboard.press("End");
    await expectDateFocus("2026-10-24");
    await page.keyboard.press("Home");
    await expectDateFocus("2026-10-18");
    await page.keyboard.press("ArrowUp");
    await expectDateFocus("2026-10-11");
    await page.keyboard.press("ArrowLeft");
    await expectDateFocus("2026-10-10");
    await page.keyboard.press("PageDown");
    await expectDateFocus("2026-11-10", "2026-11");
    await page.keyboard.press("Shift+PageUp");
    await expectDateFocus("2025-11-10", "2025-11");
    await page.keyboard.press("Shift+PageDown");
    await expectDateFocus("2026-11-10", "2026-11");
    await page.keyboard.press("PageUp");
    await expectDateFocus("2026-10-10", "2026-10");
    await page.keyboard.press("Tab");
    await expect(dateContent.getByRole("button", { name: "Go to previous month" })).toBeFocused();
    await page.keyboard.press("Shift+Tab");
    await expectDateFocus("2026-10-10");
    await page.keyboard.press("Enter");
    await expect(dateContent).toBeHidden();
    await expect(datePicker).toHaveAttribute("data-value", "2026-10-10");
    await expect(dateTrigger).toHaveText("2026-10-10");
    await expect(dateTrigger).toBeFocused();
    await dateTrigger.click();
    await expectAnchoredReady(datePicker);
    await expectDateFocus("2026-10-10", "2026-10");
    await page.keyboard.press("Escape");
    await expect(dateContent).toBeHidden();
    await expect(dateTrigger).toBeFocused();
    await dateTrigger.click();
    await expectAnchoredReady(datePicker);
    await dateContent.getByRole("button", { name: "Go to next month" }).click();
    await expect(dateCaption).toHaveText("2026-11");
    await dateDay("2026-11-03").click();
    await expect(dateContent).toBeHidden();
    await expect(datePicker).toHaveAttribute("data-value", "2026-11-03");
    await expect(dateTrigger).toBeFocused();
    await dateTrigger.click();
    await expectAnchoredReady(datePicker);
    // The browser blurs to the body for a press on non-focusable content, as
    // it would for any outside click, so only the close is asserted here.
    await datePicker.getByRole("heading", { name: "Date picker interaction" }).click();
    await expect(dateContent).toBeHidden();
    await dateTrigger.click();
    await expectAnchoredReady(datePicker);
    // An outside click that focuses another control keeps that focus.
    await page.locator("#interaction-combobox-input").click();
    await expect(dateContent).toBeHidden();
    await expect(page.locator("#interaction-combobox-input")).toBeFocused();
    await expect(datePicker).toHaveAttribute("data-value", "2026-11-03");

    const dropdown = page.locator('[data-interaction-target="dropdown"]');
    const dropdownTrigger = page.locator('[data-interaction-control="dropdown-trigger"]');
    const dropdownMenu = dropdown.locator('[role="menu"]');
    const dropdownItem = (name) => dropdownMenu.getByRole("menuitem", { name, exact: true });
    const openDropdown = async () => {
      await dropdownTrigger.click();
      await expectAnchoredReady(dropdown);
    };
    await dropdownTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(dropdownMenu).toBeHidden();
    await openDropdown();
    await expectAccessibleOpen(page, "open dropdown");
    await expect(dropdownItem("Edit")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(dropdownItem("Duplicate")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(dropdownItem("Delete")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(dropdownItem("Edit")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(dropdownItem("Delete")).toBeFocused();
    await page.keyboard.press("Home");
    await expect(dropdownItem("Edit")).toBeFocused();
    await page.keyboard.press("End");
    await expect(dropdownItem("Delete")).toBeFocused();
    await page.keyboard.press("Home");
    await page.keyboard.press("d");
    await expect(dropdownItem("Duplicate")).toBeFocused();
    await page.keyboard.press("d");
    await expect(dropdownItem("Delete")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(dropdownMenu).toBeHidden();
    await expect(dropdown).toHaveAttribute("data-action", "delete");
    await expect(dropdownTrigger).toBeFocused();
    await openDropdown();
    await page.keyboard.press("ArrowDown");
    await page.waitForTimeout(600);
    await page.keyboard.press(" ");
    await expect(dropdownMenu).toBeHidden();
    await expect(dropdown).toHaveAttribute("data-action", "duplicate");
    await expect(dropdownTrigger).toBeFocused();
    // Space must not also click the trigger once focus returns to it.
    await page.waitForTimeout(300);
    await expect(dropdownMenu).toBeHidden();
    await openDropdown();
    await dropdownItem("Archive").dispatchEvent("click");
    await page.waitForTimeout(300);
    await expect(dropdownMenu).toBeVisible();
    await expect(dropdown).toHaveAttribute("data-action", "duplicate");
    await page.keyboard.press("Escape");
    await expect(dropdownMenu).toBeHidden();
    await expect(dropdownTrigger).toBeFocused();
    await openDropdown();
    await page.keyboard.press("Tab");
    await expect(dropdownMenu).toBeHidden();
    await openDropdown();
    await dropdownItem("Edit").hover();
    await expect(dropdownItem("Edit")).toBeFocused();
    await dropdownItem("Edit").click();
    await expect(dropdownMenu).toBeHidden();
    await expect(dropdown).toHaveAttribute("data-action", "edit");
    await expect(dropdownTrigger).toBeFocused();

    const options = page.locator('[data-interaction-target="dropdown-options"]');
    const optionsTrigger = page.locator('[data-interaction-control="dropdown-options-trigger"]');
    const optionsMenu = options.locator('[role="menu"]');
    const optionCheckbox = (name) => optionsMenu.getByRole("menuitemcheckbox", { name, exact: false });
    const optionRadio = (name) => optionsMenu.getByRole("menuitemradio", { name, exact: true });
    const markOpacity = (locator) =>
      locator.evaluate((element) => getComputedStyle(element, "::before").opacity);
    const openOptions = async () => {
      await optionsTrigger.click();
      await expectAnchoredReady(options);
    };
    await optionsTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await openOptions();
    await expectAccessibleOpen(page, "open dropdown options");
    // Checkbox and radio items are menu items for focus movement.
    await expect(optionCheckbox("Status bar")).toBeFocused();
    await expect(optionCheckbox("Status bar")).toHaveAttribute("aria-checked", "true");
    await expect(optionCheckbox("Minimap")).toHaveAttribute("aria-checked", "false");
    expect(await markOpacity(optionCheckbox("Status bar"))).toBe("1");
    expect(await markOpacity(optionCheckbox("Minimap"))).toBe("0");
    expect(await markOpacity(optionRadio("Panel bottom"))).toBe("1");
    expect(await markOpacity(optionRadio("Panel right"))).toBe("0");
    await expect(optionsMenu.getByText("Ctrl+/", { exact: true })).toBeVisible();
    await page.keyboard.press("ArrowDown");
    await expect(optionCheckbox("Minimap")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(optionsMenu).toBeHidden();
    await expect(options).toHaveAttribute("data-minimap", "true");
    await expect(optionsTrigger).toBeFocused();
    await openOptions();
    await expect(optionCheckbox("Minimap")).toHaveAttribute("aria-checked", "true");
    expect(await markOpacity(optionCheckbox("Minimap"))).toBe("1");
    await page.keyboard.press("End");
    await expect(optionRadio("Panel right")).toBeFocused();
    await page.keyboard.press(" ");
    await expect(optionsMenu).toBeHidden();
    await expect(options).toHaveAttribute("data-panel", "right");
    await openOptions();
    await expect(optionRadio("Panel right")).toHaveAttribute("aria-checked", "true");
    await expect(optionRadio("Panel bottom")).toHaveAttribute("aria-checked", "false");
    expect(await markOpacity(optionRadio("Panel right"))).toBe("1");
    expect(await markOpacity(optionRadio("Panel bottom"))).toBe("0");
    await optionCheckbox("Status bar").click();
    await expect(optionsMenu).toBeHidden();
    await expect(options).toHaveAttribute("data-status-bar", "false");

    const fileMenu = page.locator('[data-interaction-target="dropdown-submenu"]');
    const fileTrigger = page.locator('[data-interaction-control="dropdown-submenu-trigger"]');
    const fileContent = fileMenu.locator('[role="menu"]:not([data-dxui-submenu])');
    const shareContent = fileMenu.locator("[data-dxui-submenu]");
    const fileItem = (name) => fileMenu.getByRole("menuitem", { name, exact: true });
    const openFile = async () => {
      await fileTrigger.click();
      await expectAnchoredReady(fileMenu);
    };
    await fileTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await openFile();
    await expect(fileItem("New file")).toBeFocused();
    await expect(fileItem("Share")).toHaveAttribute("aria-expanded", "false");
    await expect(shareContent).toBeHidden();
    // Arrow keys reach the sub trigger but not the hidden submenu's items.
    await page.keyboard.press("ArrowDown");
    await expect(fileItem("Share")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(shareContent).toBeVisible();
    await expect(fileItem("Share")).toHaveAttribute("aria-expanded", "true");
    await expect(fileItem("Copy link")).toBeFocused();
    await expectAccessibleOpen(page, "open dropdown submenu");
    // The submenu sits beside its trigger.
    const shareBox = await fileItem("Share").boundingBox();
    const submenuBox = await shareContent.boundingBox();
    expect(Math.abs(submenuBox.x - (shareBox.x + shareBox.width))).toBeLessThan(16);
    // The submenu's arrows move among its own items only.
    await page.keyboard.press("ArrowDown");
    await expect(fileItem("Email")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(fileItem("Copy link")).toBeFocused();
    // ArrowLeft and Escape close one level.
    await page.keyboard.press("ArrowLeft");
    await expect(shareContent).toBeHidden();
    await expect(fileContent).toBeVisible();
    await expect(fileItem("Share")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(fileItem("Copy link")).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(shareContent).toBeHidden();
    await expect(fileContent).toBeVisible();
    await expect(fileItem("Share")).toBeFocused();
    // Choosing a nested item closes every level.
    await page.keyboard.press("ArrowRight");
    await expect(fileItem("Copy link")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(fileContent).toBeHidden();
    await expect(fileMenu).toHaveAttribute("data-action", "email");
    await expect(fileTrigger).toBeFocused();
    await openFile();
    await expect(fileItem("Share")).toHaveAttribute("aria-expanded", "false");
    await expect(shareContent).toBeHidden();
    // Hover opens the submenu without taking focus; a sibling closes it.
    await fileItem("Share").hover();
    await expect(shareContent).toBeVisible();
    await expect(fileItem("Share")).toBeFocused();
    await fileItem("Print").hover();
    await expect(shareContent).toBeHidden();
    await fileItem("Share").hover();
    await expect(shareContent).toBeVisible();
    await fileItem("Copy link").hover();
    await expect(fileItem("Copy link")).toBeFocused();
    await fileItem("Copy link").click();
    await expect(fileContent).toBeHidden();
    await expect(fileMenu).toHaveAttribute("data-action", "copy");
    // Closing the outer menu closes an open submenu, so it reopens closed.
    await openFile();
    await fileItem("Share").hover();
    await expect(shareContent).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(fileContent).toBeHidden();
    await openFile();
    await expect(shareContent).toBeHidden();
    await expect(fileItem("Share")).toHaveAttribute("aria-expanded", "false");
    await fileItem("Print").click();
    await expect(fileContent).toBeHidden();
    await expect(fileMenu).toHaveAttribute("data-action", "print");

    const menubar = page.locator('[data-interaction-target="menubar"]');
    await expect(menubar.getByRole("menubar", { name: "Editor", exact: true })).toHaveCount(1);
    const menubarTrigger = (value) => menubar.locator(`[data-value="${value}"] [data-dxui-menubar-trigger]`);
    const menubarMenu = (value) =>
      menubar.locator(`[data-value="${value}"] [role="menu"]:not([data-dxui-submenu])`);
    const menubarItem = (value, name) => menubarMenu(value).getByRole("menuitem", { name, exact: true });
    // Same readiness signal as expectAnchoredReady, for one menu of several.
    const expectMenubarOpen = async (value) => {
      await expect(menubarMenu(value)).toHaveCSS("position", "fixed");
      for (const other of ["file", "edit", "view"].filter((menu) => menu !== value)) {
        await expect(menubarMenu(other)).toBeHidden();
      }
    };
    const expectMenubarClosed = async () => {
      for (const menu of ["file", "edit", "view"]) await expect(menubarMenu(menu)).toBeHidden();
    };
    await menubarTrigger("file").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expectMenubarClosed();
    await expect(menubarTrigger("file")).toHaveAttribute("tabindex", "0");
    await expect(menubarTrigger("edit")).toHaveAttribute("tabindex", "-1");
    await expect(menubarTrigger("view")).toHaveAttribute("tabindex", "-1");
    await menubarTrigger("file").focus();
    await page.keyboard.press("ArrowRight");
    await expect(menubarTrigger("edit")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(menubarTrigger("view")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(menubarTrigger("file")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(menubarTrigger("view")).toBeFocused();
    await expect(menubarTrigger("view")).toHaveAttribute("tabindex", "0");
    await expect(menubarTrigger("file")).toHaveAttribute("tabindex", "-1");
    await page.keyboard.press("Home");
    await expect(menubarTrigger("file")).toBeFocused();
    await page.keyboard.press("End");
    await expect(menubarTrigger("view")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(menubarTrigger("edit")).toBeFocused();
    await expectMenubarClosed();
    await page.keyboard.press("ArrowDown");
    await expectMenubarOpen("edit");
    await expect(menubarItem("edit", "Undo")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expectMenubarOpen("view");
    await expect(menubarItem("view", "Zoom in")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expectMenubarOpen("file");
    await expectAccessibleOpen(page, "open menubar");
    await expect(menubarItem("file", "New")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expectMenubarOpen("view");
    await page.keyboard.press("Escape");
    await expectMenubarClosed();
    await expect(menubarTrigger("view")).toBeFocused();
    await page.keyboard.press("Enter");
    await expectMenubarOpen("view");
    await expect(menubarItem("view", "Zoom in")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expectMenubarClosed();
    await expect(menubar).toHaveAttribute("data-action", "zoom-out");
    await expect(menubarTrigger("view")).toBeFocused();
    // In a submenu, ArrowLeft closes that level instead of switching menus,
    // and ArrowRight on a plain item switches to the next menu.
    await page.keyboard.press("Enter");
    await expectMenubarOpen("view");
    await page.keyboard.press("End");
    await expect(menubarItem("view", "Appearance")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(menubarItem("view", "Full screen")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(menubarItem("view", "Appearance")).toBeFocused();
    await expectMenubarOpen("view");
    await page.keyboard.press("ArrowRight");
    await expect(menubarItem("view", "Full screen")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expectMenubarOpen("file");
    await page.keyboard.press("ArrowLeft");
    await expectMenubarOpen("view");
    await page.keyboard.press("End");
    await page.keyboard.press("ArrowRight");
    // The closed submenu opens on ArrowRight and its script focuses its first
    // item a frame later; a key pressed before that is lost.
    await expect(menubarItem("view", "Full screen")).toBeFocused();
    await page.keyboard.press("Enter");
    await expectMenubarClosed();
    await expect(menubar).toHaveAttribute("data-action", "fullscreen");
    await expect(menubarTrigger("view")).toBeFocused();
    await menubarTrigger("file").hover();
    // Hovering switches menus only while one is open.
    await page.waitForTimeout(300);
    await expectMenubarClosed();
    await menubarTrigger("file").click();
    await expectMenubarOpen("file");
    await menubarTrigger("edit").hover();
    await expectMenubarOpen("edit");
    await menubarItem("edit", "Redo").click();
    await expectMenubarClosed();
    await expect(menubar).toHaveAttribute("data-action", "redo");
    await expect(menubarTrigger("edit")).toBeFocused();
    await menubarTrigger("file").click();
    await expectMenubarOpen("file");
    await menubarTrigger("file").click();
    await expectMenubarClosed();
    await menubarTrigger("file").click();
    await expectMenubarOpen("file");
    await page.keyboard.press("Tab");
    await expectMenubarClosed();
    await expect(menubar.locator(":focus")).toHaveCount(0);

    const navigation = page.locator('[data-interaction-target="navigation-menu"]');
    await expect(navigation.getByRole("navigation", { name: "Product", exact: true })).toHaveCount(1);
    const navigationTrigger = (name) => navigation.getByRole("button", { name, exact: true });
    const navigationContent = (value) =>
      navigation.locator(`[data-value="${value}"] [data-dxui-navigation-content]`);
    const navigationLink = (name) => navigation.getByRole("link", { name, exact: true });
    const navigationOutside = navigation.getByRole("heading", { name: "Navigation menu interaction" });
    await navigationTrigger("Docs").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(navigationContent("docs")).toBeHidden();
    await navigationTrigger("Docs").click();
    await expect(navigationContent("docs")).toBeVisible();
    await expect(navigationTrigger("Docs")).toHaveAttribute("aria-expanded", "true");
    await expectAccessibleOpen(page, "open navigation menu");
    await navigationTrigger("Docs").click();
    await expect(navigationContent("docs")).toBeHidden();
    // A trigger closed by a click stays closed while the pointer stays on it.
    const docsBox = await navigationTrigger("Docs").boundingBox();
    await page.mouse.move(docsBox.x + docsBox.width / 2 + 2, docsBox.y + docsBox.height / 2);
    await page.waitForTimeout(400);
    await expect(navigationContent("docs")).toBeHidden();
    await navigationTrigger("Docs").focus();
    await page.keyboard.press("ArrowRight");
    await expect(navigationLink("Blog")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(navigationTrigger("Examples")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(navigationTrigger("Docs")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(navigationTrigger("Examples")).toBeFocused();
    await page.keyboard.press("Home");
    await expect(navigationTrigger("Docs")).toBeFocused();
    await page.keyboard.press("End");
    await expect(navigationTrigger("Examples")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(navigationContent("examples")).toBeVisible();
    await expect(navigationTrigger("Examples")).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(navigationContent("examples")).toBeHidden();
    await expect(navigationTrigger("Examples")).toBeFocused();
    await page.keyboard.press("Home");
    await page.keyboard.press("ArrowDown");
    await expect(navigationContent("docs")).toBeVisible();
    await expect(navigationLink("Install")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(navigationLink("CLI")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(navigationLink("Install")).toBeFocused();
    await page.keyboard.press("End");
    await expect(navigationLink("CLI")).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(navigationLink("Blog")).toBeFocused();
    await expect(navigationContent("docs")).toBeVisible();
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Escape");
    await expect(navigationContent("docs")).toBeHidden();
    await expect(navigationTrigger("Docs")).toBeFocused();
    await page.keyboard.press("Space");
    await expect(navigationContent("docs")).toBeVisible();
    await dropdownTrigger.focus();
    await expect(navigationContent("docs")).toBeHidden();
    await navigationTrigger("Docs").click();
    await expect(navigationContent("docs")).toBeVisible();
    await navigationOutside.click();
    await expect(navigationContent("docs")).toBeHidden();
    await navigationTrigger("Docs").click();
    await navigationLink("Install").click();
    await expect(navigationContent("docs")).toBeHidden();
    await page.mouse.move(5, 5);
    await navigationTrigger("Examples").hover();
    // Hover opens after a delay.
    await expect(navigationContent("examples")).toBeHidden();
    await expect(navigationContent("examples")).toBeVisible();
    await navigationTrigger("Docs").hover();
    await expect(navigationContent("docs")).toBeVisible();
    await expect(navigationContent("examples")).toBeHidden();
    await navigationLink("CLI").hover();
    await page.waitForTimeout(400);
    await expect(navigationContent("docs")).toBeVisible();
    await page.mouse.move(5, 5);
    await expect(navigationContent("docs")).toBeVisible();
    await expect(navigationContent("docs")).toBeHidden();

    const tabs = page.locator('[data-interaction-target="tabs"]');
    await expect(tabs.getByRole("tablist", { name: "Account settings", exact: true })).toHaveCount(1);
    const tab = (name) => tabs.getByRole("tab", { name, exact: true });
    const tabPanel = (name) => tabs.getByRole("tabpanel", { name, exact: true });
    await tab("Account").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(tab("Account")).toHaveAttribute("tabindex", "0");
    for (const name of ["Password", "Billing", "Team"]) {
      await expect(tab(name)).toHaveAttribute("tabindex", "-1");
    }
    // Panels are named by their trigger through aria-labelledby.
    await expect(tabPanel("Account")).toBeVisible();
    const accountPanelId = await tabPanel("Account").getAttribute("id");
    await expect(tab("Account")).toHaveAttribute("aria-controls", accountPanelId);
    await tab("Account").focus();
    await page.keyboard.press("ArrowRight");
    await expect(tab("Billing")).toBeFocused();
    await expect(tabs).toHaveAttribute("data-value", "billing");
    await expect(tabPanel("Billing")).toBeVisible();
    await expect(tabs.getByRole("tabpanel")).toHaveCount(1);
    await page.keyboard.press("ArrowRight");
    await expect(tab("Team")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(tab("Account")).toBeFocused();
    await expect(tabs).toHaveAttribute("data-value", "account");
    await page.keyboard.press("ArrowLeft");
    await expect(tab("Team")).toBeFocused();
    await page.keyboard.press("Home");
    await expect(tab("Account")).toBeFocused();
    await page.keyboard.press("End");
    await expect(tab("Team")).toBeFocused();
    await expect(tabs).toHaveAttribute("data-value", "team");
    await expect(tab("Team")).toHaveAttribute("tabindex", "0");
    await expect(tab("Account")).toHaveAttribute("tabindex", "-1");
    await page.keyboard.press("Tab");
    await expect(tabPanel("Team")).toBeFocused();
    await tab("Billing").click();
    await expect(tabs).toHaveAttribute("data-value", "billing");
    await expect(tab("Billing")).toHaveAttribute("tabindex", "0");

    const verticalTabs = page.locator('[data-interaction-target="tabs-vertical"]');
    const verticalList = verticalTabs.getByRole("tablist");
    const verticalTab = (name) => verticalTabs.getByRole("tab", { name, exact: true });
    const verticalPanel = (name) => verticalTabs.getByRole("tabpanel", { name, exact: true });
    await verticalTab("General").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(verticalList).toHaveAttribute("aria-orientation", "vertical");
    // Vertical triggers stack in one column.
    const generalBox = await verticalTab("General").boundingBox();
    const securityBox = await verticalTab("Security").boundingBox();
    expect(Math.round(securityBox.x)).toBe(Math.round(generalBox.x));
    expect(securityBox.y).toBeGreaterThanOrEqual(generalBox.y + generalBox.height);
    await expect(verticalList).toHaveAttribute("data-orientation", "vertical");
    await expect(verticalPanel("General")).toHaveAttribute("data-orientation", "vertical");
    await expect(verticalTab("General")).toHaveAttribute("tabindex", "0");
    await verticalTab("General").focus();
    await page.keyboard.press("ArrowRight");
    await expect(verticalTab("General")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(verticalTab("Security")).toBeFocused();
    // Manual activation: focus moves without selecting.
    await expect(verticalTabs).toHaveAttribute("data-value", "general");
    await expect(verticalPanel("General")).toBeVisible();
    await page.keyboard.press("ArrowDown");
    await expect(verticalTab("Notifications")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(verticalTab("General")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(verticalTab("Notifications")).toBeFocused();
    await page.keyboard.press("Home");
    await expect(verticalTab("General")).toBeFocused();
    await page.keyboard.press("End");
    await expect(verticalTab("Notifications")).toBeFocused();
    await expect(verticalTabs).toHaveAttribute("data-value", "general");
    await page.keyboard.press("Enter");
    await expect(verticalTabs).toHaveAttribute("data-value", "notifications");
    await expect(verticalPanel("Notifications")).toBeVisible();
    await page.keyboard.press("ArrowUp");
    await expect(verticalTab("Security")).toBeFocused();
    await page.keyboard.press("Space");
    await expect(verticalTabs).toHaveAttribute("data-value", "security");
    await page.keyboard.press("ArrowUp");
    await expect(verticalTab("General")).toBeFocused();
    // Leaving from an unselected trigger makes the selected one the Tab stop.
    await page.keyboard.press("Tab");
    await expect(verticalPanel("Security")).toBeFocused();
    await expect(verticalTab("Security")).toHaveAttribute("tabindex", "0");
    await expect(verticalTab("General")).toHaveAttribute("tabindex", "-1");
    await page.keyboard.press("Shift+Tab");
    await expect(verticalTab("Security")).toBeFocused();
    await verticalTab("General").click();
    await expect(verticalTabs).toHaveAttribute("data-value", "general");

    const radioGroup = page.locator('[data-interaction-target="radio-group"]');
    const radio = (value) => radioGroup.locator(`[role="radio"][data-value="${value}"]`);
    await radio("small").evaluate((element) => element.scrollIntoView({ block: "center" }));
    // The group and every item have a name: Small through a Label, the rest
    // through aria-label.
    await expect(radioGroup.getByRole("radiogroup", { name: "Size", exact: true })).toHaveCount(1);
    for (const [name, value] of [["Small", "small"], ["Medium", "medium"], ["Large", "large"], ["Extra large", "x-large"]]) {
      await expect(radioGroup.getByRole("radio", { name, exact: true })).toHaveAttribute("data-value", value);
    }
    const progressBar = page.locator('[data-interaction-target="progress"]').getByRole("progressbar", { name: "Upload", exact: true });
    await expect(progressBar).toHaveAttribute("aria-valuenow", "60");
    await expect(progressBar).toHaveAttribute("aria-valuetext", "3 of 5 files");
    // Nothing checked: the first enabled item is the Tab stop.
    await expect(radio("small")).toHaveAttribute("tabindex", "0");
    await expect(radio("large")).toHaveAttribute("tabindex", "-1");
    await radio("small").focus();
    await page.keyboard.press("ArrowDown");
    await expect(radio("large")).toBeFocused();
    await expect(radioGroup).toHaveAttribute("data-value", "large");
    await expect(radio("large")).toHaveAttribute("aria-checked", "true");
    await page.keyboard.press("ArrowDown");
    await expect(radio("x-large")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(radio("small")).toBeFocused();
    await expect(radioGroup).toHaveAttribute("data-value", "small");
    await page.keyboard.press("ArrowUp");
    await expect(radio("x-large")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(radio("small")).toBeFocused();
    await expect(radio("small")).toHaveAttribute("tabindex", "0");
    await expect(radio("large")).toHaveAttribute("tabindex", "-1");
    await radio("large").click();
    await expect(radioGroup).toHaveAttribute("data-value", "large");
    await expect(radio("large")).toHaveAttribute("tabindex", "0");
    // The checked border replaces the unchecked one.
    await expect(radio("large")).toHaveCSS("border-top-color", await utilityBorderColor(page, "border-primary"));
    await expect(radio("small")).toHaveCSS("border-top-color", await utilityBorderColor(page, "border-input"));

    const toggleGroup = page.locator('[data-interaction-target="toggle-group"]');
    await expect(toggleGroup.getByRole("group", { name: "Text style", exact: true })).toHaveCount(1);
    const toggle = (name) => toggleGroup.getByRole("button", { name, exact: true });
    await toggle("Bold").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(toggle("Bold")).toHaveAttribute("tabindex", "0");
    await expect(toggle("Italic")).toHaveAttribute("tabindex", "-1");
    await toggle("Bold").focus();
    await page.keyboard.press("ArrowRight");
    await expect(toggle("Italic")).toBeFocused();
    // Focus moves without pressing.
    await expect(toggleGroup).toHaveAttribute("data-value", "none");
    await page.keyboard.press("ArrowDown");
    await expect(toggle("Italic")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(toggle("Underline")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(toggleGroup).toHaveAttribute("data-value", "underline");
    await expect(toggle("Underline")).toHaveAttribute("aria-pressed", "true");
    await page.keyboard.press("ArrowRight");
    await expect(toggle("Bold")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(toggle("Underline")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(toggle("Italic")).toBeFocused();
    // The last focused item stays the Tab stop after focus leaves.
    await page.keyboard.press("Shift+Tab");
    await expect(toggleGroup.locator(":focus")).toHaveCount(0);
    // Checking another radio re-renders the page while focus is outside the
    // group, so the Tab stop is recomputed.
    await expect(radio("large")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(radioGroup).toHaveAttribute("data-value", "x-large");
    await expect(toggle("Italic")).toHaveAttribute("tabindex", "0");
    await expect(toggle("Underline")).toHaveAttribute("tabindex", "-1");
    await page.keyboard.press("Tab");
    await expect(toggle("Italic")).toBeFocused();
    await toggle("Bold").click();
    await expect(toggleGroup).toHaveAttribute("data-value", "bold");
    await toggle("Bold").click();
    await expect(toggleGroup).toHaveAttribute("data-value", "none");

    // Right to left: ArrowLeft moves to the next item and ArrowRight to the
    // previous one. Up and Down keep their meaning.
    const setDirection = (target, value) =>
      target.evaluate((element, dir) => {
        if (dir) element.setAttribute("dir", dir);
        else element.removeAttribute("dir");
      }, value);
    const rtlTargets = [tabs, radioGroup, toggleGroup, menubar, navigation];
    for (const target of rtlTargets) await setDirection(target, "rtl");
    await tab("Billing").evaluate((element) => element.scrollIntoView({ block: "center" }));
    const billingBox = await tab("Billing").boundingBox();
    const teamBox = await tab("Team").boundingBox();
    if (!(teamBox.x < billingBox.x)) throw new Error("rtl tabs should lay out the next tab on the left");
    await tab("Billing").focus();
    await page.keyboard.press("ArrowLeft");
    await expect(tab("Team")).toBeFocused();
    await expect(tabs).toHaveAttribute("data-value", "team");
    await page.keyboard.press("ArrowRight");
    await expect(tab("Billing")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(tab("Account")).toBeFocused();
    await expect(tabs).toHaveAttribute("data-value", "account");
    await radio("x-large").focus();
    await page.keyboard.press("ArrowLeft");
    await expect(radio("small")).toBeFocused();
    await expect(radioGroup).toHaveAttribute("data-value", "small");
    await page.keyboard.press("ArrowRight");
    await expect(radio("x-large")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(radio("large")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(radio("x-large")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(radio("large")).toBeFocused();
    await expect(radioGroup).toHaveAttribute("data-value", "large");
    await toggle("Bold").focus();
    await page.keyboard.press("ArrowLeft");
    await expect(toggle("Italic")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(toggle("Underline")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(toggle("Italic")).toBeFocused();
    await expect(toggleGroup).toHaveAttribute("data-value", "none");
    await menubarTrigger("file").focus();
    await page.keyboard.press("ArrowLeft");
    await expect(menubarTrigger("edit")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(menubarTrigger("file")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(menubarTrigger("view")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expectMenubarOpen("view");
    await expect(menubarItem("view", "Zoom in")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(menubarItem("view", "Zoom out")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expectMenubarOpen("file");
    await page.keyboard.press("ArrowRight");
    await expectMenubarOpen("view");
    await page.keyboard.press("Escape");
    await expectMenubarClosed();
    await expect(menubarTrigger("view")).toBeFocused();
    await navigationTrigger("Docs").focus();
    await page.keyboard.press("ArrowLeft");
    await expect(navigationLink("Blog")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(navigationTrigger("Docs")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(navigationTrigger("Examples")).toBeFocused();
    // Removing dir restores left-to-right keys.
    for (const target of rtlTargets) await setDirection(target, null);
    await page.keyboard.press("ArrowRight");
    await expect(navigationTrigger("Docs")).toBeFocused();
    await tab("Account").focus();
    await page.keyboard.press("ArrowRight");
    await expect(tab("Billing")).toBeFocused();
    await menubarTrigger("view").focus();
    await page.keyboard.press("ArrowRight");
    await expect(menubarTrigger("file")).toBeFocused();
    await expectMenubarClosed();

    const switchFixture = page.locator('[data-interaction-target="switch"]');
    const wifi = switchFixture.getByRole("switch", { name: "Wi-Fi", exact: true });
    const airplane = switchFixture.getByRole("switch", { name: "Airplane mode", exact: true });
    const terms = switchFixture.getByRole("checkbox", { name: "Accept terms", exact: true });
    const newsletter = switchFixture.getByRole("checkbox", { name: "Newsletter", exact: true });
    await wifi.evaluate((element) => element.scrollIntoView({ block: "center" }));
    // The accessible names come from Label for= and aria-label passed through.
    await expect(wifi).toHaveAttribute("aria-checked", "false");
    await expect(wifi).toHaveAttribute("data-state", "unchecked");
    await wifi.click();
    await expect(switchFixture).toHaveAttribute("data-switch", "true");
    await expect(wifi).toHaveAttribute("aria-checked", "true");
    await expect(wifi).toHaveAttribute("data-state", "checked");
    await expect(wifi.locator("span")).toHaveAttribute("data-state", "checked");
    await wifi.focus();
    await page.keyboard.press("Space");
    await expect(switchFixture).toHaveAttribute("data-switch", "false");
    await expect(wifi).toHaveAttribute("aria-checked", "false");
    await page.keyboard.press("Enter");
    await expect(switchFixture).toHaveAttribute("data-switch", "true");
    await switchFixture.locator('label[for="interaction-switch-wifi"]').click();
    await expect(switchFixture).toHaveAttribute("data-switch", "false");
    await expect(wifi).toHaveAttribute("data-state", "unchecked");
    await expect(airplane).toBeDisabled();
    await airplane.click({ force: true });
    await expect(airplane).toHaveAttribute("aria-checked", "true");
    // The checkbox box toggles natively, so assert the app state as well.
    await expect(terms).not.toBeChecked();
    await expect(terms).toHaveAttribute("name", "terms");
    await terms.click();
    await expect(switchFixture).toHaveAttribute("data-checkbox", "true");
    await expect(terms).toBeChecked();
    await terms.focus();
    await page.keyboard.press("Space");
    await expect(switchFixture).toHaveAttribute("data-checkbox", "false");
    await expect(terms).not.toBeChecked();
    await switchFixture.locator('label[for="interaction-checkbox-terms"]').click();
    await expect(switchFixture).toHaveAttribute("data-checkbox", "true");
    await expect(terms).toBeChecked();
    // The checkbox draws its own box: no native control, the primary fill,
    // and a `::before` masked to a tick that only the checked state shows,
    // filled with --primary-foreground so it follows any theme (RFC 0057).
    const mark = (locator) =>
      locator.evaluate((element) => {
        const style = getComputedStyle(element, "::before");
        return { opacity: style.opacity, color: style.backgroundColor, mask: style.maskImage, width: style.width };
      });
    const primaryForeground = () =>
      page.evaluate(() => {
        const probe = document.createElement("div");
        probe.style.backgroundColor = "var(--primary-foreground)";
        document.body.append(probe);
        const color = getComputedStyle(probe).backgroundColor;
        probe.remove();
        return color;
      });
    await expect(terms).toHaveCSS("appearance", "none");
    await expect(terms).toHaveCSS("background-color", await utilityBackgroundColor(page, "bg-primary"));
    const tick = await mark(terms);
    expect(tick.opacity).toBe("1");
    expect(tick.mask).toContain("data:image/svg+xml");
    expect(tick.width).not.toBe("0px");
    expect(tick.color).toBe(await primaryForeground());
    for (const dark of [true, false]) {
      await page.evaluate((on) => document.documentElement.classList.toggle("dark", on), dark);
      expect((await mark(terms)).color).toBe(await primaryForeground());
    }
    await page.evaluate(() => document.documentElement.style.setProperty("--primary-foreground", "rgb(1, 2, 3)"));
    expect((await mark(terms)).color).toBe("rgb(1, 2, 3)");
    await page.evaluate(() => document.documentElement.style.removeProperty("--primary-foreground"));
    expect((await mark(newsletter)).opacity).toBe("0");
    await expect(newsletter).toBeDisabled();
    await newsletter.click({ force: true });
    await expect(newsletter).not.toBeChecked();
    await expect(switchFixture).toHaveAttribute("data-disabled-changes", "0");
    // A partial selection makes Select all mixed through the native property.
    const mixedCheckbox = (name) => switchFixture.getByRole("checkbox", { name, exact: true });
    const isIndeterminate = (name) => mixedCheckbox(name).evaluate((element) => element.indeterminate);
    await expect.poll(() => isIndeterminate("Select all")).toBe(true);
    await expect(mixedCheckbox("Select all")).toHaveAttribute("data-state", "indeterminate");
    // Mixed draws a dash on the primary fill, not the tick.
    await expect(mixedCheckbox("Select all")).toHaveCSS("background-color", await utilityBackgroundColor(page, "bg-primary"));
    const dash = await mark(mixedCheckbox("Select all"));
    expect(dash.opacity).toBe("1");
    expect(dash.mask).toContain("data:image/svg+xml");
    expect(dash.mask).not.toBe(tick.mask);
    // A change while mixed requests checked.
    await mixedCheckbox("Select all").click();
    await expect(switchFixture).toHaveAttribute("data-mixed-items", "true-true");
    await expect.poll(() => isIndeterminate("Select all")).toBe(false);
    await expect(mixedCheckbox("Select all")).toBeChecked();
    await expect(mixedCheckbox("Select all")).toHaveAttribute("data-state", "checked");
    await mixedCheckbox("Item two").click();
    await expect(switchFixture).toHaveAttribute("data-mixed-items", "true-false");
    await expect.poll(() => isIndeterminate("Select all")).toBe(true);
    // The browser clears the property on click; the component restores it
    // when the app keeps the checkbox mixed.
    await expect.poll(() => isIndeterminate("Stays mixed")).toBe(true);
    await mixedCheckbox("Stays mixed").click();
    await expect(switchFixture).toHaveAttribute("data-stays-mixed-changes", "1");
    // Mixed requests checked even when `checked` is already true.
    await expect(switchFixture).toHaveAttribute("data-stays-mixed-request", "Some(true)");
    await expect.poll(() => isIndeterminate("Stays mixed")).toBe(true);

    const formControls = page.locator('[data-interaction-target="form-controls"]');
    const save = formControls.getByRole("button", { name: "Save", exact: true });
    const archive = formControls.getByRole("button", { name: "Archive", exact: true });
    const bold = formControls.getByRole("button", { name: "Bold", exact: true });
    const email = formControls.getByLabel("Email", { exact: true });
    const notes = formControls.getByLabel("Notes", { exact: true });
    await save.evaluate((element) => element.scrollIntoView({ block: "center" }));
    // Attributes passed to the components reach the rendered elements.
    await expect(save).toHaveAttribute("type", "button");
    await expect(save).toHaveAttribute("name", "save");
    await save.click();
    await expect(formControls).toHaveAttribute("data-clicks", "1");
    await save.focus();
    await page.keyboard.press("Enter");
    await expect(formControls).toHaveAttribute("data-clicks", "2");
    await page.keyboard.press("Space");
    await expect(formControls).toHaveAttribute("data-clicks", "3");
    await expect(archive).toBeDisabled();
    await archive.click({ force: true });
    await expect(formControls).toHaveAttribute("data-clicks", "3");
    await expect(bold).toHaveAttribute("aria-pressed", "false");
    await bold.click();
    await expect(formControls).toHaveAttribute("data-bold", "true");
    await expect(bold).toHaveAttribute("aria-pressed", "true");
    await bold.focus();
    await page.keyboard.press("Space");
    await expect(formControls).toHaveAttribute("data-bold", "false");
    await expect(bold).toHaveAttribute("aria-pressed", "false");
    await expect(email).toHaveAttribute("type", "email");
    await expect(email).toHaveAttribute("name", "email");
    await email.pressSequentially("ada@example.com");
    await expect(formControls).toHaveAttribute("data-email", "ada@example.com");
    await expect(email).toHaveValue("ada@example.com");
    await email.press("Backspace");
    await expect(formControls).toHaveAttribute("data-email", "ada@example.co");
    await expect(notes).toHaveAttribute("rows", "3");
    await notes.pressSequentially("Line one");
    await notes.press("Enter");
    await notes.pressSequentially("Line two");
    await expect(formControls).toHaveAttribute("data-notes", "Line one\nLine two");
    await expect(notes).toHaveValue("Line one\nLine two");

    // Diff moves its divider through a native range input: the arrow keys
    // step it, the app state follows, and the after layer's clip and the
    // divider read the new position (RFC 0059).
    const diffFixture = page.locator('[data-interaction-target="diff"]');
    const diffRange = diffFixture.getByRole("slider", { name: "Comparison position", exact: true });
    await expect(diffRange).toHaveValue("50");
    await diffRange.focus();
    for (let press = 0; press < 3; press += 1) {
      await diffRange.press("ArrowRight");
    }
    await expect(diffFixture).toHaveAttribute("data-position", "53");
    await expect(diffRange).toHaveValue("53");
    const diffAfter = diffFixture.getByText("After", { exact: true });
    await expect(diffAfter).toHaveCSS("clip-path", "inset(0px 0px 0px 53%)");
    await diffRange.press("Home");
    await expect(diffFixture).toHaveAttribute("data-position", "0");
    const diffBox = await diffRange.boundingBox();
    await page.mouse.click(diffBox.x + diffBox.width * 0.75, diffBox.y + diffBox.height / 2);
    await expect.poll(async () => Number(await diffFixture.getAttribute("data-position"))).toBeGreaterThan(70);
    // Rating is a native radio group: the arrow keys move the value, a click
    // sets it, and the stars up to the value are filled (RFC 0060).
    const ratingFixture = page.locator('[data-interaction-target="rating"]');
    const ratingGroup = ratingFixture.getByRole("radiogroup", { name: "Product rating", exact: true });
    const star = (n) => ratingGroup.getByRole("radio", { name: `${n} of 5`, exact: true });
    await expect(star(3)).toBeChecked();
    await star(3).focus();
    await star(3).press("ArrowRight");
    await expect(ratingFixture).toHaveAttribute("data-rating", "4");
    await expect(star(4)).toBeChecked();
    await expect(star(4)).toHaveAttribute("data-filled", "true");
    await expect(star(5)).toHaveAttribute("data-filled", "false");
    await star(2).click();
    await expect(ratingFixture).toHaveAttribute("data-rating", "2");
    await expect(star(3)).toHaveAttribute("data-filled", "false");
    // Number Input keeps the typed text, reports complete numbers, steps with
    // the arrow keys and buttons, and clamps on blur (RFC 0060).
    const numberFixture = page.locator('[data-interaction-target="number-input"]');
    const quantity = numberFixture.getByRole("spinbutton", { name: "Quantity", exact: true });
    const increase = numberFixture.getByRole("button", { name: "Increase", exact: true });
    const decrease = numberFixture.getByRole("button", { name: "Decrease", exact: true });
    await expect(quantity).toHaveValue("5");
    await quantity.fill("7");
    await expect(numberFixture).toHaveAttribute("data-value", "7");
    await quantity.press("ArrowUp");
    await expect(numberFixture).toHaveAttribute("data-value", "8");
    await expect(quantity).toHaveAttribute("aria-valuenow", "8");
    // Incomplete text survives the render and reverts on blur.
    await quantity.fill("-");
    await expect(quantity).toHaveValue("-");
    await expect(numberFixture).toHaveAttribute("data-value", "8");
    await quantity.blur();
    await expect(quantity).toHaveValue("8");
    await quantity.fill("15");
    await quantity.blur();
    await expect(numberFixture).toHaveAttribute("data-value", "10");
    await expect(quantity).toHaveValue("10");
    await expect(increase).toBeDisabled();
    await decrease.click();
    await expect(numberFixture).toHaveAttribute("data-value", "9");
    await quantity.press("Home");
    await expect(numberFixture).toHaveAttribute("data-value", "0");
    await expect(decrease).toBeDisabled();
    // A button at a bound must not dim the field.
    await expect(quantity.locator("..")).toHaveCSS("opacity", "1");
    // Tags Input adds on Enter and commas, removes the last tag with
    // Backspace in an empty input, and removes a tag by its button (RFC 0060).
    const tagsFixture = page.locator('[data-interaction-target="tags-input"]');
    const topicsInput = tagsFixture.getByRole("textbox", { name: "Topics", exact: true });
    await topicsInput.fill("rust");
    await topicsInput.press("Enter");
    await expect(tagsFixture).toHaveAttribute("data-tags", "rust");
    await expect(topicsInput).toHaveValue("");
    await topicsInput.fill("ui, web, rust, da");
    await expect(tagsFixture).toHaveAttribute("data-tags", "rust|ui|web");
    await expect(topicsInput).toHaveValue("da");
    await topicsInput.fill("");
    await topicsInput.press("Backspace");
    await expect(tagsFixture).toHaveAttribute("data-tags", "rust|ui");
    await tagsFixture.getByRole("button", { name: "Remove rust", exact: true }).click();
    await expect(tagsFixture).toHaveAttribute("data-tags", "ui");
    await expect(tagsFixture.getByRole("listitem")).toHaveCount(1);
    // File Input passes the change event, whose files the app reads
    // (RFC 0060).
    const fileFixture = page.locator('[data-interaction-target="file-input"]');
    await fileFixture.getByLabel("Attachments", { exact: true }).setInputFiles([
      { name: "notes.txt", mimeType: "text/plain", buffer: Buffer.from("notes") },
      { name: "plan.md", mimeType: "text/markdown", buffer: Buffer.from("# plan") },
    ]);
    await expect(fileFixture).toHaveAttribute("data-files", "notes.txt|plan.md");
    // Swap toggles aria-pressed and shows the matching layer, hiding the
    // other from assistive technology (RFC 0060).
    const swapFixture = page.locator('[data-interaction-target="swap"]');
    const swapButton = swapFixture.getByRole("button", { name: "Menu", exact: true });
    const swapLayer = (state) => swapFixture.locator(`[data-swap-layer="${state}"]`).locator("..");
    await expect(swapButton).toHaveAttribute("aria-pressed", "false");
    await expect(swapLayer("on")).toHaveAttribute("aria-hidden", "true");
    await swapButton.press("Enter");
    await expect(swapFixture).toHaveAttribute("data-active", "true");
    await expect(swapButton).toHaveAttribute("aria-pressed", "true");
    await expect(swapLayer("off")).toHaveAttribute("aria-hidden", "true");
    await expect(swapLayer("on")).not.toHaveAttribute("aria-hidden", "true");
    await expect(swapLayer("on")).toHaveCSS("opacity", "1");
    await swapButton.click();
    await expect(swapFixture).toHaveAttribute("data-active", "false");
    // The Fab speed dial opens and closes from its trigger, renders its
    // actions only while open, closes on Escape with focus back on the
    // trigger, and runs an action (RFC 0061).
    const fabFixture = page.locator('[data-interaction-target="fab"]');
    const fabTrigger = fabFixture.getByRole("button", { name: "Create", exact: true });
    const photo = fabFixture.getByRole("button", { name: "Photo", exact: true });
    await expect(fabTrigger).toHaveAttribute("aria-expanded", "false");
    await expect(photo).toHaveCount(0);
    await fabTrigger.click();
    await expect(fabTrigger).toHaveAttribute("aria-expanded", "true");
    await expect(photo).toBeVisible();
    await expectAccessibleOpen(page, "open fab");
    await photo.focus();
    await page.keyboard.press("Escape");
    await expect(fabFixture).toHaveAttribute("data-open", "false");
    await expect(fabTrigger).toBeFocused();
    await fabTrigger.press("Enter");
    await fabFixture.getByRole("button", { name: "Note", exact: true }).click();
    await expect(fabFixture).toHaveAttribute("data-action", "note");
    await expect(fabFixture).toHaveAttribute("data-open", "false");
    // A multiple Select stays open across choices, reports each value, checks
    // each chosen option, and still closes on Escape (RFC 0062).
    const multiFixture = page.locator('[data-interaction-target="multi-select"]');
    const multiTrigger = multiFixture.getByRole("combobox", { name: "Fruits", exact: true });
    const multiList = page.locator("#interaction-multi-select-trigger-content");
    await multiTrigger.click();
    await expect(multiList).toBeVisible();
    await expect(multiList).toHaveAttribute("aria-multiselectable", "true");
    // As with the single Select, keys go to the list once its scripts run,
    // which placement shows; a key pressed in the frame before that is lost.
    await expectAnchoredReady(multiFixture);
    await multiTrigger.press("Enter");
    await expect(multiFixture).toHaveAttribute("data-values", "apple");
    await expect(multiList).toBeVisible();
    await multiTrigger.press("ArrowDown");
    await multiTrigger.press("Enter");
    await expect(multiFixture).toHaveAttribute("data-values", "apple|banana");
    await multiList.getByRole("option", { name: "Cherry", exact: true }).click();
    await expect(multiFixture).toHaveAttribute("data-values", "apple|banana|cherry");
    await expect(multiList.locator('[aria-selected="true"]')).toHaveCount(3);
    await expect(multiList.getByRole("option", { name: "Banana", exact: true })).toHaveClass(/after:opacity-100/);
    await expectAccessibleOpen(page, "open multi-select");
    // The click left the highlight on Cherry; Home goes back to Apple.
    await multiTrigger.press("Home");
    await multiTrigger.press("Enter");
    await expect(multiFixture).toHaveAttribute("data-values", "banana|cherry");
    await multiTrigger.press("Escape");
    await expect(multiList).toBeHidden();
    await expect(multiTrigger).toContainText("2 selected");
    // A multiple Combobox stays open after each click (RFC 0062).
    const tagFixture = page.locator('[data-interaction-target="multi-combobox"]');
    const tagInput = tagFixture.getByRole("combobox", { name: "Fruit tags", exact: true });
    const tagList = page.locator("#interaction-multi-combobox-input-list");
    await tagInput.click();
    await tagInput.press("ArrowDown");
    await expect(tagList).toBeVisible();
    await expect(tagList).toHaveAttribute("aria-multiselectable", "true");
    await tagList.getByRole("option", { name: "Banana", exact: true }).click();
    await tagList.getByRole("option", { name: "Cherry", exact: true }).click();
    await expect(tagFixture).toHaveAttribute("data-values", "banana|cherry");
    await expect(tagList).toBeVisible();
    await tagInput.press("Escape");
    await expect(tagList).toBeHidden();
    // A vertical Navigation Menu nested in content is a submenu: its triggers
    // switch the panel beside them by click, hover, and arrow keys without
    // touching the outer value, and Escape closes both (RFC 0063).
    const megaFixture = page.locator('[data-interaction-target="navigation-submenu"]');
    const solutions = megaFixture.getByRole("button", { name: "Solutions", exact: true });
    const web = megaFixture.getByRole("button", { name: "Web", exact: true });
    const mobile = megaFixture.getByRole("button", { name: "Mobile", exact: true });
    await solutions.click();
    await expect(megaFixture).toHaveAttribute("data-value", "solutions");
    await expect(megaFixture.getByRole("link", { name: "Dashboards", exact: true })).toBeVisible();
    await mobile.click();
    await expect(megaFixture).toHaveAttribute("data-sub", "mobile");
    await expect(megaFixture).toHaveAttribute("data-value", "solutions");
    await expect(megaFixture.getByRole("link", { name: "iOS apps", exact: true })).toBeVisible();
    await web.hover();
    await expect(megaFixture).toHaveAttribute("data-sub", "web");
    await web.focus();
    await web.press("ArrowDown");
    await expect(mobile).toBeFocused();
    await mobile.press("ArrowRight");
    await expect(megaFixture).toHaveAttribute("data-sub", "mobile");
    await expect(megaFixture.getByRole("link", { name: "iOS apps", exact: true })).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(megaFixture.getByRole("link", { name: "Android apps", exact: true })).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(mobile).toBeFocused();
    await expect(megaFixture).toHaveAttribute("data-value", "solutions");
    await expectAccessibleOpen(page, "open navigation submenu");
    await page.keyboard.press("Escape");
    await expect(megaFixture).toHaveAttribute("data-value", "");
    await expect(solutions).toBeFocused();
    // Date Picker Input parses typed dates in its order, keeps and marks text
    // that does not parse, clears to no date, and formats on blur (RFC 0064).
    const dateInputFixture = page.locator('[data-interaction-target="date-input"]');
    const birthday = dateInputFixture.getByRole("textbox", { name: "Birthday", exact: true });
    await expect(birthday).toHaveAttribute("placeholder", "DD/MM/YYYY");
    await birthday.fill("5.3.1990");
    await expect(dateInputFixture).toHaveAttribute("data-value", "1990-03-05");
    await expect(birthday).not.toHaveAttribute("aria-invalid", "true");
    await birthday.blur();
    await expect(birthday).toHaveValue("05/03/1990");
    await birthday.fill("31/02/1990");
    await expect(birthday).toHaveAttribute("aria-invalid", "true");
    await expect(dateInputFixture).toHaveAttribute("data-value", "1990-03-05");
    await birthday.blur();
    await expect(birthday).toHaveValue("31/02/1990");
    await birthday.fill("1990-12-24");
    await expect(dateInputFixture).toHaveAttribute("data-value", "1990-12-24");
    await birthday.fill("");
    await expect(dateInputFixture).toHaveAttribute("data-value", "");
    const sliderFixture = page.locator('[data-interaction-target="slider"]');
    const volume = sliderFixture.getByRole("slider", { name: "Volume", exact: true });
    const locked = sliderFixture.getByRole("slider", { name: "Locked", exact: true });
    const expectVolume = async (value) => {
      await expect(sliderFixture).toHaveAttribute("data-volume", value);
      await expect(volume).toHaveAttribute("aria-valuenow", value);
    };
    await volume.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(volume).toHaveAttribute("data-step", "5");
    await expectVolume("40");
    await volume.focus();
    const scrollBefore = await page.evaluate(() => window.scrollY);
    await page.keyboard.press("ArrowRight");
    await expectVolume("45");
    await page.keyboard.press("ArrowUp");
    await expectVolume("50");
    await page.keyboard.press("ArrowLeft");
    await expectVolume("45");
    await page.keyboard.press("ArrowDown");
    await expectVolume("40");
    await page.keyboard.press("PageUp");
    await expectVolume("90");
    await page.keyboard.press("PageUp");
    await expectVolume("100");
    await page.keyboard.press("ArrowRight");
    await expectVolume("100");
    await page.keyboard.press("PageDown");
    await expectVolume("50");
    await page.keyboard.press("Home");
    await expectVolume("0");
    await page.keyboard.press("ArrowDown");
    await expectVolume("0");
    await page.keyboard.press("End");
    await expectVolume("100");
    // The thumb is centered on the value.
    const thumbCenter = async (slider) => {
      const thumb = await slider.locator(":scope > span").boundingBox();
      return { x: thumb.x + thumb.width / 2, y: thumb.y + thumb.height / 2 };
    };
    const volumeBox = await volume.boundingBox();
    await expect.poll(async () => Math.round((await thumbCenter(volume)).x - volumeBox.x)).toBe(Math.round(volumeBox.width));
    await page.keyboard.press("Home");
    await expectVolume("0");
    await expect.poll(async () => Math.round((await thumbCenter(volume)).x - volumeBox.x)).toBe(0);
    await page.keyboard.press("End");
    await expectVolume("100");
    // Unprevented, PageUp, PageDown, Home, and End would scroll this long page.
    if ((await page.evaluate(() => window.scrollY)) !== scrollBefore) {
      throw new Error("slider: handled keys scrolled the page");
    }
    const box = await volume.boundingBox();
    if (!box) throw new Error("slider: the volume slider has no box");
    const at = (ratio) => box.x + box.width * ratio;
    const middle = box.y + box.height / 2;
    await page.mouse.click(at(0.25), middle);
    await expectVolume("25");
    await expect(volume).toBeFocused();
    await page.mouse.move(at(0.6), middle);
    await page.mouse.down();
    await expectVolume("60");
    await page.mouse.move(at(0.83), middle);
    await expectVolume("85");
    // Captured: the drag keeps working past the right edge.
    await page.mouse.move(box.x + box.width + 40, middle);
    await expectVolume("100");
    await page.mouse.up();
    await page.mouse.move(at(0.1), middle);
    await expectVolume("100");
    await expect(locked).toHaveAttribute("aria-disabled", "true");
    await expect(locked).toHaveAttribute("tabindex", "-1");
    const lockedBox = await locked.boundingBox();
    if (!lockedBox) throw new Error("slider: the locked slider has no box");
    await page.mouse.click(lockedBox.x + lockedBox.width * 0.9, lockedBox.y + lockedBox.height / 2);
    // A vertical slider grows from the bottom.
    const balance = sliderFixture.getByRole("slider", { name: "Balance", exact: true });
    await expect(balance).toHaveAttribute("aria-orientation", "vertical");
    const balanceBox = await balance.boundingBox();
    // The vertical root shrinks to its track instead of the full width.
    expect(balanceBox.width).toBeLessThanOrEqual(20);
    expect(balanceBox.height).toBe(128);
    const thumbFromBottom = async () => Math.round(balanceBox.y + balanceBox.height - (await thumbCenter(balance)).y);
    await expect.poll(thumbFromBottom).toBe(Math.round(balanceBox.height * 0.5));
    await page.mouse.click(balanceBox.x + balanceBox.width / 2, balanceBox.y + balanceBox.height * 0.2);
    await expect(sliderFixture).toHaveAttribute("data-balance", "80");
    await expect(balance).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(sliderFixture).toHaveAttribute("data-balance", "90");
    await expect.poll(thumbFromBottom).toBe(Math.round(balanceBox.height * 0.9));
    await locked.focus();
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("End");
    await expect(locked).toHaveAttribute("aria-valuenow", "30");
    await expect(sliderFixture).toHaveAttribute("data-locked-changes", "0");

    const collapsibleSelect = page.locator('[data-interaction-target="collapsible-select"]');
    const details = collapsibleSelect.getByRole("button", { name: "Details", exact: true });
    const detailsContent = collapsibleSelect.getByRole("region", { name: "Details content", exact: true });
    const size = collapsibleSelect.getByLabel("Size", { exact: true });
    await details.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(details).toHaveAttribute("title", "Show details");
    // The trigger points at the content only while the content is rendered.
    await expect(details).not.toHaveAttribute("aria-controls");
    await expect(details).toHaveAttribute("aria-expanded", "false");
    await expect(detailsContent).toHaveCount(0);
    await details.click();
    await expect(collapsibleSelect).toHaveAttribute("data-open", "true");
    await expect(details).toHaveAttribute("aria-expanded", "true");
    await expect(details).toHaveAttribute("aria-controls", await detailsContent.getAttribute("id"));
    await expect(detailsContent).toHaveText("More information");
    await expect(collapsibleSelect.getByRole("region", { name: "Details content" })).toBeVisible();
    await expect(collapsibleSelect.locator('[title="Details section"]')).toHaveAttribute(
      "data-state",
      "open",
    );
    await details.focus();
    await page.keyboard.press("Enter");
    await expect(collapsibleSelect).toHaveAttribute("data-open", "false");
    await expect(detailsContent).toHaveCount(0);
    await page.keyboard.press("Space");
    await expect(collapsibleSelect).toHaveAttribute("data-open", "true");
    await expect(detailsContent).toBeVisible();
    await expect(size).toHaveAttribute("name", "size");
    await expect(size).toHaveValue("md");
    await size.selectOption("lg");
    await expect(collapsibleSelect).toHaveAttribute("data-size", "lg");
    await size.selectOption({ label: "Small" });
    await expect(collapsibleSelect).toHaveAttribute("data-size", "sm");
    await expect(size).toHaveValue("sm");
    // Typeahead on the closed select changes it from the keyboard.
    await size.focus();
    await page.keyboard.press("l");
    await expect(collapsibleSelect).toHaveAttribute("data-size", "lg");
    await expect(size).toHaveValue("lg");

    const inputOtp = page.locator('[data-interaction-target="input-otp"]');
    const otpInput = inputOtp.getByRole("textbox", { name: "Verification code", exact: true });
    const otpSlot = (index) => inputOtp.locator(`[data-index="${index}"]`);
    await otpInput.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(otpInput).toHaveAttribute("name", "code");
    await expect(otpInput).toHaveAttribute("inputmode", "numeric");
    await expect(otpInput).toHaveAttribute("autocomplete", "one-time-code");
    const otpRoot = inputOtp.locator('[title="Verification code slots"]');
    await expect(otpRoot).toHaveAttribute("role", "group");
    // A press over the slots lands on the overlay input.
    await otpRoot.click();
    await expect(otpInput).toBeFocused();
    await page.keyboard.type("12");
    await expect(inputOtp).toHaveAttribute("data-code", "12");
    await expect(otpSlot(0)).toHaveText("1");
    await expect(otpSlot(1)).toHaveText("2");
    await expect(otpSlot(2)).toHaveAttribute("data-active", "true");
    // A rejected letter leaves no hidden character for Backspace to remove.
    await page.keyboard.type("a");
    await expect(inputOtp).toHaveAttribute("data-code", "12");
    await expect(otpInput).toHaveValue("12");
    await page.keyboard.press("Backspace");
    await expect(inputOtp).toHaveAttribute("data-code", "1");
    await expect(otpInput).toHaveValue("1");
    await page.keyboard.press("Backspace");
    await expect(inputOtp).toHaveAttribute("data-code", "");
    // Inserted text, as a paste or autofill delivers it, drops separators
    // and is cut at the length.
    await page.keyboard.insertText("123-4567");
    await expect(inputOtp).toHaveAttribute("data-code", "123456");
    await expect(otpInput).toHaveValue("123456");
    await expect(otpSlot(5)).toHaveText("6");
    // A full code takes no more characters.
    await page.keyboard.type("9");
    await expect(otpInput).toHaveValue("123456");
    await page.keyboard.press("Backspace");
    await expect(inputOtp).toHaveAttribute("data-code", "12345");

    const pagination = page.locator('[data-interaction-target="pagination"]');
    const [pageButtons, pageAnchors] = [pagination.getByRole("navigation").nth(0), pagination.getByRole("navigation").nth(1)];
    const pageButton = (name) => pageButtons.getByRole("button", { name, exact: true });
    await pageButton("1").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(pageButton("1")).toHaveAttribute("aria-current", "page");
    await expect(pageButton("1")).toHaveAttribute("title", "Results page 1");
    await expect(pageButton("Go to previous page")).toBeDisabled();
    await pageButton("2").click();
    await expect(pagination).toHaveAttribute("data-page", "2");
    await expect(pageButton("2")).toHaveAttribute("aria-current", "page");
    await expect(pageButton("1")).toHaveAttribute("aria-current", "false");
    await expect(pageButton("Go to previous page")).toBeEnabled();
    await pageButton("Go to next page").focus();
    await page.keyboard.press("Enter");
    await expect(pagination).toHaveAttribute("data-page", "3");
    await page.keyboard.press("Space");
    await expect(pagination).toHaveAttribute("data-page", "4");
    await pageButton("Go to next page").click();
    await expect(pagination).toHaveAttribute("data-page", "5");
    await expect(pageButton("Go to next page")).toBeDisabled();
    await pageButton("Go to previous page").click();
    await expect(pagination).toHaveAttribute("data-page", "4");
    // Anchors with an href still navigate; a disabled one has no href.
    const nextAnchor = pageAnchors.getByRole("link", { name: "Go to next page", exact: true });
    await expect(nextAnchor).toHaveAttribute("href", "#results-2");
    await expect(nextAnchor).toHaveAttribute("target", "_self");
    await expect(pageAnchors.getByRole("link", { name: "1", exact: true })).toHaveAttribute("aria-current", "page");
    const previousAnchor = pageAnchors.locator('[aria-label="Go to previous page"]');
    await expect(previousAnchor).toHaveAttribute("aria-disabled", "true");
    await expect(previousAnchor).not.toHaveAttribute("href");
    await previousAnchor.focus();
    await expect(previousAnchor).not.toBeFocused();
    // pointer-events-none stops a real press; a dispatched click still reaches
    // the element, and the component must ignore it.
    await expect(previousAnchor).toHaveCSS("pointer-events", "none");
    await previousAnchor.dispatchEvent("click");
    await expect(pagination).toHaveAttribute("data-disabled-clicks", "0");

    const carousel = page.locator('[data-interaction-target="carousel"]');
    const carouselRegion = carousel.getByRole("region", { name: "Featured products", exact: true });
    const carouselButton = (name) => carousel.getByRole("button", { name, exact: true });
    const slide = (number) => carousel.getByRole("group", { name: `${number} of 3`, exact: true });
    // A slide's content edge relative to the 240px viewport; items carry the
    // gap as left padding.
    const slideOffset = (number) =>
      slide(number).evaluate((element) => {
        const viewport = element.parentElement.parentElement;
        const padding = parseFloat(getComputedStyle(element).paddingLeft);
        return Math.round(
          element.getBoundingClientRect().left + padding - viewport.getBoundingClientRect().left,
        );
      });
    await carouselRegion.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(carouselRegion).toHaveAttribute("aria-roledescription", "carousel");
    await expect(carouselButton("Previous slide")).toBeDisabled();
    await expect(carouselButton("Previous slide")).toHaveAttribute("title", "Previous product");
    await expect(carouselButton("Next slide")).toHaveAttribute("title", "Next product");
    await expect.poll(() => slideOffset(1)).toBe(0);

    const resizable = page.locator('[data-interaction-target="resizable"]');
    const resizeHandle = resizable.getByRole("separator", { name: "Resize files", exact: true });
    const handleValue = async () => Number(await resizeHandle.getAttribute("aria-valuenow"));
    await resizeHandle.evaluate((element) => element.scrollIntoView({ block: "center" }));
    // ARIA orients a separator by its line, which is vertical between
    // side-by-side panels.
    await expect(resizeHandle).toHaveAttribute("aria-orientation", "vertical");
    await expect(resizeHandle).toHaveAttribute("aria-valuenow", "30");
    await expect(resizeHandle).toHaveAttribute("aria-valuemin", "20");
    await expect(resizeHandle).toHaveAttribute("aria-valuemax", "80");
    await expect(resizeHandle).toHaveAttribute("aria-controls", "interaction-resizable-files");
    await expect(resizable.locator("#interaction-resizable-files")).toHaveText("Files");
    await resizeHandle.focus();
    await expect(resizeHandle).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(resizable).toHaveAttribute("data-sizes", "40-60");
    await page.keyboard.press("ArrowLeft");
    await page.keyboard.press("ArrowLeft");
    await expect(resizable).toHaveAttribute("data-sizes", "20-80");
    // The minimum stops the resize.
    await page.keyboard.press("ArrowLeft");
    await expect(resizable).toHaveAttribute("data-sizes", "20-80");
    await page.keyboard.press("End");
    await expect(resizable).toHaveAttribute("data-sizes", "80-20");
    await page.keyboard.press("Home");
    await expect(resizable).toHaveAttribute("data-sizes", "20-80");
    await page.keyboard.press("ArrowRight");
    await expect(resizeHandle).toHaveAttribute("aria-valuenow", "30");
    // A drag over the 300px group moves the edge with the pointer: 30px is 10%.
    const handleBox = await resizeHandle.boundingBox();
    const [startX, startY] = [handleBox.x + handleBox.width / 2, handleBox.y + handleBox.height / 2];
    await page.mouse.move(startX, startY);
    await page.mouse.down();
    await page.mouse.move(startX + 30, startY, { steps: 3 });
    await expect.poll(handleValue).toBeCloseTo(40, 5);
    // Past the maximum the resize stops; coming back puts the edge under the
    // pointer again instead of carrying the overshoot.
    await page.mouse.move(startX + 300, startY, { steps: 3 });
    await expect.poll(handleValue).toBeCloseTo(80, 5);
    await page.mouse.move(startX + 60, startY, { steps: 3 });
    await expect.poll(handleValue).toBeCloseTo(50, 5);
    await page.mouse.up();
    await expect(resizeHandle).toBeFocused();

    const menuFixture = page.locator('[data-interaction-target="menu"]');
    const menuNav = menuFixture.getByRole("navigation", { name: "Workspace pages", exact: true });
    const menuButton = (name) => menuNav.getByRole("button", { name, exact: true });
    await menuNav.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(menuNav.getByRole("list").first()).toBeVisible();
    await expect(menuButton("Overview")).toHaveAttribute("aria-current", "page");
    await expect(menuButton("Billing")).toBeDisabled();
    // The group's button controls the nested list.
    const reports = menuButton("Reports");
    await expect(reports).toHaveAttribute("aria-expanded", "false");
    const nestedId = await reports.getAttribute("aria-controls");
    await expect(page.locator(`#${nestedId}`)).toBeHidden();
    await reports.click();
    await expect(reports).toHaveAttribute("aria-expanded", "true");
    await expect(page.locator(`#${nestedId}`)).toBeVisible();
    await menuButton("Sales").click();
    await expect(menuFixture).toHaveAttribute("data-page", "sales");
    await expect(menuButton("Sales")).toHaveAttribute("aria-current", "page");
    await expect(menuButton("Overview")).not.toHaveAttribute("aria-current");
    // Keyboard: Tab reaches the group button, Enter closes it.
    await menuButton("Overview").focus();
    await page.keyboard.press("Tab");
    await expect(reports).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(reports).toHaveAttribute("aria-expanded", "false");
    await expect(page.locator(`#${nestedId}`)).toBeHidden();

    const themeFixture = page.locator('[data-interaction-target="theme-controller"]');
    const themeButton = (name) => themeFixture.getByRole("button", { name, exact: true });
    const htmlRoot = page.locator("html");
    const storedTheme = () => page.evaluate(() => window.localStorage.getItem("dxui-preview-theme"));
    await themeButton("Dark").evaluate((element) => element.scrollIntoView({ block: "center" }));
    // System follows the color scheme, live.
    await page.emulateMedia({ colorScheme: "dark" });
    await expect(htmlRoot).toHaveClass(/\bdark\b/);
    await page.emulateMedia({ colorScheme: "light" });
    await expect(htmlRoot).not.toHaveClass(/\bdark\b/);
    // Each theme lands on the root and in storage.
    await themeButton("Dark").click();
    await expect(htmlRoot).toHaveClass(/\bdark\b/);
    await expect.poll(storedTheme).toBe("dark");
    await themeButton("Nord").click();
    await expect(htmlRoot).toHaveAttribute("data-theme", "nord");
    await expect(htmlRoot).not.toHaveClass(/\bdark\b/);
    await expect.poll(storedTheme).toBe("nord");
    await themeButton("Light").click();
    await expect(htmlRoot).not.toHaveAttribute("data-theme");
    await expect(htmlRoot).not.toHaveClass(/\bdark\b/);
    // A controller mounting with a stored theme applies and reports it.
    await themeButton("Unmount controller").click();
    await page.evaluate(() => window.localStorage.setItem("dxui-preview-theme", "dark"));
    await themeButton("Mount controller").click();
    await expect(themeFixture).toHaveAttribute("data-theme-choice", "dark");
    await expect(htmlRoot).toHaveClass(/\bdark\b/);
    await expect(themeButton("Dark")).toHaveAttribute("aria-pressed", "true");
    // Back to the system scheme for the checks that follow.
    await themeButton("System").click();
    await expect(htmlRoot).not.toHaveClass(/\bdark\b/);
    await page.evaluate(() => window.localStorage.removeItem("dxui-preview-theme"));

    const range = page.locator('[data-interaction-target="range-slider"]');
    const rangeGroup = range.getByRole("group", { name: "Price", exact: true });
    const lowThumb = range.getByRole("slider", { name: "Minimum", exact: true });
    const highThumb = range.getByRole("slider", { name: "Maximum", exact: true });
    const expectRange = async (low, high) => {
      await expect(range).toHaveAttribute("data-low", String(low));
      await expect(range).toHaveAttribute("data-high", String(high));
      await expect(lowThumb).toHaveAttribute("aria-valuenow", String(low));
      await expect(highThumb).toHaveAttribute("aria-valuenow", String(high));
      // Each thumb is bounded by the other, 2 steps of 5 apart.
      await expect(lowThumb).toHaveAttribute("aria-valuemax", String(high - 10));
      await expect(highThumb).toHaveAttribute("aria-valuemin", String(low + 10));
    };
    await rangeGroup.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expectRange(20, 80);
    await lowThumb.focus();
    await page.keyboard.press("ArrowRight");
    await expectRange(25, 80);
    await page.keyboard.press("End");
    await expectRange(70, 80);
    await page.keyboard.press("Home");
    await expectRange(0, 80);
    await highThumb.focus();
    await page.keyboard.press("PageDown");
    await expectRange(0, 30);
    await page.keyboard.press("Home");
    await expectRange(0, 10);
    await page.keyboard.press("End");
    await expectRange(0, 100);
    // A press moves the nearer thumb, and a drag stops at the gap.
    const rangeBox = await rangeGroup.boundingBox();
    const rangeAt = (percent) => rangeBox.x + (rangeBox.width * percent) / 100;
    const rangeMiddle = rangeBox.y + rangeBox.height / 2;
    await page.mouse.click(rangeAt(90), rangeMiddle);
    await expectRange(0, 90);
    await expect(highThumb).toBeFocused();
    await page.mouse.click(rangeAt(30), rangeMiddle);
    await expectRange(30, 90);
    await expect(lowThumb).toBeFocused();
    await page.mouse.move(rangeAt(90), rangeMiddle);
    await page.mouse.down();
    await page.mouse.move(rangeAt(50), rangeMiddle, { steps: 4 });
    await page.mouse.move(rangeAt(10), rangeMiddle, { steps: 4 });
    await page.mouse.up();
    await expectRange(30, 40);
    const fill = await rangeGroup.locator(".bg-primary").boundingBox();
    expect(Math.abs(fill.x - rangeAt(30))).toBeLessThan(2);
    expect(Math.abs(fill.width - (rangeBox.width * 10) / 100)).toBeLessThan(2);

    const shell = page.locator('[data-interaction-target="sidebar-mobile"]');
    const shellTrigger = shell.getByRole("button", { name: "Toggle navigation", exact: true });
    const shellPanel = page.locator("#interaction-sidebar-mobile");
    const shellOverlay = shell.locator('[aria-hidden="true"].fixed');
    const shellDialog = shell.getByRole("dialog", { name: "Navigation", exact: true });
    await shellTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    // Wide: an in-flow aside; the trigger and Ctrl+B collapse it.
    await expect(shellPanel).toHaveAttribute("data-mobile", "false");
    await expect(shellPanel).toBeVisible();
    await expect(shellTrigger).toHaveAttribute("aria-expanded", "true");
    await shellTrigger.click();
    await expect(shell).toHaveAttribute("data-collapsed", "true");
    await expect(shellTrigger).toHaveAttribute("aria-expanded", "false");
    await page.keyboard.press("Control+b");
    await expect(shell).toHaveAttribute("data-collapsed", "false");
    await expect(shell).toHaveAttribute("data-mobile-open", "false");
    // Narrow: off-canvas, opened as a modal panel over an overlay.
    await page.setViewportSize({ width: 375, height: 800 });
    await expect(shellPanel).toHaveAttribute("data-mobile", "true");
    await expect(shellPanel).toBeHidden();
    await expect(shellTrigger).toHaveAttribute("aria-expanded", "false");
    await shellTrigger.click();
    await expect(shell).toHaveAttribute("data-mobile-open", "true");
    await expect(shellPanel).toBeVisible();
    await expect(shellDialog).toHaveAttribute("aria-modal", "true");
    await expect(shellTrigger).toHaveAttribute("aria-expanded", "true");
    await expect(shellPanel.getByRole("button", { name: "Projects", exact: true })).toBeFocused();
    await expect.poll(() => page.evaluate(() => document.documentElement.style.overflow)).toBe("hidden");
    const panelBox = await shellDialog.boundingBox();
    expect(panelBox.x).toBe(0);
    await expectAccessibleOpen(page, "open off-canvas sidebar");
    await page.keyboard.press("Escape");
    await expect(shellPanel).toBeHidden();
    await expect(shell).toHaveAttribute("data-mobile-open", "false");
    await expect(shellTrigger).toBeFocused();
    await expect.poll(() => page.evaluate(() => document.documentElement.style.overflow)).toBe("");
    await shellTrigger.click();
    await expect(shellPanel).toBeVisible();
    await shellOverlay.click({ position: { x: 360, y: 400 } });
    await expect(shellPanel).toBeHidden();
    // Ctrl+B opens and closes the off-canvas panel, not the collapse.
    await page.keyboard.press("Control+b");
    await expect(shellPanel).toBeVisible();
    await page.keyboard.press("Control+b");
    await expect(shellPanel).toBeHidden();
    await expect(shell).toHaveAttribute("data-collapsed", "false");
    await page.setViewportSize(viewport);
    await expect(shellPanel).toHaveAttribute("data-mobile", "false");
    await expect(shellPanel).toBeVisible();

    const sidebar = page.locator('[data-interaction-target="sidebar"]');
    const sidebarTrigger = sidebar.getByRole("button", { name: "Toggle sidebar", exact: true });
    const sidebarLandmark = sidebar.getByRole("complementary", { name: "Workspace", exact: true });
    const sidebarButton = (name) => sidebar.getByRole("button", { name, exact: true });
    await sidebarTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(sidebarTrigger).toHaveAttribute("aria-expanded", "true");
    // The provider names the sidebar and points the trigger at it.
    await expect(sidebarTrigger).toHaveAttribute("aria-controls", await sidebarLandmark.getAttribute("id"));
    await sidebarTrigger.click();
    await expect(sidebar).toHaveAttribute("data-collapsed", "true");
    await expect(sidebarTrigger).toHaveAttribute("aria-expanded", "false");
    await expect(sidebarLandmark).toHaveAttribute("data-collapsed", "true");
    await sidebarTrigger.focus();
    await page.keyboard.press("Enter");
    await expect(sidebar).toHaveAttribute("data-collapsed", "false");
    await expect(sidebar.getByRole("group", { name: "Sections", exact: true })).toHaveCount(1);
    // Button items change the section and move aria-current.
    await expect(sidebarButton("Inbox")).toHaveAttribute("aria-current", "page");
    await sidebarButton("Drafts").click();
    await expect(sidebar).toHaveAttribute("data-section", "drafts");
    await expect(sidebarButton("Drafts")).toHaveAttribute("aria-current", "page");
    await expect(sidebarButton("Inbox")).not.toHaveAttribute("aria-current");
    await sidebarButton("Inbox").focus();
    await page.keyboard.press("Space");
    await expect(sidebar).toHaveAttribute("data-section", "inbox");
    // Link items keep their href; a disabled item cannot take focus or run
    // its onclick.
    const settingsLink = sidebar.getByRole("link", { name: "Settings", exact: true });
    await expect(settingsLink).toHaveAttribute("href", "#sidebar-settings");
    await expect(settingsLink).toHaveAttribute("title", "Open settings");
    const archiveItem = sidebar.locator('[aria-disabled="true"]', { hasText: "Archive" });
    await expect(archiveItem).not.toHaveAttribute("href");
    await archiveItem.focus();
    await expect(archiveItem).not.toBeFocused();
    // pointer-events-none stops a real press; a dispatched click still reaches
    // the element, and the component must ignore it.
    await expect(archiveItem).toHaveCSS("pointer-events", "none");
    await archiveItem.dispatchEvent("click");
    await expect(sidebarButton("Trash")).toBeDisabled();
    await expect(sidebar).toHaveAttribute("data-disabled-clicks", "0");
    // An item with neither href nor onclick stays a wrapper.
    await expect(sidebar.locator('[title="Help wrapper"]')).toHaveJSProperty("tagName", "DIV");
    // Slides step by the viewport width plus the 16px gap.
    await expect.poll(() => slideOffset(2)).toBe(256);
    await carouselButton("Next slide").click();
    await expect(carousel).toHaveAttribute("data-index", "1");
    await expect.poll(() => slideOffset(2)).toBe(0);
    await expect.poll(() => slideOffset(1)).toBe(-256);
    await expect(slide(2)).toHaveAttribute("data-selected", "true");
    // A passed label replaces the shared default on every indicator.
    await expect(carouselButton("Go to slide")).toHaveCount(0);
    await carouselButton("Show product 3").click();
    await expect(carousel).toHaveAttribute("data-index", "2");
    await expect(carouselButton("Show product 3")).toHaveAttribute("aria-current", "true");
    await expect(carouselButton("Next slide")).toBeDisabled();
    await expect.poll(() => slideOffset(3)).toBe(0);
    await carouselButton("Previous slide").click();
    await expect(carousel).toHaveAttribute("data-index", "1");
    // Arrows step from anywhere inside the region; Down does nothing when
    // horizontal.
    await carouselButton("Show product 1").focus();
    await page.keyboard.press("ArrowRight");
    await expect(carousel).toHaveAttribute("data-index", "2");
    await page.keyboard.press("ArrowDown");
    await expect(carousel).toHaveAttribute("data-index", "2");
    await page.keyboard.press("ArrowLeft");
    await page.keyboard.press("ArrowLeft");
    await expect(carousel).toHaveAttribute("data-index", "0");
    await expect.poll(() => slideOffset(1)).toBe(0);

    const accordion = page.locator('[data-interaction-target="accordion"]');
    const accordionTrigger = (name) => accordion.getByRole("button", { name, exact: true });
    const accordionRegion = (name) => accordion.getByRole("region", { name, exact: true, includeHidden: true });
    await accordionTrigger("Shipping").evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(accordion.getByRole("heading", { name: "Shipping", exact: true })).toHaveCount(1);
    // Regions are named by their trigger through aria-labelledby.
    const shippingRegionId = await accordionRegion("Shipping").getAttribute("id");
    await expect(accordionTrigger("Shipping")).toHaveAttribute("aria-controls", shippingRegionId);
    await expect(accordionRegion("Shipping")).toBeHidden();
    // Every enabled trigger is a Tab stop.
    for (const name of ["Shipping", "Warranty", "Support"]) {
      await expect(accordionTrigger(name)).not.toHaveAttribute("tabindex", "-1");
    }
    await accordionTrigger("Shipping").click();
    await expect(accordion).toHaveAttribute("data-value", "shipping");
    await expect(accordionTrigger("Shipping")).toHaveAttribute("aria-expanded", "true");
    await expect(accordionRegion("Shipping")).toBeVisible();
    await accordionTrigger("Shipping").click();
    await expect(accordion).toHaveAttribute("data-value", "none");
    await expect(accordionRegion("Shipping")).toBeHidden();
    await accordionTrigger("Shipping").focus();
    await page.keyboard.press("ArrowDown");
    await expect(accordionTrigger("Warranty")).toBeFocused();
    // Focus moves without toggling.
    await expect(accordion).toHaveAttribute("data-value", "none");
    await page.keyboard.press("ArrowDown");
    await expect(accordionTrigger("Support")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(accordionTrigger("Shipping")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(accordionTrigger("Support")).toBeFocused();
    await page.keyboard.press("Home");
    await expect(accordionTrigger("Shipping")).toBeFocused();
    await page.keyboard.press("End");
    await expect(accordionTrigger("Support")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(accordionTrigger("Support")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(accordion).toHaveAttribute("data-value", "support");
    await expect(accordionRegion("Support")).toBeVisible();
    await page.keyboard.press("Shift+Tab");
    await expect(accordionTrigger("Warranty")).toBeFocused();
    await page.keyboard.press("Shift+Tab");
    await expect(accordionTrigger("Shipping")).toBeFocused();
    await page.keyboard.press("Space");
    await expect(accordion).toHaveAttribute("data-value", "shipping");
    await expect(accordionRegion("Support")).toBeHidden();

    const contextMenu = page.locator('[data-interaction-target="context-menu"]');
    const contextArea = page.locator('[data-interaction-control="context-area"]');
    const contextContent = contextMenu.locator('[role="menu"]:not([data-dxui-submenu])');
    const contextItem = (name) => contextContent.locator('[role^="menuitem"]', { hasText: name });
    const openContextMenu = async (position) => {
      await contextArea.click({ button: "right", position });
      await expectAnchoredReady(contextMenu);
    };
    await contextArea.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(contextContent).toBeHidden();
    await openContextMenu({ x: 40, y: 30 });
    await expectAccessibleOpen(page, "open context menu");
    const areaBox = await contextArea.boundingBox();
    let contextBox = await contextContent.boundingBox();
    if (Math.abs(contextBox.x - (areaBox.x + 40)) > 1 || Math.abs(contextBox.y - (areaBox.y + 30)) > 1) {
      throw new Error(`context menu should open at the pointer: ${JSON.stringify({ areaBox, contextBox })}`);
    }
    await expect(contextItem("Back")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(contextItem("Bookmark")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(contextItem("More tools")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(contextItem("Back")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await page.keyboard.press("ArrowUp");
    await page.keyboard.press("Enter");
    await expect(contextContent).toBeHidden();
    await expect(contextMenu).toHaveAttribute("data-bookmarked", "true");
    await openContextMenu({ x: 120, y: 60 });
    contextBox = await contextContent.boundingBox();
    if (Math.abs(contextBox.x - (areaBox.x + 120)) > 1) {
      throw new Error(`context menu should follow the new pointer: ${JSON.stringify({ areaBox, contextBox })}`);
    }
    await page.keyboard.press("Escape");
    await expect(contextContent).toBeHidden();
    await openContextMenu({ x: 40, y: 30 });
    await contextItem("Back").click();
    await expect(contextContent).toBeHidden();
    await expect(contextMenu).toHaveAttribute("data-action", "back");
    // A submenu opens from the keyboard, and choosing in it closes both.
    await openContextMenu({ x: 40, y: 30 });
    await page.keyboard.press("End");
    await expect(contextItem("More tools")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(contextMenu.getByRole("menuitem", { name: "Save page" })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(contextContent).toBeHidden();
    await expect(contextMenu).toHaveAttribute("data-action", "save");
    await contextArea.evaluate((element) => element.scrollIntoView({ block: "end" }));
    await openContextMenu({ x: 40, y: 90 });
    const flippedArea = await contextArea.boundingBox();
    contextBox = await contextContent.boundingBox();
    if (contextBox.y + contextBox.height > flippedArea.y + 90 + 1) {
      throw new Error(`context menu should flip above the pointer: ${JSON.stringify({ flippedArea, contextBox })}`);
    }
    expectInViewport(contextBox, "context menu flip");
    await contextMenu.getByRole("heading", { name: "Context menu interaction" }).click();
    await expect(contextContent).toBeHidden();

    const tooltip = page.locator('[data-interaction-target="tooltip"]');
    const tooltipTrigger = tooltip.getByRole("button", { name: "Hover for tooltip" });
    const tooltipContent = tooltip.locator('[role="tooltip"]');
    await tooltipTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(tooltipContent).toBeHidden();
    await expect(tooltipTrigger).not.toHaveAttribute("aria-describedby", /./);
    // Hover opens after the 700 ms delay.
    await tooltipTrigger.hover();
    await page.waitForTimeout(300);
    await expect(tooltipContent).toBeHidden();
    await expectAnchoredReady(tooltip);
    await expect(tooltipContent).toHaveAttribute("data-side", "top");
    await expectAccessibleOpen(page, "open tooltip");
    const tooltipContentId = await tooltipContent.getAttribute("id");
    await expect(tooltipTrigger).toHaveAttribute("aria-describedby", tooltipContentId);
    const tooltipTriggerBox = await tooltipTrigger.boundingBox();
    const tooltipBox = await tooltipContent.boundingBox();
    if (tooltipBox.y + tooltipBox.height > tooltipTriggerBox.y) {
      throw new Error(`tooltip should sit above its trigger: ${JSON.stringify({ tooltipBox, tooltipTriggerBox })}`);
    }
    expectInViewport(tooltipBox, "tooltip placement");
    // The pointer can cross the gap onto the content without closing it.
    await page.mouse.move(tooltipBox.x + tooltipBox.width / 2, tooltipBox.y + tooltipBox.height / 2, { steps: 4 });
    await page.waitForTimeout(300);
    await expect(tooltipContent).toBeVisible();
    await page.mouse.move(5, 5);
    await expect(tooltipContent).toBeHidden();
    await expect(tooltipTrigger).not.toHaveAttribute("aria-describedby", /./);
    // A press closes it, and hover does not reopen it while the pointer rests.
    await tooltipTrigger.hover();
    await expect(tooltipContent).toBeVisible();
    await tooltipTrigger.click();
    await expect(tooltipContent).toBeHidden();
    await page.waitForTimeout(1000);
    await expect(tooltipContent).toBeHidden();
    await page.mouse.move(5, 5);
    await tooltip.getByRole("heading", { name: "Tooltip interaction" }).click();
    // Keyboard focus opens it before the hover delay would.
    await page.keyboard.press("Tab");
    await expect(tooltipTrigger).toBeFocused();
    await expect(tooltipContent).toBeVisible({ timeout: 400 });
    // Tooltips ignore outside presses; dispatch the press alone so focus stays.
    await page.evaluate(() => {
      document.body.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
    });
    // Dismissal is asynchronous, so give a wrongly sent close time to land.
    await page.waitForTimeout(300);
    await expect(tooltipContent).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(tooltipContent).toBeHidden();
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Tab");
    await expect(tooltipContent).toBeVisible({ timeout: 400 });
    await page.keyboard.press("Tab");
    await expect(tooltipContent).toBeHidden();

    const hoverCard = page.locator('[data-interaction-target="hover-card"]');
    const hoverCardTrigger = hoverCard.getByRole("link", { name: "@dioxus" });
    const hoverCardContent = hoverCard.locator('[data-dxui-hover-content]');
    const hoverCardHeading = hoverCard.getByRole("heading", { name: "Hover card interaction" });
    await hoverCardTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    // The tooltip check ends with Tab onto this trigger; move focus away first.
    await expect(hoverCardTrigger).toBeFocused();
    await hoverCardHeading.click();
    await expect(hoverCardContent).toBeHidden();
    await page.mouse.move(5, 5);
    // Hover opens after the 700 ms delay.
    await hoverCardTrigger.hover();
    await page.waitForTimeout(300);
    await expect(hoverCardContent).toBeHidden();
    await expectAnchoredReady(hoverCard);
    await expect(hoverCardContent).toHaveAttribute("data-side", "bottom");
    await expectAccessibleOpen(page, "open hover card");
    // The card is not named in the trigger's description.
    await expect(hoverCardTrigger).not.toHaveAttribute("aria-describedby", /./);
    // The pointer can cross the gap onto the card.
    const hoverCardBox = await hoverCardContent.boundingBox();
    await page.mouse.move(hoverCardBox.x + 20, hoverCardBox.y + 20, { steps: 4 });
    await page.waitForTimeout(400);
    await expect(hoverCardContent).toBeVisible();
    // Leaving closes it after the 300 ms close delay.
    await page.mouse.move(5, 5);
    await page.waitForTimeout(150);
    await expect(hoverCardContent).toBeVisible();
    await expect(hoverCardContent).toBeHidden();
    // A press on the trigger keeps it open, and so does a press on the card's
    // text after it, which moves focus off the trigger.
    await hoverCardTrigger.hover();
    await expect(hoverCardContent).toBeVisible();
    await hoverCardTrigger.click();
    await expect(hoverCardTrigger).toBeFocused();
    await page.waitForTimeout(400);
    await expect(hoverCardContent).toBeVisible();
    await hoverCard.getByText("Fullstack app framework for Rust.").click();
    await expect(hoverCardTrigger).not.toBeFocused();
    await page.waitForTimeout(400);
    await expect(hoverCardContent).toBeVisible();
    await expect(hoverCardTrigger).not.toHaveAttribute("aria-describedby", /./);
    // An outside press closes it; dispatch the press alone so the pointer stays.
    await page.evaluate(() => {
      document.body.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
    });
    await expect(hoverCardContent).toBeHidden();
    await page.mouse.move(5, 5);
    // Keyboard focus opens it, and Tab into the card keeps it open.
    await hoverCardHeading.click();
    await page.keyboard.press("Tab");
    await expect(hoverCardTrigger).toBeFocused();
    await expect(hoverCardContent).toBeVisible({ timeout: 400 });
    await page.keyboard.press("Tab");
    await expect(hoverCard.getByRole("link", { name: "View profile" })).toBeFocused();
    await page.waitForTimeout(300);
    await expect(hoverCardContent).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(hoverCardContent).toBeHidden();
    await hoverCardHeading.click();
    await page.keyboard.press("Tab");
    await expect(hoverCardContent).toBeVisible({ timeout: 400 });
    // Escape is handled once the card is placed.
    await expectAnchoredReady(hoverCard);
    await page.keyboard.press("Escape");
    await expect(hoverCardContent).toBeHidden();

    const toast = page.locator('[data-interaction-target="toast"]');
    const toastTrigger = page.locator('[data-interaction-control="toast-trigger"]');
    const toastRoot = toast.locator('[role="status"]');
    await toastTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    const toastViewport = toast.locator('[role="region"]');
    await expect(toastViewport).toHaveAttribute("aria-live", "polite");
    await expect(toastViewport).toHaveAttribute("aria-label", "Notifications");
    await expect(toastRoot).toBeHidden();
    await toastTrigger.click();
    await expect(toastRoot).toBeVisible();
    await expect(toastRoot).toBeHidden({ timeout: 5000 });
    await expect(toast).toHaveAttribute("data-reason", "timeout");
    await toastTrigger.click();
    await expect(toastRoot).toHaveAttribute("data-timer", "running");
    await toastRoot.hover();
    await page.waitForTimeout(2500);
    await expect(toastRoot).toBeVisible();
    await expectAccessibleOpen(page, "open toast");
    await toastTrigger.hover();
    await expect(toastRoot).toBeHidden({ timeout: 5000 });
    await toastTrigger.click();
    await toastRoot.getByRole("button", { name: "Undo" }).click();
    await expect(toastRoot).toBeHidden();
    await expect(toast).toHaveAttribute("data-reason", "action");
    await toastTrigger.click();
    await toastRoot.getByRole("button", { name: "Close notification" }).click();
    await expect(toastRoot).toBeHidden();
    await expect(toast).toHaveAttribute("data-reason", "close");

    const sonner = page.locator('[data-interaction-target="sonner"]');
    const sonnerTrigger = page.locator('[data-interaction-control="sonner-trigger"]');
    const sonnerViewport = sonner.locator('[role="region"]');
    const sonnerToast = sonner.locator('[role="status"]');
    await sonnerTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    await expect(sonnerViewport).toHaveAttribute("aria-live", "polite");
    await expect(sonnerToast).toHaveCount(0);
    await sonnerTrigger.click();
    await expect(sonnerToast).toHaveAttribute("aria-live", "polite");
    await expectAccessibleOpen(page, "open sonner");
    await expect(sonnerToast).toHaveCount(0, { timeout: 5000 });
    await expect(sonner).toHaveAttribute("data-reason", "timeout");
    await sonnerTrigger.click();
    await sonnerToast.getByRole("button", { name: "Close notification" }).click();
    await expect(sonnerToast).toHaveCount(0);
    await expect(sonner).toHaveAttribute("data-reason", "close");

    const alertDialog = page.locator('[data-interaction-target="alert-dialog"]');
    const alertDialogTrigger = page.locator('[data-interaction-control="alert-dialog-trigger"]');
    const alertDialogContent = alertDialog.locator('[role="alertdialog"]');
    const alertDialogCancel = alertDialogContent.getByRole("button", { name: "Cancel" });
    const alertDialogAction = alertDialogContent.getByRole("button", { name: "Delete" });
    await expect(alertDialogContent).toBeHidden();
    await alertDialogTrigger.click();
    // A passed aria-label replaces the title as the name; the description
    // still describes it.
    await expect(alertDialogContent).toHaveAccessibleName("Confirm deletion");
    await expect(alertDialogContent).not.toHaveAttribute("aria-labelledby");
    await expect(alertDialogContent).toHaveAccessibleDescription("This cannot be undone.");
    await expectAccessibleOpen(page, "open alert dialog");
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
    const dialogOverlay = page.locator('[data-interaction-target="dialog"] > [data-state]:not([role])');
    await expect(dialog).toHaveAttribute("data-state", "closed");
    await expect(dialogContent).toBeHidden();
    await dialogTrigger.click();
    await expect(dialogContent).toHaveAccessibleName("Rename project");
    await expect(dialogContent).toHaveAccessibleDescription("Focus stays inside until the dialog closes.");
    // Every id reference on the page, hidden content included, resolves.
    const danglingReferences = await page.evaluate(() =>
      [...document.querySelectorAll("[aria-labelledby], [aria-describedby], [aria-controls]")].flatMap((element) =>
        ["aria-labelledby", "aria-describedby", "aria-controls"].flatMap((name) =>
          (element.getAttribute(name) ?? "")
            .split(/\s+/)
            .filter((id) => id && !document.getElementById(id))
            .map((id) => `${name}=${id}`),
        ),
      ),
    );
    if (danglingReferences.length > 0) {
      throw new Error(`id references point at missing elements: ${danglingReferences.join(", ")}`);
    }
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
    // A press on the overlay outside the content closes the dialog.
    await dialogOverlay.click({ position: { x: 8, y: 8 } });
    await expect(dialogContent).toBeHidden();
    await expect(dialogTrigger).toBeFocused();
    await dialogTrigger.click();
    await expect(dialogContent).toBeVisible();
    await expectReadableText(page, "open dialog");
    await dialogClose.click();
    await expect(dialogContent).toBeHidden();
    await expect(dialogTrigger).toBeFocused();
    // A modal locks page scroll and pads the root for the hidden scrollbar
    // (RFC 0068), and closing restores both.
    await dialogTrigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
    const scrollState = () =>
      page.evaluate(() => ({
        y: Math.round(window.scrollY),
        overflow: document.documentElement.style.overflow,
        padding: parseFloat(getComputedStyle(document.documentElement).paddingRight),
        locks: document.documentElement.dataset.dxuiScrollLocks ?? null,
        scrollbar: window.innerWidth - document.documentElement.clientWidth,
      }));
    const beforeOpen = await scrollState();
    await dialogTrigger.click();
    await expect(dialogInput).toBeFocused();
    await expect.poll(async () => (await scrollState()).locks).toBe("1");
    const whileOpen = await scrollState();
    expect(whileOpen.overflow).toBe("hidden");
    expect(whileOpen.padding).toBe(beforeOpen.padding + beforeOpen.scrollbar);
    await page.mouse.move(10, 10);
    await page.mouse.wheel(0, 600);
    await page.waitForTimeout(300);
    expect((await scrollState()).y).toBe(beforeOpen.y);
    await dialogClose.click();
    await expect(dialogContent).toBeHidden();
    await expect.poll(async () => (await scrollState()).locks).toBe(null);
    const afterClose = await scrollState();
    expect(afterClose.overflow).toBe("");
    expect(afterClose.padding).toBe(beforeOpen.padding);
    expect(afterClose.y).toBe(beforeOpen.y);
    await page.mouse.wheel(0, 200);
    await expect.poll(async () => (await scrollState()).y).toBeGreaterThan(beforeOpen.y);
    // Action parts run the app's onclick (RFC 0053).
    const actionParts = page.locator('[data-interaction-target="action-parts"]');
    const actionPart = (name) => actionParts.getByRole(name === "Fruit" ? "combobox" : "button", { name, exact: true });
    for (const [name, action] of [
      ["Bold", "button-group"],
      ["Clear", "input-group"],
      ["Attach", "attachment-trigger"],
      ["Remove", "attachment-action"],
      ["Fruit", "combobox"],
      ["Copy link", "tooltip"],
      ["Jump to latest", "jump"],
    ]) {
      await actionPart(name).click();
      await expect(actionParts).toHaveAttribute("data-last-action", action);
    }
    // The jump button is a plain button, so its form was never submitted.
    await expect(actionPart("Jump to latest")).toHaveAttribute("type", "button");
    await expect(actionParts).toHaveAttribute("data-submits", "0");
    // A FieldLabel names its input and focuses it on a click.
    const contactEmail = actionParts.getByRole("textbox", { name: "Contact email", exact: true });
    await actionParts.getByText("Contact email", { exact: true }).click();
    await expect(contactEmail).toBeFocused();
    // A press on a disabled part does not call onclick.
    await expect(actionPart("Italic")).toBeDisabled();
    await actionPart("Italic").click({ force: true });
    await expect(actionParts).toHaveAttribute("data-last-action", "jump");

    await expectNoUtilityConflicts(page, "after interactions");
    await expectReadableText(page, "after interactions");
    await expectPhoneWidthLayout(page, "after interactions");
    const refused = (await csp?.violations()) ?? [];
    if (refused.length > 0) {
      throw new Error(`strict CSP: the browser refused:\n${refused.join("\n")}`);
    }
  } catch (error) {
    // A refused script usually surfaces as a later interaction timing out;
    // name what the browser refused alongside it.
    const refused = (await csp?.violations()) ?? [];
    if (refused.length > 0 && !String(error?.message).startsWith("strict CSP")) {
      throw new Error(`${error?.message ?? error}\nstrict CSP: the browser refused:\n${[...new Set(refused)].join("\n")}`);
    }
    throw error;
  } finally {
    await browser.close();
  }
}

try {
  await server.ready();
  await runBrowserAssertions();
  console.log(`runtime interaction verification passed (52 fixtures${strictCsp ? ", strict CSP" : ""})`);
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
