# RFC 0020: Android Interaction Verification

- Status: Accepted
- Created: 2026-10-04

## Summary

Run the Mobile preview's interaction self-test (RFC 0018) in an Android
emulator with `npm run verify:android-interactions`. The command:

1. builds the APK;
2. selects or boots an emulator;
3. installs and launches the app with a self-test request;
4. reads the result from logcat.

## Current State

As of M144:

- `npm run verify:mobile-interactions` runs the eight shared interaction
  scenarios in the iOS Simulator only. Android was blocked because the Rust
  Android target was not installed. It is installed now.
- Android renders the preview in the system WebView (Chromium). iOS uses
  WKWebView. The interaction scripts are the same `document::eval` code on
  both.
- A probe built `examples/mobile-demo` with
  `dx build --android --target aarch64-linux-android` and ran it in an
  API 36.1 arm64 emulator. All eight scenarios passed. The app printed
  `mobile interaction verification passed (8 scenarios: ...)`, which logcat
  showed under the `RustStdoutStderr` tag.
- The probe found the local NDK 26.1 install broken. Its 23 symbolic links
  (for example `bin/clang`, which should point to `clang-17`) were stored as
  small text files holding the target name, so linking failed with
  `clang-17: command not found`. The probe built from a scratch copy whose
  links were restored.

## Decision

### Request Channel

`adb shell am start` passes no environment to the app, so the iOS
`SIMCTL_CHILD_` approach has no Android equivalent.

- The command sets the `debug.dioxus_shadcn.self_test` system property to `1`
  before launch.
- On Android, the app runs `getprop debug.dioxus_shadcn.self_test` once at
  startup and runs the self-test when the value is `1`.
- The shell user can set `debug.*` properties without root.
- The property lasts until reboot, so the command clears it after the run,
  whether the run passed or failed. A manual launch afterwards shows the
  normal preview.

Other platforms keep reading `DIOXUS_UI_MOBILE_SELF_TEST`. Intent extras were
rejected because reading them needs JNI calls into the activity, which is more
code than one `getprop` call.

### Result Channel

The app prints the same result line as on iOS. On Android, Rust stdout and
stderr reach logcat under `RustStdoutStderr`.

The command:

- clears logcat before launch;
- polls `adb logcat -d -s RustStdoutStderr` for a result line;
- passes only on `mobile interaction verification passed`;
- fails on the failure line or after 180 s without a result.

The app's exit status is not used. After the result is printed,
`process::exit` can log `FORTIFY: pthread_mutex_lock called on a destroyed
mutex` while WebView threads shut down. That happens after the result line
and does not affect it.

### Toolchain And Device

- The SDK is `ANDROID_HOME`, then `ANDROID_SDK_ROOT`, then
  `~/Library/Android/sdk`.
- The NDK is `ANDROID_NDK_HOME`, or the newest version under `<sdk>/ndk`.
- `JAVA_HOME` defaults to the JDK bundled with Android Studio when it is not
  set.
- These are passed to `dx` through the environment; no system setting changes.
- Before building, the command checks that the NDK's `bin/clang` is not a
  small text file. A broken install fails with a message that names the file
  and asks for the NDK to be reinstalled. Repairing a local install is the
  user's decision, not the command's.
- The APK is built for `aarch64-linux-android`, matching arm64 emulator images
  on Apple silicon.
- The device is `DIOXUS_UI_ANDROID_SERIAL` when set, otherwise the first
  running emulator.
- With no emulator running, the command boots `DIOXUS_UI_ANDROID_AVD`, or the
  first AVD from `emulator -list-avds`. It boots headless without saving a
  snapshot, waits for `sys.boot_completed`, and stops the emulator afterwards.

## Scope

In scope:

- the eight RFC 0018 scenarios in an Android emulator
- the property request channel and logcat result channel
- emulator selection, boot, and shutdown
- a broken-NDK check before building

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Physical devices | `DIOXUS_UI_ANDROID_SERIAL` could select one, but none was available to verify | A device run is needed for a release |
| x86_64 emulators | The APK targets arm64 only | A contributor runs the command on an x86_64 host |
| Touch gestures, the software keyboard, safe areas | The scenarios use dispatched events, as on iOS | Touch gesture behavior is designed |
| CI activation | The command needs the Android SDK, NDK, an AVD, and minutes of build time | CI gains an Android emulator runner |
| Repairing a broken NDK install | The fix belongs to the user's SDK, not this repository | Never; the command only reports it |

## Verification

- The command passes on an API 36.1 arm64 emulator.
- Reverse checks: it fails when an interaction path is broken, when the app
  never reports a result, and when the NDK has flattened symbolic links.
- The command stays out of `npm run verify:release`, like the Desktop and iOS
  self-tests.
