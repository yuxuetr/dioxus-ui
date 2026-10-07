# TODOs

## Progress

- Overall: 17% (1 of 6 tasks)
- Current milestone: M215 (Dioxus 0.8 readiness)
- Current task: M215.2

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0), `docs/archive/TODOs.completed-20261006-m195-m201.md` (M195 to M201, 0.3.0), `docs/archive/TODOs.completed-20261006-m202-m207.md` (M202 to M207, 0.4.0 to 0.4.2), `docs/archive/TODOs.completed-20261007-m208-m212.md` (M208 to M212, 0.5.0), and `docs/archive/TODOs.completed-20261007-m213-m214.md` (M213 to M214, 0.6.0)
- Previous roadmap: `docs/archive/roadmap-20261006-stages-0-10.md`

## Goals

- Follow [the roadmap](docs/roadmap.md) to 1.0. This plan covers Stage 14 (0.7.0, Dioxus 0.8). M215 needs no Dioxus release and starts now; M216 starts when the Stage 14 gate in Deferred exits 0. Stage 15 needs Stage 14 and outside use, so it gets no tasks yet.

## Evidence (measured 2026-10-07 at `v0.6.0`, in a scratch worktree on `dioxus` and `dioxus-ssr` `=0.8.0-alpha.1`)

- Stage 14 gate exits 1: the newest Dioxus is `0.8.0-alpha.1` (2026-07-31), after `0.8.0-alpha.0` (2026-05-19); no release date is announced.
- No source change is needed on the alpha: the lockfile resolves one Dioxus (`0.8.0-alpha.1`); `cargo test --workspace --all-features` passes (all suites, 115 primitives and 333 styled tests); `cargo clippy --workspace --all-targets --all-features -D warnings` is clean, including the site and the Web, Desktop, and Mobile demos; the generated fixture with every template and block passes `cargo check` under `#![deny(warnings)]`.
- The browser checks fail on `dx` 0.8.0-alpha.1 as written: it enables Rust hot-patching unless told `--hot-patch false`, and the fat-binary link fails (`rust-lld: error: unknown file type: .../libdeps-*.a`), while `dx` 0.7.9 takes `--hot-patch` as a bare flag that is off by default. `dx` 0.8 also answers 200 with its build placeholder page before the app is built, so `serveDioxusWeb().ready()` returns early and the failure surfaces as a 30 s `page.goto` timeout instead of the build error. With `--hot-patch false`, `npm run verify:runtime-interactions` passes all 52 fixtures on the alpha.
- Not run on the alpha: the Desktop and Mobile self-tests, `verify:site`, `verify:preview`, and a fullstack hydration check.
- The fullstack hydration check exists only as a manual step: the M206 app (RFC 0075, `docs/release.md` 0.4.2 Publish) was run by hand, and Stage 14's exit names it.
- Five files pin or link Dioxus 0.7 outside the archive, RFCs, and CHANGELOG: `Cargo.toml`, `README.md`, `docs/quality-gates.md`, `docs/workspace.md`, `scripts/generated-fixture-smoke.sh`.
- Users: 67 to 130 downloads per crate, no issues.

## Scope Rules

