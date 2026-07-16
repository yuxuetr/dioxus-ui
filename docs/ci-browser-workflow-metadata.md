# CI Browser Workflow Template Metadata Gate

This document defines the M93 plan for keeping the documented CI browser
workflow template aligned with the CI browser guide and RFC 0009 activation
policy.

Status: Planned in M93.1.

## Problem

The repository intentionally keeps the mobile browser smoke workflow as
documentation. The template is copyable, but `.github/workflows/browser-smoke.yml`
must not be committed until maintainers explicitly activate the workflow.

The template, CI guide, RFC, package scripts, and repository hygiene checks all
describe the same activation boundary. M93 should add a read-only metadata gate
so those documents do not drift while the actual workflow remains inactive.

## Contract

The CI workflow template metadata gate should verify:

- `package.json` exposes `verify:ci-workflow-template`.
- `.github/workflows/browser-smoke.yml` is absent.
- The workflow template keeps `workflow_dispatch`.
- The workflow template keeps `continue-on-error: true`.
- The workflow template keeps `permissions: contents: read`.
- The workflow template keeps `timeout-minutes: 30`.
- The workflow template installs Node dependencies with `npm ci`.
- The workflow template runs deterministic gates before browser smoke.
- The workflow template installs Playwright Chromium.
- The workflow template runs `npm run verify:mobile-browser`.
- The workflow template uploads only `dioxus-ui-mobile-browser-preview-*.png`.
- The external Chrome variant keeps `DIOXUS_UI_BROWSER_EXECUTABLE`.
- RFC 0009 keeps phased activation and required-gate boundaries documented.

## Non-goals

- no active workflow file creation
- no GitHub Actions execution
- no browser installation
- no screenshot capture
- no artifact upload
- no promotion of browser smoke to required merge checks

## Planned Command

M93 should add:

```bash
npm run verify:ci-workflow-template
```

The command should be deterministic and read-only. It should be included in:

```bash
npm run verify:release
```
