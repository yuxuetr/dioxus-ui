# Documentation

This directory defines the architecture and execution plan for `dioxus-ui`.

Read in this order:

1. [Design Overview](design.md)
2. [Roadmap](roadmap.md)
3. [Workspace Specification](workspace.md)
4. [Component API Specification](component-api.md)
5. [Release and Package Strategy](release.md)
6. [RFC 0001: Project Architecture](rfcs/0001-project-architecture.md)
7. [RFC 0002: CLI Registry and Code Generation](rfcs/0002-cli-registry-and-code-generation.md)
8. [RFC 0003: Tailwind Styling Contract](rfcs/0003-tailwind-styling-contract.md)
9. [RFC 0004: Benchmark and CSS Output Strategy](rfcs/0004-benchmark-and-css-output.md)
10. [RFC 0005: Modules and Platform Profiles](rfcs/0005-modules-and-platform-profiles.md)
11. [RFC 0006: Focus and Portal Primitives](rfcs/0006-focus-and-portal-primitives.md)
12. [TODO Plan](../TODOs.md)

## Project Principles

- Documentation before implementation.
- Source-copy workflow before packaged crate workflow.
- Headless primitives before complex styled components.
- Complete Tailwind class tokens in source files.
- Accessibility requirements are part of component behavior, not optional polish.
