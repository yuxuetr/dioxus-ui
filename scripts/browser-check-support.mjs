// Shared pieces of the browser checks: serving a Dioxus Web package with
// `dx serve`, launching Chromium, and measuring text contrast.
import { request } from "node:http";
import { once } from "node:events";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { chromium } from "@playwright/test";

const repoRoot = new URL("..", import.meta.url).pathname;
const host = "127.0.0.1";
const installHint = "npx playwright install chromium";
const executablePath = process.env.DIOXUS_UI_BROWSER_EXECUTABLE;
const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");

// Launches headless Chromium, or the browser DIOXUS_UI_BROWSER_EXECUTABLE
// names, with an install hint when Playwright's own build is missing.
export async function launchBrowser(scriptName) {
  if (executablePath && !existsSync(executablePath)) {
    throw new Error(`DIOXUS_UI_BROWSER_EXECUTABLE does not exist: ${executablePath}`);
  }
  try {
    return await chromium.launch(executablePath ? { headless: true, executablePath } : { headless: true });
  } catch (error) {
    if (!executablePath && String(error?.message ?? error).includes("Executable doesn't exist")) {
      throw new Error(`Playwright Chromium is not installed. Run \`${installHint}\` before node ${scriptName}.`);
    }
    throw error;
  }
}

// Before its first build, `dx serve` answers 200 with a placeholder page
// ("dx is not serving a web app"); only the built app loads a bundle from
// /wasm/, so readiness waits for that.
const servesApp = (url) =>
  new Promise((resolve) => {
    const req = request(url, { method: "GET", timeout: 1000 }, (res) => {
      let body = "";
      res.setEncoding("utf8");
      res.on("data", (chunk) => {
        body += chunk;
      });
      res.on("end", () => resolve(res.statusCode === 200 && body.includes("/wasm/")));
    });
    req.on("timeout", () => {
      req.destroy();
      resolve(false);
    });
    req.on("error", () => resolve(false));
    req.end();
  });

// dx 0.8 hot-patches unless told `--hot-patch false`, and its fat-binary link
// fails on this workspace; dx 0.7 takes `--hot-patch` as a bare flag that is
// off by default and rejects a value.
function hotPatchOffArgs() {
  const version = spawnSync("dx", ["--version"], { encoding: "utf8" });
  const match = /(\d+)\.(\d+)\./.exec(version.stdout ?? "");
  if (version.status !== 0 || !match) {
    throw new Error(`could not read the dx version: ${version.error?.message ?? version.stderr ?? ""}`);
  }
  const [major, minor] = [Number(match[1]), Number(match[2])];
  return major > 0 || minor >= 8 ? ["--hot-patch", "false"] : [];
}

// Serves one Dioxus Web package without hot reload or watching. `ready()`
// resolves once the server serves the built app; `stop()` shuts it down.
export function serveDioxusWeb({ packageName, bin, port }) {
  const url = `http://${host}:${port}`;
  const args = ["serve", "--web", "--package", packageName];
  if (bin) {
    args.push("--bin", bin);
  }
  args.push(
    "--port", String(port), "--addr", host, "--open", "false",
    "--hot-reload", "false", "--watch", "false", "--interactive", "false",
    ...hotPatchOffArgs(),
  );
  let output = "";
  const server = spawn("dx", args, { cwd: repoRoot, stdio: ["ignore", "pipe", "pipe"] });
  server.stdout.on("data", (chunk) => {
    output += chunk.toString();
  });
  server.stderr.on("data", (chunk) => {
    output += chunk.toString();
  });
  const exited = () => server.exitCode !== null || server.signalCode !== null;

  return {
    url,
    async ready() {
      const startedAt = Date.now();
      // A cold fullstack build (server and wasm) took about 105 s on the CI
      // runner and once passed 120 s; a failed build still ends the wait at once.
      while (Date.now() - startedAt < 300000) {
        if (exited()) {
          throw new Error(`dx serve exited before ${packageName} became ready.\n${output}`);
        }
        // dx keeps serving the placeholder after a failed build.
        if (output.includes("Build failed")) {
          throw new Error(`dx serve could not build ${packageName}.\n${output}`);
        }
        if (await servesApp(url)) {
          return;
        }
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }
      throw new Error(`Timed out waiting for ${url}.\n${output}`);
    },
    async stop() {
      if (exited()) {
        return;
      }
      server.kill("SIGINT");
      const timeout = setTimeout(() => {
        if (!exited()) {
          server.kill("SIGTERM");
        }
      }, 3000);
      try {
        await once(server, "exit");
      } finally {
        clearTimeout(timeout);
      }
    },
  };
}

