#!/usr/bin/env node
// Builds the Mobile preview for Android and runs its in-app interaction
// self-test in an emulator (RFC 0020). Needs the Android SDK, NDK, and an AVD.
import { spawn, spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

const repoRoot = new URL("..", import.meta.url).pathname;
const packageName = "com.example.DioxusUiMobileDemo";
const activity = `${packageName}/dev.dioxus.main.MainActivity`;
const requestProperty = "debug.dioxus_shadcn.self_test";
const passLine = "mobile interaction verification passed";
const resultTimeoutMs = 180000;
const bootTimeoutMs = 300000;
// A fixed port gives a booted emulator a known serial.
const emulatorPort = 5584;

function fail(message, output = "") {
  if (output) console.error(output.trim());
  console.error(message);
  process.exit(1);
}

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const sdk = process.env.ANDROID_HOME || process.env.ANDROID_SDK_ROOT || join(homedir(), "Library/Android/sdk");
const adb = join(sdk, "platform-tools/adb");
const emulator = join(sdk, "emulator/emulator");
if (!existsSync(adb)) fail(`adb not found at ${adb}; set ANDROID_HOME to the Android SDK`);

function newestNdk() {
  const ndkRoot = join(sdk, "ndk");
  if (!existsSync(ndkRoot)) return undefined;
  const versions = readdirSync(ndkRoot).sort((left, right) => left.localeCompare(right, undefined, { numeric: true }));
  return versions.length ? join(ndkRoot, versions[versions.length - 1]) : undefined;
}

const ndk = process.env.ANDROID_NDK_HOME || newestNdk();
if (!ndk) fail(`no NDK found under ${join(sdk, "ndk")}; install one with the Android SDK Manager`);

// Some installs store the NDK's symbolic links as small text files holding
// the target name, which fails linking with "clang-17: command not found".
const hostTag = process.platform === "darwin" ? "darwin-x86_64" : "linux-x86_64";
const clang = join(ndk, "toolchains/llvm/prebuilt", hostTag, "bin/clang");
if (!existsSync(clang)) fail(`NDK clang not found at ${clang}`);
if (statSync(clang).size < 256) {
  const target = readFileSync(clang, "utf8").trim();
  fail(
    `${clang} is a ${statSync(clang).size}-byte text file naming "${target}" instead of a symbolic link; ` +
      "the NDK install lost its symbolic links. Reinstall the NDK with the Android SDK Manager.",
  );
}

const studioJdk = "/Applications/Android Studio.app/Contents/jbr/Contents/Home";
const env = {
  ...process.env,
  ANDROID_HOME: sdk,
  ANDROID_SDK_ROOT: sdk,
  ANDROID_NDK_HOME: ndk,
  ...(!process.env.JAVA_HOME && existsSync(studioJdk) ? { JAVA_HOME: studioJdk } : {}),
};

function run(command, args, { timeout = 120000 } = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: "utf8",
    env,
    timeout,
    maxBuffer: 64 * 1024 * 1024,
  });
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`;
  if (result.error) fail(`${command} ${args.join(" ")} failed: ${result.error.message}`, output);
  return { status: result.status, output };
}

function adbOn(serial, args, options) {
  const result = run(adb, ["-s", serial, ...args], options);
  if (result.status !== 0) fail(`adb ${args.join(" ")} failed`, result.output);
  return result.output;
}

const build = run(
  "dx",
  ["build", "--android", "--package", "dioxus-ui-mobile-demo", "--target", "aarch64-linux-android"],
  { timeout: 1800000 },
);
const projectPath = build.output.match(/path="([^"]+)"/)?.[1];
const apk = projectPath && join(projectPath, "app/build/outputs/apk/debug/app-debug.apk");
if (build.status !== 0 || !apk || !existsSync(apk)) fail("dx build --android did not produce an APK", build.output);

const runningEmulators = run(adb, ["devices"])
  .output.split(/\r?\n/)
  .map((line) => line.split(/\s+/))
  .filter(([serial, state]) => serial.startsWith("emulator-") && state === "device")
  .map(([serial]) => serial);
let serial = process.env.DIOXUS_UI_ANDROID_SERIAL || runningEmulators[0];
const bootedHere = !serial;

async function bootEmulator() {
  const avds = run(emulator, ["-list-avds"]).output.split(/\r?\n/).map((line) => line.trim()).filter(Boolean);
  const avd = process.env.DIOXUS_UI_ANDROID_AVD || avds[0];
  if (!avd) fail("no Android emulator found; create an AVD in Android Studio");
  console.log(`booting the ${avd} emulator`);
  const child = spawn(
    emulator,
    ["-avd", avd, "-port", String(emulatorPort), "-no-window", "-no-audio", "-no-boot-anim", "-no-snapshot-save"],
    { env, detached: true, stdio: "ignore" },
  );
  child.unref();
  const bootedSerial = `emulator-${emulatorPort}`;
  const start = Date.now();
  while (Date.now() - start < bootTimeoutMs) {
    const booted = run(adb, ["-s", bootedSerial, "shell", "getprop", "sys.boot_completed"]);
    if (booted.status === 0 && booted.output.trim() === "1") return bootedSerial;
    await sleep(2000);
  }
  run(adb, ["-s", bootedSerial, "emu", "kill"]);
  fail(`the ${avd} emulator did not finish booting within ${bootTimeoutMs / 1000} s`);
}

// Rust stdout and stderr reach logcat under the RustStdoutStderr tag.
function resultLines() {
  return adbOn(serial, ["logcat", "-d", "-s", "RustStdoutStderr"])
    .split(/\r?\n/)
    .map((line) => line.split("RustStdoutStderr: ")[1] ?? "")
    .filter((line) => line.includes("interaction verification") || line.includes("passed before"));
}

async function runSelfTest() {
  adbOn(serial, ["install", "-r", apk], { timeout: 300000 });
  adbOn(serial, ["shell", "am", "force-stop", packageName]);
  adbOn(serial, ["shell", "setprop", requestProperty, "1"]);
  try {
    adbOn(serial, ["logcat", "-c"]);
    console.log(`running the self-test on ${serial}`);
    adbOn(serial, ["shell", "am", "start", "-n", activity]);
    const start = Date.now();
    while (Date.now() - start < resultTimeoutMs) {
      if (resultLines().some((line) => line.includes("interaction verification"))) {
        // A failure prints its "passed before" line just after the result.
        await sleep(1000);
        return resultLines();
      }
      await sleep(1000);
    }
    return [];
  } finally {
    // The property lasts until reboot; a later manual launch shows the preview.
    run(adb, ["-s", serial, "shell", "setprop", requestProperty, '""']);
    run(adb, ["-s", serial, "shell", "am", "force-stop", packageName]);
  }
}

if (bootedHere) serial = await bootEmulator();
let lines;
try {
  lines = await runSelfTest();
} finally {
  if (bootedHere) run(adb, ["-s", serial, "emu", "kill"]);
}

if (lines.some((line) => line.includes(passLine))) {
  console.log(lines.join("\n"));
} else {
  fail(
    "android interaction verification did not report success",
    lines.join("\n") || `no result in logcat within ${resultTimeoutMs / 1000} s`,
  );
}
