#!/usr/bin/env node
// Serves examples/fullstack-hydration with `dx serve` and checks RFC 0075:
// three requests write the same element ids, and after the browser hydrates
// the page, the parts that find each other by id work. ArrowRight moves focus
// between tabs only when the browser's roving group id matches the server's,
// and the Select list opens under its trigger only when its anchor id does.
import { launchBrowser, serveDioxusWeb } from "./browser-check-support.mjs";

const server = serveDioxusWeb({ packageName: "dioxus-ui-fullstack-hydration", port: 45242 });

// Attributes that hold a generated id or point at one.
const idAttributes = /\s((?:id|aria-controls|aria-labelledby|data-dxui-[a-z-]+))="(dxui-[^"]+)"/g;

const generatedIds = (html) => [...html.matchAll(idAttributes)].map(([, name, value]) => `${name}=${value}`);

async function checkRequests() {
  const pages = [];
  for (let request = 0; request < 3; request += 1) {
    const response = await fetch(server.url);
    if (!response.ok) {
      throw new Error(`request ${request + 1} answered ${response.status}`);
    }
    pages.push(generatedIds(await response.text()));
  }
  const [first, ...rest] = pages;
  if (!first.some((id) => id.startsWith("data-dxui-roving-group=")) || !first.some((id) => id.startsWith("data-dxui-anchored="))) {
    throw new Error(`the server page has no roving group or anchored id:\n${first.join("\n")}`);
  }
  rest.forEach((ids, index) => {
    if (ids.join("\n") !== first.join("\n")) {
      throw new Error(`request ${index + 2} wrote different ids than request 1:\n${ids.join("\n")}\n---\n${first.join("\n")}`);
    }
  });
  return first.length;
}

async function checkHydratedPage(browser) {
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  try {
    await page.goto(server.url, { waitUntil: "networkidle", timeout: 60000 });

    // Before hydration ArrowRight does nothing, so keep trying until it moves
    // focus or the deadline passes.
    const first = page.locator('[role="tab"]').nth(0);
    const second = page.locator('[role="tab"]').nth(1);
    const deadline = Date.now() + 15000;
    let moved = false;
    while (!moved && Date.now() < deadline) {
      await first.focus();
      await page.keyboard.press("ArrowRight");
      moved = await second.evaluate((element) => element === document.activeElement);
      if (!moved) await page.waitForTimeout(250);
    }
    if (!moved) {
      throw new Error("after hydration, ArrowRight on the first tab did not move focus to the second");
    }
    if ((await second.getAttribute("aria-selected")) !== "true") {
      throw new Error("after hydration, ArrowRight moved focus but did not select the second tab");
    }

    const trigger = page.locator('[role="combobox"]');
    await trigger.click();
    const list = page.locator(`#${await trigger.getAttribute("aria-controls")}`);
    await list.waitFor({ state: "visible", timeout: 5000 });
    const triggerBox = await trigger.boundingBox();
    // The anchoring script places the list after it finds the trigger by id.
    let listBox = await list.boundingBox();
    const placeDeadline = Date.now() + 5000;
    while (listBox && triggerBox && listBox.y < triggerBox.y + triggerBox.height && Date.now() < placeDeadline) {
      await page.waitForTimeout(100);
      listBox = await list.boundingBox();
    }
    if (!listBox || !triggerBox || listBox.y < triggerBox.y + triggerBox.height) {
      throw new Error(`the Select list is not placed under its trigger: list ${JSON.stringify(listBox)}, trigger ${JSON.stringify(triggerBox)}`);
    }

    if (errors.length > 0) {
      throw new Error(`the hydrated page logged errors:\n${errors.join("\n")}`);
    }
  } finally {
    await page.close();
  }
}

try {
  await server.ready();
  const idCount = await checkRequests();
  const browser = await launchBrowser("scripts/fullstack-hydration-verify.mjs");
  try {
    await checkHydratedPage(browser);
  } finally {
    await browser.close();
  }
  console.log(`fullstack hydration verification passed (${idCount} ids on 3 requests, tabs and select after hydration)`);
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
} finally {
  await server.stop();
}
