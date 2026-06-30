#!/usr/bin/env node
import { execFileSync } from "node:child_process";

const output = execFileSync(
  "cargo",
  ["run", "-q", "-p", "dioxus-ui-runtime-web-verification"],
  {
    cwd: new URL("..", import.meta.url),
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  },
);

const requiredFragments = [
  "testid=runtime-focus-panel",
  "testid=runtime-focus-initial",
  "testid=runtime-focus-trap",
  "testid=runtime-focus-return",
  "testid=runtime-focus-escape-close",
  "testid=runtime-portal-panel",
  "testid=runtime-portal-inline",
  "testid=runtime-portal-body",
  "testid=runtime-portal-named-target",
  "testid=runtime-portal-missing-target",
  "testid=runtime-timer-panel",
  "testid=runtime-timer-schedule",
  "testid=runtime-timer-cancel",
  "testid=runtime-timer-disabled",
  "testid=runtime-timer-cleanup",
  "testid=runtime-live-region-panel",
  "testid=runtime-live-region-polite",
  "testid=runtime-live-region-assertive",
  "testid=runtime-live-region-duplicate",
  "testid=runtime-live-region-empty",
  "testid=runtime-measurement-panel",
  "testid=runtime-measurement-node",
  "testid=runtime-measurement-viewport",
  "testid=runtime-measurement-scroll-resize",
  "testid=runtime-measurement-missing",
  "testid=runtime-scroll-command-panel",
  "testid=runtime-scroll-sticky-bottom",
  "testid=runtime-scroll-hold-position",
  "testid=runtime-scroll-unread-marker",
  "testid=runtime-scroll-jump-latest",
  "testid=runtime-scroll-focus-preserved",
  "testid=runtime-pointer-panel",
  "testid=runtime-pointer-start",
  "testid=runtime-pointer-move",
  "testid=runtime-pointer-end",
  "testid=runtime-pointer-cancel",
  "testid=runtime-pointer-capture-release",
  "testid=runtime-gesture-panel",
  "testid=runtime-gesture-next",
  "testid=runtime-gesture-previous",
  "testid=runtime-gesture-cancel",
  "testid=runtime-gesture-native-scroll",
];

const missing = requiredFragments.filter((fragment) => !output.includes(fragment));

if (missing.length > 0) {
  console.error("runtime web verification output is missing required assertions:");
  for (const fragment of missing) {
    console.error(`- ${fragment}`);
  }
  process.exit(1);
}

const requiredPendingAssertions = [
  "focus_escape testid=runtime-focus-escape-close",
  "timer_cleanup testid=runtime-timer-cleanup",
  "scroll_focus_preserved testid=runtime-scroll-focus-preserved",
  "gesture_native_scroll testid=runtime-gesture-native-scroll",
];

const outputLines = output.split(/\r?\n/);
const missingPending = requiredPendingAssertions.filter((fragment) => {
  return !outputLines.some((line) => {
    return line.includes(fragment) && line.includes("pending-browser-assertion");
  });
});

if (missingPending.length > 0) {
  console.error("runtime web verification output is missing pending browser assertion markers:");
  for (const fragment of missingPending) {
    console.error(`- ${fragment}`);
  }
  process.exit(1);
}

console.log("runtime web verification browser assertion prerequisites passed");
