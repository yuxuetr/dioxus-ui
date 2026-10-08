# RFC 0080: Page Scripts Without Eval

- Status: Accepted
- Created: 2026-10-08

## Summary

Run the components' page scripts on the web as wasm-bindgen snippets
instead of through `document::eval`, so the components work under a Content
Security Policy without `'unsafe-eval'`. The JavaScript stays as it is; only
how it is loaded and how it talks to Rust change.

## Current State

15 modules start a page script with `document::eval`, 16 call sites with
about 1,200 lines of JavaScript: focus scopes (Dialog, Sheet, Drawer, Alert
Dialog), anchored overlays (Popover, Dropdown, Select, Tooltip, Command),
listboxes, hover cards, dismiss timers, roving groups (Tabs, Toggle Group,
Accordion), and the Checkbox, Input OTP, media query, Menubar, Navigation
Menu, Resizable, Sidebar, Slider, and theme controller scripts. Each CLI
template keeps a copy.

`dioxus-web` 0.7.10 turns the script into a function with
`Function::new_with_args` (`src/document.rs:220`), and `0.8.0-alpha.1` does
the same, so waiting for Dioxus 0.8 does not help. `WebDocument::set_title`,
behind `document::Title`, also goes through eval.

An app that serves `script-src 'self' 'wasm-unsafe-eval'` reported this
(FB-02, FB-13, FB-14): the browser refuses the script, the import that
wasm-bindgen did not mark `catch` throws, and the page's interactions stop
being reliable. Tabs fails on mount, and Dialog, Dropdown, Command, and
Navigation Menu block that app's migration.

Measured on 2026-10-08 with a probe app on Dioxus 0.7.10 and wasm-bindgen
0.2.129, served with that policy as a static release build and by `dx serve`:

- `document::eval` fails with "Evaluating a string as JavaScript violates the
  following Content Security Policy directive"; without the policy it works.
- A function from `#[wasm_bindgen(inline_js = ...)]` runs. The release build
  bundles the snippet into the app's own module, and `dx serve` serves it
  from `'self'`; neither needs an inline script.
- The literal can reach `inline_js` through a `macro_rules!` `tt` fragment.
- Two snippets that each export `run`, imported under the same Rust name
  `run` in two modules, both called the second snippet. Distinct Rust names
  (`#[wasm_bindgen(js_name = run)] fn keys_script(...)`) call each its own.

## Decision

A `script` helper module holds a `component_script!` macro and a `Script`
handle.

```rust
component_script!(roving_group_script = r#"
export async function run(dioxus) {
  const scopeId = await dioxus.recv();
  // the script as before
}
"#);

let mut script = roving_group_script::start();
let _ = script.send(effect_scope_id.as_str());
while let Ok(value) = script.recv::<String>().await { ... }
```

- Each script becomes one exported `async function run(dioxus)` around the
  current body. `dioxus.send` and `dioxus.recv` keep their meaning, so the
  JavaScript changes only where Modal Focus pastes values into the source
  (`__SCOPE_ID__`, `__LOCK_SCROLL__`); it receives them like the others.
- The macro declares a module with the source as a `SOURCE` constant, and on
  `wasm32` the snippet's `run` under the module's own name. `start()` picks
  the transport:
  - `wasm32`: a small channel snippet gives `run` a `dioxus` object. Values
    cross as JSON strings, `recv` resolves in order, and when `run` settles
    the channel closes, so the Rust `recv` returns `EvalError::Finished`, as
    an `Eval` does when its script ends. Dropping the handle detaches the
    callbacks; the script keeps running until its element goes away, as
    today.
  - every other target (Desktop, Mobile, the server): `document::eval` of
    the same source without `export`, followed by `return await run(dioxus);`.
    These have no CSP, and the behavior is today's.
- `Script::send` and `Script::recv` keep `Eval`'s signatures and
  `EvalError`, so component code changes only where a script starts.
- Every script module name is unique in the crate (`<module>_script`), and
  the macro uses it as the Rust import name.

Dependencies: `dioxus-shadcn` adds `serde` on every target and `serde_json`
and `wasm-bindgen` on `wasm32`, all already in the tree through
`dioxus-web`. Copy mode gets a `script.rs` helper template and a `script`
helper entry; each entry whose template starts a script lists it. The copied
helper needs the same three crates, and `dxui add` names the ones the app's
`Cargo.toml` lacks.

The preview app stops using eval itself: its page gets `lang` and the title
from an `index.html` in the web demo instead of `document::Title` and an
eval.

## Alternatives

| Option | Why not |
| --- | --- |
| Rewrite each script with `web-sys` | Turns 1,200 lines of tested JavaScript into Rust, keeps the eval copy for Desktop and Mobile, so every behavior has two implementations, and copy mode needs `web-sys` with a long feature list |
| `MountedData` (`set_focus`, `get_client_rect`) | Cross-platform, but covers focus and geometry only: no document listeners, `MutationObserver`, `matchMedia`, storage, or pointer capture |
| Implement Dioxus's `Evaluator` for `Eval::new` | Its signature names `serde_json::Value`, and `Eval::new` needs a `GenerationalBox` owner that Dioxus re-exports only as hidden API; copy mode would depend on both |
| `#[wasm_bindgen(module = "/path.js")]` | The path is relative to the app's crate root, so copied components would need the CLI to place JavaScript files there |
| Ask apps to allow `'unsafe-eval'` | The reporting app's owner rejected it; it lets any injected string run as code |

## Impact

- No public API change; the release is a patch (0.6.1).
- `dioxus-shadcn` builds for `wasm32` with `wasm-bindgen` and `serde_json`,
  which Dioxus Web already brings.
- Copy mode: re-copied interactive components need `serde`, and on
  `wasm32` `serde_json` and `wasm-bindgen`, in the app's `Cargo.toml`.
- What the components need from a CSP: `script-src 'self'
  'wasm-unsafe-eval'`. The theme init script is inline, so an app that uses
  it allows it by hash or nonce. App code that calls `document::eval` or
  renders `document::Title` still needs `'unsafe-eval'`; that is Dioxus's.

## Validation

- A strict-CSP browser run serves the web preview with `script-src 'self'
  'wasm-unsafe-eval'`, allows the served page's own inline scripts by hash,
  and fails on any `securitypolicyviolation` or page error while it runs the
  interaction checks. It fails before the change and passes after it; one
  `document::eval` put back into a component fails it again.
- The existing browser checks, the Desktop interaction self-test (the eval
  transport), the fullstack hydration check, the template parity test, the
  generated fixture smoke, Clippy, and `verify:semver` against `v0.6.0`
  pass.