- A task lands complete: crate, templates, registry, docs, site, and tests together. What cannot meet this is cut, not stubbed.
- M215 changes scripts, examples, and docs only; it needs no crate release.
- The Dioxus 0.8 move is the only breaking change in 0.7.0, with a Migration note in the CHANGELOG.
- No new components or blocks without an issue or a block that needs them.
- Publish 0.7.0 once every task before M216.3 is done (the release owner's direction of 2026-10-07).

## M215 Dioxus 0.8 Readiness

- DONE M215.1 Browser checks on both `dx` lines
  - `serveDioxusWeb` turns hot-patching off in the form the installed `dx` accepts (a bare flag stays off on 0.7; `--hot-patch false` on 0.8), and `ready()` waits until the server answers with the app rather than the CLI's build placeholder, failing with `dx`'s output when the build fails.
  - Exit: `npm run verify:runtime-interactions` passes with `dx` 0.7.9 on the main tree and with `dx` 0.8.0-alpha.1 on a scratch 0.8 tree; a build that fails under `dx` 0.8 makes `ready()` throw with the build error instead of a `page.goto` timeout (reverse-verify by leaving hot-patching on).
  - Done (f005db4): both `dx` lines answer 200 with a placeholder ("dx is not serving a web app") before the first build, so `ready()` now waits for the page that loads `/wasm/`; `dx --version` picks `--hot-patch false` on 0.8 and nothing on 0.7. The screenshot, rendered DOM, and mobile browser smokes dropped their own `dx serve` copies for `serveDioxusWeb`. All four browser checks (`verify:browser-local`) pass on `dx` 0.7.9 and, from a cold `dx` output directory, on `dx` 0.8.0-alpha.1 with Dioxus 0.8.0-alpha.1; with hot-patching left on, `ready()` fails after 5 s with `dx serve could not build` and dx's `Build failed` output.
- TODO M215.2 Fullstack hydration check in the release gate
  - A fullstack example renders Tabs and one more component with generated ids on the server; a script serves it, compares the ids across three requests, hydrates the page in Chromium, and checks that ArrowRight moves focus between tabs. `npm run verify:release` and CI run it; the Deferred entry for it is removed.
  - Exit: the check passes on Dioxus 0.7, and fails when `next_element_id()` is replaced with a process-wide counter (RFC 0075's failing case).
- TODO M215.3 `npm run verify:dioxus-next`
  - A script copies the tree into a scratch worktree, pins `dioxus` and `dioxus-ssr` to the newest 0.8 pre-release on crates.io, and runs the workspace tests, Clippy, the generated fixture smoke, the fullstack check, and the browser interactions with a `dx` of the same version, which it names when it is missing. `docs/release.md` records each run's result.
  - Exit: it passes on `0.8.0-alpha.1`, fails with the compiler error when a template is given a type error (reverse-verify), and leaves the main tree unchanged.

## M216 0.7.0 Dioxus 0.8 (starts when the Stage 14 gate exits 0)

- TODO M216.1 Move to Dioxus 0.8
  - Workspace, `dioxus-ssr`, the generated fixture, examples, site, and the five files that pin or link 0.7 move to the 0.8 release; fix what `verify:dioxus-next` reports on it.
  - Exit: Stage 14's criteria: the release gate, browser checks, Desktop and Mobile self-tests, and the fullstack hydration check pass on Dioxus 0.8.
- TODO M216.2 Prepare 0.7.0
  - CHANGELOG with the Dioxus 0.8 Migration note and any API change 0.8 forced, versions, release gate (with `verify:semver` against `v0.6.0`), publish dry run.
- TODO M216.3 Publish 0.7.0
  - Push, CI, publish in dependency order, annotated tag `v0.7.0`, then build fresh apps in both modes from crates.io on Dioxus 0.8 that use a component, a class override, and a state callback.

## Deferred (re-evaluate when)

- Dioxus 0.8 (Stage 14, M216): when `cargo search dioxus --limit 1 --color never | grep -qE '^dioxus = "0\.8\.[0-9]+"'` exits 0 (a 0.8 release, not a pre-release). Checked 2026-10-07: exits 1 on 0.8.0-alpha.1.
- iOS 27 launch failure (Dioxus 0.7 lacks the UIScene lifecycle): when `gh release view -R DioxusLabs/dioxus --json body -q .body | grep -qi uiscene` exits 0.
- Form state and validation, chart tooltips, swipe gestures, DOM portal, Command fuzzy ranking, editing an Input OTP slot in the middle, and right-to-left Slider, Resizable, and Calendar keys: when `gh issue list -R yuxuetr/dioxus-ui --state all --search "<topic>" --json number -q length` prints more than 0, or a block needs one.
- More blocks: when an issue asks for a screen, by the same command.
- Generating templates from the crate: when `CRATE_ONLY` in `crates/dioxus-shadcn-cli/tests/template_parity.rs` passes 10 entries, or M210 needs a template difference the parity rules cannot express. Checked 2026-10-07: 1 entry.
- A fullstack hydration app in CI: M215.2.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
