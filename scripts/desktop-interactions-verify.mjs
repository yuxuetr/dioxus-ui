#!/usr/bin/env node
// Builds the Desktop preview and runs its in-app interaction self-test
// (RFC 0017). Opens a window, so it needs a GUI session.
import { spawn } from "node:child_process";

const repoRoot = new URL("..", import.meta.url).pathname;
const timeoutMs = 600000;

const child = spawn(
  "cargo",
  ["run", "--quiet", "--package", "dioxus-ui-desktop-demo", "--bin", "preview"],
  {
    cwd: repoRoot,
    stdio: "inherit",
    env: { ...process.env, DIOXUS_UI_DESKTOP_SELF_TEST: "1" },
  },
);

const timer = setTimeout(() => {
  console.error(`desktop interaction verification timed out after ${timeoutMs / 1000} s`);
  child.kill("SIGTERM");
}, timeoutMs);

child.on("exit", (code, signal) => {
  clearTimeout(timer);
  if (signal) {
    console.error(`desktop preview exited on ${signal}`);
    process.exit(1);
  }
  process.exit(code ?? 1);
});
