# Repository Identity Local Follow-up Map

This map describes local follow-up after maintainers record a repository
identity decision. It is hypothetical until a decision record is committed.

Use this map with:

- [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
- [Repository Identity Decision Record Template](repository-identity-decision-record-template.md)
- [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md)
- [Repository Identity Blocker Handoff](repository-identity-blocker-handoff.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Outcome Map

| Decision | Local Files | Validation | Notes |
| --- | --- | --- | --- |
| `approved` | `Cargo.toml`, crate manifests if they stop inheriting workspace metadata, `docs/repository-identity-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/publish-readiness-coverage-metadata.md`, `docs/publish-readiness-resolution-runbook.md`, `docs/cargo-publish-metadata.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:repository-identity-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:publish-readiness-coverage`; `npm run verify:cargo-publish-metadata`; `npm run verify:release-docs`; `npm run verify:docs` | Move repository identity out of current blockers only after the approved URL, Cargo metadata, and docs agree |
| `blocked` | Decision notes or TODO planning only | `npm run verify:repository-identity-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:docs` | Keep the placeholder URL and record missing owner or remote evidence |
| `deferred` | `TODOs.md`, repository ownership planning docs, optional release ownership notes | `npm run verify:repository-identity-readiness`; `npm run verify:docs`; `git diff --check` | Create a focused ownership milestone before resolving publish readiness |

## Surface-specific Follow-up

| Surface | Possible Approved Follow-up | Possible Deferred Follow-up |
| --- | --- | --- |
| Workspace repository metadata | Replace the placeholder with the approved canonical URL | Keep the placeholder and plan ownership evidence collection |
| Crate manifests | Keep `repository.workspace = true` unless maintainers approve crate-specific URLs | Plan crate-specific metadata review before publish |
| Cargo publish metadata | Move repository identity from unresolved blocker to verified publish metadata | Keep publish metadata marked not ready |
| Publish blocker docs | Remove the placeholder URL from current blockers only after focused gates pass | Keep blocker unresolved and document missing evidence |
| Release docs and quality gates | Document the approved URL and ongoing validation command | Keep read-only placeholder validation wording |
| README and docs-site notes | Update command descriptions so they no longer imply unresolved identity | Link the deferred ownership milestone |

## Common Validation

After any local follow-up, run:

```bash
npm run verify:repository-identity-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
git status --short
```

## Non-goals

This map does not:

- apply repository URL changes
- approve repository ownership
- check remote repository existence
- contact crates.io
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts

In short, it does not apply repository URL changes without maintainer approval.
