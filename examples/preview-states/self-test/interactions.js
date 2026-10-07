// Interaction self-test for the Desktop and Mobile previews (RFC 0017,
// RFC 0018). Runs inside the app's own WebView through `document::eval` and
// sends one result:
// `{ ok, passed, targets, error }`, where `targets` holds the rendered size of
// each interactive control at defaults. Dispatched events are untrusted, so browser default
// actions (Tab movement, Enter clicking a button) do not run; the scenarios
// exercise the interaction scripts and Rust handlers instead.
const overallTimeoutMs = 60000;
const stepTimeoutMs = 3000;
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const $ = (selector) => document.querySelector(selector);
const describe = (element) => (element ? element.outerHTML.slice(0, 120) : String(element));
const waitFor = async (check, label) => {
  const start = performance.now();
  while (performance.now() - start < stepTimeoutMs) {
    if (check()) return;
    await sleep(20);
  }
  throw new Error(`${label} (focused: ${describe(document.activeElement)})`);
};
const focused = (element, label) => waitFor(() => element !== null && document.activeElement === element, label);
const key = (target, name, init = {}) =>
  target.dispatchEvent(new KeyboardEvent("keydown", { key: name, bubbles: true, cancelable: true, ...init }));
const pressFocused = (name, init) => key(document.activeElement, name, init);
const visible = (element) => element !== null && !element.hidden;
const placed = (element) => visible(element) && getComputedStyle(element).position === "fixed";
const focus = (element) => {
  element.scrollIntoView({ block: "center" });
  element.focus();
};