// Adds or removes the opt-in `dark` class on the root element and waits for
// the components' color transitions to settle. A transition can be cancelled
// and restarted mid-way, so it waits until no transition is running.
export async function setDarkTheme(page, dark) {
  await page.evaluate(async (dark) => {
    const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));
    const running = () =>
      document
        .getAnimations()
        .filter((animation) => animation instanceof CSSTransition && animation.playState === "running");
    document.documentElement.classList.toggle("dark", dark);
    for (let attempt = 0; attempt < 20; attempt += 1) {
      await frame();
      const transitions = running();
      if (transitions.length === 0) {
        return;
      }
      await Promise.all(transitions.map((animation) => animation.finished.catch(() => {})));
    }
  }, dark);
}

// Lists visible text whose color falls below the WCAG AA contrast minimum
// against its composited background: 4.5:1, or 3:1 for large text. Disabled
// and faded elements are exempt. Runs in the page.
export function lowContrastText() {
  const canvas = document.createElement("canvas");
  canvas.width = 1;
  canvas.height = 1;
  const context = canvas.getContext("2d", { willReadFrequently: true });
  // The canvas converts oklch and color-mix values to sRGB.
  const rgba = (color) => {
    context.clearRect(0, 0, 1, 1);
    context.fillStyle = color;
    context.fillRect(0, 0, 1, 1);
    const [red, green, blue, alpha] = context.getImageData(0, 0, 1, 1).data;
    return [red, green, blue, alpha / 255];
  };
  const over = (top, bottom) => [0, 1, 2].map((i) => top[i] * top[3] + bottom[i] * (1 - top[3])).concat(1);
  const luminance = (color) => {
    const [red, green, blue] = color.slice(0, 3).map((channel) => {
      const value = channel / 255;
      return value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
    });
    return 0.2126 * red + 0.7152 * green + 0.0722 * blue;
  };
  const backgroundOf = (element) => {
    const layers = [];
    for (let node = element; node; node = node.parentElement) {
      const layer = rgba(getComputedStyle(node).backgroundColor);
      if (layer[3] > 0) {
        layers.push(layer);
      }
      if (layer[3] === 1) {
        break;
      }
    }
    return layers.reverse().reduce((below, layer) => over(layer, below), [255, 255, 255, 1]);
  };
  const failures = [];
  for (const element of document.querySelectorAll("body *")) {
    const hasText = [...element.childNodes].some((node) => node.nodeType === Node.TEXT_NODE && node.textContent.trim());
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    if (!hasText || rect.width <= 1 || rect.height <= 1 || style.visibility === "hidden") {
      continue;
    }
    if (element.closest("[disabled], [aria-disabled='true'], [data-disabled='true']")) {
      continue;
    }
    let faded = false;
    for (let node = element; node; node = node.parentElement) {
      faded ||= Number(getComputedStyle(node).opacity) < 1;
    }
    if (faded) {
      continue;
    }
    const background = backgroundOf(element);
    const [lighter, darker] = [luminance(over(rgba(style.color), background)), luminance(background)].sort(
      (a, b) => b - a,
    );
    const ratio = (lighter + 0.05) / (darker + 0.05);
    const size = Number.parseFloat(style.fontSize);
    const minimum = size >= 24 || (size >= 18.66 && Number(style.fontWeight) >= 700) ? 3 : 4.5;
    if (ratio < minimum) {
      failures.push(`"${element.textContent.trim().slice(0, 40)}" ${ratio.toFixed(2)}:1 (${element.className})`);
    }
  }
  return failures;
}

// Runs axe-core on the page with the WCAG 2.1 A and AA and best-practice
// rules (RFC 0054) and lists each violation with its first element.
export async function accessibilityViolations(page, { disabledRules = [] } = {}) {
  if (!(await page.evaluate(() => "axe" in window))) {
    await page.addScriptTag({ content: axeSource });
  }
  return page.evaluate(async (disabled) => {
    const result = await window.axe.run(document, {
      runOnly: { type: "tag", values: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "best-practice"] },
      rules: Object.fromEntries(disabled.map((id) => [id, { enabled: false }])),
    });
    return result.violations.map(
      (violation) => `${violation.id} (${violation.impact}): ${violation.help}: ${violation.nodes[0]?.html.slice(0, 160)}`,
    );
  }, disabledRules);
}
