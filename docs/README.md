# Documentation

This directory defines the architecture and execution plan for `dioxus-ui`.

Read in this order:

1. [Design Overview](design.md)
2. [Roadmap](roadmap.md)
3. [Workspace Specification](workspace.md)
4. [Component API Specification](component-api.md)
5. [Release and Package Strategy](release.md)
6. [Quality Gates](quality-gates.md)
7. [Component Catalog](components/README.md)
8. [Documentation Site Plan](site.md)
9. [CI Browser Smoke Guide](ci-browser-smoke.md)
10. [CI Browser Workflow Template](ci-browser-workflow-template.md)
11. [CI Browser Workflow Metadata](ci-browser-workflow-metadata.md)
12. [Browser Artifact Policy Metadata](browser-artifact-policy-metadata.md)
13. [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)
14. [Cargo Publish Metadata](cargo-publish-metadata.md)
15. [Runtime Adapter Plan](components/runtime-adapters.md)
16. [Runtime Renderer Verification Matrix](components/runtime-renderer-verification.md)
17. [Web Runtime Verification Harness](components/runtime-web-verification-harness.md)
18. [Desktop and Mobile Runtime Verification Strategy](components/runtime-desktop-mobile-verification.md)
19. [Runtime Implementation Milestone Seeds](components/runtime-implementation-milestones.md)
20. [Web Runtime Adapter Module Boundaries](components/runtime-web-adapter-boundaries.md)
21. [RFC 0001: Project Architecture](rfcs/0001-project-architecture.md)
22. [RFC 0002: CLI Registry and Code Generation](rfcs/0002-cli-registry-and-code-generation.md)
23. [RFC 0003: Tailwind Styling Contract](rfcs/0003-tailwind-styling-contract.md)
24. [RFC 0004: Benchmark and CSS Output Strategy](rfcs/0004-benchmark-and-css-output.md)
25. [RFC 0005: Modules and Platform Profiles](rfcs/0005-modules-and-platform-profiles.md)
26. [RFC 0006: Focus and Portal Primitives](rfcs/0006-focus-and-portal-primitives.md)
27. [RFC 0007: Keyboard Navigation Primitives](rfcs/0007-keyboard-navigation-primitives.md)
28. [RFC 0008: Overlay Positioning and Portals](rfcs/0008-overlay-positioning-and-portals.md)
29. [RFC 0009: CI Browser Workflow Activation](rfcs/0009-ci-browser-workflow-activation.md)
30. [TODO Plan](../TODOs.md)

## Project Principles

- Documentation before implementation.
- Source-copy workflow before packaged crate workflow.
- Headless primitives before complex styled components.
- Complete Tailwind class tokens in source files.
- Accessibility requirements are part of component behavior, not optional polish.
