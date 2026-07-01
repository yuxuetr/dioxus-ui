# Browser Automation Dependency Strategy

This document defines the M42 dependency strategy for future browser smoke
tests.

Status: Planned in M42.1.

## Decision

Add minimal Node package metadata for browser automation only if it stays
explicitly opt-in. The repository should not make browser downloads part of the
default Rust, source-copy, or release gates.

The preferred path is:

1. add `package.json` with explicit scripts for existing Node gates
2. add Playwright as a development dependency only when a lockfile can be
   generated and reviewed
3. keep browser installation as a separate command
4. add browser-rendered smoke scripts only after dependencies and browser
   binaries are deterministic

This keeps the current contributor path intact while making future browser
automation discoverable.

## Options

| Option | Pros | Cons | M42 Position |
| --- | --- | --- | --- |
| Playwright-managed browsers | Portable browser version, predictable APIs, screenshots and viewport support. | Requires npm dependency and browser download; network-heavy install. | Preferred future path, opt-in only. |
| External Chrome contract | Avoids Playwright browser download if Chrome is already installed. | Platform-specific paths, version drift, local permission failures. | Useful for local experiments, not enough for release gate. |
| MCP or Codex browser only | No repository dependency churn. | Not portable for contributors or CI; tool-specific. | Manual/local verification only. |
| No browser dependency | Keeps repo minimal. | Blocks repeatable mobile browser smoke. | Current state, acceptable until M42 decides metadata. |

## Package Metadata Boundary

If M42 adds `package.json`, it should:

- use ESM scripts
- expose existing script names without replacing shell commands
- keep browser automation commands separate from default validation
- avoid postinstall browser downloads
- avoid adding generated browser artifacts

Suggested script names:

```json
{
  "scripts": {
    "verify:web-preview": "node scripts/web-preview-verify.mjs",
    "verify:desktop-preview": "node scripts/desktop-preview-verify.mjs",
    "verify:mobile-web-profile": "node scripts/mobile-web-profile-verify.mjs",
    "verify:examples": "scripts/example-smoke.sh"
  }
}
```

Browser smoke scripts should be added later, after the dependency and browser
binary installation path is committed and tested.

## Install Policy

Future browser automation should document two separate install steps:

```bash
npm install
npx playwright install chromium
```

The first installs JavaScript dependencies. The second downloads browser
binaries and must remain opt-in because it is network-heavy and platform
specific.

If this repository later uses a package manager lockfile, the lockfile should be
committed with the package metadata change so dependency versions remain
reviewable.

## Release Gate Policy

Default release gates should remain:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
scripts/example-smoke.sh
node scripts/web-preview-verify.mjs
node scripts/mobile-web-profile-verify.mjs
node scripts/desktop-preview-verify.mjs
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```

Browser-rendered smoke should stay outside release gates until:

- the repository has reviewed package metadata and lockfile state
- the browser install command is documented
- CI or local release machines can run the browser consistently
- screenshots and temporary browser profiles are ignored by Git

## Native Mobile Boundary

Adding Playwright or package metadata does not change the Mobile support claim.
Playwright mobile profiles verify browser rendering at a mobile viewport. They
do not verify native iOS or Android WebView behavior, software keyboard changes,
safe-area insets, mobile assistive technology, or native gesture arbitration.
