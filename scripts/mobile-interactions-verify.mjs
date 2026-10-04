#!/usr/bin/env node
// Builds the Mobile preview for the iOS Simulator and runs its in-app
// interaction self-test (RFC 0018). Needs Xcode with an iOS simulator runtime.
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";

const repoRoot = new URL("..", import.meta.url).pathname;
const xcodeDeveloperDir = "/Applications/Xcode.app/Contents/Developer";
const passLine = "mobile interaction verification passed";

function fail(message, output = "") {
  if (output) console.error(output.trim());
  console.error(message);
  process.exit(1);
}

// simctl needs full Xcode; use it without changing the system xcode-select.
function developerDir() {
  if (process.env.DEVELOPER_DIR) return process.env.DEVELOPER_DIR;
  const selected = spawnSync("xcode-select", ["-p"], { encoding: "utf8" });
  const commandLineTools = (selected.stdout ?? "").includes("CommandLineTools");
  return commandLineTools && existsSync(xcodeDeveloperDir) ? xcodeDeveloperDir : undefined;
}

const env = { ...process.env };
const selectedDeveloperDir = developerDir();
if (selectedDeveloperDir) env.DEVELOPER_DIR = selectedDeveloperDir;

function run(command, args, { timeout = 120000, extraEnv = {} } = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: "utf8",
    env: { ...env, ...extraEnv },
    timeout,
    maxBuffer: 64 * 1024 * 1024,
  });
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`;
  if (result.error) fail(`${command} ${args.join(" ")} failed: ${result.error.message}`, output);
  return { status: result.status, output };
}

function simctl(args, options) {
  const result = run("xcrun", ["simctl", ...args], options);
  if (result.status !== 0) fail(`xcrun simctl ${args[0]} failed`, result.output);
  return result.output;
}

const build = run("dx", ["build", "--ios", "--package", "dioxus-ui-mobile-demo"], { timeout: 1800000 });
const appPath = build.output.match(/path="([^"]+\.app)"/)?.[1];
if (build.status !== 0 || !appPath) fail("dx build --ios did not produce an app bundle", build.output);

const bundleId = run("/usr/libexec/PlistBuddy", ["-c", "Print CFBundleIdentifier", `${appPath}/Info.plist`]).output.trim();

// Dioxus 0.7 apps do not adopt the UIScene lifecycle, and iOS 27 stops them at
// launch (UIApplicationEvaluateRuntimeIssueForNoSceneLifecycleAdoption), so
// the default device comes from iOS 26 or older. DIOXUS_UI_IOS_SIMULATOR (a
// device name or UDID) picks any device, for example to recheck iOS 27.
const newestSupportedMajor = 26;
const requestedDevice = process.env.DIOXUS_UI_IOS_SIMULATOR;
const runtimes = JSON.parse(simctl(["list", "devices", "available", "--json"])).devices;
const version = (runtime) => (runtime.match(/iOS-(\d+)-(\d+)/) ?? []).slice(1).map(Number);
const supported = (runtime) => requestedDevice || (version(runtime)[0] ?? 0) <= newestSupportedMajor;
const phones = Object.entries(runtimes)
  .filter(([runtime]) => runtime.includes("iOS") && supported(runtime))
  .sort(([left], [right]) => {
    const [leftMajor = 0, leftMinor = 0] = version(left);
    const [rightMajor = 0, rightMinor = 0] = version(right);
    return rightMajor - leftMajor || rightMinor - leftMinor;
  })
  .flatMap(([, devices]) => devices);
const device = requestedDevice
  ? phones.find((phone) => phone.udid === requestedDevice || phone.name === requestedDevice)
  : (phones.find((phone) => phone.state === "Booted" && phone.name.startsWith("iPhone")) ??
    phones.find((phone) => phone.name.startsWith("iPhone")));
if (!device) {
  fail(
    requestedDevice
      ? `no available simulator named or identified by ${requestedDevice}`
      : `no available iPhone simulator on iOS ${newestSupportedMajor} or older; install one in Xcode`,
  );
}

const bootedHere = device.state !== "Booted";
if (bootedHere) simctl(["boot", device.udid]);

let launch;
try {
  simctl(["bootstatus", device.udid, "-b"], { timeout: 300000 });
  simctl(["install", device.udid, appPath]);
  console.log(`running the self-test on ${device.name} (${device.udid})`);
  launch = run("xcrun", ["simctl", "launch", "--console-pty", "--terminate-running-process", device.udid, bundleId], {
    timeout: 180000,
    extraEnv: { SIMCTL_CHILD_DIOXUS_UI_MOBILE_SELF_TEST: "1" },
  });
} finally {
  if (bootedHere) run("xcrun", ["simctl", "shutdown", device.udid]);
}

// simctl does not report the app's exit status, so the printed line decides.
const lines = launch.output.split(/\r?\n/).filter((line) => line.includes("interaction verification") || line.includes("passed before"));
if (lines.some((line) => line.includes(passLine))) {
  console.log(lines.join("\n"));
} else {
  fail("mobile interaction verification did not report success", lines.join("\n") || launch.output);
}
