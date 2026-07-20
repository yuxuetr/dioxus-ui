# Documentation

This directory defines the architecture and execution plan for `dioxus-ui`.

Read in this order:

1. [Design Overview](design.md)
2. [Roadmap](roadmap.md)
3. [Workspace Specification](workspace.md)
4. [Component API Specification](component-api.md)
5. [Release and Package Strategy](release.md)
6. [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)
7. [Quality Gates](quality-gates.md)
8. [Component Catalog](components/README.md)
9. [Documentation Site Plan](site.md)
10. [CI Browser Smoke Guide](ci-browser-smoke.md)
11. [CI Browser Workflow Template](ci-browser-workflow-template.md)
12. [CI Browser Workflow Metadata](ci-browser-workflow-metadata.md)
13. [Browser Artifact Policy Metadata](browser-artifact-policy-metadata.md)
14. [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)
15. [Cargo Publish Metadata](cargo-publish-metadata.md)
16. [Publish Readiness Blockers](publish-readiness-blockers.md)
17. [Changelog Metadata](changelog-metadata.md)
18. [Release Notes Readiness Metadata](release-notes-readiness-metadata.md)
19. [License Readiness Metadata](license-readiness-metadata.md)
20. [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md)
21. [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
22. [CLI Template Packaging Readiness Metadata](cli-template-packaging-readiness-metadata.md)
23. [Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md)
24. [Publish Readiness Coverage Metadata](publish-readiness-coverage-metadata.md)
25. [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
26. [Publish Order Metadata](publish-order-metadata.md)
27. [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md)
28. [Runtime Adapter Plan](components/runtime-adapters.md)
29. [Runtime Renderer Verification Matrix](components/runtime-renderer-verification.md)
30. [Web Runtime Verification Harness](components/runtime-web-verification-harness.md)
31. [Desktop and Mobile Runtime Verification Strategy](components/runtime-desktop-mobile-verification.md)
32. [Runtime Implementation Milestone Seeds](components/runtime-implementation-milestones.md)
33. [Web Runtime Adapter Module Boundaries](components/runtime-web-adapter-boundaries.md)
34. [Rendered Component Verification](components/rendered-component-verification.md)
35. [Browser DOM Component Verification](components/browser-dom-component-verification.md)
36. [Runtime Interaction Verification](components/runtime-interaction-verification.md)
37. [Web Preview Screenshot Smoke](components/web-preview-screenshot-smoke.md)
38. [Browser Smoke Aggregate](components/browser-smoke-aggregate.md)
39. [Release Screenshot Review Checklist](components/release-screenshot-review-checklist.md)
40. [Screenshot Artifact Retention](components/screenshot-artifact-retention.md)
41. [Release Screenshot Review Notes Template](components/release-screenshot-review-notes-template.md)
42. [Release Candidate Browser Review Runbook](components/release-candidate-browser-review-runbook.md)
43. [RFC 0001: Project Architecture](rfcs/0001-project-architecture.md)
44. [RFC 0002: CLI Registry and Code Generation](rfcs/0002-cli-registry-and-code-generation.md)
45. [RFC 0003: Tailwind Styling Contract](rfcs/0003-tailwind-styling-contract.md)
46. [RFC 0004: Benchmark and CSS Output Strategy](rfcs/0004-benchmark-and-css-output.md)
47. [RFC 0005: Modules and Platform Profiles](rfcs/0005-modules-and-platform-profiles.md)
48. [RFC 0006: Focus and Portal Primitives](rfcs/0006-focus-and-portal-primitives.md)
49. [RFC 0007: Keyboard Navigation Primitives](rfcs/0007-keyboard-navigation-primitives.md)
50. [RFC 0008: Overlay Positioning and Portals](rfcs/0008-overlay-positioning-and-portals.md)
51. [RFC 0009: CI Browser Workflow Activation](rfcs/0009-ci-browser-workflow-activation.md)
52. [TODO Plan](../TODOs.md)

## Project Principles

- Documentation before implementation.
- Source-copy workflow before packaged crate workflow.
- Headless primitives before complex styled components.
- Complete Tailwind class tokens in source files.
- Accessibility requirements are part of component behavior, not optional polish.
- Rendered preview targets should be deterministic metadata before browser
  screenshot claims.