const scenarios = [
  ["stylesheet", async () => {
    // PreviewSurface links compiled Tailwind (RFC 0049); without it the
    // scenarios below would run on an unstyled page.
    const hidden = $(".sr-only");
    await waitFor(() => hidden !== null && getComputedStyle(hidden).position === "absolute", "the compiled preview stylesheet applies");
  }],
  ["theme", async () => {
    // The header toggle switches the page to the opt-in dark theme (RFC 0050).
    const toggle = $("#preview-theme-toggle");
    const main = $("[data-preview-root]");
    const light = getComputedStyle(main).backgroundColor;
    toggle.click();
    await waitFor(() => getComputedStyle(main).colorScheme === "dark", "the toggle turns on the dark theme");
    await waitFor(() => getComputedStyle(main).backgroundColor !== light, "the dark theme remaps the page surface");
    toggle.click();
    await waitFor(() => getComputedStyle(main).backgroundColor === light, "a second press restores the light theme");
  }],
  ["dialog", async () => {
    const root = $('[data-interaction-target="dialog"]');
    const trigger = root.querySelector('[data-interaction-control="dialog-trigger"]');
    const content = root.querySelector('[role="dialog"]');
    focus(trigger);
    trigger.click();
    await focused(root.querySelector('[data-interaction-control="dialog-input"]'), "dialog focuses its first field");
    pressFocused("Escape");
    await waitFor(() => !visible(content), "Escape closes the dialog");
    await focused(trigger, "dialog returns focus to its trigger");
  }],
  ["popover", async () => {
    const root = $('[data-interaction-target="popover"]');
    const trigger = $('[data-interaction-control="popover-trigger"]');
    const content = root.querySelector("[data-dxui-anchored]");
    focus(trigger);
    trigger.click();
    await waitFor(() => placed(content), "popover is placed with fixed positioning");
    // iOS scrolls the focused trigger into view natively and reports the
    // scroll a few frames later, so placement is checked once it settles.
    const nextToTrigger = () => {
      const triggerBox = trigger.getBoundingClientRect();
      const contentBox = content.getBoundingClientRect();
      return Math.abs(contentBox.top - triggerBox.bottom) <= 16 || Math.abs(triggerBox.top - contentBox.bottom) <= 16;
    };
    try {
      await waitFor(nextToTrigger, "popover is next to its trigger");
    } catch (error) {
      const boxes = { triggerBox: trigger.getBoundingClientRect(), contentBox: content.getBoundingClientRect() };
      throw new Error(`${error.message}: ${JSON.stringify(boxes)}`);
    }
    document.body.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
    await waitFor(() => !visible(content), "an outside press closes the popover");
  }],
  ["select", async () => {
    const root = $('[data-interaction-target="select"]');
    const trigger = $("#interaction-select-trigger");
    const listbox = root.querySelector('[role="listbox"]');
    const highlighted = () => document.getElementById(trigger.getAttribute("aria-activedescendant") || "");
    focus(trigger);
    key(trigger, "ArrowDown");
    await waitFor(() => placed(listbox), "ArrowDown on the trigger opens the listbox");
    await waitFor(() => highlighted()?.dataset.value === "banana", "the selected option starts highlighted");
    key(trigger, "ArrowDown");
    await waitFor(() => highlighted()?.dataset.value === "blueberry", "ArrowDown highlights the next option");
    key(trigger, "Enter");
    await waitFor(() => !visible(listbox), "Enter closes the listbox");
    await waitFor(() => root.dataset.value === "blueberry", "Enter selects the highlighted option");
    await focused(trigger, "focus stays on the select trigger");
  }],
  ["dropdown", async () => {
    const root = $('[data-interaction-target="dropdown"]');
    const trigger = $('[data-interaction-control="dropdown-trigger"]');
    const menu = root.querySelector('[role="menu"]');
    const item = (name) => Array.from(menu.querySelectorAll('[role="menuitem"]')).find((element) => element.textContent === name);
    focus(trigger);
    trigger.click();
    await focused(item("Edit"), "opening focuses the first item");
    pressFocused("ArrowUp");
    await focused(item("Delete"), "ArrowUp wraps to the last item");
    pressFocused("ArrowUp");
    await focused(item("Duplicate"), "ArrowUp skips the disabled item");
    pressFocused("Enter");
    await waitFor(() => !visible(menu), "Enter closes the menu");
    await waitFor(() => root.dataset.action === "duplicate", "Enter runs the item's onclick");
    await focused(trigger, "the menu returns focus to its trigger");
  }],
  ["toast", async () => {
    const root = $('[data-interaction-target="toast"]');
    const trigger = root.querySelector('[data-interaction-control="toast-trigger"]');
    const status = root.querySelector('[role="status"]');
    focus(trigger);
    trigger.click();
    await waitFor(() => visible(status), "the toast opens");
    // Move focus away so the focus pause does not hold the countdown.
    trigger.blur();
    const start = performance.now();
    while (visible(status) && performance.now() - start < 5000) await sleep(50);
    if (visible(status)) throw new Error("the toast countdown does not dismiss it");
    await waitFor(() => root.dataset.reason === "timeout", "the toast reports a timeout dismissal");
  }],
  ["date-picker", async () => {
    const root = $('[data-interaction-target="date-picker"]');
    const trigger = $("#interaction-date-trigger");
    const content = root.querySelector('[role="dialog"]');
    const day = (date) => content.querySelector(`[data-date="${date}"]`);
    const start = root.dataset.value;
    const [year, month, date] = start.split("-").map(Number);
    const next = new Date(Date.UTC(year, month - 1, date + 1));
    const nextIso = next.toISOString().slice(0, 10);
    const later = new Date(Date.UTC(next.getUTCFullYear(), next.getUTCMonth() + 1, next.getUTCDate()));
    const laterIso = later.toISOString().slice(0, 10);
    focus(trigger);
    trigger.click();
    await waitFor(() => placed(content), "the date picker is placed");
    await focused(day(start), "opening focuses the selected day");
    pressFocused("ArrowRight");
    await waitFor(() => day(nextIso) !== null, `the next day ${nextIso} is rendered`);
    await focused(day(nextIso), "ArrowRight moves focus to the next day");
    pressFocused("PageDown");
    await waitFor(() => day(laterIso) !== null, `the next month renders ${laterIso}`);
    await focused(day(laterIso), "PageDown moves focus into the next month");
    pressFocused("Escape");
    await waitFor(() => !visible(content), "Escape closes the date picker");
    await focused(trigger, "the date picker returns focus to its trigger");
  }],
  ["menubar", async () => {
    const trigger = (value) => $(`[data-interaction-target="menubar"] [data-value="${value}"] [data-dxui-menubar-trigger]`);
    const menu = (value) => $(`[data-interaction-target="menubar"] [data-value="${value}"] [role="menu"]`);
    const firstItem = (value) => menu(value).querySelector('[role="menuitem"]');
    await waitFor(() => trigger("file").tabIndex === 0 && trigger("edit").tabIndex === -1, "the triggers form one Tab stop");
    focus(trigger("file"));
    pressFocused("ArrowRight");
    await focused(trigger("edit"), "ArrowRight moves to the next trigger");
    pressFocused("ArrowDown");
    await waitFor(() => placed(menu("edit")), "ArrowDown opens the menu");
    await focused(firstItem("edit"), "the open menu focuses its first item");
    pressFocused("ArrowRight");
    await waitFor(() => placed(menu("view")) && !visible(menu("edit")), "ArrowRight switches to the adjacent menu past the disabled trigger");
    await focused(firstItem("view"), "the switched menu focuses its first item");
    pressFocused("Escape");
    await waitFor(() => !visible(menu("view")), "Escape closes the menu");
    await focused(trigger("view"), "Escape returns focus to the open menu's trigger");
  }],
  ["navigation-menu", async () => {
    const root = $('[data-interaction-target="navigation-menu"]');
    const trigger = Array.from(root.querySelectorAll("button")).find((element) => element.textContent === "Docs");
    const content = root.querySelector('[data-value="docs"] [data-dxui-navigation-content]');
    focus(trigger);
    trigger.click();
    await waitFor(() => visible(content), "a click opens the content");
    trigger.click();
    await waitFor(() => !visible(content), "a second click closes the content");
    key(trigger, "ArrowDown");
    await waitFor(() => visible(content), "ArrowDown opens the content");
    await focused(content.querySelector("a"), "ArrowDown focuses the first link");
    pressFocused("Escape");
    await waitFor(() => !visible(content), "Escape closes the content");
    await focused(trigger, "Escape returns focus to the trigger");
  }],
];

