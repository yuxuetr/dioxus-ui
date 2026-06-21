# RFC 0002: CLI Registry and Code Generation

- Status: Draft
- Created: 2026-06-21

## Summary

Provide a `dxui` CLI that initializes a project and copies component source from
versioned templates described by a local registry.

## Goals

- `dxui init` prepares a Dioxus project for generated components.
- `dxui add <component>` copies source files into `src/components/ui`.
- Registry metadata declares files, dependencies, and assets.
- Generated code remains readable and easy to edit.

## Non-Goals

- The CLI should not become a full package manager.
- The initial CLI should not rewrite arbitrary user code.
- Remote registries are not required for the first version.

## Commands

```bash
dxui init
dxui list
dxui add button
dxui add dialog
```

## Default Generated Layout

```text
src/components/ui/
├─ button.rs
├─ dialog.rs
└─ mod.rs
assets/
└─ dioxus-ui.css
```

`assets/dioxus-ui.css` is an input stylesheet. For Tailwind CSS v4, the default
content should start with:

```css
@import "tailwindcss";
```

The CLI should not generate a complete compiled Tailwind CSS artifact. The
application build should compile the final CSS after Tailwind scans the user's
source files and generated component templates.

## Registry Metadata

Initial registry fields:

```json
{
  "name": "button",
  "description": "Button component",
  "files": [
    {
      "source": "templates/button.rs",
      "target": "src/components/ui/button.rs"
    }
  ],
  "dependencies": [],
  "assets": []
}
```

## Generation Rules

- Existing user files must not be overwritten without an explicit flag.
- Component dependencies should be installed before the requested component.
- `mod.rs` updates should be deterministic.
- Paths should be configurable after the first working version.
- Generated source should use 2-space indentation to match project preference.

## Validation

Registry validation should check:

- every registry entry has a matching template file
- file targets are relative paths
- component dependencies exist
- component names are unique

## Open Questions

- Should the first CLI support `--overwrite`, or should conflicts fail only?
- Should `dxui init` detect an existing Tailwind v4 setup automatically or only
  create a CSS entry?
- Should generated components use project-local imports or a shared `ui` module?
