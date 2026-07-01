# Playwright Dependency Plan

This document defines the M43 plan for introducing Playwright as an optional
browser automation dependency.

Status: Planned in M43.1.

## Decision

Use npm and commit `package-lock.json` if Playwright is added. The repository
already has minimal `package.json` metadata, so npm is the lowest-churn package
manager choice for this milestone.

The preferred package is `@playwright/test` as a development dependency because
it provides:

- the Playwright browser API
- a stable CLI for future test scripts
- standard browser installation commands
- conventional lockfile behavior under npm

Playwright browser binaries must not be installed as a package install side
effect. Browser installation remains an explicit command:

```bash
npx playwright install chromium
```

## Dependency Boundary

M43 may add:

- `devDependencies["@playwright/test"]`
- `package-lock.json`
- documentation for dependency and browser installation

M43 should not add:

- browser-rendered smoke scripts as default release gates
- generated browser binaries
- screenshots or temporary browser profiles
- native Mobile support claims
- postinstall hooks that download browsers automatically

## Offline Behavior

After dependencies are installed, these commands should still work without
Playwright browser binaries:

```bash
npm run verify:smoke
node scripts/web-preview-verify.mjs
node scripts/mobile-web-profile-verify.mjs
node scripts/desktop-preview-verify.mjs
```

Future browser-rendered scripts should fail with clear instructions when browser
binaries are missing, rather than triggering an implicit download.

## Future Browser Smoke Shape

The first future browser-rendered smoke should be opt-in, for example:

```bash
npm run verify:mobile-browser
```

That command should:

1. start `dx serve` on a fixed localhost port
2. wait for the preview URL
3. launch Playwright Chromium with a mobile viewport
4. assert the M41 selectors
5. close the browser
6. stop the server

It should remain outside default release gates until local and CI stability are
proven.

## Native Mobile Boundary

Playwright mobile browser profiles do not verify native Dioxus Mobile behavior,
software keyboard changes, safe-area inset measurement, mobile assistive
technology, or native gesture arbitration.