// Target size of every visible interactive control in the interaction
// fixtures, measured before any scenario runs: its box, or the RFC 0078 hit
// area of the control or the label around it, whichever is larger.
const interactiveSelector = [
  "button",
  "a[href]",
  "input:not([type=hidden])",
  "select",
  "textarea",
  "summary",
  '[role="button"]',
  '[role="checkbox"]',
  '[role="radio"]',
  '[role="switch"]',
  '[role="tab"]',
  '[role="slider"]',
  '[role="combobox"]',
  '[role="menuitem"]',
  '[role="option"]',
  '[tabindex]:not([tabindex="-1"])',
].join(", ");
const hitArea = (element) => {
  const after = element ? getComputedStyle(element, "::after") : null;
  if (!after || after.position !== "absolute" || after.content === "none") return [0, 0];
  return [parseFloat(after.width) || 0, parseFloat(after.height) || 0];
};
const measureTargets = () =>
  Array.from(document.querySelectorAll(`[data-interaction-target] :is(${interactiveSelector})`))
    // A tab panel takes focus for its content, not presses, and a link inside
    // a line of text is exempt (WCAG 2.5.8).
    .filter((element) => element.getAttribute("role") !== "tabpanel")
    .filter((element) => !(element.tagName === "A" && getComputedStyle(element).display === "inline"))
    .filter((element) => element.getClientRects().length > 0)
    .map((element) => {
      const box = element.getBoundingClientRect();
      const [ownWidth, ownHeight] = hitArea(element);
      const [labelWidth, labelHeight] = hitArea(element.closest("label"));
      const width = Math.max(box.width, ownWidth, labelWidth);
      const height = Math.max(box.height, ownHeight, labelHeight);
      const name = (element.getAttribute("aria-label") || element.textContent || element.getAttribute("placeholder") || "")
        .trim()
        .replace(/\s+/g, " ")
        .slice(0, 32);
      return {
        target: element.closest("[data-interaction-target]").dataset.interactionTarget,
        control: element.getAttribute("role") || element.tagName.toLowerCase(),
        name,
        width: Math.round(width * 10) / 10,
        height: Math.round(height * 10) / 10,
      };
    });

const run = async () => {
  const targets = measureTargets();
  const passed = [];
  for (const [name, scenario] of scenarios) {
    try {
      await scenario();
    } catch (error) {
      return { ok: false, passed, targets, error: `${name}: ${error instanceof Error ? error.message : error}` };
    }
    passed.push(name);
  }
  return { ok: true, passed, targets, error: null };
};
const timeout = sleep(overallTimeoutMs).then(() => ({
  ok: false,
  passed: [],
  error: `timed out after ${overallTimeoutMs} ms`,
}));
// Let the preview mount and its interaction scripts attach first.
await waitFor(() => document.querySelector("[data-dxui-navigation-menu]") !== null, "the preview mounts");
await sleep(500);
dioxus.send(await Promise.race([run(), timeout]));
