# RFC 0009: CI Browser Workflow Activation

- Status: Draft
- Created: 2026-07-02

## Summary

Activate the mobile browser smoke workflow in phases. Start with the documented
template only, then move to a manual non-blocking workflow, then scheduled or
pull-request non-blocking runs, and only consider a required merge gate after
repeated stable runs.

This RFC does not activate a workflow. It defines the decision policy for when
`docs/ci-browser-workflow-template.md` can be copied into `.github/workflows/`.

## Motivation

The repository now has deterministic Rust and structural preview gates plus an
opt-in Playwright mobile browser smoke. The browser smoke catches failures that
source-level checks cannot see, such as a rendered blank page, missing DOM
selectors, invalid chart layout boxes, and invalid screenshot artifacts.

Browser jobs are also more fragile than deterministic local gates. They can
fail because of browser downloads, runner network policy, localhost binding,
Dioxus CLI install time, or browser launch behavior. Activating CI without a
rollout policy would risk noisy failures and unclear ownership.

## Decision

Use a phased activation model.

### Phase 0: Documentation Only

Current state.

- keep `docs/ci-browser-smoke.md`
- keep `docs/ci-browser-workflow-template.md`
- do not add `.github/workflows`
- keep browser smoke out of required release gates

### Phase 1: Manual Non-blocking Workflow

Allowed after maintainers approve copying the template.

- trigger: `workflow_dispatch`
- job policy: `continue-on-error: true`
- permissions: `contents: read`
- timeout: 30 minutes
- artifact upload: screenshot PNG files only
- browser strategy: Playwright-managed Chromium first

This phase proves the runner can install dependencies, install Chromium, bind
the preview server, run browser assertions, upload artifacts, and finish within
the timeout.

### Phase 2: Scheduled Non-blocking Workflow

Allowed after at least several successful manual runs on the selected runner.

- trigger: weekly schedule
- keep `continue-on-error: true`
- keep screenshots as artifacts
- record recurring failures before adding pull-request coverage

This phase detects drift in browser downloads, runner images, and dependency
installation without blocking contributors.

### Phase 3: Pull-request Non-blocking Workflow

Allowed only after scheduled runs are stable.

- trigger: `pull_request`
- keep `continue-on-error: true`
- keep the job out of required checks
- use artifact retention for debugging failed browser runs

This phase gives maintainers signal during review while preserving contributor
velocity.

### Phase 4: Required Merge Gate

Allowed only after an explicit follow-up decision.

Requirements before promotion:

- repeated stable runs across normal pull requests
- clear ownership for Dioxus CLI install failures
- clear ownership for Playwright or browser download failures
- clear ownership for preview selector failures
- predictable runtime and artifact size
- documented rollback path

Until those are true, deterministic Rust and structural preview gates remain the
required checks. In short, deterministic Rust and structural preview gates remain the required checks until an explicit follow-up decision promotes browser smoke.

## Browser Strategy

Default to Playwright-managed Chromium because it is portable across GitHub
hosted runners and controlled by the Playwright package. The workflow may use
an external Chrome executable only when the runner image provides a stable path
and the path is documented through `DIOXUS_UI_BROWSER_EXECUTABLE`.

External Chrome is most appropriate for self-hosted runners or prebuilt CI
images. It should not be the default for portable GitHub-hosted workflow
documentation.

## Cache Policy

Recommended cache boundaries:

- Node dependencies: `actions/setup-node` with npm cache and `package-lock.json`
- Cargo artifacts: optional, keyed by `Cargo.lock`, runner OS, and Rust version
- Playwright browser binaries: optional; do not commit or upload as source
  artifacts

Cache misses must not fail the job. Cache corruption should be handled by
clearing the cache and rerunning the non-blocking workflow.

## Artifact Policy

Upload only screenshot PNG files matching:

```text
dioxus-ui-mobile-browser-preview-*.png
```

Do not upload browser profiles, full Playwright caches, target directories, or
temporary preview server output unless a later debugging workflow explicitly
opts into those artifacts.

Recommended retention starts short, such as 7 days, until artifact usefulness is
proven.

## Failure Ownership

Failures should be classified before escalation:

- dependency install failure: CI/tooling owner
- Dioxus CLI install failure: CI/tooling owner unless caused by upstream breakage
- preview server bind failure: CI/tooling owner
- missing selector or assertion failure: component/runtime owner
- screenshot metadata failure: browser smoke owner until proven to be a preview
  rendering issue
- browser download failure: CI/tooling owner

No failure in phases 1 through 3 should block merges by default.

## Rollback

Rollback from an active workflow to documentation-only or manual-only is allowed
when:

- browser smoke blocks unrelated work
- the runner image changes unexpectedly
- browser download failures repeat
- localhost serving becomes unreliable
- artifacts grow too large or lose debugging value
- ownership is unclear for repeated failures

Rollback should remove or disable active triggers, not remove the script or
documentation.

## Alternatives Considered

### Add Required Browser Gate Immediately

Rejected. The smoke depends on browser installation, local serving, and
rendering behavior. Those are valuable but more fragile than the deterministic
gates.

### Keep Browser Smoke Local Only

Rejected as the long-term state. Local browser smoke is useful, but CI can catch
rendering regressions before release once the job is stable.

### Use External Chrome by Default

Rejected for portable workflow documentation. External Chrome paths vary across
runners and images. It remains a supported variant.

## Consequences

Benefits:

- gives maintainers a clear path to activate browser CI
- avoids surprising required checks
- keeps rollback explicit
- keeps screenshot artifacts useful but bounded

Costs:

- browser CI takes longer to become required
- maintainers must track stability before promotion
- more documentation must be kept in sync with workflow behavior

## Open Questions

- Which runner image should be used for the first active workflow?
- Should Cargo artifacts be cached in the browser workflow, or should the job
  prioritize simplicity?
- What retention period is appropriate for screenshot artifacts after the first
  month?
- How many successful non-blocking runs are enough before pull-request coverage?
