# Browser Artifact Policy Metadata

M94 defines the read-only contract for browser smoke artifacts. The goal is to
keep CI browser documentation, workflow template upload behavior, RFC policy,
ignore rules, and repository hygiene aligned before any browser workflow becomes
active.

This gate protects the current policy:

- normal browser smoke uploads are screenshot PNG files only
- screenshot artifacts use `dioxus-ui-mobile-browser-preview-*.png`
- rendered component DOM verification writes no screenshots or traces by
  default
- browser profiles, Playwright caches, Rust target directories, and temporary
  preview server output stay outside the normal upload path
- screenshot outputs stay ignored by Git
- `.github/workflows/browser-smoke.yml` remains absent until activation is
  reviewed separately

## Scope

The metadata gate should verify committed source and documentation only.

In scope:

- `docs/ci-browser-smoke.md` artifact guidance
- `docs/ci-browser-workflow-template.md` upload-artifact configuration
- `docs/rfcs/0009-ci-browser-workflow-activation.md` artifact policy
- `.gitignore` screenshot and browser automation patterns
- `scripts/repo-hygiene-verify.mjs` tracked-artifact boundaries
- `package.json` release wiring

Out of scope:

- launching Playwright
- starting `dx serve`
- creating or uploading artifacts
- tracing rendered component DOM verification
- deleting local browser profiles, caches, or screenshots
- enforcing remote artifact retention settings
- validating screenshot pixels or PNG metadata

## Expected Check

The new verifier should fail when the committed policy drifts. Examples include:

- the workflow template uploads a broad directory instead of screenshot PNG files
- CI docs imply browser profiles or Playwright caches are normal artifacts
- RFC 0009 omits the screenshot-only artifact policy
- `.gitignore` stops ignoring browser screenshot outputs
- release verification stops running the artifact policy metadata gate

The check should stay deterministic and offline. It should inspect text and
package metadata, then report actionable missing fragments.

## Relationship To Existing Gates

`verify:gitignore` checks local ignore policy metadata. `verify:repo-hygiene`
checks tracked forbidden files and the inactive workflow boundary.
`verify:ci-workflow-template` checks the copyable workflow template shape. M94
adds the cross-document policy layer that keeps those gates consistent around
artifact uploads and retention guidance.
